#[doc = "Register `TYPE` reader"]
pub type R = crate::R<TypeSpec>;
#[doc = "Field `VALUE` reader - Component Type"]
pub type ValueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Component Type"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
#[doc = "GPDMA Component Type\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`type_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TypeSpec;
impl crate::RegisterSpec for TypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`type_::R`](R) reader structure"]
impl crate::Readable for TypeSpec {}
#[doc = "`reset()` method sets TYPE to value 0x4457_1110"]
impl crate::Resettable for TypeSpec {
    const RESET_VALUE: u32 = 0x4457_1110;
}
