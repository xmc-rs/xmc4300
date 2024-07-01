#[doc = "Register `RX_VLAN_FRAMES_GOOD_BAD` reader"]
pub type R = crate::R<RX_VLAN_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Field `RXVLANFRGB` reader - This field indicates the number of received good and bad VLAN frames."]
pub type RXVLANFRGB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad VLAN frames."]
    #[inline(always)]
    pub fn rxvlanfrgb(&self) -> RXVLANFRGB_R {
        RXVLANFRGB_R::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Good and Bad VLAN Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_vlan_frames_good_bad::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_VLAN_FRAMES_GOOD_BAD_SPEC;
impl crate::RegisterSpec for RX_VLAN_FRAMES_GOOD_BAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_vlan_frames_good_bad::R`](R) reader structure"]
impl crate::Readable for RX_VLAN_FRAMES_GOOD_BAD_SPEC {}
#[doc = "`reset()` method sets RX_VLAN_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for RX_VLAN_FRAMES_GOOD_BAD_SPEC {
    const RESET_VALUE: u32 = 0;
}
