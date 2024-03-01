#[doc = "Register `TX_CARRIER_ERROR_FRAMES` reader"]
pub type R = crate::R<TxCarrierErrorFramesSpec>;
#[doc = "Field `TXCARR` reader - This field indicates the number of frames aborted because of carrier sense error (no carrier or loss of carrier)."]
pub type TxcarrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames aborted because of carrier sense error (no carrier or loss of carrier)."]
    #[inline(always)]
    pub fn txcarr(&self) -> TxcarrR {
        TxcarrR::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Carrier Sense Error Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_carrier_error_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxCarrierErrorFramesSpec;
impl crate::RegisterSpec for TxCarrierErrorFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_carrier_error_frames::R`](R) reader structure"]
impl crate::Readable for TxCarrierErrorFramesSpec {}
#[doc = "`reset()` method sets TX_CARRIER_ERROR_FRAMES to value 0"]
impl crate::Resettable for TxCarrierErrorFramesSpec {
    const RESET_VALUE: u32 = 0;
}
