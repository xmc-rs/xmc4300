#[doc = "Register `CEFCLR` writer"]
pub struct W(crate::W<CEFCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEFCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CEFCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEFCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear Channel Event for Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV0_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the channel event flag in GxCEFLAG"]
    VALUE2 = 1,
}
impl From<CEV0_AW> for bool {
    #[inline(always)]
    fn from(variant: CEV0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV0` writer - Clear Channel Event for Channel 0"]
pub type CEV0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEFCLR_SPEC, CEV0_AW, O>;
impl<'a, const O: u8> CEV0_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV0_AW::VALUE1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV0_AW::VALUE2)
    }
}
#[doc = "Clear Channel Event for Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV1_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the channel event flag in GxCEFLAG"]
    VALUE2 = 1,
}
impl From<CEV1_AW> for bool {
    #[inline(always)]
    fn from(variant: CEV1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV1` writer - Clear Channel Event for Channel 1"]
pub type CEV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEFCLR_SPEC, CEV1_AW, O>;
impl<'a, const O: u8> CEV1_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV1_AW::VALUE1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV1_AW::VALUE2)
    }
}
#[doc = "Clear Channel Event for Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV2_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the channel event flag in GxCEFLAG"]
    VALUE2 = 1,
}
impl From<CEV2_AW> for bool {
    #[inline(always)]
    fn from(variant: CEV2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV2` writer - Clear Channel Event for Channel 2"]
pub type CEV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEFCLR_SPEC, CEV2_AW, O>;
impl<'a, const O: u8> CEV2_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV2_AW::VALUE1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV2_AW::VALUE2)
    }
}
#[doc = "Clear Channel Event for Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV3_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the channel event flag in GxCEFLAG"]
    VALUE2 = 1,
}
impl From<CEV3_AW> for bool {
    #[inline(always)]
    fn from(variant: CEV3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV3` writer - Clear Channel Event for Channel 3"]
pub type CEV3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEFCLR_SPEC, CEV3_AW, O>;
impl<'a, const O: u8> CEV3_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV3_AW::VALUE1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV3_AW::VALUE2)
    }
}
#[doc = "Clear Channel Event for Channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV4_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the channel event flag in GxCEFLAG"]
    VALUE2 = 1,
}
impl From<CEV4_AW> for bool {
    #[inline(always)]
    fn from(variant: CEV4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV4` writer - Clear Channel Event for Channel 4"]
pub type CEV4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEFCLR_SPEC, CEV4_AW, O>;
impl<'a, const O: u8> CEV4_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV4_AW::VALUE1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV4_AW::VALUE2)
    }
}
#[doc = "Clear Channel Event for Channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV5_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the channel event flag in GxCEFLAG"]
    VALUE2 = 1,
}
impl From<CEV5_AW> for bool {
    #[inline(always)]
    fn from(variant: CEV5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV5` writer - Clear Channel Event for Channel 5"]
pub type CEV5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEFCLR_SPEC, CEV5_AW, O>;
impl<'a, const O: u8> CEV5_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV5_AW::VALUE1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV5_AW::VALUE2)
    }
}
#[doc = "Clear Channel Event for Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV6_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the channel event flag in GxCEFLAG"]
    VALUE2 = 1,
}
impl From<CEV6_AW> for bool {
    #[inline(always)]
    fn from(variant: CEV6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV6` writer - Clear Channel Event for Channel 6"]
pub type CEV6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEFCLR_SPEC, CEV6_AW, O>;
impl<'a, const O: u8> CEV6_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV6_AW::VALUE1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV6_AW::VALUE2)
    }
}
#[doc = "Clear Channel Event for Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV7_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the channel event flag in GxCEFLAG"]
    VALUE2 = 1,
}
impl From<CEV7_AW> for bool {
    #[inline(always)]
    fn from(variant: CEV7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV7` writer - Clear Channel Event for Channel 7"]
pub type CEV7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEFCLR_SPEC, CEV7_AW, O>;
impl<'a, const O: u8> CEV7_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV7_AW::VALUE1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV7_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Channel Event for Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn cev0(&mut self) -> CEV0_W<0> {
        CEV0_W::new(self)
    }
    #[doc = "Bit 1 - Clear Channel Event for Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn cev1(&mut self) -> CEV1_W<1> {
        CEV1_W::new(self)
    }
    #[doc = "Bit 2 - Clear Channel Event for Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn cev2(&mut self) -> CEV2_W<2> {
        CEV2_W::new(self)
    }
    #[doc = "Bit 3 - Clear Channel Event for Channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn cev3(&mut self) -> CEV3_W<3> {
        CEV3_W::new(self)
    }
    #[doc = "Bit 4 - Clear Channel Event for Channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn cev4(&mut self) -> CEV4_W<4> {
        CEV4_W::new(self)
    }
    #[doc = "Bit 5 - Clear Channel Event for Channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn cev5(&mut self) -> CEV5_W<5> {
        CEV5_W::new(self)
    }
    #[doc = "Bit 6 - Clear Channel Event for Channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn cev6(&mut self) -> CEV6_W<6> {
        CEV6_W::new(self)
    }
    #[doc = "Bit 7 - Clear Channel Event for Channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn cev7(&mut self) -> CEV7_W<7> {
        CEV7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Event Flag Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cefclr](index.html) module"]
pub struct CEFCLR_SPEC;
impl crate::RegisterSpec for CEFCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cefclr::W](W) writer structure"]
impl crate::Writable for CEFCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CEFCLR to value 0"]
impl crate::Resettable for CEFCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
