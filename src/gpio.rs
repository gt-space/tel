use std::fs::File;
use std::io::Write;
use std::io::Read;
// use std::fmt;

enum PinMode {
    Input,
    Output,
}

enum PinValue {
    Low = 0,
    High = 0,
}

// fn SetPinMode(gpio: i16, dir: PinMode) {
//     let filename: [char, 36];

//     filename = format!("/sys/class/gpio/gpio{}/direction", gpio);
//     let mut file = File::create(filename)?;

//     match dir {
//         Input => file.write_all(b"in\n"),
//         Output => file.write_all(b"out\n"),
//     }
//     Ok(());
// }

// fn WriteGPIO(gpio: i16, v: PinValue) {
//     let filename: [char, 32];
    
//     filename = format!("/sys/class/gpio/gpio{}/value", gpio);
//     let mut file = File::create(filename)?;

//     file.write_all("{}", v);
//     Ok(());
// }

// fn ReadGPIO(gpio: i16) -> PinValue {
//     let filename: [char, 32];
//     let v: PinValue;
    
//     filename = format!("/sys/class/gpio/gpio{}/value", gpio);
//     let mut file = File::open(filename)?;

//     file.read_to_end(&v);
//     Ok(());

//     return v;
// }

// SetPinMode(66, Output);

// WriteGPIO(66, High);