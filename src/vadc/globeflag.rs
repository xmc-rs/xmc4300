#[doc = "Register `GLOBEFLAG` reader"]
pub type R = crate::R<GLOBEFLAG_SPEC>;
#[doc = "Register `GLOBEFLAG` writer"]
pub type W = crate::W<GLOBEFLAG_SPEC>;
#[doc = "Source Event (Background)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEVGLB_A {
    #[doc = "0: No source event"]
    VALUE1 = 0,
    #[doc = "1: A source event has occurred"]
    VALUE2 = 1,
}
impl From<SEVGLB_A> for bool {
    #[inline(always)]
    fn from(variant: SEVGLB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEVGLB` reader - Source Event (Background)"]
pub type SEVGLB_R = crate::BitReader<SEVGLB_A>;
impl SEVGLB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SEVGLB_A {
        match self.bits {
            false => SEVGLB_A::VALUE1,
            true => SEVGLB_A::VALUE2,
        }
    }
    #[doc = "No source event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SEVGLB_A::VALUE1
    }
    #[doc = "A source event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SEVGLB_A::VALUE2
    }
}
#[doc = "Field `SEVGLB` writer - Source Event (Background)"]
pub type SEVGLB_W<'a, REG> = crate::BitWriter<'a, REG, SEVGLB_A>;
impl<'a, REG> SEVGLB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No source event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SEVGLB_A::VALUE1)
    }
    #[doc = "A source event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SEVGLB_A::VALUE2)
    }
}
#[doc = "Global Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REVGLB_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: New result was stored in register GLOBRES"]
    VALUE2 = 1,
}
impl From<REVGLB_A> for bool {
    #[inline(always)]
    fn from(variant: REVGLB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REVGLB` reader - Global Result Event"]
pub type REVGLB_R = crate::BitReader<REVGLB_A>;
impl REVGLB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REVGLB_A {
        match self.bits {
            false => REVGLB_A::VALUE1,
            true => REVGLB_A::VALUE2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REVGLB_A::VALUE1
    }
    #[doc = "New result was stored in register GLOBRES"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REVGLB_A::VALUE2
    }
}
#[doc = "Field `REVGLB` writer - Global Result Event"]
pub type REVGLB_W<'a, REG> = crate::BitWriter<'a, REG, REVGLB_A>;
impl<'a, REG> REVGLB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REVGLB_A::VALUE1)
    }
    #[doc = "New result was stored in register GLOBRES"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REVGLB_A::VALUE2)
    }
}
#[doc = "Clear Source Event (Background)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEVGLBCLR_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the source event flag SEVGLB"]
    VALUE2 = 1,
}
impl From<SEVGLBCLR_A> for bool {
    #[inline(always)]
    fn from(variant: SEVGLBCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEVGLBCLR` writer - Clear Source Event (Background)"]
pub type SEVGLBCLR_W<'a, REG> = crate::BitWriter<'a, REG, SEVGLBCLR_A>;
impl<'a, REG> SEVGLBCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SEVGLBCLR_A::VALUE1)
    }
    #[doc = "Clear the source event flag SEVGLB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SEVGLBCLR_A::VALUE2)
    }
}
#[doc = "Clear Global Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REVGLBCLR_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag REVGLB"]
    VALUE2 = 1,
}
impl From<REVGLBCLR_A> for bool {
    #[inline(always)]
    fn from(variant: REVGLBCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REVGLBCLR` writer - Clear Global Result Event"]
pub type REVGLBCLR_W<'a, REG> = crate::BitWriter<'a, REG, REVGLBCLR_A>;
impl<'a, REG> REVGLBCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REVGLBCLR_A::VALUE1)
    }
    #[doc = "Clear the result event flag REVGLB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REVGLBCLR_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Source Event (Background)"]
    #[inline(always)]
    pub fn sevglb(&self) -> SEVGLB_R {
        SEVGLB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Global Result Event"]
    #[inline(always)]
    pub fn revglb(&self) -> REVGLB_R {
        REVGLB_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source Event (Background)"]
    #[inline(always)]
    #[must_use]
    pub fn sevglb(&mut self) -> SEVGLB_W<GLOBEFLAG_SPEC> {
        SEVGLB_W::new(self, 0)
    }
    #[doc = "Bit 8 - Global Result Event"]
    #[inline(always)]
    #[must_use]
    pub fn revglb(&mut self) -> REVGLB_W<GLOBEFLAG_SPEC> {
        REVGLB_W::new(self, 8)
    }
    #[doc = "Bit 16 - Clear Source Event (Background)"]
    #[inline(always)]
    #[must_use]
    pub fn sevglbclr(&mut self) -> SEVGLBCLR_W<GLOBEFLAG_SPEC> {
        SEVGLBCLR_W::new(self, 16)
    }
    #[doc = "Bit 24 - Clear Global Result Event"]
    #[inline(always)]
    #[must_use]
    pub fn revglbclr(&mut self) -> REVGLBCLR_W<GLOBEFLAG_SPEC> {
        REVGLBCLR_W::new(self, 24)
    }
}
#[doc = "Global Event Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`globeflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`globeflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GLOBEFLAG_SPEC;
impl crate::RegisterSpec for GLOBEFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`globeflag::R`](R) reader structure"]
impl crate::Readable for GLOBEFLAG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`globeflag::W`](W) writer structure"]
impl crate::Writable for GLOBEFLAG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLOBEFLAG to value 0"]
impl crate::Resettable for GLOBEFLAG_SPEC {
    const RESET_VALUE: u32 = 0;
}
