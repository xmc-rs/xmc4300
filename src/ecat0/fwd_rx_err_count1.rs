#[doc = "Register `FWD_RX_ERR_COUNT1` reader"]
pub type R = crate::R<FWD_RX_ERR_COUNT1_SPEC>;
#[doc = "Field `FORW_ERROR` reader - Forwarded error counter of Port y"]
pub type FORW_ERROR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Forwarded error counter of Port y"]
    #[inline(always)]
    pub fn forw_error(&self) -> FORW_ERROR_R {
        FORW_ERROR_R::new(self.bits)
    }
}
#[doc = "Forwarded RX Error Counter Port 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwd_rx_err_count1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FWD_RX_ERR_COUNT1_SPEC;
impl crate::RegisterSpec for FWD_RX_ERR_COUNT1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fwd_rx_err_count1::R`](R) reader structure"]
impl crate::Readable for FWD_RX_ERR_COUNT1_SPEC {}
#[doc = "`reset()` method sets FWD_RX_ERR_COUNT1 to value 0"]
impl crate::Resettable for FWD_RX_ERR_COUNT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
