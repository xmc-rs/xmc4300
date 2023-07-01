#[doc = "Register `TYPE` reader"]
pub struct R(crate::R<TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `Type` reader - Type of EtherCAT controller"]
pub type TYPE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Type of EtherCAT controller"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(self.bits)
    }
}
#[doc = "Type of EtherCAT Controller\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [type_](index.html) module"]
pub struct TYPE_SPEC;
impl crate::RegisterSpec for TYPE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [type_::R](R) reader structure"]
impl crate::Readable for TYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TYPE to value 0x98"]
impl crate::Resettable for TYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0x98;
}
