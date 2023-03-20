extern crate i2c_linux;
use std::u8;
use i2c_linux::I2c;

fn set_state(state: u8) {
    let data = state;
    let mut i2c =  I2c::from_path("/dev/i2c-1").unwrap();
    let mut _res = i2c.smbus_set_slave_address(0x11, false);
    _res = i2c.smbus_write_byte_data(0x10, data);
}

fn invalid_arg() {
    println!("Invalid argument, valid ones: [0, 1, 2, 3, 4], setting state to 0b0000");
    set_state(0b0000);
}

fn get_arguments() {
    let args: Vec<_> = std::env::args().collect(); // get all arguements passed to app
    // println!("{:?}", args);
    if args.len() >= 2 {
        let arg = &args[1..]; 
        let mut state: u8 = 0b0000; 
        for id in arg {
            if id != "0" && id != "1" && id != "2" && id != "3" && id != "4" {
                invalid_arg();
                return
            }
            if id == "0" {
                set_state(0b000);
                return
            }
            let i = id.parse::<u8>().unwrap()-1;
            state += 2u8.pow(i.into());
        }
        set_state(state);
        return;
    } else {
        println!("No argument given, valid ones: [0, 1, 2, 3, 4], setting state to 0b0000");
        set_state(0b0000);
    }
}

fn main() {
    get_arguments();
}
