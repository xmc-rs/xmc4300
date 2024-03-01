#[doc = "Register `AHB_STATUS` reader"]
pub type R = crate::R<AhbStatusSpec>;
#[doc = "Field `AHBMS` reader - AHB Master Status"]
pub type AhbmsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - AHB Master Status"]
    #[inline(always)]
    pub fn ahbms(&self) -> AhbmsR {
        AhbmsR::new((self.bits & 1) != 0)
    }
}
#[doc = "AHB Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbStatusSpec;
impl crate::RegisterSpec for AhbStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_status::R`](R) reader structure"]
impl crate::Readable for AhbStatusSpec {}
#[doc = "`reset()` method sets AHB_STATUS to value 0"]
impl crate::Resettable for AhbStatusSpec {
    const RESET_VALUE: u32 = 0;
}
