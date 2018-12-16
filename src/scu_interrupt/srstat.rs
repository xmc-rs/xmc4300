#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SRSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `PRWARN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRWARNR {
    #[doc = "Inactive"]
    CONST_0,
    #[doc = "Active"]
    CONST_1,
}
impl PRWARNR {
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
            PRWARNR::CONST_0 => false,
            PRWARNR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRWARNR {
        match value {
            false => PRWARNR::CONST_0,
            true => PRWARNR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PRWARNR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PRWARNR::CONST_1
    }
}
#[doc = r" Value of the field"]
pub struct PIR {
    bits: bool,
}
impl PIR {
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
pub struct AIR {
    bits: bool,
}
impl AIR {
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
pub struct DLROVRR {
    bits: bool,
}
impl DLROVRR {
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
#[doc = "Possible values of the field `HDCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDCLRR {
    #[doc = "Not updated"]
    CONST_0,
    #[doc = "Update completed"]
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
    #[doc = "Not updated"]
    CONST_0,
    #[doc = "Update completed"]
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
    #[doc = "Not updated"]
    CONST_0,
    #[doc = "Update completed"]
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
    #[doc = "Not updated"]
    CONST_0,
    #[doc = "Update completed"]
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
    #[doc = "Not updated"]
    CONST_0,
    #[doc = "Update completed"]
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
    #[doc = "Not updated"]
    CONST_0,
    #[doc = "Update completed"]
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
    #[doc = "Not updated"]
    CONST_0,
    #[doc = "Update completed"]
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
    #[doc = "Not updated"]
    CONST_0,
    #[doc = "Update completed"]
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
    #[doc = "Not updated"]
    CONST_0,
    #[doc = "Update completed"]
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
    #[doc = "Not updated"]
    CONST_0,
    #[doc = "Update completed"]
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
    #[doc = "Not updated"]
    CONST_0,
    #[doc = "Update completed"]
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - WDT pre-warning Interrupt Status"]
    #[inline]
    pub fn prwarn(&self) -> PRWARNR {
        PRWARNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - RTC Periodic Interrupt Status"]
    #[inline]
    pub fn pi(&self) -> PIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIR { bits }
    }
    #[doc = "Bit 2 - Alarm Interrupt Status"]
    #[inline]
    pub fn ai(&self) -> AIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AIR { bits }
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt Status"]
    #[inline]
    pub fn dlrovr(&self) -> DLROVRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DLROVRR { bits }
    }
    #[doc = "Bit 17 - HDCLR Mirror Register Update Status"]
    #[inline]
    pub fn hdclr(&self) -> HDCLRR {
        HDCLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - HDSET Mirror Register Update Status"]
    #[inline]
    pub fn hdset(&self) -> HDSETR {
        HDSETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Status"]
    #[inline]
    pub fn hdcr(&self) -> HDCRR {
        HDCRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Status"]
    #[inline]
    pub fn oscsictrl(&self) -> OSCSICTRLR {
        OSCSICTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Status"]
    #[inline]
    pub fn osculctrl(&self) -> OSCULCTRLR {
        OSCULCTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Status"]
    #[inline]
    pub fn rtc_ctr(&self) -> RTC_CTRR {
        RTC_CTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Status"]
    #[inline]
    pub fn rtc_atim0(&self) -> RTC_ATIM0R {
        RTC_ATIM0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Status"]
    #[inline]
    pub fn rtc_atim1(&self) -> RTC_ATIM1R {
        RTC_ATIM1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Status"]
    #[inline]
    pub fn rtc_tim0(&self) -> RTC_TIM0R {
        RTC_TIM0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Status"]
    #[inline]
    pub fn rtc_tim1(&self) -> RTC_TIM1R {
        RTC_TIM1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Status"]
    #[inline]
    pub fn rmx(&self) -> RMXR {
        RMXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
