#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::SM_ACT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `SM_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM_ENR {
    #[doc = "Disable: Access to Memory without SyncManager control"]
    VALUE1,
    #[doc = "Enable: SyncManager is active and controls Memory area set in configuration"]
    VALUE2,
}
impl SM_ENR {
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
            SM_ENR::VALUE1 => false,
            SM_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SM_ENR {
        match value {
            false => SM_ENR::VALUE1,
            true => SM_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SM_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SM_ENR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct REP_REQR {
    bits: bool,
}
impl REP_REQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `LE_ECAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LE_ECATR {
    #[doc = "No"]
    VALUE1,
    #[doc = "Generate Latch event if EtherCAT master issues a buffer exchange"]
    VALUE2,
}
impl LE_ECATR {
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
            LE_ECATR::VALUE1 => false,
            LE_ECATR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LE_ECATR {
        match value {
            false => LE_ECATR::VALUE1,
            true => LE_ECATR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LE_ECATR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LE_ECATR::VALUE2
    }
}
#[doc = "Possible values of the field `LE_PDI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LE_PDIR {
    #[doc = "No"]
    VALUE1,
    #[doc = "Generate Latch events if PDI issues a buffer exchange or if PDI accesses buffer start address"]
    VALUE2,
}
impl LE_PDIR {
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
            LE_PDIR::VALUE1 => false,
            LE_PDIR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LE_PDIR {
        match value {
            false => LE_PDIR::VALUE1,
            true => LE_PDIR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LE_PDIR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LE_PDIR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - SyncManager Enable/Disable"]
    #[inline]
    pub fn sm_en(&self) -> SM_ENR {
        SM_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Repeat Request"]
    #[inline]
    pub fn rep_req(&self) -> REP_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        REP_REQR { bits }
    }
    #[doc = "Bit 6 - LatchEvent ECAT"]
    #[inline]
    pub fn le_ecat(&self) -> LE_ECATR {
        LE_ECATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - LatchEvent PDI"]
    #[inline]
    pub fn le_pdi(&self) -> LE_PDIR {
        LE_PDIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
