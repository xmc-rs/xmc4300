#[doc = "Register `TX_VLAN_FRAMES_GOOD` reader"]
pub type R = crate::R<TX_VLAN_FRAMES_GOOD_SPEC>;
#[doc = "Field `TXVLANG` reader - This register maintains the number of transmitted good VLAN frames, exclusive of retried frames."]
pub type TXVLANG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register maintains the number of transmitted good VLAN frames, exclusive of retried frames."]
    #[inline(always)]
    pub fn txvlang(&self) -> TXVLANG_R {
        TXVLANG_R::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Good VLAN Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_vlan_frames_good::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_VLAN_FRAMES_GOOD_SPEC;
impl crate::RegisterSpec for TX_VLAN_FRAMES_GOOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_vlan_frames_good::R`](R) reader structure"]
impl crate::Readable for TX_VLAN_FRAMES_GOOD_SPEC {}
#[doc = "`reset()` method sets TX_VLAN_FRAMES_GOOD to value 0"]
impl crate::Resettable for TX_VLAN_FRAMES_GOOD_SPEC {
    const RESET_VALUE: u32 = 0;
}
