#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RESD {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RESULTR {
    bits: u16,
}
impl RESULTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DRCR {
    bits: u8,
}
impl DRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CHNRR {
    bits: u8,
}
impl CHNRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EMUXR {
    bits: u8,
}
impl EMUXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRSR {
    #[doc = "Request source 0"]
    VALUE1,
    #[doc = "Request source 1"]
    VALUE2,
    #[doc = "Request source 2"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CRSR::VALUE1 => 0,
            CRSR::VALUE2 => 1,
            CRSR::VALUE3 => 2,
            CRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CRSR {
        match value {
            0 => CRSR::VALUE1,
            1 => CRSR::VALUE2,
            2 => CRSR::VALUE3,
            i => CRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CRSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CRSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CRSR::VALUE3
    }
}
#[doc = "Possible values of the field `FCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCRR {
    #[doc = "Signal level was below compare value"]
    VALUE1,
    #[doc = "Signal level was above compare value"]
    VALUE2,
}
impl FCRR {
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
            FCRR::VALUE1 => false,
            FCRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FCRR {
        match value {
            false => FCRR::VALUE1,
            true => FCRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FCRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FCRR::VALUE2
    }
}
#[doc = "Possible values of the field `VF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VFR {
    #[doc = "No new result available"]
    VALUE1,
    #[doc = "Bitfield RESULT has been updated with new result value and has not yet been read, or bit FCR has been updated"]
    VALUE2,
}
impl VFR {
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
            VFR::VALUE1 => false,
            VFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VFR {
        match value {
            false => VFR::VALUE1,
            true => VFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VFR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Result of Most Recent Conversion"]
    #[inline]
    pub fn result(&self) -> RESULTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESULTR { bits }
    }
    #[doc = "Bits 16:19 - Data Reduction Counter"]
    #[inline]
    pub fn drc(&self) -> DRCR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DRCR { bits }
    }
    #[doc = "Bits 20:24 - Channel Number"]
    #[inline]
    pub fn chnr(&self) -> CHNRR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CHNRR { bits }
    }
    #[doc = "Bits 25:27 - External Multiplexer Setting"]
    #[inline]
    pub fn emux(&self) -> EMUXR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EMUXR { bits }
    }
    #[doc = "Bits 28:29 - Converted Request Source"]
    #[inline]
    pub fn crs(&self) -> CRSR {
        CRSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 30 - Fast Compare Result"]
    #[inline]
    pub fn fcr(&self) -> FCRR {
        FCRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Valid Flag"]
    #[inline]
    pub fn vf(&self) -> VFR {
        VFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
