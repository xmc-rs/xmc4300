#[doc = "Register `BUILD` reader"]
pub type R = crate::R<BUILD_SPEC>;
#[doc = "Field `BUILD` reader - Actual build of EtherCAT controller"]
pub type BUILD_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Actual build of EtherCAT controller"]
    #[inline(always)]
    pub fn build(&self) -> BUILD_R {
        BUILD_R::new(self.bits)
    }
}
#[doc = "Build Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`build::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUILD_SPEC;
impl crate::RegisterSpec for BUILD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`build::R`](R) reader structure"]
impl crate::Readable for BUILD_SPEC {}
#[doc = "`reset()` method sets BUILD to value 0x01"]
impl crate::Resettable for BUILD_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
