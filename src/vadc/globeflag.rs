#[doc = "Register `GLOBEFLAG` reader"]
pub type R = crate::R<GlobeflagSpec>;
#[doc = "Register `GLOBEFLAG` writer"]
pub type W = crate::W<GlobeflagSpec>;
#[doc = "Source Event (Background)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sevglb {
    #[doc = "0: No source event"]
    Value1 = 0,
    #[doc = "1: A source event has occurred"]
    Value2 = 1,
}
impl From<Sevglb> for bool {
    #[inline(always)]
    fn from(variant: Sevglb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEVGLB` reader - Source Event (Background)"]
pub type SevglbR = crate::BitReader<Sevglb>;
impl SevglbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sevglb {
        match self.bits {
            false => Sevglb::Value1,
            true => Sevglb::Value2,
        }
    }
    #[doc = "No source event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sevglb::Value1
    }
    #[doc = "A source event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sevglb::Value2
    }
}
#[doc = "Field `SEVGLB` writer - Source Event (Background)"]
pub type SevglbW<'a, REG> = crate::BitWriter<'a, REG, Sevglb>;
impl<'a, REG> SevglbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No source event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sevglb::Value1)
    }
    #[doc = "A source event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sevglb::Value2)
    }
}
#[doc = "Global Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Revglb {
    #[doc = "0: No result event"]
    Value1 = 0,
    #[doc = "1: New result was stored in register GLOBRES"]
    Value2 = 1,
}
impl From<Revglb> for bool {
    #[inline(always)]
    fn from(variant: Revglb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REVGLB` reader - Global Result Event"]
pub type RevglbR = crate::BitReader<Revglb>;
impl RevglbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Revglb {
        match self.bits {
            false => Revglb::Value1,
            true => Revglb::Value2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Revglb::Value1
    }
    #[doc = "New result was stored in register GLOBRES"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Revglb::Value2
    }
}
#[doc = "Field `REVGLB` writer - Global Result Event"]
pub type RevglbW<'a, REG> = crate::BitWriter<'a, REG, Revglb>;
impl<'a, REG> RevglbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Revglb::Value1)
    }
    #[doc = "New result was stored in register GLOBRES"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Revglb::Value2)
    }
}
#[doc = "Clear Source Event (Background)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sevglbclr {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the source event flag SEVGLB"]
    Value2 = 1,
}
impl From<Sevglbclr> for bool {
    #[inline(always)]
    fn from(variant: Sevglbclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEVGLBCLR` writer - Clear Source Event (Background)"]
pub type SevglbclrW<'a, REG> = crate::BitWriter<'a, REG, Sevglbclr>;
impl<'a, REG> SevglbclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sevglbclr::Value1)
    }
    #[doc = "Clear the source event flag SEVGLB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sevglbclr::Value2)
    }
}
#[doc = "Clear Global Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Revglbclr {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the result event flag REVGLB"]
    Value2 = 1,
}
impl From<Revglbclr> for bool {
    #[inline(always)]
    fn from(variant: Revglbclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REVGLBCLR` writer - Clear Global Result Event"]
pub type RevglbclrW<'a, REG> = crate::BitWriter<'a, REG, Revglbclr>;
impl<'a, REG> RevglbclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Revglbclr::Value1)
    }
    #[doc = "Clear the result event flag REVGLB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Revglbclr::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Source Event (Background)"]
    #[inline(always)]
    pub fn sevglb(&self) -> SevglbR {
        SevglbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Global Result Event"]
    #[inline(always)]
    pub fn revglb(&self) -> RevglbR {
        RevglbR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source Event (Background)"]
    #[inline(always)]
    #[must_use]
    pub fn sevglb(&mut self) -> SevglbW<GlobeflagSpec> {
        SevglbW::new(self, 0)
    }
    #[doc = "Bit 8 - Global Result Event"]
    #[inline(always)]
    #[must_use]
    pub fn revglb(&mut self) -> RevglbW<GlobeflagSpec> {
        RevglbW::new(self, 8)
    }
    #[doc = "Bit 16 - Clear Source Event (Background)"]
    #[inline(always)]
    #[must_use]
    pub fn sevglbclr(&mut self) -> SevglbclrW<GlobeflagSpec> {
        SevglbclrW::new(self, 16)
    }
    #[doc = "Bit 24 - Clear Global Result Event"]
    #[inline(always)]
    #[must_use]
    pub fn revglbclr(&mut self) -> RevglbclrW<GlobeflagSpec> {
        RevglbclrW::new(self, 24)
    }
}
#[doc = "Global Event Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globeflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globeflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlobeflagSpec;
impl crate::RegisterSpec for GlobeflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`globeflag::R`](R) reader structure"]
impl crate::Readable for GlobeflagSpec {}
#[doc = "`write(|w| ..)` method takes [`globeflag::W`](W) writer structure"]
impl crate::Writable for GlobeflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLOBEFLAG to value 0"]
impl crate::Resettable for GlobeflagSpec {
    const RESET_VALUE: u32 = 0;
}
