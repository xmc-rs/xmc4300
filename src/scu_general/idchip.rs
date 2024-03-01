#[doc = "Register `IDCHIP` reader"]
pub type R = crate::R<IdchipSpec>;
#[doc = "Field `IDCHIP` reader - Chip ID"]
pub type IdchipR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Chip ID"]
    #[inline(always)]
    pub fn idchip(&self) -> IdchipR {
        IdchipR::new(self.bits)
    }
}
#[doc = "Chip ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idchip::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdchipSpec;
impl crate::RegisterSpec for IdchipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idchip::R`](R) reader structure"]
impl crate::Readable for IdchipSpec {}
#[doc = "`reset()` method sets IDCHIP to value 0"]
impl crate::Resettable for IdchipSpec {
    const RESET_VALUE: u32 = 0;
}
