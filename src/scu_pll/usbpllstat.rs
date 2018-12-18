#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::USBPLLSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `VCOBYST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOBYSTR {
    #[doc = "Normal Mode is entered"]
    CONST_0,
    #[doc = "Prescaler Mode is entered"]
    CONST_1,
}
impl VCOBYSTR {
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
            VCOBYSTR::CONST_0 => false,
            VCOBYSTR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VCOBYSTR {
        match value {
            false => VCOBYSTR::CONST_0,
            true => VCOBYSTR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == VCOBYSTR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == VCOBYSTR::CONST_1
    }
}
#[doc = "Possible values of the field `PWDSTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWDSTATR {
    #[doc = "PLL Power-saving Mode was not entered"]
    CONST_0,
    #[doc = "PLL Power-saving Mode was entered"]
    CONST_1,
}
impl PWDSTATR {
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
            PWDSTATR::CONST_0 => false,
            PWDSTATR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWDSTATR {
        match value {
            false => PWDSTATR::CONST_0,
            true => PWDSTATR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PWDSTATR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PWDSTATR::CONST_1
    }
}
#[doc = "Possible values of the field `VCOLOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOLOCKR {
    #[doc = "The frequency difference of fREF and fDIV is greater than allowed. The VCO part of the PLL can not lock on a target frequency."]
    CONST_0,
    #[doc = "The frequency difference of fREF and fDIV is small enough to enable a stable VCO operation"]
    CONST_1,
}
impl VCOLOCKR {
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
            VCOLOCKR::CONST_0 => false,
            VCOLOCKR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VCOLOCKR {
        match value {
            false => VCOLOCKR::CONST_0,
            true => VCOLOCKR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == VCOLOCKR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == VCOLOCKR::CONST_1
    }
}
#[doc = "Possible values of the field `BY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYR {
    #[doc = "Bypass Mode is not entered"]
    CONST_0,
    #[doc = "Bypass Mode is entered. Input fOSC is selected as output fPLL."]
    CONST_1,
}
impl BYR {
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
            BYR::CONST_0 => false,
            BYR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BYR {
        match value {
            false => BYR::CONST_0,
            true => BYR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == BYR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == BYR::CONST_1
    }
}
#[doc = "Possible values of the field `VCOLOCKED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOLOCKEDR {
    #[doc = "PLL not locked"]
    CONST_0,
    #[doc = "PLL locked"]
    CONST_1,
}
impl VCOLOCKEDR {
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
            VCOLOCKEDR::CONST_0 => false,
            VCOLOCKEDR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VCOLOCKEDR {
        match value {
            false => VCOLOCKEDR::CONST_0,
            true => VCOLOCKEDR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == VCOLOCKEDR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == VCOLOCKEDR::CONST_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - VCO Bypass Status"]
    #[inline]
    pub fn vcobyst(&self) -> VCOBYSTR {
        VCOBYSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - PLL Power-saving Mode Status"]
    #[inline]
    pub fn pwdstat(&self) -> PWDSTATR {
        PWDSTATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - PLL VCO Lock Status"]
    #[inline]
    pub fn vcolock(&self) -> VCOLOCKR {
        VCOLOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Bypass Mode Status"]
    #[inline]
    pub fn by(&self) -> BYR {
        BYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - PLL LOCK Status"]
    #[inline]
    pub fn vcolocked(&self) -> VCOLOCKEDR {
        VCOLOCKEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
