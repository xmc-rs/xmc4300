#[doc = "Register `WADDR` reader"]
pub type R = crate::R<WaddrSpec>;
#[doc = "Field `WADDR` reader - Write Error Address"]
pub type WaddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Write Error Address"]
    #[inline(always)]
    pub fn waddr(&self) -> WaddrR {
        WaddrR::new(self.bits)
    }
}
#[doc = "PBA Write Error Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`waddr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WaddrSpec;
impl crate::RegisterSpec for WaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`waddr::R`](R) reader structure"]
impl crate::Readable for WaddrSpec {}
#[doc = "`reset()` method sets WADDR to value 0"]
impl crate::Resettable for WaddrSpec {
    const RESET_VALUE: u32 = 0;
}
