#[doc = "Reader of register CR1"]
pub type R = crate::R<u32, super::CR1>;
#[doc = "Reader of field `CR1`"]
pub type CR1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Compare Register for Channel 1"]
    #[inline(always)]
    pub fn cr1(&self) -> CR1_R {
        CR1_R::new((self.bits & 0xffff) as u16)
    }
}
