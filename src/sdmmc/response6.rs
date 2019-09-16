#[doc = "Reader of register RESPONSE6"]
pub type R = crate::R<u32, super::RESPONSE6>;
#[doc = "Reader of field `RESPONSE7`"]
pub type RESPONSE7_R = crate::R<u16, u16>;
#[doc = "Reader of field `RESPONSE6`"]
pub type RESPONSE6_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - Response7"]
    #[inline(always)]
    pub fn response7(&self) -> RESPONSE7_R {
        RESPONSE7_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Response6"]
    #[inline(always)]
    pub fn response6(&self) -> RESPONSE6_R {
        RESPONSE6_R::new((self.bits & 0xffff) as u16)
    }
}
