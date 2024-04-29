#[doc = "Register `TYPE` reader"]
pub type R = crate::R<TYPE_SPEC>;
#[doc = "Field `Type` reader - Type of EtherCAT controller"]
pub type TYPE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Type of EtherCAT controller"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(self.bits)
    }
}
#[doc = "Type of EtherCAT Controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`type_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TYPE_SPEC;
impl crate::RegisterSpec for TYPE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`type_::R`](R) reader structure"]
impl crate::Readable for TYPE_SPEC {}
#[doc = "`reset()` method sets TYPE to value 0x98"]
impl crate::Resettable for TYPE_SPEC {
    const RESET_VALUE: u8 = 0x98;
}
