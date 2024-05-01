#[doc = "Register `RX_MULTICAST_FRAMES_GOOD` reader"]
pub type R = crate::R<RxMulticastFramesGoodSpec>;
#[doc = "Field `RXMCASTG` reader - This field indicates the number of received good multicast frames."]
pub type RxmcastgR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good multicast frames."]
    #[inline(always)]
    pub fn rxmcastg(&self) -> RxmcastgR {
        RxmcastgR::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Good Multicast Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_multicast_frames_good::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxMulticastFramesGoodSpec;
impl crate::RegisterSpec for RxMulticastFramesGoodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_multicast_frames_good::R`](R) reader structure"]
impl crate::Readable for RxMulticastFramesGoodSpec {}
#[doc = "`reset()` method sets RX_MULTICAST_FRAMES_GOOD to value 0"]
impl crate::Resettable for RxMulticastFramesGoodSpec {
    const RESET_VALUE: u32 = 0;
}
