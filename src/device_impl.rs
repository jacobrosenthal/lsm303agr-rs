use crate::{
    interface::{I2cInterface, ReadData, SpiInterface, WriteData},
    mode,
    register_address::{WHO_AM_I_A_VAL, WHO_AM_I_M_VAL},
    types::{FifoMode, FifoStatus, IntNumber},
    AccelMode, AccelScale, BitFlags as BF, Config, Error, InterruptStatus, Lsm303agr, Measurement,
    PhantomData, Register, Status, TemperatureStatus, UnscaledMeasurement,
};

impl<I2C> Lsm303agr<I2cInterface<I2C>, mode::MagOneShot> {
    /// Create new instance of the LSM303AGR device communicating through I2C.
    pub fn new_with_i2c(i2c: I2C) -> Self {
        Lsm303agr {
            iface: I2cInterface { i2c },
            ctrl_reg1_a: Config { bits: 0x7 },
            ctrl_reg3_a: Config { bits: 0 },
            ctrl_reg4_a: Config { bits: 0 },
            ctrl_reg5_a: Config { bits: 0 },
            ctrl_reg6_a: Config { bits: 0 },
            cfg_reg_a_m: Config { bits: 0x3 },
            cfg_reg_c_m: Config { bits: 0 },
            temp_cfg_reg_a: Config { bits: 0 },
            fifo_ctrl_reg_a: Config { bits: 0 },
            int_ctrl_reg_m: Config { bits: 0 },
            accel_odr: None,
            _mag_mode: PhantomData,
        }
    }
}

impl<I2C, MODE> Lsm303agr<I2cInterface<I2C>, MODE> {
    /// Destroy driver instance, return I2C bus.
    pub fn destroy(self) -> I2C {
        self.iface.i2c
    }
}

impl<SPI, CSXL, CSMAG> Lsm303agr<SpiInterface<SPI, CSXL, CSMAG>, mode::MagOneShot> {
    /// Create new instance of the LSM303AGR device communicating through SPI.
    pub fn new_with_spi(spi: SPI, chip_select_accel: CSXL, chip_select_mag: CSMAG) -> Self {
        Lsm303agr {
            iface: SpiInterface {
                spi,
                cs_xl: chip_select_accel,
                cs_mag: chip_select_mag,
            },
            ctrl_reg1_a: Config { bits: 0x7 },
            ctrl_reg3_a: Config { bits: 0 },
            ctrl_reg4_a: Config { bits: 0 },
            ctrl_reg5_a: Config { bits: 0 },
            ctrl_reg6_a: Config { bits: 0 },
            cfg_reg_a_m: Config { bits: 0x3 },
            cfg_reg_c_m: Config { bits: 0 },
            temp_cfg_reg_a: Config { bits: 0 },
            fifo_ctrl_reg_a: Config { bits: 0 },
            int_ctrl_reg_m: Config { bits: 0 },
            accel_odr: None,
            _mag_mode: PhantomData,
        }
    }
}

impl<SPI, CSXL, CSMAG, MODE> Lsm303agr<SpiInterface<SPI, CSXL, CSMAG>, MODE> {
    /// Destroy driver instance, return SPI bus instance and chip select pin.
    pub fn destroy(self) -> (SPI, CSXL, CSMAG) {
        (self.iface.spi, self.iface.cs_xl, self.iface.cs_mag)
    }
}

