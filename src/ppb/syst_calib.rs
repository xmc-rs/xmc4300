#[doc = "Register `SYST_CALIB` reader"]
pub type R = crate::R<SYST_CALIB_SPEC>;
#[doc = "Register `SYST_CALIB` writer"]
pub type W = crate::W<SYST_CALIB_SPEC>;
#[doc = "Field `TENMS` reader - Ten Milliseconds Reload Value"]
pub type TENMS_R = crate::FieldReader<u32>;
#[doc = "Field `TENMS` writer - Ten Milliseconds Reload Value"]
pub type TENMS_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `SKEW` reader - Ten Milliseconds Skewed"]
pub type SKEW_R = crate::BitReader<SKEW_A>;
#[doc = "Ten Milliseconds Skewed\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SKEW_A {
    #[doc = "0: TENMS value is exact"]
    VALUE1 = 0,
    #[doc = "1: TENMS value is inexact, or not given."]
    VALUE2 = 1,
}
impl From<SKEW_A> for bool {
    #[inline(always)]
    fn from(variant: SKEW_A) -> Self {
        variant as u8 != 0
    }
}
impl SKEW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SKEW_A {
        match self.bits {
            false => SKEW_A::VALUE1,
            true => SKEW_A::VALUE2,
        }
    }
    #[doc = "TENMS value is exact"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SKEW_A::VALUE1
    }
    #[doc = "TENMS value is inexact, or not given."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SKEW_A::VALUE2
    }
}
#[doc = "Field `SKEW` writer - Ten Milliseconds Skewed"]
pub type SKEW_W<'a, REG> = crate::BitWriter<'a, REG, SKEW_A>;
impl<'a, REG> SKEW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TENMS value is exact"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SKEW_A::VALUE1)
    }
    #[doc = "TENMS value is inexact, or not given."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SKEW_A::VALUE2)
    }
}
#[doc = "Field `NOREF` reader - No Reference Clock"]
pub type NOREF_R = crate::BitReader<NOREF_A>;
#[doc = "No Reference Clock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOREF_A {
    #[doc = "0: reference clock provided"]
    VALUE1 = 0,
    #[doc = "1: no reference clock provided."]
    VALUE2 = 1,
}
impl From<NOREF_A> for bool {
    #[inline(always)]
    fn from(variant: NOREF_A) -> Self {
        variant as u8 != 0
    }
}
impl NOREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NOREF_A {
        match self.bits {
            false => NOREF_A::VALUE1,
            true => NOREF_A::VALUE2,
        }
    }
    #[doc = "reference clock provided"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NOREF_A::VALUE1
    }
    #[doc = "no reference clock provided."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NOREF_A::VALUE2
    }
}
#[doc = "Field `NOREF` writer - No Reference Clock"]
pub type NOREF_W<'a, REG> = crate::BitWriter<'a, REG, NOREF_A>;
impl<'a, REG> NOREF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "reference clock provided"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(NOREF_A::VALUE1)
    }
    #[doc = "no reference clock provided."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(NOREF_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:23 - Ten Milliseconds Reload Value"]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 30 - Ten Milliseconds Skewed"]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - No Reference Clock"]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Ten Milliseconds Reload Value"]
    #[inline(always)]
    #[must_use]
    pub fn tenms(&mut self) -> TENMS_W<SYST_CALIB_SPEC> {
        TENMS_W::new(self, 0)
    }
    #[doc = "Bit 30 - Ten Milliseconds Skewed"]
    #[inline(always)]
    #[must_use]
    pub fn skew(&mut self) -> SKEW_W<SYST_CALIB_SPEC> {
        SKEW_W::new(self, 30)
    }
    #[doc = "Bit 31 - No Reference Clock"]
    #[inline(always)]
    #[must_use]
    pub fn noref(&mut self) -> NOREF_W<SYST_CALIB_SPEC> {
        NOREF_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SysTick Calibration Value Register r\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syst_calib::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syst_calib::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYST_CALIB_SPEC;
impl crate::RegisterSpec for SYST_CALIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syst_calib::R`](R) reader structure"]
impl crate::Readable for SYST_CALIB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`syst_calib::W`](W) writer structure"]
impl crate::Writable for SYST_CALIB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYST_CALIB to value 0xc000_0000"]
impl crate::Resettable for SYST_CALIB_SPEC {
    const RESET_VALUE: u32 = 0xc000_0000;
}
