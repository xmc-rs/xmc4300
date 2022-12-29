#[doc = "Register `BUILD` reader"]
pub struct R(crate::R<BUILD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUILD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUILD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUILD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUILD` reader - Actual build of EtherCAT controller"]
pub type BUILD_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Actual build of EtherCAT controller"]
    #[inline(always)]
    pub fn build(&self) -> BUILD_R {
        BUILD_R::new(self.bits)
    }
}
#[doc = "Build Version\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [build](index.html) module"]
pub struct BUILD_SPEC;
impl crate::RegisterSpec for BUILD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [build::R](R) reader structure"]
impl crate::Readable for BUILD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUILD to value 0x01"]
impl crate::Resettable for BUILD_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
