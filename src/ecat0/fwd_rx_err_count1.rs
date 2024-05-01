#[doc = "Register `FWD_RX_ERR_COUNT1` reader"]
pub type R = crate::R<FwdRxErrCount1Spec>;
#[doc = "Field `FORW_ERROR` reader - Forwarded error counter of Port y"]
pub type ForwErrorR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Forwarded error counter of Port y"]
    #[inline(always)]
    pub fn forw_error(&self) -> ForwErrorR {
        ForwErrorR::new(self.bits)
    }
}
#[doc = "Forwarded RX Error Counter Port 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwd_rx_err_count1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FwdRxErrCount1Spec;
impl crate::RegisterSpec for FwdRxErrCount1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fwd_rx_err_count1::R`](R) reader structure"]
impl crate::Readable for FwdRxErrCount1Spec {}
#[doc = "`reset()` method sets FWD_RX_ERR_COUNT1 to value 0"]
impl crate::Resettable for FwdRxErrCount1Spec {
    const RESET_VALUE: u8 = 0;
}
