pub unsafe fn build_static_list<S, I: Sized>(ptr_loc: *mut I, list: S) -> usize
where
    S: Iterator<Item = I>
{
    let mut cntr = 0;

    for (idx, item) in list.enumerate() {
        let write_loc = ptr_loc.add(idx);
        write_loc.write_volatile(item);
        cntr += 1;
    }

    ptr_loc.add(cntr + 1) as usize
}

pub unsafe fn box_object<S: Sized + Clone>(ptr_loc: *mut S, item: S) -> usize {
    ptr_loc.write(item);

    ptr_loc.add(1) as usize
}

pub unsafe fn box_object_volatile<S: Sized + Clone>(ptr_loc: *mut S, item: S) -> usize {
    ptr_loc.write_volatile(item);

    ptr_loc.add(1) as usize
}
