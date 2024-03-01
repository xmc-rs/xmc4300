#[doc = "Register `FWD_RX_ERR_COUNT0` reader"]
pub type R = crate::R<FwdRxErrCount0Spec>;
#[doc = "Field `FORW_ERROR` reader - Forwarded error counter of Port y"]
pub type ForwErrorR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Forwarded error counter of Port y"]
    #[inline(always)]
    pub fn forw_error(&self) -> ForwErrorR {
        ForwErrorR::new(self.bits)
    }
}
#[doc = "Forwarded RX Error Counter Port 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwd_rx_err_count0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FwdRxErrCount0Spec;
impl crate::RegisterSpec for FwdRxErrCount0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fwd_rx_err_count0::R`](R) reader structure"]
impl crate::Readable for FwdRxErrCount0Spec {}
#[doc = "`reset()` method sets FWD_RX_ERR_COUNT0 to value 0"]
impl crate::Resettable for FwdRxErrCount0Spec {
    const RESET_VALUE: u8 = 0;
}
