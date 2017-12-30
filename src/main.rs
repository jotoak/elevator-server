extern crate libc;

use std::ffi::CString;

/// This is a opaque rust equivalent for comedi_t inside libcomedi.h
enum comedi_t {}

#[link(name = "comedi")]
extern "C" {
    fn comedi_open(interface_name: *const libc::c_char) -> *const comedi_t;
    fn comedi_dio_write(it: *const comedi_t, subd: libc::c_uint, chan: libc::c_uint, bit: libc::c_uint) -> libc::c_int;
    fn comedi_dio_read(it: *const comedi_t, subd: libc::c_uint, chan: libc::c_uint, bit: *mut libc::c_uint) -> libc::c_int;
    fn comedi_data_write(it: *const comedi_t, subd: libc::c_uint, chan: libc::c_uint, range: libc::c_uint, aref: libc::c_uint, data: libc::c_uint) -> libc::c_int;
    fn comedi_data_read(it: *const comedi_t, subd: libc::c_uint, chan: libc::c_uint, range: libc::c_uint, aref: libc::c_uint, data: *mut libc::c_uint) -> libc::c_int;
}

#[cfg(test)]
mod tests {
    use *;
    
    #[test]
    fn open_comedi_device() {
        let comedi = unsafe{ comedi_open(CString::new("/dev/comedi0").unwrap().as_ptr()) };
        assert!(!comedi.is_null());
    }
}
