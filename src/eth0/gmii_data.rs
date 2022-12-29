#[doc = "Register `GMII_DATA` reader"]
pub struct R(crate::R<GMII_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMII_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMII_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMII_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMII_DATA` writer"]
pub struct W(crate::W<GMII_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMII_DATA_SPEC>;
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
impl From<crate::W<GMII_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMII_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MD` reader - MII Data"]
pub type MD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MD` writer - MII Data"]
pub type MD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GMII_DATA_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - MII Data"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MII Data"]
    #[inline(always)]
    #[must_use]
    pub fn md(&mut self) -> MD_W<0> {
        MD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MII Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmii_data](index.html) module"]
pub struct GMII_DATA_SPEC;
impl crate::RegisterSpec for GMII_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmii_data::R](R) reader structure"]
impl crate::Readable for GMII_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmii_data::W](W) writer structure"]
impl crate::Writable for GMII_DATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GMII_DATA to value 0"]
impl crate::Resettable for GMII_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
