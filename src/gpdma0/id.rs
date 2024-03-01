#[doc = "Register `ID` reader"]
pub type R = crate::R<IdSpec>;
#[doc = "Field `VALUE` reader - Hardcoded GPDMA Peripheral ID"]
pub type ValueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Hardcoded GPDMA Peripheral ID"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
#[doc = "GPDMA0 ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdSpec;
impl crate::RegisterSpec for IdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id::R`](R) reader structure"]
impl crate::Readable for IdSpec {}
#[doc = "`reset()` method sets ID to value 0x00af_c000"]
impl crate::Resettable for IdSpec {
    const RESET_VALUE: u32 = 0x00af_c000;
}