impl<DI, CommE, PinE, MODE> Lsm303agr<DI, MODE>
where
    DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
{
    /// Initialize registers
    pub fn init(&mut self) -> Result<(), Error<CommE, PinE>> {
        let temp_cfg_reg = self
            .temp_cfg_reg_a
            .with_high(BF::TEMP_EN0)
            .with_high(BF::TEMP_EN1);
        self.iface
            .write_accel_register(Register::TEMP_CFG_REG_A, temp_cfg_reg.bits)?;
        self.temp_cfg_reg_a = temp_cfg_reg;

        self.iface
            .write_accel_register(Register::CTRL_REG3_A, self.ctrl_reg3_a.bits)?;

        let reg4 = self.ctrl_reg4_a.with_high(BF::ACCEL_BDU);
        self.iface
            .write_accel_register(Register::CTRL_REG4_A, reg4.bits)?;
        self.ctrl_reg4_a = reg4;

        self.iface
            .write_accel_register(Register::CTRL_REG5_A, self.ctrl_reg5_a.bits)?;

        self.iface
            .write_accel_register(Register::FIFO_CTRL_REG_A, self.fifo_ctrl_reg_a.bits)?;

        self.iface
            .write_accel_register(Register::INT_CTRL_REG_M, self.int_ctrl_reg_m.bits)?;

        let regc = self.cfg_reg_c_m.with_high(BF::MAG_BDU);
        self.iface
            .write_mag_register(Register::CFG_REG_C_M, regc.bits)?;
        self.cfg_reg_c_m = regc;
        Ok(())
    }

    /// Accelerometer status
    pub fn accel_status(&mut self) -> Result<Status, Error<CommE, PinE>> {
        self.iface
            .read_accel_register(Register::STATUS_REG_A)
            .map(convert_status)
    }

    /// Fifo status
    pub fn fifo_status(&mut self) -> Result<FifoStatus, Error<CommE, PinE>> {
        self.iface
            .read_accel_register(Register::FIFO_SRC_REG_A)
            .map(convert_fifo_status)
    }

    /// Accelerometer data
    ///
    /// Returned in mg (milli-g) where 1g is 9.8m/s².
    ///
    /// If you need the raw unscaled measurement see [`Lsm303agr::accel_data_unscaled`].
    pub fn accel_data(&mut self) -> Result<Measurement, Error<CommE, PinE>> {
        let unscaled = self.accel_data_unscaled()?;

        let mode = self.get_accel_mode();
        let scale = self.get_accel_scale();

        let scaling_factor = match mode {
            AccelMode::PowerDown => 0,
            AccelMode::HighResolution => match scale {
                AccelScale::G2 => 1,
                AccelScale::G4 => 2,
                AccelScale::G8 => 4,
                AccelScale::G16 => 8,
            },
            AccelMode::LowPower => match scale {
                AccelScale::G2 => 16,
                AccelScale::G4 => 32,
                AccelScale::G8 => 64,
                AccelScale::G16 => 128,
            },
            AccelMode::Normal => match scale {
                AccelScale::G2 => 4,
                AccelScale::G4 => 8,
                AccelScale::G8 => 16,
                AccelScale::G16 => 32,
            },
        };

        Ok(Measurement {
            x: (unscaled.x as i32) * scaling_factor,
            y: (unscaled.y as i32) * scaling_factor,
            z: (unscaled.z as i32) * scaling_factor,
        })
    }

    /// Accelerometer data
    ///
    /// Return n u8 accelerometer registers, bulk version of accel_data_unscaled
    pub fn accel_data_n(&mut self, data: &mut [u8]) -> Result<(), Error<CommE, PinE>> {
        self.iface.read_n(Register::OUT_X_L_A, data)
    }

    /// Unscaled accelerometer data
    pub fn accel_data_unscaled(&mut self) -> Result<UnscaledMeasurement, Error<CommE, PinE>> {
        let data = self
            .iface
            .read_accel_3_double_registers(Register::OUT_X_L_A)?;

        let mode = self.get_accel_mode();

        let resolution_factor = match mode {
            AccelMode::PowerDown => 1,
            AccelMode::HighResolution => 1 << 4,
            AccelMode::LowPower => 1 << 8,
            AccelMode::Normal => 1 << 6,
        };

        Ok(UnscaledMeasurement {
            x: (data.0 as i16) / resolution_factor,
            y: (data.1 as i16) / resolution_factor,
            z: (data.2 as i16) / resolution_factor,
        })
    }

    /// Magnetometer status
    pub fn mag_status(&mut self) -> Result<Status, Error<CommE, PinE>> {
        self.iface
            .read_mag_register(Register::STATUS_REG_M)
            .map(convert_status)
    }

    /// Get accelerometer device ID
    pub fn accelerometer_id(&mut self) -> Result<u8, Error<CommE, PinE>> {
        self.iface.read_accel_register(Register::WHO_AM_I_A)
    }

    /// Read and verify the accelerometer device ID
    pub fn accelerometer_is_detected(&mut self) -> Result<bool, Error<CommE, PinE>> {
        Ok(self.accelerometer_id()? == WHO_AM_I_A_VAL)
    }

    /// Get magnetometer device ID
    pub fn magnetometer_id(&mut self) -> Result<u8, Error<CommE, PinE>> {
        self.iface.read_mag_register(Register::WHO_AM_I_M)
    }

    /// Read and verify the magnetometer device ID
    pub fn magnetometer_is_detected(&mut self) -> Result<bool, Error<CommE, PinE>> {
        Ok(self.magnetometer_id()? == WHO_AM_I_M_VAL)
    }

    /// Read temperature sensor data
    pub fn temperature_data(&mut self) -> Result<i16, Error<CommE, PinE>> {
        let data = self
            .iface
            .read_accel_double_register(Register::OUT_TEMP_L_A)?;
        Ok(data as i16)
    }

    /// Read temperature sensor data as celsius
    pub fn temperature_celsius(&mut self) -> Result<f32, Error<CommE, PinE>> {
        let data = self.temperature_data()?;
        let temp_offset = (data as f32) / 256.0;
        let default_temp = 25.0;
        Ok(temp_offset + default_temp)
    }

    /// Temperature sensor status
    pub fn temperature_status(&mut self) -> Result<TemperatureStatus, Error<CommE, PinE>> {
        self.iface
            .read_accel_register(Register::STATUS_REG_AUX_A)
            .map(convert_temperature_status)
    }

    /// Interrupt status
    pub fn interrupt_status(&mut self) -> Result<InterruptStatus, Error<CommE, PinE>> {
        self.iface
            .read_accel_register(Register::INT1_SRC_A)
            .map(convert_interrupt_status)
    }

    /// Interrupt status
    pub fn interrupt_status_u8(&mut self) -> Result<u8, Error<CommE, PinE>> {
        self.iface.read_accel_register(Register::INT1_SRC_A)
    }

    /// Interrupt status
    pub fn rg3(&mut self) -> Result<u8, Error<CommE, PinE>> {
        self.iface.read_accel_register(Register::CTRL_REG3_A)
    }

    // /// Interrupt status
    // pub fn fifo_ctrl_u8(&mut self) -> Result<u8, Error<CommE, PinE>> {
    //     self.iface.read_accel_register(Register::FIFO_CTRL_REG_A)
    // }

    /// Interrupt status
    pub fn fifo_status_u8(&mut self) -> Result<u8, Error<CommE, PinE>> {
        self.iface.read_accel_register(Register::FIFO_SRC_REG_A)
    }

    /// Enable fifo mode.
    ///
    /// You must call start_fifo to trigger a fifo read.
    ///
    /// Fifo stores up to 32 readings of each of the 3 chanels. Choose an
    /// interrupt, and a fifo mode. Reading the accell will return oldest data
    /// first. Using accel_data_n will allow you to burst read up to 32 readings
    /// assuming they're available.
    ///
    /// The complete FIFO read would be performed faster than 1*ODR, which means
    /// that using a standard I2C, the selectable ODR must be lower than 57 Hz.
    /// If a fast I2C mode is used (max rate 400 kHz), the selectable ODR must
    /// be lower than 228 Hz.
    pub fn enable_fifo(
        &mut self,
        mode: FifoMode,
        int: IntNumber,
        latched: bool,
        active_high: bool,
    ) -> Result<(), Error<CommE, PinE>> {
        // todo watermark count configuration

        // enable overrun interrupt
        let reg3 = self.ctrl_reg3_a.with_high(BF::I1_OVERRUN);
        self.iface
            .write_accel_register(Register::CTRL_REG3_A, reg3.bits)?;
        self.ctrl_reg3_a = reg3;

        // enable the interrupts
        let mut int_ctrl = self.int_ctrl_reg_m.with_high(BF::IEN);
        int_ctrl = if active_high {
            int_ctrl.with_high(BF::IEA)
        } else {
            int_ctrl.with_low(BF::IEA)
        };
        int_ctrl = if latched {
            int_ctrl.with_high(BF::IEL)
        } else {
            int_ctrl.with_low(BF::IEL)
        };
        self.iface
            .write_accel_register(Register::INT_CTRL_REG_M, int_ctrl.bits)?;
        self.int_ctrl_reg_m = int_ctrl;

        // let int_cfg = Config { bits: 0 }.with_high(BF::AOI);
        // self.iface
        //     .write_accel_register(Register::INT1_CFG_A, int_cfg.bits)?;

        // let reg6 = self.ctrl_reg6_a.with_high(BF::H_LACTIVE);
        // self.iface
        //     .write_accel_register(Register::CTRL_REG6_A, reg6.bits)?;
        // self.ctrl_reg6_a = reg6;

        // enable fifo
        let reg5 = self.ctrl_reg5_a.with_high(BF::FIFO_EN);
        self.iface
            .write_accel_register(Register::CTRL_REG5_A, reg5.bits)?;
        self.ctrl_reg5_a = reg5;

        // mode change is actually the enabler, this must be after FIFO_EN
        // https://www.st.com/resource/en/application_note/an4825-ultracompact-highperformance-ecompass-module-based-on-the-lsm303agr-stmicroelectronics.pdf
        let mode_bits = match mode {
            FifoMode::Bypass => BF::FIFO_MODE_BYPASS,
            FifoMode::Fifo => BF::FIFO_MODE_FIFO,
            FifoMode::Stream => BF::FIFO_MODE_STREAM,
            FifoMode::StreamToFifo => BF::FIFO_MODE_STREAM_TO_FIFO,
        };
        let mut fifo_ctrl = self
            .fifo_ctrl_reg_a
            .with_low(BF::FIFO_MODE_MASK)
            .with_high(mode_bits);
        fifo_ctrl = match int {
            IntNumber::Int1 => fifo_ctrl.with_low(BF::FIFO_TRIGGER),
            IntNumber::Int2 => fifo_ctrl.with_high(BF::FIFO_TRIGGER),
        };
        self.iface
            .write_accel_register(Register::FIFO_CTRL_REG_A, fifo_ctrl.bits)?;
        self.fifo_ctrl_reg_a = fifo_ctrl;

        Ok(())
    }

    /// Restart the FifoMode::Fifo mode.
    ///
    /// For FifoMode::Fifo after the registers fill up and overrun, you must
    /// start again
    pub fn restart_fifo(&mut self) -> Result<(), Error<CommE, PinE>> {
        // reset register and set to bypass mode
        let fifo_ctrl = self
            .fifo_ctrl_reg_a
            .with_low(BF::FIFO_MODE_MASK)
            .with_high(BF::FIFO_MODE_BYPASS);
        self.iface
            .write_accel_register(Register::FIFO_CTRL_REG_A, fifo_ctrl.bits)?;
        self.fifo_ctrl_reg_a = fifo_ctrl;

        // reset register and set to fifo mode
        let fifo_ctrl = self
            .fifo_ctrl_reg_a
            .with_low(BF::FIFO_MODE_MASK)
            .with_high(BF::FIFO_MODE_FIFO);
        self.iface
            .write_accel_register(Register::FIFO_CTRL_REG_A, fifo_ctrl.bits)?;
        self.fifo_ctrl_reg_a = fifo_ctrl;
        Ok(())
    }
}

