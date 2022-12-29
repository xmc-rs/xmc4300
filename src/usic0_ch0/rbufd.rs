#[doc = "Register `RBUFD` reader"]
pub struct R(crate::R<RBUFD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBUFD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBUFD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBUFD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DSR` reader - Data from Shift Register"]
pub type DSR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data from Shift Register"]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receiver Buffer Register for Debugger\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbufd](index.html) module"]
pub struct RBUFD_SPEC;
impl crate::RegisterSpec for RBUFD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbufd::R](R) reader structure"]
impl crate::Readable for RBUFD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RBUFD to value 0"]
impl crate::Resettable for RBUFD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
