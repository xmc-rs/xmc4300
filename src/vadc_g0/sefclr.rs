#[doc = "Register `SEFCLR` writer"]
pub type W = crate::W<SefclrSpec>;
#[doc = "Clear Source Event 0/1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sev0 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the source event flag in GxSEFLAG"]
    Value2 = 1,
}
impl From<Sev0> for bool {
    #[inline(always)]
    fn from(variant: Sev0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEV0` writer - Clear Source Event 0/1"]
pub type Sev0W<'a, REG> = crate::BitWriter<'a, REG, Sev0>;
impl<'a, REG> Sev0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sev0::Value1)
    }
    #[doc = "Clear the source event flag in GxSEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sev0::Value2)
    }
}
#[doc = "Clear Source Event 0/1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sev1 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the source event flag in GxSEFLAG"]
    Value2 = 1,
}
impl From<Sev1> for bool {
    #[inline(always)]
    fn from(variant: Sev1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEV1` writer - Clear Source Event 0/1"]
pub type Sev1W<'a, REG> = crate::BitWriter<'a, REG, Sev1>;
impl<'a, REG> Sev1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sev1::Value1)
    }
    #[doc = "Clear the source event flag in GxSEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sev1::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Source Event 0/1"]
    #[inline(always)]
    #[must_use]
    pub fn sev0(&mut self) -> Sev0W<SefclrSpec> {
        Sev0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Source Event 0/1"]
    #[inline(always)]
    #[must_use]
    pub fn sev1(&mut self) -> Sev1W<SefclrSpec> {
        Sev1W::new(self, 1)
    }
}
#[doc = "Source Event Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sefclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SefclrSpec;
impl crate::RegisterSpec for SefclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sefclr::W`](W) writer structure"]
impl crate::Writable for SefclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEFCLR to value 0"]
impl crate::Resettable for SefclrSpec {
    const RESET_VALUE: u32 = 0;
}
