#[doc = "Register `TYPE` reader"]
pub type R = crate::R<TypeSpec>;
#[doc = "Field `Type` reader - Type of EtherCAT controller"]
pub type TypeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Type of EtherCAT controller"]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new(self.bits)
    }
}
#[doc = "Type of EtherCAT Controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`type_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TypeSpec;
impl crate::RegisterSpec for TypeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`type_::R`](R) reader structure"]
impl crate::Readable for TypeSpec {}
#[doc = "`reset()` method sets TYPE to value 0x98"]
impl crate::Resettable for TypeSpec {
    const RESET_VALUE: u8 = 0x98;
}
