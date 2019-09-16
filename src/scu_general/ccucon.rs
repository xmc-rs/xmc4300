#[doc = "Reader of register CCUCON"]
pub type R = crate::R<u32, super::CCUCON>;
#[doc = "Writer for register CCUCON"]
pub type W = crate::W<u32, super::CCUCON>;
#[doc = "Register CCUCON `reset()`'s with value 0"]
impl crate::ResetValue for super::CCUCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GSC40`"]
pub type GSC40_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GSC40`"]
pub struct GSC40_W<'a> {
    w: &'a mut W,
}
impl<'a> GSC40_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `GSC41`"]
pub type GSC41_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GSC41`"]
pub struct GSC41_W<'a> {
    w: &'a mut W,
}
impl<'a> GSC41_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `GSC80`"]
pub type GSC80_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GSC80`"]
pub struct GSC80_W<'a> {
    w: &'a mut W,
}
impl<'a> GSC80_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Global Start Control CCU40"]
    #[inline(always)]
    pub fn gsc40(&self) -> GSC40_R {
        GSC40_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Global Start Control CCU41"]
    #[inline(always)]
    pub fn gsc41(&self) -> GSC41_R {
        GSC41_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Global Start Control CCU80"]
    #[inline(always)]
    pub fn gsc80(&self) -> GSC80_R {
        GSC80_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global Start Control CCU40"]
    #[inline(always)]
    pub fn gsc40(&mut self) -> GSC40_W {
        GSC40_W { w: self }
    }
    #[doc = "Bit 1 - Global Start Control CCU41"]
    #[inline(always)]
    pub fn gsc41(&mut self) -> GSC41_W {
        GSC41_W { w: self }
    }
    #[doc = "Bit 8 - Global Start Control CCU80"]
    #[inline(always)]
    pub fn gsc80(&mut self) -> GSC80_W {
        GSC80_W { w: self }
    }
}
