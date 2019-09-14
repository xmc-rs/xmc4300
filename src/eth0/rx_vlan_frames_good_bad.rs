#[doc = "Reader of register RX_VLAN_FRAMES_GOOD_BAD"]
pub type R = crate::R<u32, super::RX_VLAN_FRAMES_GOOD_BAD>;
#[doc = "Reader of field `RXVLANFRGB`"]
pub type RXVLANFRGB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad VLAN frames."]
    #[inline(always)]
    pub fn rxvlanfrgb(&self) -> RXVLANFRGB_R {
        RXVLANFRGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
