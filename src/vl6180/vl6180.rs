use crate::vl6180::{ResultAddress, SysRangeAddress, SystemAddress, SystemReset};
use core::{fmt, marker};
use embedded_hal::{delay::DelayNs, i2c::I2c};
use esp_println::println;

pub struct Vl6180<I2C, E, D> {
    i2c: I2C,
    address: u8, //0x29
    delay: D,
    is_standby: bool,
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
            is_standby: false,
            _error: marker::PhantomData,
        }
    }

    pub fn read_continuous(&mut self) {
        let sysrange_cmd = [SysRangeAddress::SysRangeStart as u8, 0b0000_0011];
        self.i2c.write(self.address, &sysrange_cmd).unwrap();

        let result = self
            .i2c
            .read(self.address, &mut [ResultAddress::ResultRangeRaw as u8])
            .unwrap();
        println!("raw {:?}", result);
    }

    ///read data from device register
    pub fn read_single(&mut self) {
        let ece_cmd = [
            SysRangeAddress::SysRangeConvergenceEstimate as u8,
            0b0000_0000_0000_000,
        ];
        self.i2c.write(self.address, &ece_cmd).unwrap();

        let sysrange_cmd = [SysRangeAddress::SysRangeStart as u8, 0b0000_0001];
        self.i2c.write(self.address, &sysrange_cmd).unwrap();
        let offset = [
            SysRangeAddress::SysRangePartToPartRangeOffset as u8,
            0b0000_0000,
        ];
        self.i2c.write(self.address, &offset).unwrap();
        let cross_talk_compensation = [
            SysRangeAddress::SysRangeCrossTalkCompensationRate as u8,
            0b0000_0000,
        ];
        self.i2c
            .write(self.address, &cross_talk_compensation)
            .unwrap();
        
        self.delay.delay_ms(10);

        let mut result = [0u8, 8];

        self.i2c
            .write_read(
                self.address,
                &mut [ResultAddress::ResultRangeRaw as u8],
                &mut result,
            )
            .unwrap();
        println!("raw {:?}", result);
    }

    ///initialize device, this sets the device in standby mode
    pub fn init(&mut self) {
        //at this stage, it is possible to configure the device and start single-shot or continuous
        //ranging operation through API functions calls
        self.reset_gpio0();
        self.delay.delay_ms(1);
        self.setup_gpio0();
        self.delay.delay_us(400);
        self.delay.delay_ms(1);
    }

    ///function sets hardware standby. This function can be called after initializing the device.
    ///Calling this function while in standby will reset standby.
    pub fn set_standby(&mut self) {
        let cmd = match self.is_standby {
            true => [SystemAddress::SystemModeGpio0 as u8, 0b0000_0000],
            _ => [SystemAddress::SystemModeGpio0 as u8, 0b0010_0000],
        };
        self.is_standby = !self.is_standby;

        self.i2c.write(self.address, &cmd).unwrap();
    }

    ///function sets gpio to value 1
    fn setup_gpio0(&mut self) {
        let cmd = [SystemAddress::SystemModeGpio0 as u8, 0b0011_0000];
        self.i2c.write(self.address, &cmd).unwrap();
    }

    fn reset_gpio0(&mut self) {
        let reset_cmd = [
            SystemAddress::SystemModeGpio0 as u8,
            SystemReset::ResetGpio0 as u8,
        ];
        self.i2c.write(self.address, &reset_cmd).unwrap();
    }

    fn setup_gpio1(&mut self) {
        let cmd = [SystemAddress::SystemModeGpio1 as u8, 0b0011_0000];
        self.i2c.write(self.address, &cmd).unwrap();
    }

    fn reset_gpio1(&mut self) {
        let reset_cmd = [
            SystemAddress::SystemModeGpio1 as u8,
            SystemReset::ResetGpio1 as u8,
        ];
        self.i2c.write(self.address, &reset_cmd).unwrap();
    }
}
