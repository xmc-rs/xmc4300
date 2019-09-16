#[doc = "Reader of register RESPONSE4"]
pub type R = crate::R<u32, super::RESPONSE4>;
#[doc = "Reader of field `RESPONSE5`"]
pub type RESPONSE5_R = crate::R<u16, u16>;
#[doc = "Reader of field `RESPONSE4`"]
pub type RESPONSE4_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - Response5"]
    #[inline(always)]
    pub fn response5(&self) -> RESPONSE5_R {
        RESPONSE5_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Response4"]
    #[inline(always)]
    pub fn response4(&self) -> RESPONSE4_R {
        RESPONSE4_R::new((self.bits & 0xffff) as u16)
    }
}
