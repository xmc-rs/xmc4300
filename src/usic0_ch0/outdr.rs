#[doc = "Register `OUTDR` reader"]
pub struct R(crate::R<OUTDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DSR` reader - Data from Shift Register"]
pub type DSR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RCI` reader - Receive Control Information from Shift Register"]
pub type RCI_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Data from Shift Register"]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - Receive Control Information from Shift Register"]
    #[inline(always)]
    pub fn rci(&self) -> RCI_R {
        RCI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[doc = "Receiver Buffer Output Register L for Debugger\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outdr](index.html) module"]
pub struct OUTDR_SPEC;
impl crate::RegisterSpec for OUTDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outdr::R](R) reader structure"]
impl crate::Readable for OUTDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OUTDR to value 0"]
impl crate::Resettable for OUTDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
