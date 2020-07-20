#[doc = "Reader of register NECNT"]
pub type R = crate::R<u32, super::NECNT>;
#[doc = "Writer for register NECNT"]
pub type W = crate::W<u32, super::NECNT>;
#[doc = "Register NECNT `reset()`'s with value 0x0060_0000"]
impl crate::ResetValue for super::NECNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0060_0000
    }
}
#[doc = "Reader of field `REC`"]
pub type REC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REC`"]
pub struct REC_W<'a> {
    w: &'a mut W,
}
impl<'a> REC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `TEC`"]
pub type TEC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TEC`"]
pub struct TEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `EWRNLVL`"]
pub type EWRNLVL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EWRNLVL`"]
pub struct EWRNLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> EWRNLVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Last Error Transfer Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LETD_A {
    #[doc = "0: The last error occurred while the CAN node x was receiver (REC has been incremented)."]
    VALUE1 = 0,
    #[doc = "1: The last error occurred while the CAN node x was transmitter (TEC has been incremented)."]
    VALUE2 = 1,
}
impl From<LETD_A> for bool {
    #[inline(always)]
    fn from(variant: LETD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LETD`"]
pub type LETD_R = crate::R<bool, LETD_A>;
impl LETD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LETD_A {
        match self.bits {
            false => LETD_A::VALUE1,
            true => LETD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LETD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LETD_A::VALUE2
    }
}
#[doc = "Last Error Increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEINC_A {
    #[doc = "0: The last error led to an error counter increment of 1."]
    VALUE1 = 0,
    #[doc = "1: The last error led to an error counter increment of 8."]
    VALUE2 = 1,
}
impl From<LEINC_A> for bool {
    #[inline(always)]
    fn from(variant: LEINC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEINC`"]
pub type LEINC_R = crate::R<bool, LEINC_A>;
impl LEINC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEINC_A {
        match self.bits {
            false => LEINC_A::VALUE1,
            true => LEINC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LEINC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LEINC_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive Error Counter"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Transmit Error Counter"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Error Warning Level"]
    #[inline(always)]
    pub fn ewrnlvl(&self) -> EWRNLVL_R {
        EWRNLVL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Last Error Transfer Direction"]
    #[inline(always)]
    pub fn letd(&self) -> LETD_R {
        LETD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Last Error Increment"]
    #[inline(always)]
    pub fn leinc(&self) -> LEINC_R {
        LEINC_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Error Counter"]
    #[inline(always)]
    pub fn rec(&mut self) -> REC_W {
        REC_W { w: self }
    }
    #[doc = "Bits 8:15 - Transmit Error Counter"]
    #[inline(always)]
    pub fn tec(&mut self) -> TEC_W {
        TEC_W { w: self }
    }
    #[doc = "Bits 16:23 - Error Warning Level"]
    #[inline(always)]
    pub fn ewrnlvl(&mut self) -> EWRNLVL_W {
        EWRNLVL_W { w: self }
    }
}
