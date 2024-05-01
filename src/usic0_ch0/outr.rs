#[doc = "Register `OUTR` reader"]
pub type R = crate::R<OutrSpec>;
#[doc = "Field `DSR` reader - Received Data"]
pub type DsrR = crate::FieldReader<u16>;
#[doc = "Field `RCI` reader - Receiver Control Information"]
pub type RciR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Received Data"]
    #[inline(always)]
    pub fn dsr(&self) -> DsrR {
        DsrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - Receiver Control Information"]
    #[inline(always)]
    pub fn rci(&self) -> RciR {
        RciR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[doc = "Receiver Buffer Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outr::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutrSpec;
impl crate::RegisterSpec for OutrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outr::R`](R) reader structure"]
impl crate::Readable for OutrSpec {}
#[doc = "`reset()` method sets OUTR to value 0"]
impl crate::Resettable for OutrSpec {
    const RESET_VALUE: u32 = 0;
}
