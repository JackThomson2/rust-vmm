use std::hint::spin_loop;
use std::{ffi::CString, ptr::null_mut};

use kvm_bindings::*;
use libc::*;

use crate::drivers::Drivers;
use crate::kvm_regs::*;
use crate::libc_macros::get_os_error;
use crate::mmio::handle_mmio;
use crate::portio::handle_pio;
use crate::{check_libc, check_libc_no_print};

use virtio::virtqueue::VirtQueue;

use eyre::{eyre, Result};

pub struct Vm {
    kvmfd: c_int,
    vmfd: c_int,
    memory: u64,
    memory_amount: usize,
    vcpufd: c_int,
    run: *mut kvm_run,
    drivers: Drivers,
}

const FILE_OFFSET: usize = 0x5000;
const PHYSICAL_SLOT: u64 = 0x1000;

unsafe fn load_vm_memory(memory_amount: usize) -> Result<u64> {
    let location = mmap(
        null_mut(),
        memory_amount,
        PROT_READ | PROT_WRITE,
        MAP_SHARED | MAP_ANONYMOUS,
        -1,
        0,
    );
    if location == MAP_FAILED {
        return Err(eyre!("Error allocating memory"));
    }

    Ok(location as u64)
}

unsafe fn read_file_into_mem(
    file_location: &str,
    memory_location: u64,
    _memory_amount: usize,
) -> Result<()> {
    let file_loc = CString::new(file_location)?;
    let read_flag = CString::new("r")?;

    let file_ptr = fopen(file_loc.as_ptr(), read_flag.as_ptr());
    if file_ptr.is_null() {
        return Err(eyre!("Error reading file {file_location}"));
    }
    fseek(file_ptr, 0, SEEK_END);
    let file_size = ftell(file_ptr);
    rewind(file_ptr);
    println!("File size is: {file_size}. Writing at pos: {FILE_OFFSET:X}");

    let read_location = (memory_location as *mut u8).add(FILE_OFFSET);

    let loaded = fread(
        read_location as *mut c_void,
        1,
        file_size as usize,
        file_ptr,
    );
    println!("Read {loaded} bytes into the vm");

    fclose(file_ptr);

    Ok(())
}

unsafe fn setup_kvm_data_segment_long_mode(segment: &mut kvm_segment) {
    segment.base = 0x0;
    segment.limit = 0x0;
    segment.selector = 0x10;
    segment.type_ = 2;
    segment.present = 1;
    segment.dpl = 0;
    segment.db = 0;
    segment.s = 1;
    segment.l = 0;
    segment.g = 0;
}

unsafe fn setup_vcpu_registers(vcpu: c_int, memory_location: u64, memory_amount: u64) -> Result<()> {
    let mut regs = kvm_regs::default();
    check_libc!(ioctl(vcpu, KVM_GET_REGS(), &mut regs), "KVM GET REGS");
    regs.rip = 0x0;
    regs.rflags = 0x2;
    check_libc!(ioctl(vcpu, KVM_SET_REGS(), &mut regs), "KVM SET REGS");

    let mut sregs = kvm_sregs::default();
    check_libc!(ioctl(vcpu, KVM_GET_SREGS(), &mut sregs), "KVM GET SREGS");

    sregs.cs.base = 0x0;
    sregs.cs.limit = 0x0;
    sregs.cs.selector = 0x8;
    sregs.cs.type_ = 10;
    sregs.cs.present = 1;
    sregs.cs.dpl = 0;
    sregs.cs.db = 0;
    sregs.cs.s = 1;
    sregs.cs.l = 1;
    sregs.cs.g = 0;

    setup_kvm_data_segment_long_mode(&mut sregs.ds);
    setup_kvm_data_segment_long_mode(&mut sregs.ss);
    setup_kvm_data_segment_long_mode(&mut sregs.fs);
    setup_kvm_data_segment_long_mode(&mut sregs.gs);
    setup_kvm_data_segment_long_mode(&mut sregs.es);

    setup_long_4level_paging(memory_location, memory_amount);

    sregs.cr3 = PHYSICAL_SLOT;
    sregs.cr0 |= CR0_PG | CR0_PE;
    sregs.cr4 = CR4_PAE;
    sregs.efer = EFER_LMA | EFER_LME;

    check_libc!(ioctl(vcpu, KVM_SET_SREGS(), &mut sregs), "KVM SET SREGS");

    Ok(())
}

