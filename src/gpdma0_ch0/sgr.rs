#[doc = "Reader of register SGR"]
pub type R = crate::R<u32, super::SGR>;
#[doc = "Writer for register SGR"]
pub type W = crate::W<u32, super::SGR>;
#[doc = "Register SGR `reset()`'s with value 0"]
impl crate::ResetValue for super::SGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SGC`"]
pub type SGC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SGC`"]
pub struct SGC_W<'a> {
    w: &'a mut W,
}
impl<'a> SGC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | (((value as u32) & 0x0fff) << 20);
        self.w
    }
}
#[doc = "Reader of field `SGI`"]
pub type SGI_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SGI`"]
pub struct SGI_W<'a> {
    w: &'a mut W,
}
impl<'a> SGI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:31 - Source gather count"]
    #[inline(always)]
    pub fn sgc(&self) -> SGC_R {
        SGC_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:19 - Source gather interval"]
    #[inline(always)]
    pub fn sgi(&self) -> SGI_R {
        SGI_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 20:31 - Source gather count"]
    #[inline(always)]
    pub fn sgc(&mut self) -> SGC_W {
        SGC_W { w: self }
    }
    #[doc = "Bits 0:19 - Source gather interval"]
    #[inline(always)]
    pub fn sgi(&mut self) -> SGI_W {
        SGI_W { w: self }
    }
}
