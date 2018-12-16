#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::MII_ECAT_ACS_STATE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `EN_ACS_MII_BY_PDI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_ACS_MII_BY_PDIR {
    #[doc = "ECAT enables PDI takeover of MII management control"]
    VALUE1,
    #[doc = "ECAT claims exclusive access to MII management"]
    VALUE2,
}
impl EN_ACS_MII_BY_PDIR {
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
            EN_ACS_MII_BY_PDIR::VALUE1 => false,
            EN_ACS_MII_BY_PDIR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_ACS_MII_BY_PDIR {
        match value {
            false => EN_ACS_MII_BY_PDIR::VALUE1,
            true => EN_ACS_MII_BY_PDIR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EN_ACS_MII_BY_PDIR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EN_ACS_MII_BY_PDIR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Access to MII management"]
    #[inline]
    pub fn en_acs_mii_by_pdi(&self) -> EN_ACS_MII_BY_PDIR {
        EN_ACS_MII_BY_PDIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
