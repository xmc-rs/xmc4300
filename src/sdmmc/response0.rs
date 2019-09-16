#[doc = "Reader of register RESPONSE0"]
pub type R = crate::R<u32, super::RESPONSE0>;
#[doc = "Reader of field `RESPONSE1`"]
pub type RESPONSE1_R = crate::R<u16, u16>;
#[doc = "Reader of field `RESPONSE0`"]
pub type RESPONSE0_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - Response1"]
    #[inline(always)]
    pub fn response1(&self) -> RESPONSE1_R {
        RESPONSE1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Response0"]
    #[inline(always)]
    pub fn response0(&self) -> RESPONSE0_R {
        RESPONSE0_R::new((self.bits & 0xffff) as u16)
    }
}
