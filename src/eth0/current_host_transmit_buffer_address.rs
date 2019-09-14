#[doc = "Reader of register CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS"]
pub type R = crate::R<u32, super::CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS>;
#[doc = "Reader of field `CURTBUFAPTR`"]
pub type CURTBUFAPTR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Buffer Address Pointer"]
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CURTBUFAPTR_R {
        CURTBUFAPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
