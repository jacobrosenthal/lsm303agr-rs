pub struct Register;
impl Register {
    pub const STATUS_REG_AUX_A: u8 = 0x07;
    pub const OUT_TEMP_L_A: u8 = 0x0C;
    pub const WHO_AM_I_A: u8 = 0x0F;
    pub const TEMP_CFG_REG_A: u8 = 0x1F;
    pub const CTRL_REG1_A: u8 = 0x20;
    pub const CTRL_REG3_A: u8 = 0x22;
    pub const CTRL_REG4_A: u8 = 0x23;
    pub const CTRL_REG5_A: u8 = 0x24;
    pub const CTRL_REG6_A: u8 = 0x25;
    pub const STATUS_REG_A: u8 = 0x27;
    pub const OUT_X_L_A: u8 = 0x28;
    pub const INT1_SRC_A: u8 = 0x31;
    pub const WHO_AM_I_M: u8 = 0x4F;
    pub const CFG_REG_A_M: u8 = 0x60;
    pub const CFG_REG_C_M: u8 = 0x62;
    pub const STATUS_REG_M: u8 = 0x67;
    pub const OUTX_L_REG_M: u8 = 0x68;
    pub const FIFO_CTRL_REG_A: u8 = 0x2e;
    pub const FIFO_SRC_REG_A: u8 = 0x2f;
    pub const INT1_CFG_A: u8 = 0x30;

    pub const INT_CTRL_REG_M: u8 = 0x63;
}

pub const WHO_AM_I_A_VAL: u8 = 0x33;
pub const WHO_AM_I_M_VAL: u8 = 0x40;

pub struct BitFlags;
impl BitFlags {
    pub const SPI_RW: u8 = 1 << 7;
    pub const SPI_MS: u8 = 1 << 6;

    pub const LP_EN: u8 = 1 << 3;

    pub const ACCEL_BDU: u8 = 1 << 7;
    pub const HR: u8 = 1 << 3;

    pub const MAG_BDU: u8 = 1 << 4;

    pub const XDR: u8 = 1;
    pub const YDR: u8 = 1 << 1;
    pub const ZDR: u8 = 1 << 2;
    pub const XYZDR: u8 = 1 << 3;
    pub const XOR: u8 = 1 << 4;
    pub const YOR: u8 = 1 << 5;
    pub const ZOR: u8 = 1 << 6;
    pub const XYZOR: u8 = 1 << 7;

    pub const TDA: u8 = 1 << 2;
    pub const TOR: u8 = 1 << 6;

    pub const TEMP_EN0: u8 = 1 << 6;
    pub const TEMP_EN1: u8 = 1 << 7;

    pub const FIFO_EN: u8 = 1 << 6;
    pub const I1_OVERRUN: u8 = 1 << 1;
    pub const I1_WTM: u8 = 1 << 2;

    pub const FIFO_TRIGGER: u8 = 1 << 5;
    pub const FIFO_MODE_MASK: u8 = 3 << 6;
    pub const FIFO_MODE_BYPASS: u8 = 0 << 6;
    pub const FIFO_MODE_FIFO: u8 = 1 << 6;
    pub const FIFO_MODE_STREAM: u8 = 2 << 6;
    pub const FIFO_MODE_STREAM_TO_FIFO: u8 = 3 << 6;

    pub const FIFO_WTM: u8 = 1 << 7;
    pub const FIFO_OVRN: u8 = 1 << 6;
    pub const FIFO_EMPTY: u8 = 1 << 5;
    pub const FIFO_FSS_MASK: u8 = 0x1F;

    pub const INT1_SRC_IA: u8 = 1 << 6;
    pub const INT1_SRC_ZH: u8 = 1 << 5;
    pub const INT1_SRC_ZL: u8 = 1 << 4;
    pub const INT1_SRC_YH: u8 = 1 << 3;
    pub const INT1_SRC_YL: u8 = 1 << 2;
    pub const INT1_SRC_XH: u8 = 1 << 1;
    pub const INT1_SRC_XL: u8 = 1 << 0;

    pub const H_LACTIVE: u8 = 1 << 1;
    pub const AOI: u8 = 1 << 7;

    pub const IEA: u8 = 1 << 2;
    pub const IEL: u8 = 1 << 1;
    pub const IEN: u8 = 1 << 0;
}
