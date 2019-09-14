#[doc = "Reader of register SRS"]
pub type R = crate::R<u32, super::SRS>;
#[doc = "Writer for register SRS"]
pub type W = crate::W<u32, super::SRS>;
#[doc = "Register SRS `reset()`'s with value 0"]
impl crate::ResetValue for super::SRS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Period/One match Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSR_A {
    #[doc = "0: Forward to CC4ySR0"]
    VALUE1,
    #[doc = "1: Forward to CC4ySR1"]
    VALUE2,
    #[doc = "2: Forward to CC4ySR2"]
    VALUE3,
    #[doc = "3: Forward to CC4ySR3"]
    VALUE4,
}
impl From<POSR_A> for u8 {
    #[inline(always)]
    fn from(variant: POSR_A) -> Self {
        match variant {
            POSR_A::VALUE1 => 0,
            POSR_A::VALUE2 => 1,
            POSR_A::VALUE3 => 2,
            POSR_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `POSR`"]
pub type POSR_R = crate::R<u8, POSR_A>;
impl POSR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POSR_A {
        match self.bits {
            0 => POSR_A::VALUE1,
            1 => POSR_A::VALUE2,
            2 => POSR_A::VALUE3,
            3 => POSR_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == POSR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == POSR_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == POSR_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == POSR_A::VALUE4
    }
}
#[doc = "Write proxy for field `POSR`"]
pub struct POSR_W<'a> {
    w: &'a mut W,
}
impl<'a> POSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POSR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Forward to CC4ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(POSR_A::VALUE1)
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(POSR_A::VALUE2)
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(POSR_A::VALUE3)
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(POSR_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Compare match Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMSR_A {
    #[doc = "0: Forward to CC4ySR0"]
    VALUE1,
    #[doc = "1: Forward to CC4ySR1"]
    VALUE2,
    #[doc = "2: Forward to CC4ySR2"]
    VALUE3,
    #[doc = "3: Forward to CC4ySR3"]
    VALUE4,
}
impl From<CMSR_A> for u8 {
    #[inline(always)]
    fn from(variant: CMSR_A) -> Self {
        match variant {
            CMSR_A::VALUE1 => 0,
            CMSR_A::VALUE2 => 1,
            CMSR_A::VALUE3 => 2,
            CMSR_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `CMSR`"]
pub type CMSR_R = crate::R<u8, CMSR_A>;
impl CMSR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMSR_A {
        match self.bits {
            0 => CMSR_A::VALUE1,
            1 => CMSR_A::VALUE2,
            2 => CMSR_A::VALUE3,
            3 => CMSR_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMSR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMSR_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CMSR_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CMSR_A::VALUE4
    }
}
#[doc = "Write proxy for field `CMSR`"]
pub struct CMSR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMSR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Forward to CC4ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMSR_A::VALUE1)
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMSR_A::VALUE2)
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CMSR_A::VALUE3)
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CMSR_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Event 0 Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E0SR_A {
    #[doc = "0: Forward to CC4ySR0"]
    VALUE1,
    #[doc = "1: Forward to CC4ySR1"]
    VALUE2,
    #[doc = "2: Forward to CC4ySR2"]
    VALUE3,
    #[doc = "3: Forward to CC4ySR3"]
    VALUE4,
}
impl From<E0SR_A> for u8 {
    #[inline(always)]
    fn from(variant: E0SR_A) -> Self {
        match variant {
            E0SR_A::VALUE1 => 0,
            E0SR_A::VALUE2 => 1,
            E0SR_A::VALUE3 => 2,
            E0SR_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `E0SR`"]
pub type E0SR_R = crate::R<u8, E0SR_A>;
impl E0SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E0SR_A {
        match self.bits {
            0 => E0SR_A::VALUE1,
            1 => E0SR_A::VALUE2,
            2 => E0SR_A::VALUE3,
            3 => E0SR_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E0SR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E0SR_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == E0SR_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == E0SR_A::VALUE4
    }
}
#[doc = "Write proxy for field `E0SR`"]
pub struct E0SR_W<'a> {
    w: &'a mut W,
}
impl<'a> E0SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E0SR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Forward to CC4ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(E0SR_A::VALUE1)
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(E0SR_A::VALUE2)
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(E0SR_A::VALUE3)
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(E0SR_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Event 1 Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E1SR_A {
    #[doc = "0: Forward to CC4ySR0"]
    VALUE1,
    #[doc = "1: Forward to CC4ySR1"]
    VALUE2,
    #[doc = "2: Forward to CC4ySR2"]
    VALUE3,
    #[doc = "3: Forward to CC4ySR3"]
    VALUE4,
}
impl From<E1SR_A> for u8 {
    #[inline(always)]
    fn from(variant: E1SR_A) -> Self {
        match variant {
            E1SR_A::VALUE1 => 0,
            E1SR_A::VALUE2 => 1,
            E1SR_A::VALUE3 => 2,
            E1SR_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `E1SR`"]
pub type E1SR_R = crate::R<u8, E1SR_A>;
impl E1SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E1SR_A {
        match self.bits {
            0 => E1SR_A::VALUE1,
            1 => E1SR_A::VALUE2,
            2 => E1SR_A::VALUE3,
            3 => E1SR_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E1SR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E1SR_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == E1SR_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == E1SR_A::VALUE4
    }
}
#[doc = "Write proxy for field `E1SR`"]
pub struct E1SR_W<'a> {
    w: &'a mut W,
}
impl<'a> E1SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E1SR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Forward to CC4ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(E1SR_A::VALUE1)
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(E1SR_A::VALUE2)
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(E1SR_A::VALUE3)
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(E1SR_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Event 2 Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E2SR_A {
    #[doc = "0: Forward to CC4ySR0"]
    VALUE1,
    #[doc = "1: Forward to CC4ySR1"]
    VALUE2,
    #[doc = "2: Forward to CC4ySR2"]
    VALUE3,
    #[doc = "3: Forward to CC4ySR3"]
    VALUE4,
}
impl From<E2SR_A> for u8 {
    #[inline(always)]
    fn from(variant: E2SR_A) -> Self {
        match variant {
            E2SR_A::VALUE1 => 0,
            E2SR_A::VALUE2 => 1,
            E2SR_A::VALUE3 => 2,
            E2SR_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `E2SR`"]
pub type E2SR_R = crate::R<u8, E2SR_A>;
impl E2SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E2SR_A {
        match self.bits {
            0 => E2SR_A::VALUE1,
            1 => E2SR_A::VALUE2,
            2 => E2SR_A::VALUE3,
            3 => E2SR_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E2SR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E2SR_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == E2SR_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == E2SR_A::VALUE4
    }
}
#[doc = "Write proxy for field `E2SR`"]
pub struct E2SR_W<'a> {
    w: &'a mut W,
}
impl<'a> E2SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E2SR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Forward to CC4ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(E2SR_A::VALUE1)
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(E2SR_A::VALUE2)
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(E2SR_A::VALUE3)
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(E2SR_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Period/One match Service request selector"]
    #[inline(always)]
    pub fn posr(&self) -> POSR_R {
        POSR_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Compare match Service request selector"]
    #[inline(always)]
    pub fn cmsr(&self) -> CMSR_R {
        CMSR_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Event 0 Service request selector"]
    #[inline(always)]
    pub fn e0sr(&self) -> E0SR_R {
        E0SR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Event 1 Service request selector"]
    #[inline(always)]
    pub fn e1sr(&self) -> E1SR_R {
        E1SR_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Event 2 Service request selector"]
    #[inline(always)]
    pub fn e2sr(&self) -> E2SR_R {
        E2SR_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Period/One match Service request selector"]
    #[inline(always)]
    pub fn posr(&mut self) -> POSR_W {
        POSR_W { w: self }
    }
    #[doc = "Bits 2:3 - Compare match Service request selector"]
    #[inline(always)]
    pub fn cmsr(&mut self) -> CMSR_W {
        CMSR_W { w: self }
    }
    #[doc = "Bits 8:9 - Event 0 Service request selector"]
    #[inline(always)]
    pub fn e0sr(&mut self) -> E0SR_W {
        E0SR_W { w: self }
    }
    #[doc = "Bits 10:11 - Event 1 Service request selector"]
    #[inline(always)]
    pub fn e1sr(&mut self) -> E1SR_W {
        E1SR_W { w: self }
    }
    #[doc = "Bits 12:13 - Event 2 Service request selector"]
    #[inline(always)]
    pub fn e2sr(&mut self) -> E2SR_W {
        E2SR_W { w: self }
    }
}
