#[doc = "Register `WADDR` reader"]
pub type R = crate::R<WADDR_SPEC>;
#[doc = "Field `WADDR` reader - Write Error Address"]
pub type WADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Write Error Address"]
    #[inline(always)]
    pub fn waddr(&self) -> WADDR_R {
        WADDR_R::new(self.bits)
    }
}
#[doc = "PBA Write Error Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`waddr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WADDR_SPEC;
impl crate::RegisterSpec for WADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`waddr::R`](R) reader structure"]
impl crate::Readable for WADDR_SPEC {}
#[doc = "`reset()` method sets WADDR to value 0"]
impl crate::Resettable for WADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
