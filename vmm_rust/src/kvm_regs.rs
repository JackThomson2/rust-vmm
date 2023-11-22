use kvm_bindings::*;

ioctl_io_nr!(KVM_CREATE_VM, KVMIO, 0x01);
ioctl_io_nr!(KVM_GET_VCPU_MMAP_SIZE, KVMIO, 0x04);
ioctl_io_nr!(KVM_CREATE_VCPU, KVMIO, 0x41);

ioctl_iow_nr!(KVM_SET_USER_MEMORY_REGION, KVMIO, 0x46, kvm_userspace_memory_region);

ioctl_io_nr!(KVM_RUN, KVMIO, 0x80);
ioctl_ior_nr!(KVM_GET_REGS, KVMIO, 0x81, kvm_regs);
ioctl_iow_nr!(KVM_SET_REGS, KVMIO, 0x82, kvm_regs);
ioctl_ior_nr!(KVM_GET_SREGS, KVMIO, 0x83, kvm_sregs);
ioctl_iow_nr!(KVM_SET_SREGS, KVMIO, 0x84, kvm_sregs);

/// Page entry present.
pub const PAGE_ENTRY_PRESENT: u64 = 1 << 0;
/// Page region read/write.
///
/// If set, region reference by paging entry is writeable.
pub const PAGE_ENTRY_RW: u64 = 1 << 1;

pub const CR0_PG: u64 = 1 << 31;

/// Enables `protected mode` when set and `real-address mode` when cleared. This enables
/// `segment-level protection` not paging.
pub const CR0_PE: u64 = 1 << 0;
/// Monitor Coprocessor.
pub const CR0_MP: u64 = 1 << 1;

pub const CR4_PAE: u64 = 1 << 5;


/// Long Mode Enable.
///
/// When set enables long mode operations.
pub const EFER_LME: u64 = 1 << 8;
/// Long Mode Active (readonly).
///
/// When set indicates long mode is active.
pub const EFER_LMA: u64 = 1 << 10;


pub const ACCESS_PRESENT: u8 = 1 << 7;
pub const ACCESS_NOT_SYS: u8 = 1 << 4;
pub const ACCESS_EXEC: u8 = 1 << 3;
pub const ACCESS_DC: u8 = 1 << 2;
pub const ACCESS_RW: u8 = 1 << 1;
pub const ACCESS_ACCESSED: u8 = 1 << 0;
