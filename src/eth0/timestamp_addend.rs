#[doc = "Register `TIMESTAMP_ADDEND` reader"]
pub struct R(crate::R<TIMESTAMP_ADDEND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMESTAMP_ADDEND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMESTAMP_ADDEND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMESTAMP_ADDEND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMESTAMP_ADDEND` writer"]
pub struct W(crate::W<TIMESTAMP_ADDEND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMESTAMP_ADDEND_SPEC>;
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
impl From<crate::W<TIMESTAMP_ADDEND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMESTAMP_ADDEND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSAR` reader - Timestamp Addend Register"]
pub type TSAR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TSAR` writer - Timestamp Addend Register"]
pub type TSAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMESTAMP_ADDEND_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Addend Register"]
    #[inline(always)]
    pub fn tsar(&self) -> TSAR_R {
        TSAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp Addend Register"]
    #[inline(always)]
    #[must_use]
    pub fn tsar(&mut self) -> TSAR_W<0> {
        TSAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timestamp Addend Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timestamp_addend](index.html) module"]
pub struct TIMESTAMP_ADDEND_SPEC;
impl crate::RegisterSpec for TIMESTAMP_ADDEND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timestamp_addend::R](R) reader structure"]
impl crate::Readable for TIMESTAMP_ADDEND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timestamp_addend::W](W) writer structure"]
impl crate::Writable for TIMESTAMP_ADDEND_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMESTAMP_ADDEND to value 0"]
impl crate::Resettable for TIMESTAMP_ADDEND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
