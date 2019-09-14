#[doc = "Reader of register DC1R"]
pub type R = crate::R<u32, super::DC1R>;
#[doc = "Writer for register DC1R"]
pub type W = crate::W<u32, super::DC1R>;
#[doc = "Register DC1R `reset()`'s with value 0"]
impl crate::ResetValue for super::DC1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DT1R`"]
pub type DT1R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DT1R`"]
pub struct DT1R_W<'a> {
    w: &'a mut W,
}
impl<'a> DT1R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DT1F`"]
pub type DT1F_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DT1F`"]
pub struct DT1F_W<'a> {
    w: &'a mut W,
}
impl<'a> DT1F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Rise Value for Dead Time of Channel 1"]
    #[inline(always)]
    pub fn dt1r(&self) -> DT1R_R {
        DT1R_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fall Value for Dead Time of Channel 1"]
    #[inline(always)]
    pub fn dt1f(&self) -> DT1F_R {
        DT1F_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Rise Value for Dead Time of Channel 1"]
    #[inline(always)]
    pub fn dt1r(&mut self) -> DT1R_W {
        DT1R_W { w: self }
    }
    #[doc = "Bits 8:15 - Fall Value for Dead Time of Channel 1"]
    #[inline(always)]
    pub fn dt1f(&mut self) -> DT1F_W {
        DT1F_W { w: self }
    }
}
