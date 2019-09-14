#[doc = "Reader of register ID"]
pub type R = crate::R<u32, super::ID>;
#[doc = "Reader of field `MOD_REV`"]
pub type MOD_REV_R = crate::R<u8, u8>;
#[doc = "Reader of field `MOD_TYPE`"]
pub type MOD_TYPE_R = crate::R<u8, u8>;
#[doc = "Reader of field `MOD_NUMBER`"]
pub type MOD_NUMBER_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:7 - Module Revision"]
    #[inline(always)]
    pub fn mod_rev(&self) -> MOD_REV_R {
        MOD_REV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Module Type"]
    #[inline(always)]
    pub fn mod_type(&self) -> MOD_TYPE_R {
        MOD_TYPE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Module Number"]
    #[inline(always)]
    pub fn mod_number(&self) -> MOD_NUMBER_R {
        MOD_NUMBER_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
