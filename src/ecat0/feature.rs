#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::FEATURE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `FMMU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMMUR {
    #[doc = "Bit oriented"]
    VALUE1,
    #[doc = "Byte oriented"]
    VALUE2,
}
impl FMMUR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FMMUR::VALUE1 => false,
            FMMUR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FMMUR {
        match value {
            false => FMMUR::VALUE1,
            true => FMMUR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FMMUR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FMMUR::VALUE2
    }
}
#[doc = "Possible values of the field `CLKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSR {
    #[doc = "Not available"]
    VALUE1,
    #[doc = "Available"]
    VALUE2,
}
impl CLKSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CLKSR::VALUE1 => false,
            CLKSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKSR {
        match value {
            false => CLKSR::VALUE1,
            true => CLKSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CLKSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CLKSR::VALUE2
    }
}
#[doc = "Possible values of the field `CLKS_W`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKS_WR {
    #[doc = "32 bits"]
    VALUE1,
    #[doc = "64 bits"]
    VALUE2,
}
impl CLKS_WR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CLKS_WR::VALUE1 => false,
            CLKS_WR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKS_WR {
        match value {
            false => CLKS_WR::VALUE1,
            true => CLKS_WR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CLKS_WR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CLKS_WR::VALUE2
    }
}
#[doc = "Possible values of the field `LJ_EBUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LJ_EBUSR {
    #[doc = "Not available, standard jitter"]
    VALUE1,
    #[doc = "Available, jitter minimized"]
    VALUE2,
}
impl LJ_EBUSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LJ_EBUSR::VALUE1 => false,
            LJ_EBUSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LJ_EBUSR {
        match value {
            false => LJ_EBUSR::VALUE1,
            true => LJ_EBUSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LJ_EBUSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LJ_EBUSR::VALUE2
    }
}
#[doc = "Possible values of the field `ELD_EBUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELD_EBUSR {
    #[doc = "Not available"]
    VALUE1,
    #[doc = "Available"]
    VALUE2,
}
impl ELD_EBUSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ELD_EBUSR::VALUE1 => false,
            ELD_EBUSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ELD_EBUSR {
        match value {
            false => ELD_EBUSR::VALUE1,
            true => ELD_EBUSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ELD_EBUSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ELD_EBUSR::VALUE2
    }
}
#[doc = "Possible values of the field `ELD_MII`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELD_MIIR {
    #[doc = "Not available"]
    VALUE1,
    #[doc = "Available"]
    VALUE2,
}
impl ELD_MIIR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ELD_MIIR::VALUE1 => false,
            ELD_MIIR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ELD_MIIR {
        match value {
            false => ELD_MIIR::VALUE1,
            true => ELD_MIIR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ELD_MIIR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ELD_MIIR::VALUE2
    }
}
#[doc = "Possible values of the field `SH_FCSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SH_FCSER {
    #[doc = "Not supported"]
    VALUE1,
    #[doc = "Supported, frames with wrong FCS and additional nibble will be counted separately in Forwarded RX Error Counter"]
    VALUE2,
}
impl SH_FCSER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SH_FCSER::VALUE1 => false,
            SH_FCSER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SH_FCSER {
        match value {
            false => SH_FCSER::VALUE1,
            true => SH_FCSER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SH_FCSER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SH_FCSER::VALUE2
    }
}
#[doc = "Possible values of the field `EDC_SYNCA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDC_SYNCAR {
    #[doc = "Not available"]
    VALUE1,
    #[doc = "Available"]
    VALUE2,
}
impl EDC_SYNCAR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EDC_SYNCAR::VALUE1 => false,
            EDC_SYNCAR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDC_SYNCAR {
        match value {
            false => EDC_SYNCAR::VALUE1,
            true => EDC_SYNCAR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EDC_SYNCAR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EDC_SYNCAR::VALUE2
    }
}
#[doc = "Possible values of the field `LRW_CS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRW_CSR {
    #[doc = "Supported"]
    VALUE1,
    #[doc = "Not supported"]
    VALUE2,
}
impl LRW_CSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LRW_CSR::VALUE1 => false,
            LRW_CSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LRW_CSR {
        match value {
            false => LRW_CSR::VALUE1,
            true => LRW_CSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LRW_CSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LRW_CSR::VALUE2
    }
}
#[doc = "Possible values of the field `RW_CS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RW_CSR {
    #[doc = "Supported"]
    VALUE1,
    #[doc = "Not supported"]
    VALUE2,
}
impl RW_CSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RW_CSR::VALUE1 => false,
            RW_CSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RW_CSR {
        match value {
            false => RW_CSR::VALUE1,
            true => RW_CSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RW_CSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RW_CSR::VALUE2
    }
}
#[doc = "Possible values of the field `FX_CONF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FX_CONFR {
    #[doc = "Variable configuration"]
    VALUE1,
    #[doc = "Fixed configuration (refer to documentation of supporting ESCs)"]
    VALUE2,
}
impl FX_CONFR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FX_CONFR::VALUE1 => false,
            FX_CONFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FX_CONFR {
        match value {
            false => FX_CONFR::VALUE1,
            true => FX_CONFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FX_CONFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FX_CONFR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - FMMU Operation"]
    #[inline]
    pub fn fmmu(&self) -> FMMUR {
        FMMUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Distributed Clocks"]
    #[inline]
    pub fn clks(&self) -> CLKSR {
        CLKSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - Distributed Clocks (width)"]
    #[inline]
    pub fn clks_w(&self) -> CLKS_WR {
        CLKS_WR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Low Jitter EBUS"]
    #[inline]
    pub fn lj_ebus(&self) -> LJ_EBUSR {
        LJ_EBUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Enhanced Link Detection EBUS"]
    #[inline]
    pub fn eld_ebus(&self) -> ELD_EBUSR {
        ELD_EBUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - Enhanced Link Detection MII"]
    #[inline]
    pub fn eld_mii(&self) -> ELD_MIIR {
        ELD_MIIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Separate Handling of FCS Errors"]
    #[inline]
    pub fn sh_fcse(&self) -> SH_FCSER {
        SH_FCSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - Enhanced DC SYNC Activation"]
    #[inline]
    pub fn edc_synca(&self) -> EDC_SYNCAR {
        EDC_SYNCAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 9 - EtherCAT LRW command support"]
    #[inline]
    pub fn lrw_cs(&self) -> LRW_CSR {
        LRW_CSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 10 - EtherCAT read/write command support (BRW, APRW, FPRW)"]
    #[inline]
    pub fn rw_cs(&self) -> RW_CSR {
        RW_CSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 11 - Fixed FMMU/SyncManager configuration"]
    #[inline]
    pub fn fx_conf(&self) -> FX_CONFR {
        FX_CONFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
}
