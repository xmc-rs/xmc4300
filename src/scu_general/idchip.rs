#[doc = "Register `IDCHIP` reader"]
pub type R = crate::R<IDCHIP_SPEC>;
#[doc = "Field `IDCHIP` reader - Chip ID"]
pub type IDCHIP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Chip ID"]
    #[inline(always)]
    pub fn idchip(&self) -> IDCHIP_R {
        IDCHIP_R::new(self.bits)
    }
}
#[doc = "Chip ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`idchip::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDCHIP_SPEC;
impl crate::RegisterSpec for IDCHIP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idchip::R`](R) reader structure"]
impl crate::Readable for IDCHIP_SPEC {}
#[doc = "`reset()` method sets IDCHIP to value 0"]
impl crate::Resettable for IDCHIP_SPEC {
    const RESET_VALUE: u32 = 0;
}
