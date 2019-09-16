#[doc = "Reader of register GUID"]
pub type R = crate::R<u32, super::GUID>;
#[doc = "Writer for register GUID"]
pub type W = crate::W<u32, super::GUID>;
#[doc = "Register GUID `reset()`'s with value 0x00ae_c000"]
impl crate::ResetValue for super::GUID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00ae_c000
    }
}
#[doc = "Reader of field `MOD_REV`"]
pub type MOD_REV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MOD_REV`"]
pub struct MOD_REV_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_REV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `MOD_TYPE`"]
pub type MOD_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MOD_TYPE`"]
pub struct MOD_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `MOD_NUMBER`"]
pub type MOD_NUMBER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MOD_NUMBER`"]
pub struct MOD_NUMBER_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_NUMBER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
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
impl W {
    #[doc = "Bits 0:7 - Module Revision"]
    #[inline(always)]
    pub fn mod_rev(&mut self) -> MOD_REV_W {
        MOD_REV_W { w: self }
    }
    #[doc = "Bits 8:15 - Module Type"]
    #[inline(always)]
    pub fn mod_type(&mut self) -> MOD_TYPE_W {
        MOD_TYPE_W { w: self }
    }
    #[doc = "Bits 16:31 - Module Number"]
    #[inline(always)]
    pub fn mod_number(&mut self) -> MOD_NUMBER_W {
        MOD_NUMBER_W { w: self }
    }
}
