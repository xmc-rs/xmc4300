#[doc = "Register `RX_VLAN_FRAMES_GOOD_BAD` reader"]
pub type R = crate::R<RxVlanFramesGoodBadSpec>;
#[doc = "Field `RXVLANFRGB` reader - This field indicates the number of received good and bad VLAN frames."]
pub type RxvlanfrgbR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad VLAN frames."]
    #[inline(always)]
    pub fn rxvlanfrgb(&self) -> RxvlanfrgbR {
        RxvlanfrgbR::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Good and Bad VLAN Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_vlan_frames_good_bad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxVlanFramesGoodBadSpec;
impl crate::RegisterSpec for RxVlanFramesGoodBadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_vlan_frames_good_bad::R`](R) reader structure"]
impl crate::Readable for RxVlanFramesGoodBadSpec {}
#[doc = "`reset()` method sets RX_VLAN_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for RxVlanFramesGoodBadSpec {
    const RESET_VALUE: u32 = 0;
}
