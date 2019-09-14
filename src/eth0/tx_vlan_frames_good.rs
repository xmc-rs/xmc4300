#[doc = "Reader of register TX_VLAN_FRAMES_GOOD"]
pub type R = crate::R<u32, super::TX_VLAN_FRAMES_GOOD>;
#[doc = "Reader of field `TXVLANG`"]
pub type TXVLANG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register maintains the number of transmitted good VLAN frames, exclusive of retried frames."]
    #[inline(always)]
    pub fn txvlang(&self) -> TXVLANG_R {
        TXVLANG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
