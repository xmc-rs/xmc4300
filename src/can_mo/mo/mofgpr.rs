#[doc = "Reader of register MOFGPR"]
pub type R = crate::R<u32, super::MOFGPR>;
#[doc = "Writer for register MOFGPR"]
pub type W = crate::W<u32, super::MOFGPR>;
#[doc = "Register MOFGPR `reset()`'s with value 0"]
impl crate::ResetValue for super::MOFGPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BOT`"]
pub type BOT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BOT`"]
pub struct BOT_W<'a> {
    w: &'a mut W,
}
impl<'a> BOT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `TOP`"]
pub type TOP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOP`"]
pub struct TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CUR`"]
pub type CUR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CUR`"]
pub struct CUR_W<'a> {
    w: &'a mut W,
}
impl<'a> CUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL`"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Bottom Pointer"]
    #[inline(always)]
    pub fn bot(&self) -> BOT_R {
        BOT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Top Pointer"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Current Object Pointer"]
    #[inline(always)]
    pub fn cur(&self) -> CUR_R {
        CUR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Object Select Pointer"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bottom Pointer"]
    #[inline(always)]
    pub fn bot(&mut self) -> BOT_W {
        BOT_W { w: self }
    }
    #[doc = "Bits 8:15 - Top Pointer"]
    #[inline(always)]
    pub fn top(&mut self) -> TOP_W {
        TOP_W { w: self }
    }
    #[doc = "Bits 16:23 - Current Object Pointer"]
    #[inline(always)]
    pub fn cur(&mut self) -> CUR_W {
        CUR_W { w: self }
    }
    #[doc = "Bits 24:31 - Object Select Pointer"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
