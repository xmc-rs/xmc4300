#[doc = "Register `RESPONSE2` reader"]
pub struct R(crate::R<RESPONSE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESPONSE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESPONSE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESPONSE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESPONSE2` reader - Response2"]
pub type RESPONSE2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESPONSE3` reader - Response3"]
pub type RESPONSE3_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Response2"]
    #[inline(always)]
    pub fn response2(&self) -> RESPONSE2_R {
        RESPONSE2_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Response3"]
    #[inline(always)]
    pub fn response3(&self) -> RESPONSE3_R {
        RESPONSE3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Response 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [response2](index.html) module"]
pub struct RESPONSE2_SPEC;
impl crate::RegisterSpec for RESPONSE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [response2::R](R) reader structure"]
impl crate::Readable for RESPONSE2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESPONSE2 to value 0"]
impl crate::Resettable for RESPONSE2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
