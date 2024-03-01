#[doc = "Register `SYST_CALIB` reader"]
pub type R = crate::R<SystCalibSpec>;
#[doc = "Register `SYST_CALIB` writer"]
pub type W = crate::W<SystCalibSpec>;
#[doc = "Field `TENMS` reader - Ten Milliseconds Reload Value"]
pub type TenmsR = crate::FieldReader<u32>;
#[doc = "Field `TENMS` writer - Ten Milliseconds Reload Value"]
pub type TenmsW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Ten Milliseconds Skewed\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Skew {
    #[doc = "0: TENMS value is exact"]
    Value1 = 0,
    #[doc = "1: TENMS value is inexact, or not given."]
    Value2 = 1,
}
impl From<Skew> for bool {
    #[inline(always)]
    fn from(variant: Skew) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SKEW` reader - Ten Milliseconds Skewed"]
pub type SkewR = crate::BitReader<Skew>;
impl SkewR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Skew {
        match self.bits {
            false => Skew::Value1,
            true => Skew::Value2,
        }
    }
    #[doc = "TENMS value is exact"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Skew::Value1
    }
    #[doc = "TENMS value is inexact, or not given."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Skew::Value2
    }
}
#[doc = "Field `SKEW` writer - Ten Milliseconds Skewed"]
pub type SkewW<'a, REG> = crate::BitWriter<'a, REG, Skew>;
impl<'a, REG> SkewW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TENMS value is exact"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Skew::Value1)
    }
    #[doc = "TENMS value is inexact, or not given."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Skew::Value2)
    }
}
#[doc = "No Reference Clock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Noref {
    #[doc = "0: reference clock provided"]
    Value1 = 0,
    #[doc = "1: no reference clock provided."]
    Value2 = 1,
}
impl From<Noref> for bool {
    #[inline(always)]
    fn from(variant: Noref) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOREF` reader - No Reference Clock"]
pub type NorefR = crate::BitReader<Noref>;
impl NorefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Noref {
        match self.bits {
            false => Noref::Value1,
            true => Noref::Value2,
        }
    }
    #[doc = "reference clock provided"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Noref::Value1
    }
    #[doc = "no reference clock provided."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Noref::Value2
    }
}
#[doc = "Field `NOREF` writer - No Reference Clock"]
pub type NorefW<'a, REG> = crate::BitWriter<'a, REG, Noref>;
impl<'a, REG> NorefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "reference clock provided"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Noref::Value1)
    }
    #[doc = "no reference clock provided."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Noref::Value2)
    }
}
impl R {
    #[doc = "Bits 0:23 - Ten Milliseconds Reload Value"]
    #[inline(always)]
    pub fn tenms(&self) -> TenmsR {
        TenmsR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 30 - Ten Milliseconds Skewed"]
    #[inline(always)]
    pub fn skew(&self) -> SkewR {
        SkewR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - No Reference Clock"]
    #[inline(always)]
    pub fn noref(&self) -> NorefR {
        NorefR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Ten Milliseconds Reload Value"]
    #[inline(always)]
    #[must_use]
    pub fn tenms(&mut self) -> TenmsW<SystCalibSpec> {
        TenmsW::new(self, 0)
    }
    #[doc = "Bit 30 - Ten Milliseconds Skewed"]
    #[inline(always)]
    #[must_use]
    pub fn skew(&mut self) -> SkewW<SystCalibSpec> {
        SkewW::new(self, 30)
    }
    #[doc = "Bit 31 - No Reference Clock"]
    #[inline(always)]
    #[must_use]
    pub fn noref(&mut self) -> NorefW<SystCalibSpec> {
        NorefW::new(self, 31)
    }
}
#[doc = "SysTick Calibration Value Register r\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syst_calib::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syst_calib::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystCalibSpec;
impl crate::RegisterSpec for SystCalibSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syst_calib::R`](R) reader structure"]
impl crate::Readable for SystCalibSpec {}
#[doc = "`write(|w| ..)` method takes [`syst_calib::W`](W) writer structure"]
impl crate::Writable for SystCalibSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYST_CALIB to value 0xc000_0000"]
impl crate::Resettable for SystCalibSpec {
    const RESET_VALUE: u32 = 0xc000_0000;
}
