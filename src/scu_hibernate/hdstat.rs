#[doc = "Register `HDSTAT` reader"]
pub type R = crate::R<HDSTAT_SPEC>;
#[doc = "Field `EPEV` reader - Wake-up Pin Event Positive Edge"]
pub type EPEV_R = crate::BitReader<EPEV_A>;
#[doc = "Wake-up Pin Event Positive Edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPEV_A {
    #[doc = "0: Wake-up on positive edge pin event inactive"]
    CONST_0 = 0,
    #[doc = "1: Wake-up on positive edge pin event active"]
    CONST_1 = 1,
}
impl From<EPEV_A> for bool {
    #[inline(always)]
    fn from(variant: EPEV_A) -> Self {
        variant as u8 != 0
    }
}
impl EPEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EPEV_A {
        match self.bits {
            false => EPEV_A::CONST_0,
            true => EPEV_A::CONST_1,
        }
    }
    #[doc = "Wake-up on positive edge pin event inactive"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == EPEV_A::CONST_0
    }
    #[doc = "Wake-up on positive edge pin event active"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == EPEV_A::CONST_1
    }
}
#[doc = "Field `ENEV` reader - Wake-up Pin Event Negative Edge"]
pub type ENEV_R = crate::BitReader<ENEV_A>;
#[doc = "Wake-up Pin Event Negative Edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENEV_A {
    #[doc = "0: Wake-up on negative edge pin event inactive"]
    CONST_0 = 0,
    #[doc = "1: Wake-up on negative edge pin event active"]
    CONST_1 = 1,
}
impl From<ENEV_A> for bool {
    #[inline(always)]
    fn from(variant: ENEV_A) -> Self {
        variant as u8 != 0
    }
}
impl ENEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENEV_A {
        match self.bits {
            false => ENEV_A::CONST_0,
            true => ENEV_A::CONST_1,
        }
    }
    #[doc = "Wake-up on negative edge pin event inactive"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ENEV_A::CONST_0
    }
    #[doc = "Wake-up on negative edge pin event active"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ENEV_A::CONST_1
    }
}
#[doc = "Field `RTCEV` reader - RTC Event"]
pub type RTCEV_R = crate::BitReader<RTCEV_A>;
#[doc = "RTC Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCEV_A {
    #[doc = "0: Wake-up on RTC event inactive"]
    CONST_0 = 0,
    #[doc = "1: Wake-up on RTC event active"]
    CONST_1 = 1,
}
impl From<RTCEV_A> for bool {
    #[inline(always)]
    fn from(variant: RTCEV_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTCEV_A {
        match self.bits {
            false => RTCEV_A::CONST_0,
            true => RTCEV_A::CONST_1,
        }
    }
    #[doc = "Wake-up on RTC event inactive"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RTCEV_A::CONST_0
    }
    #[doc = "Wake-up on RTC event active"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RTCEV_A::CONST_1
    }
}
#[doc = "Field `ULPWDG` reader - ULP WDG Alarm Status"]
pub type ULPWDG_R = crate::BitReader<ULPWDG_A>;
#[doc = "ULP WDG Alarm Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULPWDG_A {
    #[doc = "0: Watchdog alarm did not occur"]
    CONST_0 = 0,
    #[doc = "1: Watchdog alarm occurred"]
    CONST_1 = 1,
}
impl From<ULPWDG_A> for bool {
    #[inline(always)]
    fn from(variant: ULPWDG_A) -> Self {
        variant as u8 != 0
    }
}
impl ULPWDG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ULPWDG_A {
        match self.bits {
            false => ULPWDG_A::CONST_0,
            true => ULPWDG_A::CONST_1,
        }
    }
    #[doc = "Watchdog alarm did not occur"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ULPWDG_A::CONST_0
    }
    #[doc = "Watchdog alarm occurred"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ULPWDG_A::CONST_1
    }
}
#[doc = "Field `HIBNOUT` reader - Hibernate Control Status"]
pub type HIBNOUT_R = crate::BitReader<HIBNOUT_A>;
#[doc = "Hibernate Control Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIBNOUT_A {
    #[doc = "0: Hibernate not driven active to pads"]
    CONST_0 = 0,
    #[doc = "1: Hibernate driven active to pads"]
    CONST_1 = 1,
}
impl From<HIBNOUT_A> for bool {
    #[inline(always)]
    fn from(variant: HIBNOUT_A) -> Self {
        variant as u8 != 0
    }
}
impl HIBNOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HIBNOUT_A {
        match self.bits {
            false => HIBNOUT_A::CONST_0,
            true => HIBNOUT_A::CONST_1,
        }
    }
    #[doc = "Hibernate not driven active to pads"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == HIBNOUT_A::CONST_0
    }
    #[doc = "Hibernate driven active to pads"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == HIBNOUT_A::CONST_1
    }
}
impl R {
    #[doc = "Bit 0 - Wake-up Pin Event Positive Edge"]
    #[inline(always)]
    pub fn epev(&self) -> EPEV_R {
        EPEV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake-up Pin Event Negative Edge"]
    #[inline(always)]
    pub fn enev(&self) -> ENEV_R {
        ENEV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Event"]
    #[inline(always)]
    pub fn rtcev(&self) -> RTCEV_R {
        RTCEV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ULP WDG Alarm Status"]
    #[inline(always)]
    pub fn ulpwdg(&self) -> ULPWDG_R {
        ULPWDG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Hibernate Control Status"]
    #[inline(always)]
    pub fn hibnout(&self) -> HIBNOUT_R {
        HIBNOUT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Hibernate Domain Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDSTAT_SPEC;
impl crate::RegisterSpec for HDSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdstat::R`](R) reader structure"]
impl crate::Readable for HDSTAT_SPEC {}
#[doc = "`reset()` method sets HDSTAT to value 0"]
impl crate::Resettable for HDSTAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
