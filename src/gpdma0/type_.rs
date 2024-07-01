#[doc = "Register `TYPE` reader"]
pub type R = crate::R<TYPE_SPEC>;
#[doc = "Field `VALUE` reader - Component Type"]
pub type VALUE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Component Type"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
#[doc = "GPDMA Component Type\n\nYou can [`read`](crate::Reg::read) this register and get [`type_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TYPE_SPEC;
impl crate::RegisterSpec for TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`type_::R`](R) reader structure"]
impl crate::Readable for TYPE_SPEC {}
#[doc = "`reset()` method sets TYPE to value 0x4457_1110"]
impl crate::Resettable for TYPE_SPEC {
    const RESET_VALUE: u32 = 0x4457_1110;
}
