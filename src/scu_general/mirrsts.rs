#[doc = "Reader of register MIRRSTS"]
pub type R = crate::R<u32, super::MIRRSTS>;
#[doc = "HDCLR Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDCLR_A {
    #[doc = "0: Ready"]
    CONST_0,
    #[doc = "1: Busy"]
    CONST_1,
}
impl From<HDCLR_A> for bool {
    #[inline(always)]
    fn from(variant: HDCLR_A) -> Self {
        match variant {
            HDCLR_A::CONST_0 => false,
            HDCLR_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `HDCLR`"]
pub type HDCLR_R = crate::R<bool, HDCLR_A>;
impl HDCLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDCLR_A {
        match self.bits {
            false => HDCLR_A::CONST_0,
            true => HDCLR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == HDCLR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == HDCLR_A::CONST_1
    }
}
#[doc = "HDSET Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDSET_A {
    #[doc = "0: Ready"]
    CONST_0,
    #[doc = "1: Busy"]
    CONST_1,
}
impl From<HDSET_A> for bool {
    #[inline(always)]
    fn from(variant: HDSET_A) -> Self {
        match variant {
            HDSET_A::CONST_0 => false,
            HDSET_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `HDSET`"]
pub type HDSET_R = crate::R<bool, HDSET_A>;
impl HDSET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDSET_A {
        match self.bits {
            false => HDSET_A::CONST_0,
            true => HDSET_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == HDSET_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == HDSET_A::CONST_1
    }
}
#[doc = "HDCR Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDCR_A {
    #[doc = "0: Ready"]
    CONST_0,
    #[doc = "1: Busy"]
    CONST_1,
}
impl From<HDCR_A> for bool {
    #[inline(always)]
    fn from(variant: HDCR_A) -> Self {
        match variant {
            HDCR_A::CONST_0 => false,
            HDCR_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `HDCR`"]
pub type HDCR_R = crate::R<bool, HDCR_A>;
impl HDCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDCR_A {
        match self.bits {
            false => HDCR_A::CONST_0,
            true => HDCR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == HDCR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == HDCR_A::CONST_1
    }
}
#[doc = "OSCSICTRL Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSICTRL_A {
    #[doc = "0: Ready"]
    CONST_0,
    #[doc = "1: Busy"]
    CONST_1,
}
impl From<OSCSICTRL_A> for bool {
    #[inline(always)]
    fn from(variant: OSCSICTRL_A) -> Self {
        match variant {
            OSCSICTRL_A::CONST_0 => false,
            OSCSICTRL_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `OSCSICTRL`"]
pub type OSCSICTRL_R = crate::R<bool, OSCSICTRL_A>;
impl OSCSICTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCSICTRL_A {
        match self.bits {
            false => OSCSICTRL_A::CONST_0,
            true => OSCSICTRL_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == OSCSICTRL_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == OSCSICTRL_A::CONST_1
    }
}
#[doc = "OSCULCTRL Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCULCTRL_A {
    #[doc = "0: Ready"]
    CONST_0,
    #[doc = "1: Busy"]
    CONST_1,
}
impl From<OSCULCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: OSCULCTRL_A) -> Self {
        match variant {
            OSCULCTRL_A::CONST_0 => false,
            OSCULCTRL_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `OSCULCTRL`"]
pub type OSCULCTRL_R = crate::R<bool, OSCULCTRL_A>;
impl OSCULCTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCULCTRL_A {
        match self.bits {
            false => OSCULCTRL_A::CONST_0,
            true => OSCULCTRL_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == OSCULCTRL_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == OSCULCTRL_A::CONST_1
    }
}
#[doc = "RTC CTR Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_CTR_A {
    #[doc = "0: Ready"]
    CONST_0,
    #[doc = "1: Busy"]
    CONST_1,
}
impl From<RTC_CTR_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_CTR_A) -> Self {
        match variant {
            RTC_CTR_A::CONST_0 => false,
            RTC_CTR_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `RTC_CTR`"]
pub type RTC_CTR_R = crate::R<bool, RTC_CTR_A>;
impl RTC_CTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_CTR_A {
        match self.bits {
            false => RTC_CTR_A::CONST_0,
            true => RTC_CTR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RTC_CTR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RTC_CTR_A::CONST_1
    }
}
#[doc = "RTC ATIM0 Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_ATIM0_A {
    #[doc = "0: Ready"]
    CONST_0,
    #[doc = "1: Busy"]
    CONST_1,
}
impl From<RTC_ATIM0_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_ATIM0_A) -> Self {
        match variant {
            RTC_ATIM0_A::CONST_0 => false,
            RTC_ATIM0_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `RTC_ATIM0`"]
pub type RTC_ATIM0_R = crate::R<bool, RTC_ATIM0_A>;
impl RTC_ATIM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_ATIM0_A {
        match self.bits {
            false => RTC_ATIM0_A::CONST_0,
            true => RTC_ATIM0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RTC_ATIM0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RTC_ATIM0_A::CONST_1
    }
}
#[doc = "RTC ATIM1 Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_ATIM1_A {
    #[doc = "0: Ready"]
    CONST_0,
    #[doc = "1: Busy"]
    CONST_1,
}
impl From<RTC_ATIM1_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_ATIM1_A) -> Self {
        match variant {
            RTC_ATIM1_A::CONST_0 => false,
            RTC_ATIM1_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `RTC_ATIM1`"]
pub type RTC_ATIM1_R = crate::R<bool, RTC_ATIM1_A>;
impl RTC_ATIM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_ATIM1_A {
        match self.bits {
            false => RTC_ATIM1_A::CONST_0,
            true => RTC_ATIM1_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RTC_ATIM1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RTC_ATIM1_A::CONST_1
    }
}
#[doc = "RTC TIM0 Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_TIM0_A {
    #[doc = "0: Ready"]
    CONST_0,
    #[doc = "1: Busy"]
    CONST_1,
}
impl From<RTC_TIM0_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_TIM0_A) -> Self {
        match variant {
            RTC_TIM0_A::CONST_0 => false,
            RTC_TIM0_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `RTC_TIM0`"]
pub type RTC_TIM0_R = crate::R<bool, RTC_TIM0_A>;
impl RTC_TIM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_TIM0_A {
        match self.bits {
            false => RTC_TIM0_A::CONST_0,
            true => RTC_TIM0_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RTC_TIM0_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RTC_TIM0_A::CONST_1
    }
}
#[doc = "RTC TIM1 Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_TIM1_A {
    #[doc = "0: Ready"]
    CONST_0,
    #[doc = "1: Busy"]
    CONST_1,
}
impl From<RTC_TIM1_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_TIM1_A) -> Self {
        match variant {
            RTC_TIM1_A::CONST_0 => false,
            RTC_TIM1_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `RTC_TIM1`"]
pub type RTC_TIM1_R = crate::R<bool, RTC_TIM1_A>;
impl RTC_TIM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_TIM1_A {
        match self.bits {
            false => RTC_TIM1_A::CONST_0,
            true => RTC_TIM1_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RTC_TIM1_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RTC_TIM1_A::CONST_1
    }
}
#[doc = "Retention Memory Access Register Update Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMX_A {
    #[doc = "0: Ready"]
    CONST_0,
    #[doc = "1: Busy"]
    CONST_1,
}
impl From<RMX_A> for bool {
    #[inline(always)]
    fn from(variant: RMX_A) -> Self {
        match variant {
            RMX_A::CONST_0 => false,
            RMX_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `RMX`"]
pub type RMX_R = crate::R<bool, RMX_A>;
impl RMX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMX_A {
        match self.bits {
            false => RMX_A::CONST_0,
            true => RMX_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RMX_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RMX_A::CONST_1
    }
}
#[doc = "RTC MSKSSR Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_MSKSR_A {
    #[doc = "0: Ready"]
    CONST_0,
    #[doc = "1: Busy"]
    CONST_1,
}
impl From<RTC_MSKSR_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_MSKSR_A) -> Self {
        match variant {
            RTC_MSKSR_A::CONST_0 => false,
            RTC_MSKSR_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `RTC_MSKSR`"]
pub type RTC_MSKSR_R = crate::R<bool, RTC_MSKSR_A>;
impl RTC_MSKSR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_MSKSR_A {
        match self.bits {
            false => RTC_MSKSR_A::CONST_0,
            true => RTC_MSKSR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RTC_MSKSR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RTC_MSKSR_A::CONST_1
    }
}
#[doc = "RTC CLRSR Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_CLRSR_A {
    #[doc = "0: Ready"]
    CONST_0,
    #[doc = "1: Busy"]
    CONST_1,
}
impl From<RTC_CLRSR_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_CLRSR_A) -> Self {
        match variant {
            RTC_CLRSR_A::CONST_0 => false,
            RTC_CLRSR_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `RTC_CLRSR`"]
pub type RTC_CLRSR_R = crate::R<bool, RTC_CLRSR_A>;
impl RTC_CLRSR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_CLRSR_A {
        match self.bits {
            false => RTC_CLRSR_A::CONST_0,
            true => RTC_CLRSR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RTC_CLRSR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RTC_CLRSR_A::CONST_1
    }
}
impl R {
    #[doc = "Bit 1 - HDCLR Mirror Register Write Status"]
    #[inline(always)]
    pub fn hdclr(&self) -> HDCLR_R {
        HDCLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HDSET Mirror Register Write Status"]
    #[inline(always)]
    pub fn hdset(&self) -> HDSET_R {
        HDSET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HDCR Mirror Register Write Status"]
    #[inline(always)]
    pub fn hdcr(&self) -> HDCR_R {
        HDCR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - OSCSICTRL Mirror Register Write Status"]
    #[inline(always)]
    pub fn oscsictrl(&self) -> OSCSICTRL_R {
        OSCSICTRL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - OSCULCTRL Mirror Register Write Status"]
    #[inline(always)]
    pub fn osculctrl(&self) -> OSCULCTRL_R {
        OSCULCTRL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RTC CTR Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_ctr(&self) -> RTC_CTR_R {
        RTC_CTR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RTC ATIM0 Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_atim0(&self) -> RTC_ATIM0_R {
        RTC_ATIM0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RTC ATIM1 Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_atim1(&self) -> RTC_ATIM1_R {
        RTC_ATIM1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RTC TIM0 Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_tim0(&self) -> RTC_TIM0_R {
        RTC_TIM0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RTC TIM1 Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_tim1(&self) -> RTC_TIM1_R {
        RTC_TIM1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Retention Memory Access Register Update Status"]
    #[inline(always)]
    pub fn rmx(&self) -> RMX_R {
        RMX_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RTC MSKSSR Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_msksr(&self) -> RTC_MSKSR_R {
        RTC_MSKSR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RTC CLRSR Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_clrsr(&self) -> RTC_CLRSR_R {
        RTC_CLRSR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
