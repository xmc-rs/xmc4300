#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DSTS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct SUSPSTSR {
    bits: bool,
}
impl SUSPSTSR {
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
#[doc = "Possible values of the field `EnumSpd`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENUMSPDR {
    #[doc = "Full speed (PHY clock is running at 48 MHz)"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ENUMSPDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENUMSPDR::VALUE4 => 3,
            ENUMSPDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENUMSPDR {
        match value {
            3 => ENUMSPDR::VALUE4,
            i => ENUMSPDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == ENUMSPDR::VALUE4
    }
}
#[doc = r" Value of the field"]
pub struct ERRTICERRR {
    bits: bool,
}
impl ERRTICERRR {
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
#[doc = r" Value of the field"]
pub struct SOFFNR {
    bits: u16,
}
impl SOFFNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Suspend Status"]
    #[inline]
    pub fn susp_sts(&self) -> SUSPSTSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SUSPSTSR { bits }
    }
    #[doc = "Bits 1:2 - Enumerated Speed"]
    #[inline]
    pub fn enum_spd(&self) -> ENUMSPDR {
        ENUMSPDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Erratic Error"]
    #[inline]
    pub fn errtic_err(&self) -> ERRTICERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ERRTICERRR { bits }
    }
    #[doc = "Bits 8:21 - Frame Number of the Received SOF"]
    #[inline]
    pub fn soffn(&self) -> SOFFNR {
        let bits = {
            const MASK: u16 = 16383;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SOFFNR { bits }
    }
}
