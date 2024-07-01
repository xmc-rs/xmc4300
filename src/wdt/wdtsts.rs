#[doc = "Register `WDTSTS` reader"]
pub type R = crate::R<WDTSTS_SPEC>;
#[doc = "Field `ALMS` reader - Pre-warning Alarm"]
pub type ALMS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Pre-warning Alarm"]
    #[inline(always)]
    pub fn alms(&self) -> ALMS_R {
        ALMS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "WDT Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTSTS_SPEC;
impl crate::RegisterSpec for WDTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtsts::R`](R) reader structure"]
impl crate::Readable for WDTSTS_SPEC {}
#[doc = "`reset()` method sets WDTSTS to value 0"]
impl crate::Resettable for WDTSTS_SPEC {
    const RESET_VALUE: u32 = 0;
}
