#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HDSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `EPEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPEVR {
    #[doc = "Wake-up on positive edge pin event inactive"]
    CONST_0,
    #[doc = "Wake-up on positive edge pin event active"]
    CONST_1,
}
impl EPEVR {
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
            EPEVR::CONST_0 => false,
            EPEVR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPEVR {
        match value {
            false => EPEVR::CONST_0,
            true => EPEVR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == EPEVR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == EPEVR::CONST_1
    }
}
#[doc = "Possible values of the field `ENEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENEVR {
    #[doc = "Wake-up on negative edge pin event inactive"]
    CONST_0,
    #[doc = "Wake-up on negative edge pin event active"]
    CONST_1,
}
impl ENEVR {
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
            ENEVR::CONST_0 => false,
            ENEVR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENEVR {
        match value {
            false => ENEVR::CONST_0,
            true => ENEVR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == ENEVR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == ENEVR::CONST_1
    }
}
#[doc = "Possible values of the field `RTCEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCEVR {
    #[doc = "Wake-up on RTC event inactive"]
    CONST_0,
    #[doc = "Wake-up on RTC event active"]
    CONST_1,
}
impl RTCEVR {
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
            RTCEVR::CONST_0 => false,
            RTCEVR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTCEVR {
        match value {
            false => RTCEVR::CONST_0,
            true => RTCEVR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == RTCEVR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == RTCEVR::CONST_1
    }
}
#[doc = "Possible values of the field `ULPWDG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULPWDGR {
    #[doc = "Watchdog alarm did not occur"]
    CONST_0,
    #[doc = "Watchdog alarm occurred"]
    CONST_1,
}
impl ULPWDGR {
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
            ULPWDGR::CONST_0 => false,
            ULPWDGR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ULPWDGR {
        match value {
            false => ULPWDGR::CONST_0,
            true => ULPWDGR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == ULPWDGR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == ULPWDGR::CONST_1
    }
}
#[doc = "Possible values of the field `HIBNOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBNOUTR {
    #[doc = "Hibernate not driven active to pads"]
    CONST_0,
    #[doc = "Hibernate driven active to pads"]
    CONST_1,
}
impl HIBNOUTR {
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
            HIBNOUTR::CONST_0 => false,
            HIBNOUTR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIBNOUTR {
        match value {
            false => HIBNOUTR::CONST_0,
            true => HIBNOUTR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == HIBNOUTR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == HIBNOUTR::CONST_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Wake-up Pin Event Positive Edge"]
    #[inline]
    pub fn epev(&self) -> EPEVR {
        EPEVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Wake-up Pin Event Negative Edge"]
    #[inline]
    pub fn enev(&self) -> ENEVR {
        ENEVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - RTC Event"]
    #[inline]
    pub fn rtcev(&self) -> RTCEVR {
        RTCEVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - ULP WDG Alarm Status"]
    #[inline]
    pub fn ulpwdg(&self) -> ULPWDGR {
        ULPWDGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Hibernate Control Status"]
    #[inline]
    pub fn hibnout(&self) -> HIBNOUTR {
        HIBNOUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
