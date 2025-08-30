use std::{io, mem, net::Ipv4Addr};

pub fn _broadcast(port: u16) -> io::Result<()> {
    unsafe {
        let mut adr: libc::sockaddr_in = mem::zeroed();
        let cfd = libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0);
        if cfd < 0 {
            return Err(io::Error::last_os_error());
        }

        let cast: libc::c_int = 1;
        libc::setsockopt(
            cfd,
            libc::SOL_SOCKET,
            libc::SO_BROADCAST,
            &cast as *const _ as *const libc::c_void,
            mem::size_of_val(&cast) as libc::socklen_t,
        );

        adr.sin_family = libc::AF_INET as u8;
        adr.sin_port = port.to_be();
        adr.sin_addr.s_addr = u32::from(Ipv4Addr::new(255, 255, 255, 255)).to_be();
        adr.sin_len = mem::size_of::<libc::sockaddr_in>() as u8;
        adr.sin_zero = [0; 8];

        let msg = b"peer broadcast";
        let bcast = libc::sendto(
            cfd,
            &msg as *const _ as *const libc::c_void,
            msg.len(),
            0,
            &adr as *const _ as *const libc::sockaddr,
            mem::size_of::<libc::sockaddr>() as libc::socklen_t,
        );
        if bcast < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

pub fn _listen_for_peers() {}