// Adapted form https://github.com/johannst/mini-kvm-rs/blob/main/examples/long_mode.rs#L63
unsafe fn setup_long_4level_paging(memory_location: u64, memory_amount: u64) {
    let memory_ptr = memory_location as *mut u8;
    let w = |offset: u64, val: u64| {
        let val = val + PHYSICAL_SLOT;
        let bytes = val.to_le_bytes();
        let offset_ptr = memory_ptr.add(offset as usize);

        let sliced_segment = std::slice::from_raw_parts_mut(offset_ptr, bytes.len());
        sliced_segment.copy_from_slice(&bytes)
    };

    w(0x0000, PAGE_ENTRY_PRESENT | PAGE_ENTRY_RW | 0x1000);
    w(0x1000, PAGE_ENTRY_PRESENT | PAGE_ENTRY_RW | 0x2000);
    w(0x2000, PAGE_ENTRY_PRESENT | PAGE_ENTRY_RW | 0x3000);

    for i in 0..100 {
        let loc = 0x3000 + (i * 8);
        let pos = (i * 0x1000) + FILE_OFFSET as u64;
        let virt_loc = i * 0x10000;

        println!("Mapping at loc: {loc:X} and pos: {pos:X}. Virual address: {virt_loc:0X}");

        w(loc, PAGE_ENTRY_PRESENT | PAGE_ENTRY_RW | pos);
    }

    for i in 0..20 {
        let real_i = i + 100;
        let loc = 0x3000 + (real_i * 8);

        let pos = (i * 0x1000) + memory_amount;
        let virt_loc = real_i * 0x10000;

        println!("Mapping VIRTIO at loc: {loc:X} and pos: {pos:X}. Virual address: {virt_loc:0X}");
        w(loc, PAGE_ENTRY_PRESENT | PAGE_ENTRY_RW | pos);
    }

    // PTE[0] maps Virt [0x0000:0x0fff] -> Phys [0x4000:0x4fff].
    // w(0x3000, PAGE_ENTRY_PRESENT | PAGE_ENTRY_RW | 0x4000);
    // // PTE[1] maps Virt [0x1000:0x1fff] -> Phys [0x5000:0x5fff].
    // w(0x3008, PAGE_ENTRY_PRESENT | PAGE_ENTRY_RW | 0x5000);
    // // PTE[2] maps Virt [0x2000:0x2fff] -> Phys [0x6000:0x6fff].
    // w(0x3010, PAGE_ENTRY_PRESENT | PAGE_ENTRY_RW | 0x6000);
    // // PTE[2] maps Virt [0x4000:0x3fff] -> Phys [0x7000:0x7fff].
    // w(0x3018, PAGE_ENTRY_PRESENT | PAGE_ENTRY_RW | 0x7000);

    // // Page entries into MMIO locations
    // // PTE[4] maps Virt [0x4000:0x4fff] -> Phys [0x8000:0x8fff]
    // w(0x3020, PAGE_ENTRY_PRESENT | PAGE_ENTRY_RW | 0x8000);
    // // PTE[5] maps Virt [0x5000:0x5fff] -> Phys [0x9000:0x9fff]
    // w(0x3028, PAGE_ENTRY_PRESENT | PAGE_ENTRY_RW | 0x9000);
    // // PTE[6] maps Virt [0x6000:0x6fff] -> Phys [0xa000:0xafff]
    // w(0x3030, PAGE_ENTRY_PRESENT | PAGE_ENTRY_RW | 0xA000);

    // Map into our MMIO region!
    // w(0x3020, PAGE_ENTRY_PRESENT | PAGE_ENTRY_RW | 0x8000);
    // w(0x3028, PAGE_ENTRY_PRESENT | PAGE_ENTRY_RW | 0x9000);
}

unsafe fn load_memor_addr(memory_location: u64, location: u64) {
    let memory_ptr = memory_location as *mut u8;
    let offset_ptr = memory_ptr.add(location as usize + FILE_OFFSET);

    println!("File loc: {}", offset_ptr.read_volatile());
}

