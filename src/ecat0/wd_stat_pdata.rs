#[doc = "Register `WD_STAT_PDATA` reader"]
pub type R = crate::R<WdStatPdataSpec>;
#[doc = "Watchdog Status of Process Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WdStatPd {
    #[doc = "0: Watchdog Process Data expired"]
    Value1 = 0,
    #[doc = "1: Watchdog Process Data is active or disabled"]
    Value2 = 1,
}
impl From<WdStatPd> for bool {
    #[inline(always)]
    fn from(variant: WdStatPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WD_STAT_PD` reader - Watchdog Status of Process Data"]
pub type WdStatPdR = crate::BitReader<WdStatPd>;
impl WdStatPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdStatPd {
        match self.bits {
            false => WdStatPd::Value1,
            true => WdStatPd::Value2,
        }
    }
    #[doc = "Watchdog Process Data expired"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WdStatPd::Value1
    }
    #[doc = "Watchdog Process Data is active or disabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WdStatPd::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog Status of Process Data"]
    #[inline(always)]
    pub fn wd_stat_pd(&self) -> WdStatPdR {
        WdStatPdR::new((self.bits & 1) != 0)
    }
}
#[doc = "Watchdog Status Process Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wd_stat_pdata::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdStatPdataSpec;
impl crate::RegisterSpec for WdStatPdataSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`wd_stat_pdata::R`](R) reader structure"]
impl crate::Readable for WdStatPdataSpec {}
#[doc = "`reset()` method sets WD_STAT_PDATA to value 0"]
impl crate::Resettable for WdStatPdataSpec {
    const RESET_VALUE: u16 = 0;
}
