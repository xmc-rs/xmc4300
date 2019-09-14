#[doc = "Reader of register PWRMON"]
pub type R = crate::R<u32, super::PWRMON>;
#[doc = "Writer for register PWRMON"]
pub type W = crate::W<u32, super::PWRMON>;
#[doc = "Register PWRMON `reset()`'s with value 0"]
impl crate::ResetValue for super::PWRMON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `THRS`"]
pub type THRS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `THRS`"]
pub struct THRS_W<'a> {
    w: &'a mut W,
}
impl<'a> THRS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `INTV`"]
pub type INTV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTV`"]
pub struct INTV_W<'a> {
    w: &'a mut W,
}
impl<'a> INTV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `ENB`"]
pub type ENB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENB`"]
pub struct ENB_W<'a> {
    w: &'a mut W,
}
impl<'a> ENB_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Threshold"]
    #[inline(always)]
    pub fn thrs(&self) -> THRS_R {
        THRS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interval"]
    #[inline(always)]
    pub fn intv(&self) -> INTV_R {
        INTV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Enable"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Threshold"]
    #[inline(always)]
    pub fn thrs(&mut self) -> THRS_W {
        THRS_W { w: self }
    }
    #[doc = "Bits 8:15 - Interval"]
    #[inline(always)]
    pub fn intv(&mut self) -> INTV_W {
        INTV_W { w: self }
    }
    #[doc = "Bit 16 - Enable"]
    #[inline(always)]
    pub fn enb(&mut self) -> ENB_W {
        ENB_W { w: self }
    }
}
