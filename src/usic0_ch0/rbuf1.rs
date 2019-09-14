#[doc = "Reader of register RBUF1"]
pub type R = crate::R<u32, super::RBUF1>;
#[doc = "Reader of field `DSR1`"]
pub type DSR1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data of Shift Registers 1\\[3:0\\]"]
    #[inline(always)]
    pub fn dsr1(&self) -> DSR1_R {
        DSR1_R::new((self.bits & 0xffff) as u16)
    }
}
