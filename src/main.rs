extern crate libc;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

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

unsafe impl Send for ElevatorInterface {}

impl ElevatorInterface {
    const MOTOR_SPEED: u32 = 2800;
    const N_FLOORS: u8 = 4;
    
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

    fn read_floorsensor(&self) -> Option<u8> {
        unsafe {
            let mut data: libc::c_uint = 0;
            comedi_dio_read(self.0, channel::SENSOR_FLOOR0 >> 8, channel::SENSOR_FLOOR0 & 0xff, &mut data);
            if data != 0 {
                return Some(0);
            }
            
            comedi_dio_read(self.0, channel::SENSOR_FLOOR1 >> 8, channel::SENSOR_FLOOR1 & 0xff, &mut data);
            if data != 0 {
                return Some(1);
            }
            
            comedi_dio_read(self.0, channel::SENSOR_FLOOR2 >> 8, channel::SENSOR_FLOOR2 & 0xff, &mut data);
            if data != 0 {
                return Some(2);
            }
            
            comedi_dio_read(self.0, channel::SENSOR_FLOOR3 >> 8, channel::SENSOR_FLOOR3 & 0xff, &mut data);
            if data != 0 {
                return Some(3);
            }
            
            None
        }
    }
        
}

fn main() {
    let interface = ElevatorInterface::open("/dev/comedi0").unwrap();
}

#[cfg(test)]
mod tests {
    use *;

    use std::sync::Mutex;

    // These tests are executed on an actual elevator. To make sure only one test is run at the same time, the elevator is protected by this mutex.
    lazy_static! {
        static ref ELEVATOR: Mutex<ElevatorInterface> = Mutex::new(ElevatorInterface::open("/dev/comedi0").unwrap());
    }
    
    
    #[test]
    fn init_elevator() {
        ELEVATOR.lock().unwrap();
    }

    #[test]
    fn test_run() {
        let elevator = ELEVATOR.lock().unwrap();
        println!("The elevator will now do a run from the bottom floor to the top floor. It will stop in the floor below the top floor");
        elevator.set_direction(ElevatorDirection::Down);
        while elevator.read_floorsensor() != Some(0) {}
        elevator.set_direction(ElevatorDirection::Up);
        while elevator.read_floorsensor() != Some(ElevatorInterface::N_FLOORS-1) {}
        elevator.set_direction(ElevatorDirection::Down);
        while elevator.read_floorsensor() != Some(ElevatorInterface::N_FLOORS-2) {}
        elevator.set_direction(ElevatorDirection::Stop);
    }
}
