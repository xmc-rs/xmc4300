#[doc = "Reader of register DC2R"]
pub type R = crate::R<u32, super::DC2R>;
#[doc = "Writer for register DC2R"]
pub type W = crate::W<u32, super::DC2R>;
#[doc = "Register DC2R `reset()`'s with value 0"]
impl crate::ResetValue for super::DC2R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DT2R`"]
pub type DT2R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DT2R`"]
pub struct DT2R_W<'a> {
    w: &'a mut W,
}
impl<'a> DT2R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DT2F`"]
pub type DT2F_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DT2F`"]
pub struct DT2F_W<'a> {
    w: &'a mut W,
}
impl<'a> DT2F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Rise Value for Dead Time of Channel 2"]
    #[inline(always)]
    pub fn dt2r(&self) -> DT2R_R {
        DT2R_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fall Value for Dead Time of Channel 2"]
    #[inline(always)]
    pub fn dt2f(&self) -> DT2F_R {
        DT2F_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Rise Value for Dead Time of Channel 2"]
    #[inline(always)]
    pub fn dt2r(&mut self) -> DT2R_W {
        DT2R_W { w: self }
    }
    #[doc = "Bits 8:15 - Fall Value for Dead Time of Channel 2"]
    #[inline(always)]
    pub fn dt2f(&mut self) -> DT2F_W {
        DT2F_W { w: self }
    }
}
