#[doc = "Reader of register ID"]
pub type R = crate::R<u32, super::ID>;
#[doc = "Reader of field `MODR`"]
pub type MODR_R = crate::R<u8, u8>;
#[doc = "Reader of field `MODT`"]
pub type MODT_R = crate::R<u8, u8>;
#[doc = "Reader of field `MODN`"]
pub type MODN_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:7 - Module Revision"]
    #[inline(always)]
    pub fn modr(&self) -> MODR_R {
        MODR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Module Type"]
    #[inline(always)]
    pub fn modt(&self) -> MODT_R {
        MODT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Module Number"]
    #[inline(always)]
    pub fn modn(&self) -> MODN_R {
        MODN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
