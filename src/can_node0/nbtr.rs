#[doc = "Reader of register NBTR"]
pub type R = crate::R<u32, super::NBTR>;
#[doc = "Writer for register NBTR"]
pub type W = crate::W<u32, super::NBTR>;
#[doc = "Register NBTR `reset()`'s with value 0"]
impl crate::ResetValue for super::NBTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BRP`"]
pub type BRP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BRP`"]
pub struct BRP_W<'a> {
    w: &'a mut W,
}
impl<'a> BRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `SJW`"]
pub type SJW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SJW`"]
pub struct SJW_W<'a> {
    w: &'a mut W,
}
impl<'a> SJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `TSEG1`"]
pub type TSEG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TSEG1`"]
pub struct TSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TSEG2`"]
pub type TSEG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TSEG2`"]
pub struct TSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Divide Prescaler Clock by 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIV8_A {
    #[doc = "0: A time quantum lasts (BRP+1) clock cycles."]
    VALUE1 = 0,
    #[doc = "1: A time quantum lasts 8 (BRP+1) clock cycles."]
    VALUE2 = 1,
}
impl From<DIV8_A> for bool {
    #[inline(always)]
    fn from(variant: DIV8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIV8`"]
pub type DIV8_R = crate::R<bool, DIV8_A>;
impl DIV8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIV8_A {
        match self.bits {
            false => DIV8_A::VALUE1,
            true => DIV8_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIV8_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIV8_A::VALUE2
    }
}
#[doc = "Write proxy for field `DIV8`"]
pub struct DIV8_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIV8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A time quantum lasts (BRP+1) clock cycles."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIV8_A::VALUE1)
    }
    #[doc = "A time quantum lasts 8 (BRP+1) clock cycles."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIV8_A::VALUE2)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn tseg1(&self) -> TSEG1_R {
        TSEG1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Time Segment After Sample Point"]
    #[inline(always)]
    pub fn tseg2(&self) -> TSEG2_R {
        TSEG2_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Divide Prescaler Clock by 8"]
    #[inline(always)]
    pub fn div8(&self) -> DIV8_R {
        DIV8_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn brp(&mut self) -> BRP_W {
        BRP_W { w: self }
    }
    #[doc = "Bits 6:7 - (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn sjw(&mut self) -> SJW_W {
        SJW_W { w: self }
    }
    #[doc = "Bits 8:11 - Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn tseg1(&mut self) -> TSEG1_W {
        TSEG1_W { w: self }
    }
    #[doc = "Bits 12:14 - Time Segment After Sample Point"]
    #[inline(always)]
    pub fn tseg2(&mut self) -> TSEG2_W {
        TSEG2_W { w: self }
    }
    #[doc = "Bit 15 - Divide Prescaler Clock by 8"]
    #[inline(always)]
    pub fn div8(&mut self) -> DIV8_W {
        DIV8_W { w: self }
    }
}
