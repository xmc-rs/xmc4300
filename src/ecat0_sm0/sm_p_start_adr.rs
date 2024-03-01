#[doc = "Register `SM_P_START_ADR` reader"]
pub type R = crate::R<SmPStartAdrSpec>;
#[doc = "Field `FIRST_BYTE` reader - Specifies first byte that will be handled by SyncManager"]
pub type FirstByteR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Specifies first byte that will be handled by SyncManager"]
    #[inline(always)]
    pub fn first_byte(&self) -> FirstByteR {
        FirstByteR::new(self.bits)
    }
}
#[doc = "Physical Start Address SyncManager 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sm_p_start_adr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmPStartAdrSpec;
impl crate::RegisterSpec for SmPStartAdrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sm_p_start_adr::R`](R) reader structure"]
impl crate::Readable for SmPStartAdrSpec {}
#[doc = "`reset()` method sets SM_P_START_ADR to value 0"]
impl crate::Resettable for SmPStartAdrSpec {
    const RESET_VALUE: u16 = 0;
}
