use std::{io, os::fd::RawFd};

const _MAX_E: usize = 64;

pub struct _Task {
    fd: RawFd,
    poll: fn(&mut _Task),
    buf: [u8; 1024],
    len: usize,
}

pub fn _nonblock(fd: RawFd) -> io::Result<()> {
    unsafe {
        let flag = libc::fcntl(fd, libc::F_SETFL, 0);
        if flag < 0 { return Err(io::Error::last_os_error());}
        let res = libc::fcntl(fd, libc::F_SETFL, flag | libc::O_NONBLOCK);
        if res < 0 { return Err(io::Error::last_os_error());}
        Ok(())
    }
}

pub fn _executor(kq: RawFd) {
    let mut evs: [libc::kevent; _MAX_E] = unsafe { std::mem::zeroed() };
    let nev = unsafe {
        libc::kevent(
            kq,
            std::ptr::null(),0,
            evs.as_mut_ptr(),0,
            std::ptr::null(),
        )
    };
    if nev < 0 {
        std::process::exit(1)
    }
    for i in 0..nev as usize {
        let event = evs[i];
        let tsf = event.udata as *mut _Task;
        if tsf.is_null() {
            continue;
        }
        unsafe {
            let task = &mut *tsf;
            (task.poll)(task)
        }
    }
}

pub fn _client_task(task: &mut _Task) {
    let read = unsafe {
        libc::read(
            task.fd,
            &mut task.buf as *mut _ as *mut libc::c_void,
            task.len,
        )
    };
    if read < 0 {
        unsafe { libc::close(task.fd) };
        std::process::exit(1)
    }
    unsafe {
        libc::write(
            task.fd,
            &mut task.buf as *mut _ as *mut libc::c_void,
            task.len,
        )
    };
}

pub fn _accept_task(task: &mut _Task, mut tasks: Box<Vec<_Task>>) {
    unsafe {
        let mut client: libc::sockaddr_in = std::mem::zeroed();
        let mut addrlen = std::mem::size_of::<libc::sockaddr>() as libc::socklen_t;
        let clientfd = libc::accept(task.fd, &mut client as *mut _ as *mut libc::sockaddr,  &mut addrlen);
        if clientfd < 0 { std::process::exit(1);}
        let mut client = _Task {
            fd: clientfd, poll:_client_task, buf:[0u8; 1024], len:1024
        };

        let mut ev:libc::kevent = std::mem::zeroed();
        ev.ident = clientfd as libc::uintptr_t;
        ev.filter = libc::EVFILT_READ;
        ev.flags = libc::EV_ADD | libc::EV_ENABLE;
        ev.fflags = 0; ev.data = 0;
        ev.udata = &mut client as *mut _ as *mut libc::c_void;

        if libc::kevent(task.fd, &ev, 1, std::ptr::null_mut(), 0, std::ptr::null_mut()) < 0 {
            std::process::exit(1);
        }
        tasks.push(client);
    }
}