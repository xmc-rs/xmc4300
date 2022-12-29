#[doc = "Register `BYP` reader"]
pub struct R(crate::R<BYP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BYP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BYP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BYP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BYP` writer"]
pub struct W(crate::W<BYP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BYP_SPEC>;
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
impl From<crate::W<BYP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BYP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BDATA` reader - Bypass Data"]
pub type BDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BDATA` writer - Bypass Data"]
pub type BDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BYP_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Bypass Data"]
    #[inline(always)]
    pub fn bdata(&self) -> BDATA_R {
        BDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bypass Data"]
    #[inline(always)]
    #[must_use]
    pub fn bdata(&mut self) -> BDATA_W<0> {
        BDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bypass Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [byp](index.html) module"]
pub struct BYP_SPEC;
impl crate::RegisterSpec for BYP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [byp::R](R) reader structure"]
impl crate::Readable for BYP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [byp::W](W) writer structure"]
impl crate::Writable for BYP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BYP to value 0"]
impl crate::Resettable for BYP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
