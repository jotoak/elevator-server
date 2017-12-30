extern crate libc;

use std::ffi::CString;

mod channel;

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

pub enum ElevatorDirection{
    Up,
    Down,
    Stop,
}

pub struct ElevatorInterface(*const comedi_t);

impl ElevatorInterface {
    const MOTOR_SPEED: u32 = 2800;
    
    fn open(interface_name: &str) -> Result<Self, ()> {
        unsafe {
            let comedi = comedi_open(CString::new(interface_name).unwrap().as_ptr());
            if comedi.is_null() {
                Err(())
            } else {
                Ok(ElevatorInterface(comedi))
            }
        }
    }

    fn set_direction(&self, dir: ElevatorDirection) {
        unsafe {
            match dir {
                ElevatorDirection::Up => {
                    comedi_dio_write(self.0, channel::MOTORDIR >> 8, channel::MOTORDIR & 0xff, 0);
                    comedi_data_write(self.0, channel::MOTOR >> 8, channel::MOTOR & 0xff, 0, 0, Self::MOTOR_SPEED);
                },
                ElevatorDirection::Down => {
                    comedi_dio_write(self.0, channel::MOTORDIR >> 8, channel::MOTORDIR & 0xff, 1);
                    comedi_data_write(self.0, channel::MOTOR >> 8, channel::MOTOR & 0xff, 0, 0, Self::MOTOR_SPEED);
                },
                ElevatorDirection::Stop => {
                    comedi_data_write(self.0, channel::MOTOR >> 8, channel::MOTOR & 0xff, 0, 0, 0);
                },
            }
        }
    }
        
}

fn main() {
    let interface = ElevatorInterface::open("/dev/comedi0").unwrap();
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
