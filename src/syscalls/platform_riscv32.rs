#[inline(always)]
pub unsafe fn yieldk() {
    /* TODO: Stop yielding */
    asm! (
            "li    a0, 0
            ecall"
            :
            :
            : "memory", "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7",
            "t0", "t1", "t2", "t3", "t4", "t5", "t6", "ra"
            : "volatile");
}

#[inline(always)]
pub unsafe fn subscribe(
    major: usize,
    minor: usize,
    cb: *const unsafe extern "C" fn(usize, usize, usize, usize),
    ud: usize,
) -> isize {
    let res;
    asm!("li    a0, 1
          ecall"
         : "={x10}" (res)
         : "{x11}" (major), "{x12}" (minor), "{x13}" (cb), "{x14}" (ud)
         : "memory"
         : "volatile" );
    res
}

#[inline(always)]
pub unsafe fn command(major: usize, minor: usize, arg1: usize, arg2: usize) -> isize {
    let res;
    asm!("li    a0, 2
          ecall"
         : "={x10}" (res)
         : "{x11}" (major), "{x12}" (minor), "{x13}" (arg1), "{x14}" (arg2)
         : "memory"
         : "volatile");
    res
}

#[inline(always)]
pub unsafe fn command1(major: usize, minor: usize, arg: usize) -> isize {
    let res;
    asm!("li    a0, 2
          ecall"
         : "={x10}" (res)
         : "{x11}" (major), "{x12}" (minor), "{x13}" (arg)
         : "memory"
         : "volatile");
    res
}

#[inline(always)]
pub unsafe fn allow(major: usize, minor: usize, slice: *mut u8, len: usize) -> isize {
    let res;
    asm!("li    a0, 3
          ecall"
         : "={x10}" (res)
         : "{x11}" (major), "{x12}" (minor), "{x13}" (slice), "{x14}" (len)
         : "memory"
         : "volatile");
    res
}

#[inline(always)]
pub unsafe fn memop(major: u32, arg1: usize) -> isize {
    let res;
    asm!("li    a0, 4
          ecall"
         : "={x10}" (res)
         : "{x11}" (major), "{x12}" (arg1)
         : "memory"
         : "volatile");
    res
}