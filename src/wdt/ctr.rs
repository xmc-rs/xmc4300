#[doc = "Reader of register CTR"]
pub type R = crate::R<u32, super::CTR>;
#[doc = "Writer for register CTR"]
pub type W = crate::W<u32, super::CTR>;
#[doc = "Register CTR `reset()`'s with value 0"]
impl crate::ResetValue for super::CTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `PRE`"]
pub type PRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRE`"]
pub struct PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_W<'a> {
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
#[doc = "Reader of field `DSP`"]
pub type DSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSP`"]
pub struct DSP_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SPW`"]
pub type SPW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPW`"]
pub struct SPW_W<'a> {
    w: &'a mut W,
}
impl<'a> SPW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pre-warning"]
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Debug Suspend"]
    #[inline(always)]
    pub fn dsp(&self) -> DSP_R {
        DSP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Service Indication Pulse Width"]
    #[inline(always)]
    pub fn spw(&self) -> SPW_R {
        SPW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enb(&mut self) -> ENB_W {
        ENB_W { w: self }
    }
    #[doc = "Bit 1 - Pre-warning"]
    #[inline(always)]
    pub fn pre(&mut self) -> PRE_W {
        PRE_W { w: self }
    }
    #[doc = "Bit 4 - Debug Suspend"]
    #[inline(always)]
    pub fn dsp(&mut self) -> DSP_W {
        DSP_W { w: self }
    }
    #[doc = "Bits 8:15 - Service Indication Pulse Width"]
    #[inline(always)]
    pub fn spw(&mut self) -> SPW_W {
        SPW_W { w: self }
    }
}
