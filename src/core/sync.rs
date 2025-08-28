use std::{io, net::Ipv4Addr};

pub fn _net(port: u16) -> io::Result<()> {
    unsafe {
        let mut addr: libc::sockaddr_in = std::mem::zeroed();
        let mut addl = std::mem::size_of::<libc::sockaddr>() as libc::socklen_t;
        #[cfg(any(target_os = "macos", target_os = "netbsd", target_os = "freebsd"))]{
            addr.sin_len = std::mem::size_of::<libc::sockaddr>() as u8;
            addr.sin_family = libc::AF_INET as u8;
        }

        #[cfg(any(target_os = "linux"))]{
            addr.sin_len = std::mem::size_of::<libc::sockaddr>() as u16;
            addr.sin_family = libc::AF_INET as u16;
        }
        let sockfd = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);
        if sockfd < 0 {
            return Err(io::Error::last_os_error());
        }

        addr.sin_port = port.to_be();
        addr.sin_addr.s_addr = u32::from(Ipv4Addr::new(127, 0, 0, 1));

        let fbd = libc::bind(sockfd, &mut addr as *mut _ as *mut libc::sockaddr, addl);
        if fbd < 0 {
            return Err(io::Error::last_os_error());
        }
        if libc::listen(sockfd, 0x0A as libc::c_int) < 0 {
            return Err(io::Error::last_os_error());
        }

        let mut buf = [0u8; 1024];
        loop {
            let cfd = libc::accept(
                sockfd,
                &mut addr as *mut _ as *mut libc::sockaddr,
                &mut addl,
            );
            if cfd < 0 {
                return Err(io::Error::last_os_error());
            }
            let n = libc::send(
                cfd as libc::c_int,
                &mut buf as *mut _ as *mut libc::c_void,
                buf.len(), 0,
            );

            if n <= 0 {
                return Err(io::Error::last_os_error());
            }

            libc::recv(cfd, &mut buf as *mut _ as *mut libc::c_void, n as usize, 0);

            libc::close(cfd);
        }
    }
}