fn convert_status(st: u8) -> Status {
    Status {
        xyz_overrun: (st & BF::XYZOR) != 0,
        z_overrun: (st & BF::ZOR) != 0,
        y_overrun: (st & BF::YOR) != 0,
        x_overrun: (st & BF::XOR) != 0,
        xyz_new_data: (st & BF::XYZDR) != 0,
        z_new_data: (st & BF::ZDR) != 0,
        y_new_data: (st & BF::YDR) != 0,
        x_new_data: (st & BF::XDR) != 0,
    }
}

fn convert_fifo_status(st: u8) -> FifoStatus {
    FifoStatus {
        watermark: (st & BF::FIFO_WTM) != 0,
        overrun: (st & BF::FIFO_OVRN) != 0,
        empty: (st & BF::FIFO_EMPTY) != 0,
        fss_unread: st & BF::FIFO_FSS_MASK,
    }
}

fn convert_temperature_status(st: u8) -> TemperatureStatus {
    TemperatureStatus {
        overrun: (st & BF::TOR) != 0,
        new_data: (st & BF::TDA) != 0,
    }
}

fn convert_interrupt_status(st: u8) -> InterruptStatus {
    InterruptStatus {
        active: (st & BF::INT1_SRC_IA) != 0,
        z_high: (st & BF::INT1_SRC_ZH) != 0,
        z_low: (st & BF::INT1_SRC_ZL) != 0,
        y_high: (st & BF::INT1_SRC_YH) != 0,
        y_low: (st & BF::INT1_SRC_YL) != 0,
        x_high: (st & BF::INT1_SRC_XH) != 0,
        x_low: (st & BF::INT1_SRC_XL) != 0,
    }
}
