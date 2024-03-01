#[doc = "Register `VERSION` reader"]
pub type R = crate::R<VersionSpec>;
#[doc = "Field `VALUE` reader - Version number of the component"]
pub type ValueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Version number of the component"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
#[doc = "DMA Component Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VersionSpec;
impl crate::RegisterSpec for VersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`version::R`](R) reader structure"]
impl crate::Readable for VersionSpec {}
#[doc = "`reset()` method sets VERSION to value 0x3231_342a"]
impl crate::Resettable for VersionSpec {
    const RESET_VALUE: u32 = 0x3231_342a;
}
