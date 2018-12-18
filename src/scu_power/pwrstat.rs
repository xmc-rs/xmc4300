#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PWRSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `HIBEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBENR {
    #[doc = "Inactive"]
    CONST_0,
    #[doc = "Active"]
    CONST_1,
}
impl HIBENR {
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
            HIBENR::CONST_0 => false,
            HIBENR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIBENR {
        match value {
            false => HIBENR::CONST_0,
            true => HIBENR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == HIBENR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == HIBENR::CONST_1
    }
}
#[doc = "Possible values of the field `USBPHYPDQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPHYPDQR {
    #[doc = "Power-down"]
    CONST_0,
    #[doc = "Active"]
    CONST_1,
}
impl USBPHYPDQR {
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
            USBPHYPDQR::CONST_0 => false,
            USBPHYPDQR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBPHYPDQR {
        match value {
            false => USBPHYPDQR::CONST_0,
            true => USBPHYPDQR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == USBPHYPDQR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == USBPHYPDQR::CONST_1
    }
}
#[doc = "Possible values of the field `USBOTGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBOTGENR {
    #[doc = "Power-down"]
    CONST_0,
    #[doc = "Active"]
    CONST_1,
}
impl USBOTGENR {
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
            USBOTGENR::CONST_0 => false,
            USBOTGENR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBOTGENR {
        match value {
            false => USBOTGENR::CONST_0,
            true => USBOTGENR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == USBOTGENR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == USBOTGENR::CONST_1
    }
}
#[doc = "Possible values of the field `USBPUWQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPUWQR {
    #[doc = "Pull-up active"]
    CONST_0,
    #[doc = "Pull-up not active"]
    CONST_1,
}
impl USBPUWQR {
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
            USBPUWQR::CONST_0 => false,
            USBPUWQR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBPUWQR {
        match value {
            false => USBPUWQR::CONST_0,
            true => USBPUWQR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == USBPUWQR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == USBPUWQR::CONST_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Hibernate Domain Enable Status"]
    #[inline]
    pub fn hiben(&self) -> HIBENR {
        HIBENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - USB PHY Transceiver State"]
    #[inline]
    pub fn usbphypdq(&self) -> USBPHYPDQR {
        USBPHYPDQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - USB On-The-Go Comparators State"]
    #[inline]
    pub fn usbotgen(&self) -> USBOTGENR {
        USBOTGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - USB Weak Pull-Up at PADN State"]
    #[inline]
    pub fn usbpuwq(&self) -> USBPUWQR {
        USBPUWQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
