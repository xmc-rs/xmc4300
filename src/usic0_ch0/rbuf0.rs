#[doc = "Reader of register RBUF0"]
pub type R = crate::R<u32, super::RBUF0>;
#[doc = "Reader of field `DSR0`"]
pub type DSR0_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data of Shift Registers 0\\[3:0\\]"]
    #[inline(always)]
    pub fn dsr0(&self) -> DSR0_R {
        DSR0_R::new((self.bits & 0xffff) as u16)
    }
}
