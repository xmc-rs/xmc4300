#[doc = "Register `SEFCLR` writer"]
pub type W = crate::W<SEFCLR_SPEC>;
#[doc = "Clear Source Event 0/1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEV0_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the source event flag in GxSEFLAG"]
    VALUE2 = 1,
}
impl From<SEV0_AW> for bool {
    #[inline(always)]
    fn from(variant: SEV0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEV0` writer - Clear Source Event 0/1"]
pub type SEV0_W<'a, REG> = crate::BitWriter<'a, REG, SEV0_AW>;
impl<'a, REG> SEV0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SEV0_AW::VALUE1)
    }
    #[doc = "Clear the source event flag in GxSEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SEV0_AW::VALUE2)
    }
}
#[doc = "Clear Source Event 0/1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEV1_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the source event flag in GxSEFLAG"]
    VALUE2 = 1,
}
impl From<SEV1_AW> for bool {
    #[inline(always)]
    fn from(variant: SEV1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEV1` writer - Clear Source Event 0/1"]
pub type SEV1_W<'a, REG> = crate::BitWriter<'a, REG, SEV1_AW>;
impl<'a, REG> SEV1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SEV1_AW::VALUE1)
    }
    #[doc = "Clear the source event flag in GxSEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SEV1_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Source Event 0/1"]
    #[inline(always)]
    #[must_use]
    pub fn sev0(&mut self) -> SEV0_W<SEFCLR_SPEC> {
        SEV0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Source Event 0/1"]
    #[inline(always)]
    #[must_use]
    pub fn sev1(&mut self) -> SEV1_W<SEFCLR_SPEC> {
        SEV1_W::new(self, 1)
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
#[doc = "Source Event Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sefclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEFCLR_SPEC;
impl crate::RegisterSpec for SEFCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sefclr::W`](W) writer structure"]
impl crate::Writable for SEFCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEFCLR to value 0"]
impl crate::Resettable for SEFCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
