use std::{io, mem, net::Ipv4Addr};

/**
 * @brief
 * broadcast signal from a peer to enable lurking
 * peers to track & sync accordingly
 */
pub fn _broadcast(port: u16) -> io::Result<()> {
    unsafe {
        let mut adr :libc::sockaddr_in =  mem::zeroed();
        let adl = mem::size_of::<libc::sockaddr>() as libc::socklen_t;
        let cfd = libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0);
        if cfd < 0 { return Err(io::Error::last_os_error());}

        adr.sin_family = libc::AF_INET as u8;
        adr.sin_port = port.to_be();
        adr.sin_addr.s_addr = u32::from(Ipv4Addr::new(255,255,255, 255));
        adr.sin_len = mem::size_of::<libc::sockaddr_in>() as u8;
        adr.sin_zero = [0;8];

        let cbd = libc::bind(cfd , &mut adr as *mut _ as  *mut libc::sockaddr, adl);
        if cbd < 0 {return Err(io::Error::last_os_error());}
        if libc::listen(cfd, 10) < 0  {return Err(io::Error::last_os_error())}
        loop {
            /* sendto & recv from calls for UDP broadcasting */
        }
    }
}

pub fn _listen_for_peers() {

}