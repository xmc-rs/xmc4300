#[doc = "Reader of register TRBSR"]
pub type R = crate::R<u32, super::TRBSR>;
#[doc = "Writer for register TRBSR"]
pub type W = crate::W<u32, super::TRBSR>;
#[doc = "Register TRBSR `reset()`'s with value 0x0808"]
impl crate::ResetValue for super::TRBSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0808
    }
}
#[doc = "Standard Receive Buffer Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRBI_A {
    #[doc = "0: A standard receive buffer event has not been detected."]
    VALUE1,
    #[doc = "1: A standard receive buffer event has been detected."]
    VALUE2,
}
impl From<SRBI_A> for bool {
    #[inline(always)]
    fn from(variant: SRBI_A) -> Self {
        match variant {
            SRBI_A::VALUE1 => false,
            SRBI_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SRBI`"]
pub type SRBI_R = crate::R<bool, SRBI_A>;
impl SRBI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRBI_A {
        match self.bits {
            false => SRBI_A::VALUE1,
            true => SRBI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRBI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRBI_A::VALUE2
    }
}
#[doc = "Write proxy for field `SRBI`"]
pub struct SRBI_W<'a> {
    w: &'a mut W,
}
impl<'a> SRBI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRBI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A standard receive buffer event has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRBI_A::VALUE1)
    }
    #[doc = "A standard receive buffer event has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRBI_A::VALUE2)
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
#[doc = "Receive Buffer Error Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBERI_A {
    #[doc = "0: A receive buffer error event has not been detected."]
    VALUE1,
    #[doc = "1: A receive buffer error event has been detected."]
    VALUE2,
}
impl From<RBERI_A> for bool {
    #[inline(always)]
    fn from(variant: RBERI_A) -> Self {
        match variant {
            RBERI_A::VALUE1 => false,
            RBERI_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RBERI`"]
pub type RBERI_R = crate::R<bool, RBERI_A>;
impl RBERI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBERI_A {
        match self.bits {
            false => RBERI_A::VALUE1,
            true => RBERI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RBERI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RBERI_A::VALUE2
    }
}
#[doc = "Write proxy for field `RBERI`"]
pub struct RBERI_W<'a> {
    w: &'a mut W,
}
impl<'a> RBERI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RBERI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A receive buffer error event has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RBERI_A::VALUE1)
    }
    #[doc = "A receive buffer error event has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RBERI_A::VALUE2)
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
#[doc = "Alternative Receive Buffer Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBI_A {
    #[doc = "0: An alternative receive buffer event has not been detected."]
    VALUE1,
    #[doc = "1: An alternative receive buffer event has been detected."]
    VALUE2,
}
impl From<ARBI_A> for bool {
    #[inline(always)]
    fn from(variant: ARBI_A) -> Self {
        match variant {
            ARBI_A::VALUE1 => false,
            ARBI_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ARBI`"]
pub type ARBI_R = crate::R<bool, ARBI_A>;
impl ARBI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBI_A {
        match self.bits {
            false => ARBI_A::VALUE1,
            true => ARBI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ARBI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ARBI_A::VALUE2
    }
}
#[doc = "Write proxy for field `ARBI`"]
pub struct ARBI_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARBI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An alternative receive buffer event has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ARBI_A::VALUE1)
    }
    #[doc = "An alternative receive buffer event has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ARBI_A::VALUE2)
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
#[doc = "Receive Buffer Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REMPTY_A {
    #[doc = "0: The receive buffer is not empty."]
    VALUE1,
    #[doc = "1: The receive buffer is empty."]
    VALUE2,
}
impl From<REMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: REMPTY_A) -> Self {
        match variant {
            REMPTY_A::VALUE1 => false,
            REMPTY_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `REMPTY`"]
pub type REMPTY_R = crate::R<bool, REMPTY_A>;
impl REMPTY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REMPTY_A {
        match self.bits {
            false => REMPTY_A::VALUE1,
            true => REMPTY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REMPTY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REMPTY_A::VALUE2
    }
}
#[doc = "Receive Buffer Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFULL_A {
    #[doc = "0: The receive buffer is not full."]
    VALUE1,
    #[doc = "1: The receive buffer is full."]
    VALUE2,
}
impl From<RFULL_A> for bool {
    #[inline(always)]
    fn from(variant: RFULL_A) -> Self {
        match variant {
            RFULL_A::VALUE1 => false,
            RFULL_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RFULL`"]
pub type RFULL_R = crate::R<bool, RFULL_A>;
impl RFULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFULL_A {
        match self.bits {
            false => RFULL_A::VALUE1,
            true => RFULL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RFULL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RFULL_A::VALUE2
    }
}
#[doc = "Receive Buffer Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBUS_A {
    #[doc = "0: The receive buffer information has been completely updated."]
    VALUE1,
    #[doc = "1: The OUTR update from the FIFO memory is ongoing. A read from OUTR will be delayed. FIFO pointers from the previous read are not yet updated."]
    VALUE2,
}
impl From<RBUS_A> for bool {
    #[inline(always)]
    fn from(variant: RBUS_A) -> Self {
        match variant {
            RBUS_A::VALUE1 => false,
            RBUS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RBUS`"]
pub type RBUS_R = crate::R<bool, RBUS_A>;
impl RBUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBUS_A {
        match self.bits {
            false => RBUS_A::VALUE1,
            true => RBUS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RBUS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RBUS_A::VALUE2
    }
}
#[doc = "Standard Receive Buffer Event Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRBT_A {
    #[doc = "0: A standard receive buffer event is not triggered using this bit."]
    VALUE1,
    #[doc = "1: A standard receive buffer event is triggered using this bit."]
    VALUE2,
}
impl From<SRBT_A> for bool {
    #[inline(always)]
    fn from(variant: SRBT_A) -> Self {
        match variant {
            SRBT_A::VALUE1 => false,
            SRBT_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SRBT`"]
pub type SRBT_R = crate::R<bool, SRBT_A>;
impl SRBT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRBT_A {
        match self.bits {
            false => SRBT_A::VALUE1,
            true => SRBT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRBT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRBT_A::VALUE2
    }
}
#[doc = "Standard Transmit Buffer Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STBI_A {
    #[doc = "0: A standard transmit buffer event has not been detected."]
    VALUE1,
    #[doc = "1: A standard transmit buffer event has been detected."]
    VALUE2,
}
impl From<STBI_A> for bool {
    #[inline(always)]
    fn from(variant: STBI_A) -> Self {
        match variant {
            STBI_A::VALUE1 => false,
            STBI_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `STBI`"]
pub type STBI_R = crate::R<bool, STBI_A>;
impl STBI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STBI_A {
        match self.bits {
            false => STBI_A::VALUE1,
            true => STBI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STBI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STBI_A::VALUE2
    }
}
#[doc = "Write proxy for field `STBI`"]
pub struct STBI_W<'a> {
    w: &'a mut W,
}
impl<'a> STBI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STBI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A standard transmit buffer event has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STBI_A::VALUE1)
    }
    #[doc = "A standard transmit buffer event has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STBI_A::VALUE2)
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
#[doc = "Transmit Buffer Error Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBERI_A {
    #[doc = "0: A transmit buffer error event has not been detected."]
    VALUE1,
    #[doc = "1: A transmit buffer error event has been detected."]
    VALUE2,
}
impl From<TBERI_A> for bool {
    #[inline(always)]
    fn from(variant: TBERI_A) -> Self {
        match variant {
            TBERI_A::VALUE1 => false,
            TBERI_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TBERI`"]
pub type TBERI_R = crate::R<bool, TBERI_A>;
impl TBERI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBERI_A {
        match self.bits {
            false => TBERI_A::VALUE1,
            true => TBERI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TBERI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TBERI_A::VALUE2
    }
}
#[doc = "Write proxy for field `TBERI`"]
pub struct TBERI_W<'a> {
    w: &'a mut W,
}
impl<'a> TBERI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBERI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A transmit buffer error event has not been detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TBERI_A::VALUE1)
    }
    #[doc = "A transmit buffer error event has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TBERI_A::VALUE2)
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
#[doc = "Transmit Buffer Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMPTY_A {
    #[doc = "0: The transmit buffer is not empty."]
    VALUE1,
    #[doc = "1: The transmit buffer is empty."]
    VALUE2,
}
impl From<TEMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: TEMPTY_A) -> Self {
        match variant {
            TEMPTY_A::VALUE1 => false,
            TEMPTY_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TEMPTY`"]
pub type TEMPTY_R = crate::R<bool, TEMPTY_A>;
impl TEMPTY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEMPTY_A {
        match self.bits {
            false => TEMPTY_A::VALUE1,
            true => TEMPTY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TEMPTY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TEMPTY_A::VALUE2
    }
}
#[doc = "Transmit Buffer Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFULL_A {
    #[doc = "0: The transmit buffer is not full."]
    VALUE1,
    #[doc = "1: The transmit buffer is full."]
    VALUE2,
}
impl From<TFULL_A> for bool {
    #[inline(always)]
    fn from(variant: TFULL_A) -> Self {
        match variant {
            TFULL_A::VALUE1 => false,
            TFULL_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TFULL`"]
pub type TFULL_R = crate::R<bool, TFULL_A>;
impl TFULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFULL_A {
        match self.bits {
            false => TFULL_A::VALUE1,
            true => TFULL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TFULL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TFULL_A::VALUE2
    }
}
#[doc = "Transmit Buffer Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBUS_A {
    #[doc = "0: The transmit buffer information has been completely updated."]
    VALUE1,
    #[doc = "1: The FIFO memory update after write to INx is ongoing. A write to INx will be delayed. FIFO pointers from the previous INx write are not yet updated."]
    VALUE2,
}
impl From<TBUS_A> for bool {
    #[inline(always)]
    fn from(variant: TBUS_A) -> Self {
        match variant {
            TBUS_A::VALUE1 => false,
            TBUS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TBUS`"]
pub type TBUS_R = crate::R<bool, TBUS_A>;
impl TBUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBUS_A {
        match self.bits {
            false => TBUS_A::VALUE1,
            true => TBUS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TBUS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TBUS_A::VALUE2
    }
}
#[doc = "Standard Transmit Buffer Event Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STBT_A {
    #[doc = "0: A standard transmit buffer event is not triggered using this bit."]
    VALUE1,
    #[doc = "1: A standard transmit buffer event is triggered using this bit."]
    VALUE2,
}
impl From<STBT_A> for bool {
    #[inline(always)]
    fn from(variant: STBT_A) -> Self {
        match variant {
            STBT_A::VALUE1 => false,
            STBT_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `STBT`"]
pub type STBT_R = crate::R<bool, STBT_A>;
impl STBT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STBT_A {
        match self.bits {
            false => STBT_A::VALUE1,
            true => STBT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STBT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STBT_A::VALUE2
    }
}
#[doc = "Reader of field `RBFLVL`"]
pub type RBFLVL_R = crate::R<u8, u8>;
#[doc = "Reader of field `TBFLVL`"]
pub type TBFLVL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Standard Receive Buffer Event"]
    #[inline(always)]
    pub fn srbi(&self) -> SRBI_R {
        SRBI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Buffer Error Event"]
    #[inline(always)]
    pub fn rberi(&self) -> RBERI_R {
        RBERI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Alternative Receive Buffer Event"]
    #[inline(always)]
    pub fn arbi(&self) -> ARBI_R {
        ARBI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive Buffer Empty"]
    #[inline(always)]
    pub fn rempty(&self) -> REMPTY_R {
        REMPTY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive Buffer Full"]
    #[inline(always)]
    pub fn rfull(&self) -> RFULL_R {
        RFULL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive Buffer Busy"]
    #[inline(always)]
    pub fn rbus(&self) -> RBUS_R {
        RBUS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Standard Receive Buffer Event Trigger"]
    #[inline(always)]
    pub fn srbt(&self) -> SRBT_R {
        SRBT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Standard Transmit Buffer Event"]
    #[inline(always)]
    pub fn stbi(&self) -> STBI_R {
        STBI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmit Buffer Error Event"]
    #[inline(always)]
    pub fn tberi(&self) -> TBERI_R {
        TBERI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmit Buffer Empty"]
    #[inline(always)]
    pub fn tempty(&self) -> TEMPTY_R {
        TEMPTY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Transmit Buffer Full"]
    #[inline(always)]
    pub fn tfull(&self) -> TFULL_R {
        TFULL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmit Buffer Busy"]
    #[inline(always)]
    pub fn tbus(&self) -> TBUS_R {
        TBUS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Standard Transmit Buffer Event Trigger"]
    #[inline(always)]
    pub fn stbt(&self) -> STBT_R {
        STBT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - Receive Buffer Filling Level"]
    #[inline(always)]
    pub fn rbflvl(&self) -> RBFLVL_R {
        RBFLVL_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Transmit Buffer Filling Level"]
    #[inline(always)]
    pub fn tbflvl(&self) -> TBFLVL_R {
        TBFLVL_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Standard Receive Buffer Event"]
    #[inline(always)]
    pub fn srbi(&mut self) -> SRBI_W {
        SRBI_W { w: self }
    }
    #[doc = "Bit 1 - Receive Buffer Error Event"]
    #[inline(always)]
    pub fn rberi(&mut self) -> RBERI_W {
        RBERI_W { w: self }
    }
    #[doc = "Bit 2 - Alternative Receive Buffer Event"]
    #[inline(always)]
    pub fn arbi(&mut self) -> ARBI_W {
        ARBI_W { w: self }
    }
    #[doc = "Bit 8 - Standard Transmit Buffer Event"]
    #[inline(always)]
    pub fn stbi(&mut self) -> STBI_W {
        STBI_W { w: self }
    }
    #[doc = "Bit 9 - Transmit Buffer Error Event"]
    #[inline(always)]
    pub fn tberi(&mut self) -> TBERI_W {
        TBERI_W { w: self }
    }
}
