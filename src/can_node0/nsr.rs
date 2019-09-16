#[doc = "Reader of register NSR"]
pub type R = crate::R<u32, super::NSR>;
#[doc = "Writer for register NSR"]
pub type W = crate::W<u32, super::NSR>;
#[doc = "Register NSR `reset()`'s with value 0"]
impl crate::ResetValue for super::NSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEC`"]
pub type LEC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LEC`"]
pub struct LEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Message Transmitted Successfully\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXOK_A {
    #[doc = "0: No successful transmission since last (most recent) flag reset."]
    VALUE1,
    #[doc = "1: A message has been transmitted successfully (error-free and acknowledged by at least another node)."]
    VALUE2,
}
impl From<TXOK_A> for bool {
    #[inline(always)]
    fn from(variant: TXOK_A) -> Self {
        match variant {
            TXOK_A::VALUE1 => false,
            TXOK_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TXOK`"]
pub type TXOK_R = crate::R<bool, TXOK_A>;
impl TXOK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXOK_A {
        match self.bits {
            false => TXOK_A::VALUE1,
            true => TXOK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TXOK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TXOK_A::VALUE2
    }
}
#[doc = "Write proxy for field `TXOK`"]
pub struct TXOK_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXOK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No successful transmission since last (most recent) flag reset."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXOK_A::VALUE1)
    }
    #[doc = "A message has been transmitted successfully (error-free and acknowledged by at least another node)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TXOK_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Message Received Successfully\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOK_A {
    #[doc = "0: No successful reception since last (most recent) flag reset."]
    VALUE1,
    #[doc = "1: A message has been received successfully."]
    VALUE2,
}
impl From<RXOK_A> for bool {
    #[inline(always)]
    fn from(variant: RXOK_A) -> Self {
        match variant {
            RXOK_A::VALUE1 => false,
            RXOK_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RXOK`"]
pub type RXOK_R = crate::R<bool, RXOK_A>;
impl RXOK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOK_A {
        match self.bits {
            false => RXOK_A::VALUE1,
            true => RXOK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXOK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RXOK_A::VALUE2
    }
}
#[doc = "Write proxy for field `RXOK`"]
pub struct RXOK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXOK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No successful reception since last (most recent) flag reset."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXOK_A::VALUE1)
    }
    #[doc = "A message has been received successfully."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXOK_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `ALERT`"]
pub type ALERT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALERT`"]
pub struct ALERT_W<'a> {
    w: &'a mut W,
}
impl<'a> ALERT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Error Warning Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWRN_A {
    #[doc = "0: No warning limit exceeded."]
    VALUE1,
    #[doc = "1: One of the error counters REC or TEC reached the warning limit EWRNLVL."]
    VALUE2,
}
impl From<EWRN_A> for bool {
    #[inline(always)]
    fn from(variant: EWRN_A) -> Self {
        match variant {
            EWRN_A::VALUE1 => false,
            EWRN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `EWRN`"]
pub type EWRN_R = crate::R<bool, EWRN_A>;
impl EWRN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWRN_A {
        match self.bits {
            false => EWRN_A::VALUE1,
            true => EWRN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EWRN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EWRN_A::VALUE2
    }
}
#[doc = "Bus-off Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFF_A {
    #[doc = "0: CAN controller is not in the bus-off state."]
    VALUE1,
    #[doc = "1: CAN controller is in the bus-off state."]
    VALUE2,
}
impl From<BOFF_A> for bool {
    #[inline(always)]
    fn from(variant: BOFF_A) -> Self {
        match variant {
            BOFF_A::VALUE1 => false,
            BOFF_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `BOFF`"]
pub type BOFF_R = crate::R<bool, BOFF_A>;
impl BOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOFF_A {
        match self.bits {
            false => BOFF_A::VALUE1,
            true => BOFF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BOFF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BOFF_A::VALUE2
    }
}
#[doc = "List Length Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLE_A {
    #[doc = "0: No List Length Error since last (most recent) flag reset."]
    VALUE1,
    #[doc = "1: A List Length Error has been detected during message acceptance filtering. The number of elements in the list that belongs to this CAN node differs from the list SIZE given in the list termination pointer."]
    VALUE2,
}
impl From<LLE_A> for bool {
    #[inline(always)]
    fn from(variant: LLE_A) -> Self {
        match variant {
            LLE_A::VALUE1 => false,
            LLE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `LLE`"]
pub type LLE_R = crate::R<bool, LLE_A>;
impl LLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LLE_A {
        match self.bits {
            false => LLE_A::VALUE1,
            true => LLE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LLE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LLE_A::VALUE2
    }
}
#[doc = "Write proxy for field `LLE`"]
pub struct LLE_W<'a> {
    w: &'a mut W,
}
impl<'a> LLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No List Length Error since last (most recent) flag reset."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LLE_A::VALUE1)
    }
    #[doc = "A List Length Error has been detected during message acceptance filtering. The number of elements in the list that belongs to this CAN node differs from the list SIZE given in the list termination pointer."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LLE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "List Object Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOE_A {
    #[doc = "0: No List Object Error since last (most recent) flag reset."]
    VALUE1,
    #[doc = "1: A List Object Error has been detected during message acceptance filtering. A message object with wrong LIST index entry in the Message Object Status Register has been detected."]
    VALUE2,
}
impl From<LOE_A> for bool {
    #[inline(always)]
    fn from(variant: LOE_A) -> Self {
        match variant {
            LOE_A::VALUE1 => false,
            LOE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `LOE`"]
pub type LOE_R = crate::R<bool, LOE_A>;
impl LOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOE_A {
        match self.bits {
            false => LOE_A::VALUE1,
            true => LOE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LOE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LOE_A::VALUE2
    }
}
#[doc = "Write proxy for field `LOE`"]
pub struct LOE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No List Object Error since last (most recent) flag reset."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LOE_A::VALUE1)
    }
    #[doc = "A List Object Error has been detected during message acceptance filtering. A message object with wrong LIST index entry in the Message Object Status Register has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LOE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Message Transmitted Successfully"]
    #[inline(always)]
    pub fn txok(&self) -> TXOK_R {
        TXOK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Message Received Successfully"]
    #[inline(always)]
    pub fn rxok(&self) -> RXOK_R {
        RXOK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Alert Warning"]
    #[inline(always)]
    pub fn alert(&self) -> ALERT_R {
        ALERT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Error Warning Status"]
    #[inline(always)]
    pub fn ewrn(&self) -> EWRN_R {
        EWRN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bus-off Status"]
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - List Length Error"]
    #[inline(always)]
    pub fn lle(&self) -> LLE_R {
        LLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - List Object Error"]
    #[inline(always)]
    pub fn loe(&self) -> LOE_R {
        LOE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    pub fn lec(&mut self) -> LEC_W {
        LEC_W { w: self }
    }
    #[doc = "Bit 3 - Message Transmitted Successfully"]
    #[inline(always)]
    pub fn txok(&mut self) -> TXOK_W {
        TXOK_W { w: self }
    }
    #[doc = "Bit 4 - Message Received Successfully"]
    #[inline(always)]
    pub fn rxok(&mut self) -> RXOK_W {
        RXOK_W { w: self }
    }
    #[doc = "Bit 5 - Alert Warning"]
    #[inline(always)]
    pub fn alert(&mut self) -> ALERT_W {
        ALERT_W { w: self }
    }
    #[doc = "Bit 8 - List Length Error"]
    #[inline(always)]
    pub fn lle(&mut self) -> LLE_W {
        LLE_W { w: self }
    }
    #[doc = "Bit 9 - List Object Error"]
    #[inline(always)]
    pub fn loe(&mut self) -> LOE_W {
        LOE_W { w: self }
    }
}
