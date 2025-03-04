use core::{fmt, marker};
use embedded_hal::{delay::DelayNs, i2c::I2c};
use crate::vl6180::{SysRangeAddress, SystemAddress, ResultAddress};

pub struct Vl6180<I2C, E, D> {
    i2c: I2C,
    address: u8, //0x29
    delay: D,
    _error: marker::PhantomData<E>,
}

impl<I2C, E, D> Vl6180<I2C, E, D>
where
    I2C: I2c<Error = E>,
    E: fmt::Debug,
    D: DelayNs,
{
    pub fn new(i2c: I2C, address: u8, delay: D) -> Self {
        Self {
            i2c,
            address,
            delay,
            _error: marker::PhantomData,
        }
    }

    ///read data from device register
    pub fn read(&mut self) {
        todo!()
    }

    ///initialize device
    pub fn init(&mut self) {
        self.delay.delay_us(400);
        self.delay.delay_ms(1);
        todo!()
    }

    fn write_to_register(&mut self) {
        todo!()
    }
}
