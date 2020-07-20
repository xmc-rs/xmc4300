#[doc = "Reader of register WD_STAT_PDATA"]
pub type R = crate::R<u16, super::WD_STAT_PDATA>;
#[doc = "Watchdog Status of Process Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `WD_STAT_PD`"]
pub type WD_STAT_PD_R = crate::R<bool, WD_STAT_PD_A>;
impl WD_STAT_PD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WD_STAT_PD_A {
        match self.bits {
            false => WD_STAT_PD_A::VALUE1,
            true => WD_STAT_PD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WD_STAT_PD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WD_STAT_PD_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog Status of Process Data"]
    #[inline(always)]
    pub fn wd_stat_pd(&self) -> WD_STAT_PD_R {
        WD_STAT_PD_R::new((self.bits & 0x01) != 0)
    }
}
