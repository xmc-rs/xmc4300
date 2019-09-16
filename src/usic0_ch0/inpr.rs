#[doc = "Reader of register INPR"]
pub type R = crate::R<u32, super::INPR>;
#[doc = "Writer for register INPR"]
pub type W = crate::W<u32, super::INPR>;
#[doc = "Register INPR `reset()`'s with value 0"]
impl crate::ResetValue for super::INPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transmit Shift Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSINP_A {
    #[doc = "0: Output SR0 becomes activated."]
    VALUE1,
    #[doc = "1: Output SR1 becomes activated."]
    VALUE2,
    #[doc = "2: Output SR2 becomes activated."]
    VALUE3,
    #[doc = "3: Output SR3 becomes activated."]
    VALUE4,
    #[doc = "4: Output SR4 becomes activated."]
    VALUE5,
    #[doc = "5: Output SR5 becomes activated."]
    VALUE6,
}
impl From<TSINP_A> for u8 {
    #[inline(always)]
    fn from(variant: TSINP_A) -> Self {
        match variant {
            TSINP_A::VALUE1 => 0,
            TSINP_A::VALUE2 => 1,
            TSINP_A::VALUE3 => 2,
            TSINP_A::VALUE4 => 3,
            TSINP_A::VALUE5 => 4,
            TSINP_A::VALUE6 => 5,
        }
    }
}
#[doc = "Reader of field `TSINP`"]
pub type TSINP_R = crate::R<u8, TSINP_A>;
impl TSINP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSINP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSINP_A::VALUE1),
            1 => Val(TSINP_A::VALUE2),
            2 => Val(TSINP_A::VALUE3),
            3 => Val(TSINP_A::VALUE4),
            4 => Val(TSINP_A::VALUE5),
            5 => Val(TSINP_A::VALUE6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TSINP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TSINP_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == TSINP_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == TSINP_A::VALUE6
    }
}
#[doc = "Write proxy for field `TSINP`"]
pub struct TSINP_W<'a> {
    w: &'a mut W,
}
impl<'a> TSINP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSINP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSINP_A::VALUE1)
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSINP_A::VALUE2)
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TSINP_A::VALUE3)
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(TSINP_A::VALUE4)
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(TSINP_A::VALUE5)
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(TSINP_A::VALUE6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `TBINP`"]
pub type TBINP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TBINP`"]
pub struct TBINP_W<'a> {
    w: &'a mut W,
}
impl<'a> TBINP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `RINP`"]
pub type RINP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RINP`"]
pub struct RINP_W<'a> {
    w: &'a mut W,
}
impl<'a> RINP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `AINP`"]
pub type AINP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AINP`"]
pub struct AINP_W<'a> {
    w: &'a mut W,
}
impl<'a> AINP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `PINP`"]
pub type PINP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PINP`"]
pub struct PINP_W<'a> {
    w: &'a mut W,
}
impl<'a> PINP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Transmit Shift Interrupt Node Pointer"]
    #[inline(always)]
    pub fn tsinp(&self) -> TSINP_R {
        TSINP_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Transmit Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn tbinp(&self) -> TBINP_R {
        TBINP_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Receive Interrupt Node Pointer"]
    #[inline(always)]
    pub fn rinp(&self) -> RINP_R {
        RINP_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Alternative Receive Interrupt Node Pointer"]
    #[inline(always)]
    pub fn ainp(&self) -> AINP_R {
        AINP_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Transmit Shift Interrupt Node Pointer"]
    #[inline(always)]
    pub fn pinp(&self) -> PINP_R {
        PINP_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmit Shift Interrupt Node Pointer"]
    #[inline(always)]
    pub fn tsinp(&mut self) -> TSINP_W {
        TSINP_W { w: self }
    }
    #[doc = "Bits 4:6 - Transmit Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn tbinp(&mut self) -> TBINP_W {
        TBINP_W { w: self }
    }
    #[doc = "Bits 8:10 - Receive Interrupt Node Pointer"]
    #[inline(always)]
    pub fn rinp(&mut self) -> RINP_W {
        RINP_W { w: self }
    }
    #[doc = "Bits 12:14 - Alternative Receive Interrupt Node Pointer"]
    #[inline(always)]
    pub fn ainp(&mut self) -> AINP_W {
        AINP_W { w: self }
    }
    #[doc = "Bits 16:18 - Transmit Shift Interrupt Node Pointer"]
    #[inline(always)]
    pub fn pinp(&mut self) -> PINP_W {
        PINP_W { w: self }
    }
}
