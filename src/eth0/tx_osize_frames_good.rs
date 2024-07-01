#[doc = "Register `TX_OSIZE_FRAMES_GOOD` reader"]
pub type R = crate::R<TX_OSIZE_FRAMES_GOOD_SPEC>;
#[doc = "Field `TXOSIZG` reader - This field indicates the number of frames transmitted without errors and with length greater than the maxsize (1,518 or 1,522 bytes for VLAN tagged frames; 2000 bytes if enabled by setting MAC Configuration.2KPE)."]
pub type TXOSIZG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames transmitted without errors and with length greater than the maxsize (1,518 or 1,522 bytes for VLAN tagged frames; 2000 bytes if enabled by setting MAC Configuration.2KPE)."]
    #[inline(always)]
    pub fn txosizg(&self) -> TXOSIZG_R {
        TXOSIZG_R::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Good Oversize Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_osize_frames_good::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_OSIZE_FRAMES_GOOD_SPEC;
impl crate::RegisterSpec for TX_OSIZE_FRAMES_GOOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_osize_frames_good::R`](R) reader structure"]
impl crate::Readable for TX_OSIZE_FRAMES_GOOD_SPEC {}
#[doc = "`reset()` method sets TX_OSIZE_FRAMES_GOOD to value 0"]
impl crate::Resettable for TX_OSIZE_FRAMES_GOOD_SPEC {
    const RESET_VALUE: u32 = 0;
}
