use crate::vl6180::{
    MeasurementAccuracy, ResultAddress, SysRangeAddress, SystemAddress
};
use embedded_hal::{delay::DelayNs, i2c::I2c};
use esp_println::println;

//address 0x29

pub struct Vl6180<I2C, D> {
    i2c: I2C,
    address: u8, //0x29
    delay: D,
}

impl<I2C, D> Vl6180<I2C, D>
where
    I2C: I2c,
    D: DelayNs,
{
    pub fn new(i2c: I2C, address: u8, delay: D) -> Self {
        Self {
            i2c,
            address,
            delay,
        }
    }

    ///Run this function before measurement
    pub fn init(&mut self, accuracy: MeasurementAccuracy) -> Result<(), I2C::Error> {
        self.delay.delay_us(400);

        println!("sggsgs");

        let mut system_status = [0u8; 1];
        let reg_addr = (SystemAddress::SystemFreshOutOfReset as u16).to_be_bytes();
        self.i2c
            .write_read(self.address, &reg_addr, &mut system_status)
            .unwrap();

        if system_status[0] == 1 {
            //for the below registers see the datasheet:
            //http://www.st.com/st-web-ui/static/active/en/resource/technical/document/application_note/DM00122600.pdf
            //page 24

            //set mandatory private registers
            self.write_register(0x0207, 0x01)?;
            self.write_register(0x0208, 0x01)?;
            self.write_register(0x0096, 0x00)?;
            self.write_register(0x0097, 0xfd)?;
            self.write_register(0x00e3, 0x01)?;
            self.write_register(0x00e4, 0x03)?;
            self.write_register(0x00e5, 0x02)?;
            self.write_register(0x00e6, 0x01)?;
            self.write_register(0x00e7, 0x03)?;
            self.write_register(0x00f5, 0x02)?;
            self.write_register(0x00d9, 0x05)?;
            self.write_register(0x00db, 0xce)?;
            self.write_register(0x00dc, 0x03)?;
            self.write_register(0x00dd, 0xf8)?;
            self.write_register(0x009f, 0x00)?;
            self.write_register(0x00a3, 0x3c)?;
            self.write_register(0x00b7, 0x00)?;
            self.write_register(0x00bb, 0x3c)?;
            self.write_register(0x00b2, 0x09)?;
            self.write_register(0x00ca, 0x09)?;
            self.write_register(0x0198, 0x01)?;
            self.write_register(0x01b0, 0x17)?;
            self.write_register(0x01ad, 0x00)?;
            self.write_register(0x00ff, 0x05)?;
            self.write_register(0x0100, 0x05)?;
            self.write_register(0x0199, 0x05)?;
            self.write_register(0x01a6, 0x1b)?;
            self.write_register(0x01ac, 0x3e)?;
            self.write_register(0x01a7, 0x1f)?;
            self.write_register(0x0030, 0x00)?;

            //recommended public registeres
            self.write_register(0x0011, 0x10)?; //Enables polling for ‘New Sample ready’ when measurement completes
            self.write_register(0x010a, 0x30)?; //Set the averaging sample period (compromise between lower noise and increased execution time)
            self.write_register(0x003f, 0x46)?; //Sets the light and dark gain (upper nibble). Dark gain should not be changed.
            self.write_register(0x0031, 0xFF)?; //sets the # of range measurements after which auto calibration of system is performed.
            self.write_register(0x0041, 0x63)?; //Set ALS integration time to 100ms
            self.write_register(0x002e, 0x01)?; //perform a single temperature calibration of the ranging sensor

            //optional registers
            self.write_register(0x001b, 0x09)?; //Set default ranging inter-measurement period to 100ms
            self.write_register(0x003e, 0x31)?; //Set default ALS inter-measurement period to 500ms
            self.write_register(0x0014, 0x24)?; //Configures interrupt on ‘New Sample Ready threshold event’

            // Early convergence estimate (ECE)
            self.write_register(0x0022, accuracy as u16)?;

            //default measurement offset value is 0 = 0x00
            self.write_register(SysRangeAddress::SysRangePartToPartRangeOffset as u16, 0x00)?;

            //cross talk compensation
            self.write_register(
                SysRangeAddress::SysRangeCrossTalkCompensationRate as u16,
                0x00,
            )?;

            self.write_register(SystemAddress::SystemFreshOutOfReset as u16, 0x00)?;

            self.delay.delay_ms(5);
            Ok(())
        } else {
            // Could not initialize because system_status != 1
            // Return an error from an I2C operation to propagate the error type
            Err(self
                .i2c
                .write(self.address, &[])
                .err()
                .unwrap_or_else(|| panic!("Unable to return I2C error")))
        }
    }

    pub fn check_result_range_status(&mut self) -> Result<(), I2C::Error> {
        let mut buffer = [0u8; 1];
        self.i2c.write_read(
            self.address,
            &[ResultAddress::ResultRangeStatus as u8],
            &mut buffer,
        )?;

        esp_println::println!("status: {:?}", buffer);
        Ok(())
    }

    ///final range result value presented to the user for use. Unit is in mm.
    pub fn read_distance(&mut self) -> Result<[u8; 2], I2C::Error> {
        let mut system_status = [0u8, 1];

        self.i2c
            .write_read(self.address, &[0], &mut system_status)
            .unwrap();

        if system_status[0] == 0 {
            self.write_register(SysRangeAddress::SysRangeStart as u16, 0x01)?;

            println!("tst");
            while !self.is_measurement_ready()? {
                self.delay.delay_ms(1);
                println!("tässä");
            }

            let mut result = [0u8, 1];
            self.i2c.write_read(self.address, &[ResultAddress::ResultRangeVal as u8], &mut result)?;

            self.write_register(SystemAddress::SystemInterruptClear as u16, 0x07)?;

            Ok(result)
        } else {
            todo!()
        }
    }

    fn is_measurement_ready(&mut self) -> Result<bool, I2C::Error> {
        let mut status = [0u8; 1];
        let reg_addr = (ResultAddress::ResultInterruptStatusGpio as u16).to_be_bytes();
        self.i2c.write_read(self.address, &reg_addr, &mut status)?;
        Ok((status[0] & 0x07) == 0x04)
    }

    fn write_register(&mut self, register: u16, value: u16) -> Result<(), I2C::Error> {
        let reg_bytes = register.to_be_bytes();
        let val_bytes = value.to_be_bytes();
        let data = [reg_bytes[0], reg_bytes[1], val_bytes[0], val_bytes[1]];
        self.i2c.write(self.address, &data)?;
        Ok(())
    }
}
