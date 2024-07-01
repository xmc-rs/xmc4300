#[doc = "Register `AHB_STATUS` reader"]
pub type R = crate::R<AHB_STATUS_SPEC>;
#[doc = "Field `AHBMS` reader - AHB Master Status"]
pub type AHBMS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - AHB Master Status"]
    #[inline(always)]
    pub fn ahbms(&self) -> AHBMS_R {
        AHBMS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "AHB Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_STATUS_SPEC;
impl crate::RegisterSpec for AHB_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_status::R`](R) reader structure"]
impl crate::Readable for AHB_STATUS_SPEC {}
#[doc = "`reset()` method sets AHB_STATUS to value 0"]
impl crate::Resettable for AHB_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
