#[doc = "Register `RES` reader"]
pub type R = crate::R<RES_SPEC>;
#[doc = "Field `RES` reader - Result Register"]
pub type RES_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Result Register"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(self.bits)
    }
}
#[doc = "CRC Result Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`res::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RES_SPEC;
impl crate::RegisterSpec for RES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res::R`](R) reader structure"]
impl crate::Readable for RES_SPEC {}
#[doc = "`reset()` method sets RES to value 0xffff_ffff"]
impl crate::Resettable for RES_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
