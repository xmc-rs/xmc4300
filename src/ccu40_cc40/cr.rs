#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Reader of field `CR`"]
pub type CR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Compare Register"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 0xffff) as u16)
    }
}
