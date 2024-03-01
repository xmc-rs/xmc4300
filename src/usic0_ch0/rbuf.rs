#[doc = "Register `RBUF` reader"]
pub type R = crate::R<RbufSpec>;
#[doc = "Field `DSR` reader - Received Data"]
pub type DsrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Received Data"]
    #[inline(always)]
    pub fn dsr(&self) -> DsrR {
        DsrR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receiver Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbuf::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbufSpec;
impl crate::RegisterSpec for RbufSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbuf::R`](R) reader structure"]
impl crate::Readable for RbufSpec {}
#[doc = "`reset()` method sets RBUF to value 0"]
impl crate::Resettable for RbufSpec {
    const RESET_VALUE: u32 = 0;
}
