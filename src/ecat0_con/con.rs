#[doc = "Reader of register CON"]
pub type R = crate::R<u32, super::CON>;
#[doc = "Writer for register CON"]
pub type W = crate::W<u32, super::CON>;
#[doc = "Register CON `reset()`'s with value 0"]
impl crate::ResetValue for super::CON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable EtherCAT Reset Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECATRSTEN_A {
    #[doc = "0: Reset request by EtherCAT Master disabled"]
    VALUE1 = 0,
    #[doc = "1: Reset request by EtherCAT Master enabled"]
    VALUE2 = 1,
}
impl From<ECATRSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ECATRSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ECATRSTEN`"]
pub type ECATRSTEN_R = crate::R<bool, ECATRSTEN_A>;
impl ECATRSTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECATRSTEN_A {
        match self.bits {
            false => ECATRSTEN_A::VALUE1,
            true => ECATRSTEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ECATRSTEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ECATRSTEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `ECATRSTEN`"]
pub struct ECATRSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ECATRSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECATRSTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset request by EtherCAT Master disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECATRSTEN_A::VALUE1)
    }
    #[doc = "Reset request by EtherCAT Master enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECATRSTEN_A::VALUE2)
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
#[doc = "LATCHIN0 Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LATCHIN0SEL_A {
    #[doc = "0: Data input LATCHIN0A is selected"]
    VALUE1 = 0,
    #[doc = "1: Data input LATCHIN0B is selected"]
    VALUE2 = 1,
    #[doc = "2: Data input LATCHIN0C is selected"]
    VALUE3 = 2,
    #[doc = "3: Data input LATCHIN0D is selected"]
    VALUE4 = 3,
}
impl From<LATCHIN0SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LATCHIN0SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LATCHIN0SEL`"]
pub type LATCHIN0SEL_R = crate::R<u8, LATCHIN0SEL_A>;
impl LATCHIN0SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LATCHIN0SEL_A {
        match self.bits {
            0 => LATCHIN0SEL_A::VALUE1,
            1 => LATCHIN0SEL_A::VALUE2,
            2 => LATCHIN0SEL_A::VALUE3,
            3 => LATCHIN0SEL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LATCHIN0SEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LATCHIN0SEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LATCHIN0SEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == LATCHIN0SEL_A::VALUE4
    }
}
#[doc = "Write proxy for field `LATCHIN0SEL`"]
pub struct LATCHIN0SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LATCHIN0SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LATCHIN0SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Data input LATCHIN0A is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LATCHIN0SEL_A::VALUE1)
    }
    #[doc = "Data input LATCHIN0B is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LATCHIN0SEL_A::VALUE2)
    }
    #[doc = "Data input LATCHIN0C is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(LATCHIN0SEL_A::VALUE3)
    }
    #[doc = "Data input LATCHIN0D is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(LATCHIN0SEL_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `LATCHIN0`"]
