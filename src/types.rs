/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<CommE, PinE> {
    /// I²C / SPI communication error
    Comm(CommE),
    /// Chip-select pin error (SPI)
    Pin(PinE),
    /// Invalid input data provided
    InvalidInputData,
}

/// All possible errors in this crate
#[derive(Debug)]
pub struct ModeChangeError<CommE, PinE, DEV> {
    /// I²C / SPI communication error
    pub error: Error<CommE, PinE>,
    /// Original device without mode changed
    pub dev: DEV,
}

/// Device operation modes
pub mod mode {
    /// Magnetometer one-shot (single) mode
    #[derive(Debug)]
    pub struct MagOneShot;
    /// Magnetometer continuous mode
    #[derive(Debug)]
    pub struct MagContinuous;
}

/// Measurement
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Measurement {
    /// X-axis data.
    pub x: i32,
    /// Y-axis data.
    pub y: i32,
    /// Z-axis data.
    pub z: i32,
}

/// Unscaled measurement
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct UnscaledMeasurement {
    /// X-axis data.
    pub x: i16,
    /// Y-axis data.
    pub y: i16,
    /// Z-axis data.
    pub z: i16,
}

/// Accelerometer output data rate
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccelOutputDataRate {
    /// 1 Hz (High-resolution/Normal/Low-power)
    Hz1,
    /// 10 Hz (High-resolution/Normal/Low-power)
    Hz10,
    /// 25 Hz (High-resolution/Normal/Low-power)
    Hz25,
    /// 50 Hz (High-resolution/Normal/Low-power)
    Hz50,
    /// 100 Hz (High-resolution/Normal/Low-power)
    Hz100,
    /// 200 Hz (High-resolution/Normal/Low-power)
    Hz200,
    /// 400 Hz (High-resolution/Normal/Low-power)
    Hz400,
    /// 1.344 kHz (High-resolution/Normal)
    Khz1_344,
    /// 1.620 kHz (Low-power)
    Khz1_620LowPower,
    /// 5.376 kHz (Low-power)
    Khz5_376LowPower,
}

/// Accelerometer mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccelMode {
    /// Power down
    PowerDown,
    /// Low power (8-bit)
    LowPower,
    /// Normal mode (10-bit)
    Normal,
    /// High resolution (12-bit)
    HighResolution,
}

/// Accelerometer scaling factor
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccelScale {
    /// Plus or minus 2g
    G2,
    /// Plus or minus 4g
    G4,
    /// Plus or minus 8g
    G8,
    /// Plus or minus 16g
    G16,
}

/// Magnetometer output data rate
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MagOutputDataRate {
    /// 10 Hz
    Hz10,
    /// 20 Hz
    Hz20,
    /// 50 Hz
    Hz50,
    /// 100 Hz
    Hz100,
}

/// Data status
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Status {
    /// X,Y,Z-axis data overrun
    pub xyz_overrun: bool,
    /// X-axis data overrun
    pub x_overrun: bool,
    /// Y-axis data overrun
    pub y_overrun: bool,
    /// Z-axis data overrun
    pub z_overrun: bool,
    /// X,Y,Z-axis new data ready
    pub xyz_new_data: bool,
    /// X-axis data new data ready
    pub x_new_data: bool,
    /// Y-axis data new data ready
    pub y_new_data: bool,
    /// Z-axis data new data ready
    pub z_new_data: bool,
}

/// Temperature sensor status
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TemperatureStatus {
    /// Data overrun
    pub overrun: bool,
    /// New data ready
    pub new_data: bool,
}

/// Fifo status
pub struct FifoStatus {
    /// Fifo content exceeds watermark level
    pub watermark: bool,
    /// Fifo buffer is full. This means that the FIFO buffer contains 32 unread
    /// samples. At the following ODR a new sample set replaces the oldest Fifo
    /// value. The overrun flag is set to 0 when the first sample set has been
    /// read.
    pub overrun: bool,
    /// All Fifo samples have been read and Fifo is empty
    pub empty: bool,
    /// The current number of unread samples stored in the Fifo buffer. When
    /// Fifo is enabled, this value increases at ODR frequency until the buffer
    /// is full, whereas, it decreases every time one sample set is retrieved
    /// from Fifo.
    pub fss_unread: u8,
}

/// Interrupt Selection
pub enum IntNumber {
    /// Int1 pin
    Int1,
    /// Int2 pin
    Int2,
}

/// Fifo Mode
pub enum FifoMode {
    /// Fifo is not operational and for this reason it remains empty. Default
    Bypass,
    /// he buffer continues filling data from the X, Y and Z accelerometer
    /// channels until it is full (a set of 32 samples stored). When the FIFO is
    /// full, it stops collecting data from the input channels and the FIFO
    /// content remains unchanged. After the last read it is necessary to exit
    /// to bypass an reenable Fifo mode.
    Fifo,
    /// In Stream mode the FIFO continues filling data from the X, Y, and Z
    /// accelerometer channels until the buffer is full (a set of 32 samples
    /// stored) at which point the FIFO buffer index restarts from the beginning
    /// and older data is replaced by the current data. The oldest values
    /// continue to be overwritten until a read operation frees the FIFO slots.
    Stream,
    /// In Stream-to-FIFO mode, data from the X, Y and Z accelerometer channels
    /// are collected in a combination of Stream mode and FIFO mode. The FIFO
    /// buffer starts operating in Stream mode and switches to FIFO mode when
    /// the selected interrupt occurs. When an interrupt event is configured on
    /// the INT_1_XL pin, the FIFO operates in Stream mode if the INT_1_XL pin
    /// value is equal to ‘0’ and it operates in FIFO mode if the INT_1_XL pin
    /// value is equal to ‘1’. Switching modes is dynamically performed
    /// according to the INT_1_XL pin value
    StreamToFifo,
}

/// Interrupt status
pub struct InterruptStatus {
    /// Interrupt is active
    pub active: bool,
    /// Z High is active
    pub z_high: bool,
    /// Z Low is active
    pub z_low: bool,
    /// Y Low is active
    pub y_high: bool,
    /// Y Low is active
    pub y_low: bool,
    /// X Low is active
    pub x_high: bool,
    /// X Low is active
    pub x_low: bool,
}
