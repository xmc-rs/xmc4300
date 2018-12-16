#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::WD_STAT_PDATA {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `WD_STAT_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WD_STAT_PDR {
    #[doc = "Watchdog Process Data expired"]
    VALUE1,
    #[doc = "Watchdog Process Data is active or disabled"]
    VALUE2,
}
impl WD_STAT_PDR {
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
            WD_STAT_PDR::VALUE1 => false,
            WD_STAT_PDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WD_STAT_PDR {
        match value {
            false => WD_STAT_PDR::VALUE1,
            true => WD_STAT_PDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WD_STAT_PDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WD_STAT_PDR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Watchdog Status of Process Data"]
    #[inline]
    pub fn wd_stat_pd(&self) -> WD_STAT_PDR {
        WD_STAT_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
}
