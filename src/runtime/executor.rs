use std::{os::fd::RawFd};


const _MAX_E : usize = 64;

struct _Task {
    fd: RawFd,
    poll: fn(&mut _Task),
    buf:[u8; 1024],
    len: usize
}

pub fn _executor(kq: RawFd){
    let mut evs:[libc::kevent; _MAX_E] = unsafe {std::mem::zeroed()};
    let nev = unsafe {
        libc::kevent(kq, std::ptr::null(),0, evs.as_mut_ptr(), 0, std::ptr::null())};
    if nev < 0 { std::process::exit(1)}
    for i in 0..nev as usize {
        let event = evs[i];
        let tsf = event.udata as *mut _Task;

        unsafe {
            let task = &mut *tsf;
            (task.poll)(task)
        }
    }
}