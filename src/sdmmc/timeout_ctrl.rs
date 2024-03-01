#[doc = "Register `TIMEOUT_CTRL` reader"]
pub type R = crate::R<TimeoutCtrlSpec>;
#[doc = "Register `TIMEOUT_CTRL` writer"]
pub type W = crate::W<TimeoutCtrlSpec>;
#[doc = "Data Timeout Counter Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DatTimeoutCntVal {
    #[doc = "0: TMCLK * 2^13"]
    Value1 = 0,
    #[doc = "1: TMCLK * 2^14"]
    Value2 = 1,
    #[doc = "14: TMCLK * 2^27"]
    Value3 = 14,
}
impl From<DatTimeoutCntVal> for u8 {
    #[inline(always)]
    fn from(variant: DatTimeoutCntVal) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DatTimeoutCntVal {
    type Ux = u8;
}
#[doc = "Field `DAT_TIMEOUT_CNT_VAL` reader - Data Timeout Counter Value"]
pub type DatTimeoutCntValR = crate::FieldReader<DatTimeoutCntVal>;
impl DatTimeoutCntValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DatTimeoutCntVal> {
        match self.bits {
            0 => Some(DatTimeoutCntVal::Value1),
            1 => Some(DatTimeoutCntVal::Value2),
            14 => Some(DatTimeoutCntVal::Value3),
            _ => None,
        }
    }
    #[doc = "TMCLK * 2^13"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DatTimeoutCntVal::Value1
    }
    #[doc = "TMCLK * 2^14"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DatTimeoutCntVal::Value2
    }
    #[doc = "TMCLK * 2^27"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DatTimeoutCntVal::Value3
    }
}
#[doc = "Field `DAT_TIMEOUT_CNT_VAL` writer - Data Timeout Counter Value"]
pub type DatTimeoutCntValW<'a, REG> = crate::FieldWriter<'a, REG, 4, DatTimeoutCntVal>;
impl<'a, REG> DatTimeoutCntValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TMCLK * 2^13"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DatTimeoutCntVal::Value1)
    }
    #[doc = "TMCLK * 2^14"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DatTimeoutCntVal::Value2)
    }
    #[doc = "TMCLK * 2^27"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(DatTimeoutCntVal::Value3)
    }
}
impl R {
    #[doc = "Bits 0:3 - Data Timeout Counter Value"]
    #[inline(always)]
    pub fn dat_timeout_cnt_val(&self) -> DatTimeoutCntValR {
        DatTimeoutCntValR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Timeout Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn dat_timeout_cnt_val(&mut self) -> DatTimeoutCntValW<TimeoutCtrlSpec> {
        DatTimeoutCntValW::new(self, 0)
    }
}
#[doc = "Timeout Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeout_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimeoutCtrlSpec;
impl crate::RegisterSpec for TimeoutCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`timeout_ctrl::R`](R) reader structure"]
impl crate::Readable for TimeoutCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`timeout_ctrl::W`](W) writer structure"]
impl crate::Writable for TimeoutCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TIMEOUT_CTRL to value 0"]
impl crate::Resettable for TimeoutCtrlSpec {
    const RESET_VALUE: u8 = 0;
}
