#[doc = "Reader of register FDR"]
pub type R = crate::R<u32, super::FDR>;
#[doc = "Writer for register FDR"]
pub type W = crate::W<u32, super::FDR>;
#[doc = "Register FDR `reset()`'s with value 0"]
impl crate::ResetValue for super::FDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STEP`"]
pub type STEP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `STEP`"]
pub struct STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> STEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `DM`"]
pub type DM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DM`"]
pub struct DM_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Step Value"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 14:15 - Divider Mode"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Step Value"]
    #[inline(always)]
    pub fn step(&mut self) -> STEP_W {
        STEP_W { w: self }
    }
    #[doc = "Bits 14:15 - Divider Mode"]
    #[inline(always)]
    pub fn dm(&mut self) -> DM_W {
        DM_W { w: self }
    }
}
