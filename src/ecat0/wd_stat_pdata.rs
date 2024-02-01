#[doc = "Register `WD_STAT_PDATA` reader"]
pub type R = crate::R<WD_STAT_PDATA_SPEC>;
#[doc = "Field `WD_STAT_PD` reader - Watchdog Status of Process Data"]
pub type WD_STAT_PD_R = crate::BitReader<WD_STAT_PD_A>;
#[doc = "Watchdog Status of Process Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WD_STAT_PD_A {
    #[doc = "0: Watchdog Process Data expired"]
    VALUE1 = 0,
    #[doc = "1: Watchdog Process Data is active or disabled"]
    VALUE2 = 1,
}
impl From<WD_STAT_PD_A> for bool {
    #[inline(always)]
    fn from(variant: WD_STAT_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl WD_STAT_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WD_STAT_PD_A {
        match self.bits {
            false => WD_STAT_PD_A::VALUE1,
            true => WD_STAT_PD_A::VALUE2,
        }
    }
    #[doc = "Watchdog Process Data expired"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WD_STAT_PD_A::VALUE1
    }
    #[doc = "Watchdog Process Data is active or disabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WD_STAT_PD_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog Status of Process Data"]
    #[inline(always)]
    pub fn wd_stat_pd(&self) -> WD_STAT_PD_R {
        WD_STAT_PD_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Watchdog Status Process Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wd_stat_pdata::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WD_STAT_PDATA_SPEC;
impl crate::RegisterSpec for WD_STAT_PDATA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`wd_stat_pdata::R`](R) reader structure"]
impl crate::Readable for WD_STAT_PDATA_SPEC {}
#[doc = "`reset()` method sets WD_STAT_PDATA to value 0"]
impl crate::Resettable for WD_STAT_PDATA_SPEC {
    const RESET_VALUE: u16 = 0;
}
