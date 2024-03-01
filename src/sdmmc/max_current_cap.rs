#[doc = "Register `MAX_CURRENT_CAP` reader"]
pub type R = crate::R<MaxCurrentCapSpec>;
#[doc = "Field `MAX_CURRENT_FOR_3_3V` reader - Maximum Current for 3.3V"]
pub type MaxCurrentFor3_3vR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Maximum Current for 3.3V"]
    #[inline(always)]
    pub fn max_current_for_3_3v(&self) -> MaxCurrentFor3_3vR {
        MaxCurrentFor3_3vR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Maximum Current Capabilities Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`max_current_cap::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxCurrentCapSpec;
impl crate::RegisterSpec for MaxCurrentCapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`max_current_cap::R`](R) reader structure"]
impl crate::Readable for MaxCurrentCapSpec {}
#[doc = "`reset()` method sets MAX_CURRENT_CAP to value 0x01"]
impl crate::Resettable for MaxCurrentCapSpec {
    const RESET_VALUE: u32 = 0x01;
}
