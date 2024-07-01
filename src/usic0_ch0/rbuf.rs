#[doc = "Register `RBUF` reader"]
pub type R = crate::R<RBUF_SPEC>;
#[doc = "Field `DSR` reader - Received Data"]
pub type DSR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Received Data"]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receiver Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rbuf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct RBUF_SPEC;
impl crate::RegisterSpec for RBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbuf::R`](R) reader structure"]
impl crate::Readable for RBUF_SPEC {}
#[doc = "`reset()` method sets RBUF to value 0"]
impl crate::Resettable for RBUF_SPEC {
    const RESET_VALUE: u32 = 0;
}
