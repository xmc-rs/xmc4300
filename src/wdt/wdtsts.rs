#[doc = "Register `WDTSTS` reader"]
pub type R = crate::R<WdtstsSpec>;
#[doc = "Field `ALMS` reader - Pre-warning Alarm"]
pub type AlmsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Pre-warning Alarm"]
    #[inline(always)]
    pub fn alms(&self) -> AlmsR {
        AlmsR::new((self.bits & 1) != 0)
    }
}
#[doc = "WDT Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtstsSpec;
impl crate::RegisterSpec for WdtstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtsts::R`](R) reader structure"]
impl crate::Readable for WdtstsSpec {}
#[doc = "`reset()` method sets WDTSTS to value 0"]
impl crate::Resettable for WdtstsSpec {
    const RESET_VALUE: u32 = 0;
}
