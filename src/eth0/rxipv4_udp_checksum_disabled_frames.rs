#[doc = "Reader of register RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES"]
pub type R = crate::R<u32, super::RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES>;
#[doc = "Reader of field `RXIPV4UDSBLFRM`"]
pub type RXIPV4UDSBLFRM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good IPv4 datagrams which have the UDP payload with checksum disabled."]
    #[inline(always)]
    pub fn rxipv4udsblfrm(&self) -> RXIPV4UDSBLFRM_R {
        RXIPV4UDSBLFRM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
