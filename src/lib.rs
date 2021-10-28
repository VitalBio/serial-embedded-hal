extern crate embedded_hal as hal;
extern crate nb;
extern crate serial;

use serial::prelude::*;
use std::ffi::OsStr;
use std::io::prelude::*;

pub use serial::BaudRate;
pub use serial::CharSize;
pub use serial::FlowControl;
pub use serial::Parity;
pub use serial::PortSettings;
pub use serial::StopBits;

/// Newtype over [`serial-rs`](https://crates.io/crates/serial)'s serial port abstraction.
struct Serial(serial::SystemPort);

impl Serial {
    pub fn new<PORT: AsRef<OsStr> + ?Sized> (port: &PORT, settings: &serial::PortSettings) -> serial::Result<Self> {
        let mut port = serial::open(&port)?;
        port.configure(settings)?;
        Ok(Serial(port))
    }
}

impl embedded_hal::serial::Read<u8> for Serial {
    type Error = serial::Error;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let mut buf: [u8; 1] = [0];
        match self.0.read(&mut buf) {
            Ok(_) => Ok(buf[0]),
            Err(e) => match e.kind() {
                std::io::ErrorKind::WouldBlock => Err(nb::Error::WouldBlock),
                std::io::ErrorKind::TimedOut => Err(nb::Error::WouldBlock),
                _ => Err(nb::Error::Other(serial::Error::new(
                    serial::ErrorKind::Io(e.kind()),
                    "bad read",
                ))),
            },
        }
    }
}

impl embedded_hal::serial::Write<u8> for Serial {
    type Error = serial::Error;

    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        match self.0.write(&[byte]) {
            Ok(_) => Ok(()),
            Err(e) => Err(nb::Error::Other(serial::Error::new(
                serial::ErrorKind::Io(e.kind()),
                "bad write",
            ))),
        }
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        match self.0.flush() {
            Ok(_) => Ok(()),
            Err(e) => Err(nb::Error::Other(serial::Error::new(
                serial::ErrorKind::Io(e.kind()),
                "bad flush",
            ))),
        }
    }
}