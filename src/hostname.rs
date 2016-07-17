extern crate libc;

extern {
    pub fn gethostname(name: *mut libc::c_char, size: libc::size_t) -> libc::c_int;
}

pub fn get_host_by_name(hostname: String) -> String {
    //Convert String as a Vec<u8>
    let mut ptr = hostname.into_bytes();

    // TODO : match err
    let err = unsafe {
        gethostname(ptr.as_mut_ptr() as *mut i8, ptr.len())
    };
    
    //Convert Vec<u8> as a String
     String::from_utf8(ptr).unwrap()
}
