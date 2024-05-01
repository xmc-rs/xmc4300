#[doc = "Register `RX_OVERSIZE_FRAMES_GOOD` reader"]
pub type R = crate::R<RxOversizeFramesGoodSpec>;
#[doc = "Field `RXOVERSZG` reader - This field indicates the number of frames received without errors, with length greater than the maxsize (1,518 or 1,522 for VLAN tagged frames; 2,000 bytes if enabled by setting MAC Configuration.2KPE)."]
pub type RxoverszgR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received without errors, with length greater than the maxsize (1,518 or 1,522 for VLAN tagged frames; 2,000 bytes if enabled by setting MAC Configuration.2KPE)."]
    #[inline(always)]
    pub fn rxoverszg(&self) -> RxoverszgR {
        RxoverszgR::new(self.bits)
    }
}
#[doc = "Rx Oversize Frames Good Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_oversize_frames_good::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxOversizeFramesGoodSpec;
impl crate::RegisterSpec for RxOversizeFramesGoodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_oversize_frames_good::R`](R) reader structure"]
impl crate::Readable for RxOversizeFramesGoodSpec {}
#[doc = "`reset()` method sets RX_OVERSIZE_FRAMES_GOOD to value 0"]
impl crate::Resettable for RxOversizeFramesGoodSpec {
    const RESET_VALUE: u32 = 0;
}
