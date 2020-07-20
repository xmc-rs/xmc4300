#[doc = "Reader of register MOAR"]
pub type R = crate::R<u32, super::MOAR>;
#[doc = "Writer for register MOAR"]
pub type W = crate::W<u32, super::MOAR>;
#[doc = "Register MOAR `reset()`'s with value 0"]
impl crate::ResetValue for super::MOAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ID`"]
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | ((value as u32) & 0x1fff_ffff);
        self.w
    }
}
#[doc = "Identifier Extension Bit of Message Object n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDE_A {
    #[doc = "0: Message object n handles standard frames with 11-bit identifier."]
    VALUE1 = 0,
    #[doc = "1: Message object n handles extended frames with 29-bit identifier."]
    VALUE2 = 1,
}
impl From<IDE_A> for bool {
    #[inline(always)]
    fn from(variant: IDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IDE`"]
pub type IDE_R = crate::R<bool, IDE_A>;
impl IDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDE_A {
        match self.bits {
            false => IDE_A::VALUE1,
            true => IDE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IDE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IDE_A::VALUE2
    }
}
#[doc = "Write proxy for field `IDE`"]
pub struct IDE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Message object n handles standard frames with 11-bit identifier."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IDE_A::VALUE1)
    }
    #[doc = "Message object n handles extended frames with 29-bit identifier."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(IDE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Priority Class\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRI_A {
    #[doc = "1: Transmit acceptance filtering is based on the list order. This means that message object n is considered for transmission only if there is no other message object with valid transmit request (MSGVAL & TXEN0 & TXEN1 = 1) somewhere before this object in the list."]
    VALUE2 = 1,
    #[doc = "2: Transmit acceptance filtering is based on the CAN identifier. This means, message object n is considered for transmission only if there is no other message object with higher priority identifier + IDE + DIR (with respect to CAN arbitration rules) somewhere in the list (see )."]
    VALUE3 = 2,
    #[doc = "3: Transmit acceptance filtering is based on the list order (as PRI = 01B)."]
    VALUE4 = 3,
}
impl From<PRI_A> for u8 {
    #[inline(always)]
    fn from(variant: PRI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRI`"]
pub type PRI_R = crate::R<u8, PRI_A>;
impl PRI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRI_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(PRI_A::VALUE2),
            2 => Val(PRI_A::VALUE3),
            3 => Val(PRI_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRI_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PRI_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PRI_A::VALUE4
    }
}
#[doc = "Write proxy for field `PRI`"]
pub struct PRI_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRI_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Transmit acceptance filtering is based on the list order. This means that message object n is considered for transmission only if there is no other message object with valid transmit request (MSGVAL & TXEN0 & TXEN1 = 1) somewhere before this object in the list."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRI_A::VALUE2)
    }
    #[doc = "Transmit acceptance filtering is based on the CAN identifier. This means, message object n is considered for transmission only if there is no other message object with higher priority identifier + IDE + DIR (with respect to CAN arbitration rules) somewhere in the list (see )."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PRI_A::VALUE3)
    }
    #[doc = "Transmit acceptance filtering is based on the list order (as PRI = 01B)."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PRI_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:28 - CAN Identifier of Message Object n"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 29 - Identifier Extension Bit of Message Object n"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - Priority Class"]
    #[inline(always)]
    pub fn pri(&self) -> PRI_R {
        PRI_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:28 - CAN Identifier of Message Object n"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
    #[doc = "Bit 29 - Identifier Extension Bit of Message Object n"]
    #[inline(always)]
    pub fn ide(&mut self) -> IDE_W {
        IDE_W { w: self }
    }
    #[doc = "Bits 30:31 - Priority Class"]
    #[inline(always)]
    pub fn pri(&mut self) -> PRI_W {
        PRI_W { w: self }
    }
}
