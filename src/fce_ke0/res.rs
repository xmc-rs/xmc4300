#[doc = "Register `RES` reader"]
pub type R = crate::R<ResSpec>;
#[doc = "Field `RES` reader - Result Register"]
pub type ResR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Result Register"]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new(self.bits)
    }
}
#[doc = "CRC Result Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`res::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResSpec;
impl crate::RegisterSpec for ResSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res::R`](R) reader structure"]
impl crate::Readable for ResSpec {}
#[doc = "`reset()` method sets RES to value 0xffff_ffff"]
impl crate::Resettable for ResSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
