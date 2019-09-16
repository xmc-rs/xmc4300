#[doc = "Reader of register RESPONSE2"]
pub type R = crate::R<u32, super::RESPONSE2>;
#[doc = "Reader of field `RESPONSE3`"]
pub type RESPONSE3_R = crate::R<u16, u16>;
#[doc = "Reader of field `RESPONSE2`"]
pub type RESPONSE2_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - Response3"]
    #[inline(always)]
    pub fn response3(&self) -> RESPONSE3_R {
        RESPONSE3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Response2"]
    #[inline(always)]
    pub fn response2(&self) -> RESPONSE2_R {
        RESPONSE2_R::new((self.bits & 0xffff) as u16)
    }
}