unsafe fn setup_real_mode(vcpu: c_int) -> Result<()> {
    let mut regs = kvm_regs::default();
    check_libc!(ioctl(vcpu, KVM_GET_REGS(), &mut regs), "KVM GET REGS");
    regs.rip = 0x0;
    regs.rflags = 0x0;
    check_libc!(ioctl(vcpu, KVM_SET_REGS(), &mut regs), "KVM SET REGS");

    let mut sregs = kvm_sregs::default();
    check_libc!(ioctl(vcpu, KVM_GET_SREGS(), &mut sregs), "KVM GET SREGS");
    sregs.cs.base = 0x0000;
    sregs.cs.selector = 0x0;

    check_libc!(ioctl(vcpu, KVM_SET_SREGS(), &mut sregs), "KVM SET SREGS");

    Ok(())
}

impl Vm {
    pub unsafe fn create(memory_amount: usize) -> Result<Self> {
        let kvmfd = {
            let kvm_location = CString::new("/dev/kvm")?;
            open(kvm_location.as_ptr(), O_RDWR | O_CLOEXEC)
        };
        check_libc!(kvmfd, "Opening /dev/kvm");

        let vmfd = ioctl(kvmfd, KVM_CREATE_VM(), 0);
        check_libc!(vmfd, "Calling KVM create vm");

        let memory = load_vm_memory(memory_amount)?;

        let memory_region = kvm_userspace_memory_region {
            slot: 0,
            flags: 0,
            guest_phys_addr: PHYSICAL_SLOT,
            memory_size: memory_amount as u64,
            userspace_addr: memory,
        };

        check_libc!(
            ioctl(vmfd, KVM_SET_USER_MEMORY_REGION(), &memory_region),
            "Setting the kvm user memory region"
        );

        let vcpufd = ioctl(vmfd, KVM_CREATE_VCPU(), 0);
        check_libc!(vcpufd, "Createing the virtual cpu");

        let run_struct_sz = ioctl(kvmfd, KVM_GET_VCPU_MMAP_SIZE(), null_mut() as *mut u32);
        check_libc!(run_struct_sz, "Get VCPU MMAP Size");

        setup_vcpu_registers(vcpufd, memory, memory_amount as u64)?;
        // setup_real_mode(vcpufd)?;

        let run = mmap(
            null_mut(),
            run_struct_sz as usize,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            vcpufd,
            0,
        );

        if run.is_null() {
            return Err(eyre!("Error mmap vcpu"));
        }

        Ok(Self {
            kvmfd,
            vmfd,
            memory,
            vcpufd,
            memory_amount,
            run: run as *mut kvm_run,
            drivers: Drivers::new_drivers(memory as *mut u8, memory_amount as u64),
        })
    }

    pub unsafe fn load_file(&self, file_location: &str) -> Result<()> {
        read_file_into_mem(file_location, self.memory, self.memory_amount)
    }

    unsafe fn get_run_ref(&self) -> &mut kvm_run {
        self.run.as_mut().unwrap()
    }

    // Horrible hack for now should probably fix this
    unsafe fn get_driver_ref(&self) -> &mut Drivers {
        (((&self.drivers) as *const _) as *mut Drivers)
            .as_mut()
            .unwrap()
    }

    pub unsafe fn dump_stack(&self) {
        let stack_ptr = 0x25000;
        for i in 0..100 {
            load_memor_addr(self.memory, stack_ptr - i);
        }

    }

    pub unsafe fn run(&mut self) -> Result<()> {
        println!("\n");

        let run_ref = self.get_run_ref();

        loop {
            let kvm_ret = ioctl(self.vcpufd, KVM_RUN(), null_mut() as *mut u32);
            check_libc!(kvm_ret, "Error running cpu");

            match run_ref.exit_reason {
                KVM_EXIT_HLT => {
                    spin_loop();
                }
                KVM_EXIT_SHUTDOWN => {
                    break;
                },
                KVM_EXIT_IO => {
                    handle_pio(run_ref, self.get_driver_ref(), self.memory_amount as u32);
                }
                KVM_EXIT_MMIO => {
                    handle_mmio(run_ref, self.get_driver_ref(), self.memory_amount as u64);
                }
                _ => {
                    println!("Unknown exit reason {}", run_ref.exit_reason);
                }
            }
        }


        // load_memor_addr(self.memory, regs.rsp);
        // self.dump_stack();

        println!("\n");
        println!("KVM has exited closing. Errors: {}", get_os_error());

        let mut regs = kvm_regs::default();
        check_libc!(ioctl(self.vcpufd, KVM_GET_REGS(), &mut regs), "KVM GET REGS");

        println!("Stats {:#?}", regs);
        println!("Flags: {}", run_ref.flags);
        println!("Line that errored: {:0x}", regs.rip);

        Ok(())
    }
}
