#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ECRD1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct CAPVR {
    bits: u16,
}
impl CAPVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FPCVR {
    bits: u8,
}
impl FPCVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SPTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPTRR {
    #[doc = "CC40"]
    VALUE1,
    #[doc = "CC41"]
    VALUE2,
    #[doc = "CC42"]
    VALUE3,
    #[doc = "CC43"]
    VALUE4,
}
impl SPTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPTRR::VALUE1 => 0,
            SPTRR::VALUE2 => 1,
            SPTRR::VALUE3 => 2,
            SPTRR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPTRR {
        match value {
            0 => SPTRR::VALUE1,
            1 => SPTRR::VALUE2,
            2 => SPTRR::VALUE3,
            3 => SPTRR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SPTRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SPTRR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SPTRR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == SPTRR::VALUE4
    }
}
#[doc = "Possible values of the field `VPTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VPTRR {
    #[doc = "Capture register 0"]
    VALUE1,
    #[doc = "Capture register 1"]
    VALUE2,
    #[doc = "Capture register 2"]
    VALUE3,
    #[doc = "Capture register 3"]
    VALUE4,
}
impl VPTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VPTRR::VALUE1 => 0,
            VPTRR::VALUE2 => 1,
            VPTRR::VALUE3 => 2,
            VPTRR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VPTRR {
        match value {
            0 => VPTRR::VALUE1,
            1 => VPTRR::VALUE2,
            2 => VPTRR::VALUE3,
            3 => VPTRR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VPTRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VPTRR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == VPTRR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == VPTRR::VALUE4
    }
}
#[doc = "Possible values of the field `FFL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFLR {
    #[doc = "No new value was captured into this register"]
    VALUE1,
    #[doc = "A new value has been captured into this register"]
    VALUE2,
}
impl FFLR {
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
            FFLR::VALUE1 => false,
            FFLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FFLR {
        match value {
            false => FFLR::VALUE1,
            true => FFLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FFLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FFLR::VALUE2
    }
}
#[doc = "Possible values of the field `LCV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCVR {
    #[doc = "No capture was lost"]
    VALUE1,
    #[doc = "A capture was lost"]
    VALUE2,
}
impl LCVR {
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
            LCVR::VALUE1 => false,
            LCVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LCVR {
        match value {
            false => LCVR::VALUE1,
            true => LCVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LCVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LCVR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Timer Capture Value"]
    #[inline]
    pub fn capv(&self) -> CAPVR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CAPVR { bits }
    }
    #[doc = "Bits 16:19 - Prescaler Capture value"]
    #[inline]
    pub fn fpcv(&self) -> FPCVR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FPCVR { bits }
    }
    #[doc = "Bits 20:21 - Slice pointer"]
    #[inline]
    pub fn sptr(&self) -> SPTRR {
        SPTRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Capture register pointer"]
    #[inline]
    pub fn vptr(&self) -> VPTRR {
        VPTRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - Full Flag"]
    #[inline]
    pub fn ffl(&self) -> FFLR {
        FFLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Lost Capture Value"]
    #[inline]
    pub fn lcv(&self) -> LCVR {
        LCVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
