#[doc = "Register `MAX_CURRENT_CAP` reader"]
pub type R = crate::R<MAX_CURRENT_CAP_SPEC>;
#[doc = "Field `MAX_CURRENT_FOR_3_3V` reader - Maximum Current for 3.3V"]
pub type MAX_CURRENT_FOR_3_3V_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Maximum Current for 3.3V"]
    #[inline(always)]
    pub fn max_current_for_3_3v(&self) -> MAX_CURRENT_FOR_3_3V_R {
        MAX_CURRENT_FOR_3_3V_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Maximum Current Capabilities Register\n\nYou can [`read`](crate::Reg::read) this register and get [`max_current_cap::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAX_CURRENT_CAP_SPEC;
impl crate::RegisterSpec for MAX_CURRENT_CAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`max_current_cap::R`](R) reader structure"]
impl crate::Readable for MAX_CURRENT_CAP_SPEC {}
#[doc = "`reset()` method sets MAX_CURRENT_CAP to value 0x01"]
impl crate::Resettable for MAX_CURRENT_CAP_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
