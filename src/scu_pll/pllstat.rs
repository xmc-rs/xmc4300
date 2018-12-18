#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PLLSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `VCOBYST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOBYSTR {
    #[doc = "Free-running / Normal Mode is entered"]
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
    #[doc = "PLL not locked"]
    CONST_0,
    #[doc = "PLL locked"]
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
#[doc = "Possible values of the field `K1RDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum K1RDYR {
    #[doc = "K1-Divider does not operate with the new value"]
    CONST_0,
    #[doc = "K1-Divider operate with the new value"]
    CONST_1,
}
impl K1RDYR {
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
            K1RDYR::CONST_0 => false,
            K1RDYR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> K1RDYR {
        match value {
            false => K1RDYR::CONST_0,
            true => K1RDYR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == K1RDYR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == K1RDYR::CONST_1
    }
}
#[doc = "Possible values of the field `K2RDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum K2RDYR {
    #[doc = "K2-Divider does not operate with the new value"]
    CONST_0,
    #[doc = "K2-Divider operate with the new value"]
    CONST_1,
}
impl K2RDYR {
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
            K2RDYR::CONST_0 => false,
            K2RDYR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> K2RDYR {
        match value {
            false => K2RDYR::CONST_0,
            true => K2RDYR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == K2RDYR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == K2RDYR::CONST_1
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
#[doc = "Possible values of the field `PLLLV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLLVR {
    #[doc = "The OSC frequency is not usable. Frequency fREF is too low."]
    CONST_0,
    #[doc = "The OSC frequency is usable"]
    CONST_1,
}
impl PLLLVR {
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
            PLLLVR::CONST_0 => false,
            PLLLVR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLLVR {
        match value {
            false => PLLLVR::CONST_0,
            true => PLLLVR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PLLLVR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PLLLVR::CONST_1
    }
}
#[doc = "Possible values of the field `PLLHV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLHVR {
    #[doc = "The OSC frequency is not usable. Frequency fOSC is too high."]
    CONST_0,
    #[doc = "The OSC frequency is usable"]
    CONST_1,
}
impl PLLHVR {
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
            PLLHVR::CONST_0 => false,
            PLLHVR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLHVR {
        match value {
            false => PLLHVR::CONST_0,
            true => PLLHVR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PLLHVR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PLLHVR::CONST_1
    }
}
#[doc = "Possible values of the field `PLLSP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSPR {
    #[doc = "The OSC frequency is not usable. Spikes are detected that disturb a locked operation"]
    CONST_0,
    #[doc = "The OSC frequency is usable"]
    CONST_1,
}
impl PLLSPR {
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
            PLLSPR::CONST_0 => false,
            PLLSPR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLSPR {
        match value {
            false => PLLSPR::CONST_0,
            true => PLLSPR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PLLSPR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PLLSPR::CONST_1
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
    #[doc = "Bit 2 - PLL LOCK Status"]
    #[inline]
    pub fn vcolock(&self) -> VCOLOCKR {
        VCOLOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - K1 Divider Ready Status"]
    #[inline]
    pub fn k1rdy(&self) -> K1RDYR {
        K1RDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - K2 Divider Ready Status"]
    #[inline]
    pub fn k2rdy(&self) -> K2RDYR {
        K2RDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
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
    #[doc = "Bit 7 - Oscillator for PLL Valid Low Status Bit"]
    #[inline]
    pub fn plllv(&self) -> PLLLVR {
        PLLLVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Oscillator for PLL Valid High Status Bit"]
    #[inline]
    pub fn pllhv(&self) -> PLLHVR {
        PLLHVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Oscillator for PLL Valid Spike Status Bit"]
    #[inline]
    pub fn pllsp(&self) -> PLLSPR {
        PLLSPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
