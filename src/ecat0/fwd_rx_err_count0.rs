#[doc = "Register `FWD_RX_ERR_COUNT0` reader"]
pub type R = crate::R<FWD_RX_ERR_COUNT0_SPEC>;
#[doc = "Field `FORW_ERROR` reader - Forwarded error counter of Port y"]
pub type FORW_ERROR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Forwarded error counter of Port y"]
    #[inline(always)]
    pub fn forw_error(&self) -> FORW_ERROR_R {
        FORW_ERROR_R::new(self.bits)
    }
}
#[doc = "Forwarded RX Error Counter Port 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwd_rx_err_count0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FWD_RX_ERR_COUNT0_SPEC;
impl crate::RegisterSpec for FWD_RX_ERR_COUNT0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fwd_rx_err_count0::R`](R) reader structure"]
impl crate::Readable for FWD_RX_ERR_COUNT0_SPEC {}
#[doc = "`reset()` method sets FWD_RX_ERR_COUNT0 to value 0"]
impl crate::Resettable for FWD_RX_ERR_COUNT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
