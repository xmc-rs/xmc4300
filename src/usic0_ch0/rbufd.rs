#[doc = "Register `RBUFD` reader"]
pub type R = crate::R<RBUFD_SPEC>;
#[doc = "Field `DSR` reader - Data from Shift Register"]
pub type DSR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Data from Shift Register"]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receiver Buffer Register for Debugger\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbufd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RBUFD_SPEC;
impl crate::RegisterSpec for RBUFD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbufd::R`](R) reader structure"]
impl crate::Readable for RBUFD_SPEC {}
#[doc = "`reset()` method sets RBUFD to value 0"]
impl crate::Resettable for RBUFD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