pub type LATCHIN0_R = crate::R<bool, bool>;
#[doc = "LATCHIN1 Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LATCHIN1SEL_A {
    #[doc = "0: Data input LATCHIN1A is selected"]
    VALUE1 = 0,
    #[doc = "1: Data input LATCHIN1B is selected"]
    VALUE2 = 1,
    #[doc = "2: Data input LATCHIN1C is selected"]
    VALUE3 = 2,
    #[doc = "3: Data input LATCHIN1D is selected"]
    VALUE4 = 3,
}
impl From<LATCHIN1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LATCHIN1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LATCHIN1SEL`"]
pub type LATCHIN1SEL_R = crate::R<u8, LATCHIN1SEL_A>;
impl LATCHIN1SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LATCHIN1SEL_A {
        match self.bits {
            0 => LATCHIN1SEL_A::VALUE1,
            1 => LATCHIN1SEL_A::VALUE2,
            2 => LATCHIN1SEL_A::VALUE3,
            3 => LATCHIN1SEL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LATCHIN1SEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LATCHIN1SEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LATCHIN1SEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == LATCHIN1SEL_A::VALUE4
    }
}
#[doc = "Write proxy for field `LATCHIN1SEL`"]
pub struct LATCHIN1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LATCHIN1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LATCHIN1SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Data input LATCHIN1A is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LATCHIN1SEL_A::VALUE1)
    }
    #[doc = "Data input LATCHIN1B is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LATCHIN1SEL_A::VALUE2)
    }
    #[doc = "Data input LATCHIN1C is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(LATCHIN1SEL_A::VALUE3)
    }
    #[doc = "Data input LATCHIN1D is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(LATCHIN1SEL_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `LATCHIN1`"]
pub type LATCHIN1_R = crate::R<bool, bool>;
#[doc = "Reader of field `PHYOFFSET`"]
pub type PHYOFFSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PHYOFFSET`"]
pub struct PHYOFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYOFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "MDIO Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MDIO_A {
    #[doc = "0: Data input MDIA is selected"]
    VALUE1 = 0,
    #[doc = "1: Data input MDIB is selected"]
    VALUE2 = 1,
    #[doc = "2: Data input MDIC is selected"]
    VALUE3 = 2,
    #[doc = "3: Data input MDID is selected"]
    VALUE4 = 3,
}
impl From<MDIO_A> for u8 {
    #[inline(always)]
    fn from(variant: MDIO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MDIO`"]
pub type MDIO_R = crate::R<u8, MDIO_A>;
impl MDIO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIO_A {
        match self.bits {
            0 => MDIO_A::VALUE1,
            1 => MDIO_A::VALUE2,
            2 => MDIO_A::VALUE3,
            3 => MDIO_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MDIO_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MDIO_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == MDIO_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == MDIO_A::VALUE4
    }
}
#[doc = "Write proxy for field `MDIO`"]
pub struct MDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDIO_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Data input MDIA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MDIO_A::VALUE1)
    }
    #[doc = "Data input MDIB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MDIO_A::VALUE2)
    }
    #[doc = "Data input MDIC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(MDIO_A::VALUE3)
    }
    #[doc = "Data input MDID is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(MDIO_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable EtherCAT Reset Request"]
    #[inline(always)]
    pub fn ecatrsten(&self) -> ECATRSTEN_R {
        ECATRSTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - LATCHIN0 Input Select"]
    #[inline(always)]
    pub fn latchin0sel(&self) -> LATCHIN0SEL_R {
        LATCHIN0SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 11 - EtherCAT LATCH_IN0 Input Signal"]
    #[inline(always)]
    pub fn latchin0(&self) -> LATCHIN0_R {
        LATCHIN0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - LATCHIN1 Input Select"]
    #[inline(always)]
    pub fn latchin1sel(&self) -> LATCHIN1SEL_R {
        LATCHIN1SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 15 - EtherCAT LATCH_IN1 Input Signal"]
    #[inline(always)]
    pub fn latchin1(&self) -> LATCHIN1_R {
        LATCHIN1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Ethernet PHY Address Offset"]
    #[inline(always)]
    pub fn phyoffset(&self) -> PHYOFFSET_R {
        PHYOFFSET_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 22:23 - MDIO Input Select"]
    #[inline(always)]
    pub fn mdio(&self) -> MDIO_R {
        MDIO_R::new(((self.bits >> 22) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable EtherCAT Reset Request"]
    #[inline(always)]
    pub fn ecatrsten(&mut self) -> ECATRSTEN_W {
        ECATRSTEN_W { w: self }
    }
    #[doc = "Bits 8:9 - LATCHIN0 Input Select"]
    #[inline(always)]
    pub fn latchin0sel(&mut self) -> LATCHIN0SEL_W {
        LATCHIN0SEL_W { w: self }
    }
    #[doc = "Bits 12:13 - LATCHIN1 Input Select"]
    #[inline(always)]
    pub fn latchin1sel(&mut self) -> LATCHIN1SEL_W {
        LATCHIN1SEL_W { w: self }
    }
    #[doc = "Bits 16:20 - Ethernet PHY Address Offset"]
    #[inline(always)]
    pub fn phyoffset(&mut self) -> PHYOFFSET_W {
        PHYOFFSET_W { w: self }
    }
    #[doc = "Bits 22:23 - MDIO Input Select"]
    #[inline(always)]
    pub fn mdio(&mut self) -> MDIO_W {
        MDIO_W { w: self }
    }
}
