#[doc = "Reader of register TIM0"]
pub type R = crate::R<u32, super::TIM0>;
#[doc = "Writer for register TIM0"]
pub type W = crate::W<u32, super::TIM0>;
#[doc = "Register TIM0 `reset()`'s with value 0"]
impl crate::ResetValue for super::TIM0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SE`"]
pub type SE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SE`"]
pub struct SE_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `MI`"]
pub type MI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MI`"]
pub struct MI_W<'a> {
    w: &'a mut W,
}
impl<'a> MI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `HO`"]
pub type HO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HO`"]
pub struct HO_W<'a> {
    w: &'a mut W,
}
impl<'a> HO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `DA`"]
pub type DA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DA`"]
pub struct DA_W<'a> {
    w: &'a mut W,
}
impl<'a> DA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Seconds Time Value"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Minutes Time Value"]
    #[inline(always)]
    pub fn mi(&self) -> MI_R {
        MI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Hours Time Value"]
    #[inline(always)]
    pub fn ho(&self) -> HO_R {
        HO_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Days Time Value"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Seconds Time Value"]
    #[inline(always)]
    pub fn se(&mut self) -> SE_W {
        SE_W { w: self }
    }
    #[doc = "Bits 8:13 - Minutes Time Value"]
    #[inline(always)]
    pub fn mi(&mut self) -> MI_W {
        MI_W { w: self }
    }
    #[doc = "Bits 16:20 - Hours Time Value"]
    #[inline(always)]
    pub fn ho(&mut self) -> HO_W {
        HO_W { w: self }
    }
    #[doc = "Bits 24:28 - Days Time Value"]
    #[inline(always)]
    pub fn da(&mut self) -> DA_W {
        DA_W { w: self }
    }
}
