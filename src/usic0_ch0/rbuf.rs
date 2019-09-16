#[doc = "Reader of register RBUF"]
pub type R = crate::R<u32, super::RBUF>;
#[doc = "Reader of field `DSR`"]
pub type DSR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Received Data"]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new((self.bits & 0xffff) as u16)
    }
}
