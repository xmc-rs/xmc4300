#[doc = "Reader of register RX_JABBER_ERROR_FRAMES"]
pub type R = crate::R<u32, super::RX_JABBER_ERROR_FRAMES>;
#[doc = "Reader of field `RXJABERR`"]
pub type RXJABERR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of giant frames received with length (including CRC) greater than 1,518 bytes (1,522 bytes for VLAN tagged) and with CRC error. If Jumbo Frame mode is enabled, then frames of length greater than 9,018 bytes (9,022 for VLAN tagged) are considered as giant frames."]
    #[inline(always)]
    pub fn rxjaberr(&self) -> RXJABERR_R {
        RXJABERR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
