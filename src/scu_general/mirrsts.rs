#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MIRRSTS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `HDCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDCLRR {
    #[doc = "Ready"]
    CONST_0,
    #[doc = "Busy"]
    CONST_1,
}
impl HDCLRR {
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
            HDCLRR::CONST_0 => false,
            HDCLRR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HDCLRR {
        match value {
            false => HDCLRR::CONST_0,
            true => HDCLRR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == HDCLRR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == HDCLRR::CONST_1
    }
}
#[doc = "Possible values of the field `HDSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDSETR {
    #[doc = "Ready"]
    CONST_0,
    #[doc = "Busy"]
    CONST_1,
}
impl HDSETR {
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
            HDSETR::CONST_0 => false,
            HDSETR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HDSETR {
        match value {
            false => HDSETR::CONST_0,
            true => HDSETR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == HDSETR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == HDSETR::CONST_1
    }
}
#[doc = "Possible values of the field `HDCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDCRR {
    #[doc = "Ready"]
    CONST_0,
    #[doc = "Busy"]
    CONST_1,
}
impl HDCRR {
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
            HDCRR::CONST_0 => false,
            HDCRR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HDCRR {
        match value {
            false => HDCRR::CONST_0,
            true => HDCRR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == HDCRR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == HDCRR::CONST_1
    }
}
#[doc = "Possible values of the field `OSCSICTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSICTRLR {
    #[doc = "Ready"]
    CONST_0,
    #[doc = "Busy"]
    CONST_1,
}
impl OSCSICTRLR {
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
            OSCSICTRLR::CONST_0 => false,
            OSCSICTRLR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSCSICTRLR {
        match value {
            false => OSCSICTRLR::CONST_0,
            true => OSCSICTRLR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == OSCSICTRLR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == OSCSICTRLR::CONST_1
    }
}
#[doc = "Possible values of the field `OSCULCTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCULCTRLR {
    #[doc = "Ready"]
    CONST_0,
    #[doc = "Busy"]
    CONST_1,
}
impl OSCULCTRLR {
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
            OSCULCTRLR::CONST_0 => false,
            OSCULCTRLR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSCULCTRLR {
        match value {
            false => OSCULCTRLR::CONST_0,
            true => OSCULCTRLR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == OSCULCTRLR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == OSCULCTRLR::CONST_1
    }
}
#[doc = "Possible values of the field `RTC_CTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_CTRR {
    #[doc = "Ready"]
    CONST_0,
    #[doc = "Busy"]
    CONST_1,
}
impl RTC_CTRR {
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
            RTC_CTRR::CONST_0 => false,
            RTC_CTRR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_CTRR {
        match value {
            false => RTC_CTRR::CONST_0,
            true => RTC_CTRR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == RTC_CTRR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == RTC_CTRR::CONST_1
    }
}
#[doc = "Possible values of the field `RTC_ATIM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_ATIM0R {
    #[doc = "Ready"]
    CONST_0,
    #[doc = "Busy"]
    CONST_1,
}
impl RTC_ATIM0R {
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
            RTC_ATIM0R::CONST_0 => false,
            RTC_ATIM0R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_ATIM0R {
        match value {
            false => RTC_ATIM0R::CONST_0,
            true => RTC_ATIM0R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == RTC_ATIM0R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == RTC_ATIM0R::CONST_1
    }
}
#[doc = "Possible values of the field `RTC_ATIM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_ATIM1R {
    #[doc = "Ready"]
    CONST_0,
    #[doc = "Busy"]
    CONST_1,
}
impl RTC_ATIM1R {
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
            RTC_ATIM1R::CONST_0 => false,
            RTC_ATIM1R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_ATIM1R {
        match value {
            false => RTC_ATIM1R::CONST_0,
            true => RTC_ATIM1R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == RTC_ATIM1R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == RTC_ATIM1R::CONST_1
    }
}
#[doc = "Possible values of the field `RTC_TIM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_TIM0R {
    #[doc = "Ready"]
    CONST_0,
    #[doc = "Busy"]
    CONST_1,
}
impl RTC_TIM0R {
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
            RTC_TIM0R::CONST_0 => false,
            RTC_TIM0R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_TIM0R {
        match value {
            false => RTC_TIM0R::CONST_0,
            true => RTC_TIM0R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == RTC_TIM0R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == RTC_TIM0R::CONST_1
    }
}
#[doc = "Possible values of the field `RTC_TIM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_TIM1R {
    #[doc = "Ready"]
    CONST_0,
    #[doc = "Busy"]
    CONST_1,
}
impl RTC_TIM1R {
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
            RTC_TIM1R::CONST_0 => false,
            RTC_TIM1R::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_TIM1R {
        match value {
            false => RTC_TIM1R::CONST_0,
            true => RTC_TIM1R::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == RTC_TIM1R::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == RTC_TIM1R::CONST_1
    }
}
#[doc = "Possible values of the field `RMX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMXR {
    #[doc = "Ready"]
    CONST_0,
    #[doc = "Busy"]
    CONST_1,
}
impl RMXR {
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
            RMXR::CONST_0 => false,
            RMXR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RMXR {
        match value {
            false => RMXR::CONST_0,
            true => RMXR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == RMXR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == RMXR::CONST_1
    }
}
#[doc = "Possible values of the field `RTC_MSKSR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_MSKSRR {
    #[doc = "Ready"]
    CONST_0,
    #[doc = "Busy"]
    CONST_1,
}
impl RTC_MSKSRR {
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
            RTC_MSKSRR::CONST_0 => false,
            RTC_MSKSRR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_MSKSRR {
        match value {
            false => RTC_MSKSRR::CONST_0,
            true => RTC_MSKSRR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == RTC_MSKSRR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == RTC_MSKSRR::CONST_1
    }
}
#[doc = "Possible values of the field `RTC_CLRSR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_CLRSRR {
    #[doc = "Ready"]
    CONST_0,
    #[doc = "Busy"]
    CONST_1,
}
impl RTC_CLRSRR {
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
            RTC_CLRSRR::CONST_0 => false,
            RTC_CLRSRR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_CLRSRR {
        match value {
            false => RTC_CLRSRR::CONST_0,
            true => RTC_CLRSRR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == RTC_CLRSRR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == RTC_CLRSRR::CONST_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - HDCLR Mirror Register Write Status"]
    #[inline]
    pub fn hdclr(&self) -> HDCLRR {
        HDCLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - HDSET Mirror Register Write Status"]
    #[inline]
    pub fn hdset(&self) -> HDSETR {
        HDSETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - HDCR Mirror Register Write Status"]
    #[inline]
    pub fn hdcr(&self) -> HDCRR {
        HDCRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - OSCSICTRL Mirror Register Write Status"]
    #[inline]
    pub fn oscsictrl(&self) -> OSCSICTRLR {
        OSCSICTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - OSCULCTRL Mirror Register Write Status"]
    #[inline]
    pub fn osculctrl(&self) -> OSCULCTRLR {
        OSCULCTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - RTC CTR Mirror Register Write Status"]
    #[inline]
    pub fn rtc_ctr(&self) -> RTC_CTRR {
        RTC_CTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - RTC ATIM0 Mirror Register Write Status"]
    #[inline]
    pub fn rtc_atim0(&self) -> RTC_ATIM0R {
        RTC_ATIM0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - RTC ATIM1 Mirror Register Write Status"]
    #[inline]
    pub fn rtc_atim1(&self) -> RTC_ATIM1R {
        RTC_ATIM1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - RTC TIM0 Mirror Register Write Status"]
    #[inline]
    pub fn rtc_tim0(&self) -> RTC_TIM0R {
        RTC_TIM0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - RTC TIM1 Mirror Register Write Status"]
    #[inline]
    pub fn rtc_tim1(&self) -> RTC_TIM1R {
        RTC_TIM1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Retention Memory Access Register Update Status"]
    #[inline]
    pub fn rmx(&self) -> RMXR {
        RMXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - RTC MSKSSR Mirror Register Write Status"]
    #[inline]
    pub fn rtc_msksr(&self) -> RTC_MSKSRR {
        RTC_MSKSRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - RTC CLRSR Mirror Register Write Status"]
    #[inline]
    pub fn rtc_clrsr(&self) -> RTC_CLRSRR {
        RTC_CLRSRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
