#[doc = "Reader of register CURRENT_HOST_RECEIVE_DESCRIPTOR"]
pub type R = crate::R<u32, super::CURRENT_HOST_RECEIVE_DESCRIPTOR>;
#[doc = "Reader of field `CURRDESAPTR`"]
pub type CURRDESAPTR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Descriptor Address Pointer"]
    #[inline(always)]
    pub fn currdesaptr(&self) -> CURRDESAPTR_R {
        CURRDESAPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
