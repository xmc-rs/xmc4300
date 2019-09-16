#[doc = "Reader of register NCR"]
pub type R = crate::R<u32, super::NCR>;
#[doc = "Writer for register NCR"]
pub type W = crate::W<u32, super::NCR>;
#[doc = "Register NCR `reset()`'s with value 0x41"]
impl crate::ResetValue for super::NCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x41
    }
}
#[doc = "Node Initialization\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INIT_A {
    #[doc = "0: Resetting bit INIT enables the participation of the node in the CAN traffic. If the CAN node is in the bus-off state, the ongoing bus-off recovery (which does not depend on the INIT bit) is continued. With the end of the bus-off recovery sequence the CAN node is allowed to take part in the CAN traffic. If the CAN node is not in the bus-off state, a sequence of 11 consecutive recessive bits must be detected before the node is allowed to take part in the CAN traffic."]
    VALUE1,
    #[doc = "1: Setting this bit terminates the participation of this node in the CAN traffic. Any ongoing frame transfer is cancelled and the transmit line goes recessive. If the CAN node is in the bus-off state, then the running bus-off recovery sequence is continued. If the INIT bit is still set after the successful completion of the bus-off recovery sequence, i.e. after detecting 128 sequences of 11 consecutive recessive bits (11 1), then the CAN node leaves the bus-off state but remains inactive as long as INIT remains set."]
    VALUE2,
}
impl From<INIT_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_A) -> Self {
        match variant {
            INIT_A::VALUE1 => false,
            INIT_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `INIT`"]
pub type INIT_R = crate::R<bool, INIT_A>;
impl INIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INIT_A {
        match self.bits {
            false => INIT_A::VALUE1,
            true => INIT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == INIT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INIT_A::VALUE2
    }
}
#[doc = "Write proxy for field `INIT`"]
pub struct INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Resetting bit INIT enables the participation of the node in the CAN traffic. If the CAN node is in the bus-off state, the ongoing bus-off recovery (which does not depend on the INIT bit) is continued. With the end of the bus-off recovery sequence the CAN node is allowed to take part in the CAN traffic. If the CAN node is not in the bus-off state, a sequence of 11 consecutive recessive bits must be detected before the node is allowed to take part in the CAN traffic."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(INIT_A::VALUE1)
    }
    #[doc = "Setting this bit terminates the participation of this node in the CAN traffic. Any ongoing frame transfer is cancelled and the transmit line goes recessive. If the CAN node is in the bus-off state, then the running bus-off recovery sequence is continued. If the INIT bit is still set after the successful completion of the bus-off recovery sequence, i.e. after detecting 128 sequences of 11 consecutive recessive bits (11 1), then the CAN node leaves the bus-off state but remains inactive as long as INIT remains set."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(INIT_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Transfer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIE_A {
    #[doc = "0: Transfer interrupt is disabled."]
    VALUE1,
    #[doc = "1: Transfer interrupt is enabled."]
    VALUE2,
}
impl From<TRIE_A> for bool {
    #[inline(always)]
    fn from(variant: TRIE_A) -> Self {
        match variant {
            TRIE_A::VALUE1 => false,
            TRIE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TRIE`"]
pub type TRIE_R = crate::R<bool, TRIE_A>;
impl TRIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIE_A {
        match self.bits {
            false => TRIE_A::VALUE1,
            true => TRIE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRIE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRIE_A::VALUE2
    }
}
#[doc = "Write proxy for field `TRIE`"]
pub struct TRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transfer interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRIE_A::VALUE1)
    }
    #[doc = "Transfer interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRIE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "LEC Indicated Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LECIE_A {
    #[doc = "0: Last error code interrupt is disabled."]
    VALUE1,
    #[doc = "1: Last error code interrupt is enabled."]
    VALUE2,
}
impl From<LECIE_A> for bool {
    #[inline(always)]
    fn from(variant: LECIE_A) -> Self {
        match variant {
            LECIE_A::VALUE1 => false,
            LECIE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `LECIE`"]
pub type LECIE_R = crate::R<bool, LECIE_A>;
impl LECIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LECIE_A {
        match self.bits {
            false => LECIE_A::VALUE1,
            true => LECIE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LECIE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LECIE_A::VALUE2
    }
}
#[doc = "Write proxy for field `LECIE`"]
pub struct LECIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LECIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LECIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Last error code interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LECIE_A::VALUE1)
    }
    #[doc = "Last error code interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LECIE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Alert Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIE_A {
    #[doc = "0: Alert interrupt is disabled."]
    VALUE1,
    #[doc = "1: Alert interrupt is enabled."]
    VALUE2,
}
impl From<ALIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALIE_A) -> Self {
        match variant {
            ALIE_A::VALUE1 => false,
            ALIE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ALIE`"]
pub type ALIE_R = crate::R<bool, ALIE_A>;
impl ALIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIE_A {
        match self.bits {
            false => ALIE_A::VALUE1,
            true => ALIE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALIE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALIE_A::VALUE2
    }
}
#[doc = "Write proxy for field `ALIE`"]
pub struct ALIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Alert interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALIE_A::VALUE1)
    }
    #[doc = "Alert interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALIE_A::VALUE2)
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
#[doc = "Reader of field `CANDIS`"]
pub type CANDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CANDIS`"]
pub struct CANDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CANDIS_W<'a> {
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
#[doc = "Reader of field `TXDIS`"]
pub type TXDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXDIS`"]
pub struct TXDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDIS_W<'a> {
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
#[doc = "Configuration Change Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCE_A {
    #[doc = "0: The Bit Timing Register, the Port Control Register, Error Counter Register may only be read. All attempts to modify them are ignored."]
    VALUE1,
    #[doc = "1: The Bit Timing Register, the Port Control Register, Error Counter Register may be read and written."]
    VALUE2,
}
impl From<CCE_A> for bool {
    #[inline(always)]
    fn from(variant: CCE_A) -> Self {
        match variant {
            CCE_A::VALUE1 => false,
            CCE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CCE`"]
pub type CCE_R = crate::R<bool, CCE_A>;
impl CCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCE_A {
        match self.bits {
            false => CCE_A::VALUE1,
            true => CCE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCE_A::VALUE2
    }
}
#[doc = "Write proxy for field `CCE`"]
pub struct CCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The Bit Timing Register, the Port Control Register, Error Counter Register may only be read. All attempts to modify them are ignored."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCE_A::VALUE1)
    }
    #[doc = "The Bit Timing Register, the Port Control Register, Error Counter Register may be read and written."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CALM`"]
pub type CALM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CALM`"]
pub struct CALM_W<'a> {
    w: &'a mut W,
}
impl<'a> CALM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Node Initialization"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer Interrupt Enable"]
    #[inline(always)]
    pub fn trie(&self) -> TRIE_R {
        TRIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LEC Indicated Error Interrupt Enable"]
    #[inline(always)]
    pub fn lecie(&self) -> LECIE_R {
        LECIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Alert Interrupt Enable"]
    #[inline(always)]
    pub fn alie(&self) -> ALIE_R {
        ALIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CAN Disable"]
    #[inline(always)]
    pub fn candis(&self) -> CANDIS_R {
        CANDIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit Disable"]
    #[inline(always)]
    pub fn txdis(&self) -> TXDIS_R {
        TXDIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Configuration Change Enable"]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CAN Analyzer Mode"]
    #[inline(always)]
    pub fn calm(&self) -> CALM_R {
        CALM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Node Initialization"]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W { w: self }
    }
    #[doc = "Bit 1 - Transfer Interrupt Enable"]
    #[inline(always)]
    pub fn trie(&mut self) -> TRIE_W {
        TRIE_W { w: self }
    }
    #[doc = "Bit 2 - LEC Indicated Error Interrupt Enable"]
    #[inline(always)]
    pub fn lecie(&mut self) -> LECIE_W {
        LECIE_W { w: self }
    }
    #[doc = "Bit 3 - Alert Interrupt Enable"]
    #[inline(always)]
    pub fn alie(&mut self) -> ALIE_W {
        ALIE_W { w: self }
    }
    #[doc = "Bit 4 - CAN Disable"]
    #[inline(always)]
    pub fn candis(&mut self) -> CANDIS_W {
        CANDIS_W { w: self }
    }
    #[doc = "Bit 5 - Transmit Disable"]
    #[inline(always)]
    pub fn txdis(&mut self) -> TXDIS_W {
        TXDIS_W { w: self }
    }
    #[doc = "Bit 6 - Configuration Change Enable"]
    #[inline(always)]
    pub fn cce(&mut self) -> CCE_W {
        CCE_W { w: self }
    }
    #[doc = "Bit 7 - CAN Analyzer Mode"]
    #[inline(always)]
    pub fn calm(&mut self) -> CALM_W {
        CALM_W { w: self }
    }
}
