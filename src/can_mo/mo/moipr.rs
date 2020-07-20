#[doc = "Reader of register MOIPR"]
pub type R = crate::R<u32, super::MOIPR>;
#[doc = "Writer for register MOIPR"]
pub type W = crate::W<u32, super::MOIPR>;
#[doc = "Register MOIPR `reset()`'s with value 0"]
impl crate::ResetValue for super::MOIPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Receive Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2 = 1,
    #[doc = "14: Interrupt output line INT_O14 is selected."]
    VALUE3 = 14,
    #[doc = "15: Interrupt output line INT_O15 is selected."]
    VALUE4 = 15,
}
impl From<RXINP_A> for u8 {
    #[inline(always)]
    fn from(variant: RXINP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RXINP`"]
pub type RXINP_R = crate::R<u8, RXINP_A>;
impl RXINP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RXINP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RXINP_A::VALUE1),
            1 => Val(RXINP_A::VALUE2),
            14 => Val(RXINP_A::VALUE3),
            15 => Val(RXINP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RXINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RXINP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RXINP_A::VALUE4
    }
}
#[doc = "Write proxy for field `RXINP`"]
pub struct RXINP_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXINP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(RXINP_A::VALUE3)
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(RXINP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Transmit Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2 = 1,
    #[doc = "14: Interrupt output line INT_O14 is selected."]
    VALUE3 = 14,
    #[doc = "15: Interrupt output line INT_O15 is selected."]
    VALUE4 = 15,
}
impl From<TXINP_A> for u8 {
    #[inline(always)]
    fn from(variant: TXINP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXINP`"]
pub type TXINP_R = crate::R<u8, TXINP_A>;
impl TXINP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXINP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXINP_A::VALUE1),
            1 => Val(TXINP_A::VALUE2),
            14 => Val(TXINP_A::VALUE3),
            15 => Val(TXINP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TXINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TXINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TXINP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TXINP_A::VALUE4
    }
}
#[doc = "Write proxy for field `TXINP`"]
pub struct TXINP_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXINP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TXINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TXINP_A::VALUE3)
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(TXINP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `MPN`"]
pub type MPN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MPN`"]
pub struct MPN_W<'a> {
    w: &'a mut W,
}
impl<'a> MPN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CFCVAL`"]
pub type CFCVAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CFCVAL`"]
pub struct CFCVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CFCVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Receive Interrupt Node Pointer"]
    #[inline(always)]
    pub fn rxinp(&self) -> RXINP_R {
        RXINP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Transmit Interrupt Node Pointer"]
    #[inline(always)]
    pub fn txinp(&self) -> TXINP_R {
        TXINP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Message Pending Number"]
    #[inline(always)]
    pub fn mpn(&self) -> MPN_R {
        MPN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - CAN Frame Counter Value"]
    #[inline(always)]
    pub fn cfcval(&self) -> CFCVAL_R {
        CFCVAL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Receive Interrupt Node Pointer"]
    #[inline(always)]
    pub fn rxinp(&mut self) -> RXINP_W {
        RXINP_W { w: self }
    }
    #[doc = "Bits 4:7 - Transmit Interrupt Node Pointer"]
    #[inline(always)]
    pub fn txinp(&mut self) -> TXINP_W {
        TXINP_W { w: self }
    }
    #[doc = "Bits 8:15 - Message Pending Number"]
    #[inline(always)]
    pub fn mpn(&mut self) -> MPN_W {
        MPN_W { w: self }
    }
    #[doc = "Bits 16:31 - CAN Frame Counter Value"]
    #[inline(always)]
    pub fn cfcval(&mut self) -> CFCVAL_W {
        CFCVAL_W { w: self }
    }
}
