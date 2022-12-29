#[doc = "Register `RBUF` reader"]
pub struct R(crate::R<RBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DSR` reader - Received Data"]
pub type DSR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Received Data"]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receiver Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbuf](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct RBUF_SPEC;
impl crate::RegisterSpec for RBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbuf::R](R) reader structure"]
impl crate::Readable for RBUF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RBUF to value 0"]
impl crate::Resettable for RBUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
