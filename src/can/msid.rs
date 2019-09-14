#[doc = "Reader of register MSID[%s]"]
pub type R = crate::R<u32, super::MSID>;
#[doc = "Reader of field `INDEX`"]
pub type INDEX_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Message Pending Index"]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new((self.bits & 0x3f) as u8)
    }
}
