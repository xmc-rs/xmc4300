#[doc = "Reader of register DAC1CFG0"]
pub type R = crate::R<u32, super::DAC1CFG0>;
#[doc = "Writer for register DAC1CFG0"]
pub type W = crate::W<u32, super::DAC1CFG0>;
#[doc = "Register DAC1CFG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DAC1CFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FREQ`"]
pub type FREQ_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FREQ`"]
pub struct FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
#[doc = "Enables and sets the Mode for DAC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: disable/switch-off DAC"]
    VALUE1,
    #[doc = "1: Single Value Mode"]
    VALUE2,
    #[doc = "2: Data Mode"]
    VALUE3,
    #[doc = "3: Patgen Mode"]
    VALUE4,
    #[doc = "4: Noise Mode"]
    VALUE5,
    #[doc = "5: Ramp Mode"]
    VALUE6,
    #[doc = "6: na"]
    VALUE7,
    #[doc = "7: na"]
    VALUE8,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        match variant {
            MODE_A::VALUE1 => 0,
            MODE_A::VALUE2 => 1,
            MODE_A::VALUE3 => 2,
            MODE_A::VALUE4 => 3,
            MODE_A::VALUE5 => 4,
            MODE_A::VALUE6 => 5,
            MODE_A::VALUE7 => 6,
            MODE_A::VALUE8 => 7,
        }
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::VALUE1,
            1 => MODE_A::VALUE2,
            2 => MODE_A::VALUE3,
            3 => MODE_A::VALUE4,
            4 => MODE_A::VALUE5,
            5 => MODE_A::VALUE6,
            6 => MODE_A::VALUE7,
            7 => MODE_A::VALUE8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MODE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == MODE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == MODE_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == MODE_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == MODE_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == MODE_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == MODE_A::VALUE8
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "disable/switch-off DAC"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MODE_A::VALUE1)
    }
    #[doc = "Single Value Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MODE_A::VALUE2)
    }
    #[doc = "Data Mode"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(MODE_A::VALUE3)
    }
    #[doc = "Patgen Mode"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(MODE_A::VALUE4)
    }
    #[doc = "Noise Mode"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(MODE_A::VALUE5)
    }
    #[doc = "Ramp Mode"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(MODE_A::VALUE6)
    }
    #[doc = "na"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(MODE_A::VALUE7)
    }
    #[doc = "na"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(MODE_A::VALUE8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Selects between signed and unsigned DAC1 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIGN_A {
    #[doc = "0: DAC expects unsigned input data"]
    VALUE1,
    #[doc = "1: DAC expects signed input data"]
    VALUE2,
}
impl From<SIGN_A> for bool {
    #[inline(always)]
    fn from(variant: SIGN_A) -> Self {
        match variant {
            SIGN_A::VALUE1 => false,
            SIGN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SIGN`"]
pub type SIGN_R = crate::R<bool, SIGN_A>;
impl SIGN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIGN_A {
        match self.bits {
            false => SIGN_A::VALUE1,
            true => SIGN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SIGN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SIGN_A::VALUE2
    }
}
#[doc = "Write proxy for field `SIGN`"]
pub struct SIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIGN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DAC expects unsigned input data"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SIGN_A::VALUE1)
    }
    #[doc = "DAC expects signed input data"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SIGN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `FIFOIND`"]
pub type FIFOIND_R = crate::R<u8, u8>;
#[doc = "Indicate if the FIFO is empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOEMP_A {
    #[doc = "0: FIFO not empty"]
    VALUE1,
    #[doc = "1: FIFO empty"]
    VALUE2,
}
impl From<FIFOEMP_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOEMP_A) -> Self {
        match variant {
            FIFOEMP_A::VALUE1 => false,
            FIFOEMP_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `FIFOEMP`"]
pub type FIFOEMP_R = crate::R<bool, FIFOEMP_A>;
impl FIFOEMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOEMP_A {
        match self.bits {
            false => FIFOEMP_A::VALUE1,
            true => FIFOEMP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FIFOEMP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FIFOEMP_A::VALUE2
    }
}
#[doc = "Indicate if the FIFO is full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOFUL_A {
    #[doc = "0: FIFO not full"]
    VALUE1,
    #[doc = "1: FIFO full"]
    VALUE2,
}
impl From<FIFOFUL_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOFUL_A) -> Self {
        match variant {
            FIFOFUL_A::VALUE1 => false,
            FIFOFUL_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `FIFOFUL`"]
pub type FIFOFUL_R = crate::R<bool, FIFOFUL_A>;
impl FIFOFUL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOFUL_A {
        match self.bits {
            false => FIFOFUL_A::VALUE1,
            true => FIFOFUL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FIFOFUL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FIFOFUL_A::VALUE2
    }
}
#[doc = "Negates the DAC1 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEGATE_A {
    #[doc = "0: DAC output not negated"]
    VALUE1,
    #[doc = "1: DAC output negated"]
    VALUE2,
}
impl From<NEGATE_A> for bool {
    #[inline(always)]
    fn from(variant: NEGATE_A) -> Self {
        match variant {
            NEGATE_A::VALUE1 => false,
            NEGATE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `NEGATE`"]
pub type NEGATE_R = crate::R<bool, NEGATE_A>;
impl NEGATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEGATE_A {
        match self.bits {
            false => NEGATE_A::VALUE1,
            true => NEGATE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NEGATE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NEGATE_A::VALUE2
    }
}
#[doc = "Write proxy for field `NEGATE`"]
pub struct NEGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> NEGATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NEGATE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DAC output not negated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(NEGATE_A::VALUE1)
    }
    #[doc = "DAC output negated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(NEGATE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Enable sign output of DAC1 pattern generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIGNEN_A {
    #[doc = "0: disable"]
    VALUE1,
    #[doc = "1: enable"]
    VALUE2,
}
impl From<SIGNEN_A> for bool {
    #[inline(always)]
    fn from(variant: SIGNEN_A) -> Self {
        match variant {
            SIGNEN_A::VALUE1 => false,
            SIGNEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SIGNEN`"]
pub type SIGNEN_R = crate::R<bool, SIGNEN_A>;
impl SIGNEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIGNEN_A {
        match self.bits {
            false => SIGNEN_A::VALUE1,
            true => SIGNEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SIGNEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SIGNEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `SIGNEN`"]
pub struct SIGNEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGNEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIGNEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SIGNEN_A::VALUE1)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SIGNEN_A::VALUE2)
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
#[doc = "Enable DAC1 service request interrupt generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SREN_A {
    #[doc = "0: disable"]
    VALUE1,
    #[doc = "1: enable"]
    VALUE2,
}
impl From<SREN_A> for bool {
    #[inline(always)]
    fn from(variant: SREN_A) -> Self {
        match variant {
            SREN_A::VALUE1 => false,
            SREN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SREN`"]
pub type SREN_R = crate::R<bool, SREN_A>;
impl SREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SREN_A {
        match self.bits {
            false => SREN_A::VALUE1,
            true => SREN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SREN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SREN_A::VALUE2
    }
}
#[doc = "Write proxy for field `SREN`"]
pub struct SREN_W<'a> {
    w: &'a mut W,
}
impl<'a> SREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SREN_A::VALUE1)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SREN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "RUN indicates the current DAC1 operation status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUN_A {
    #[doc = "0: DAC1 channel disabled"]
    VALUE1,
    #[doc = "1: DAC1 channel in operation"]
    VALUE2,
}
impl From<RUN_A> for bool {
    #[inline(always)]
    fn from(variant: RUN_A) -> Self {
        match variant {
            RUN_A::VALUE1 => false,
            RUN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RUN`"]
pub type RUN_R = crate::R<bool, RUN_A>;
impl RUN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUN_A {
        match self.bits {
            false => RUN_A::VALUE1,
            true => RUN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RUN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RUN_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:19 - Integer Frequency Divider Value"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 20:22 - Enables and sets the Mode for DAC1"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - Selects between signed and unsigned DAC1 mode"]
    #[inline(always)]
    pub fn sign(&self) -> SIGN_R {
        SIGN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Current write position inside the data FIFO"]
    #[inline(always)]
    pub fn fifoind(&self) -> FIFOIND_R {
        FIFOIND_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Indicate if the FIFO is empty"]
    #[inline(always)]
    pub fn fifoemp(&self) -> FIFOEMP_R {
        FIFOEMP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Indicate if the FIFO is full"]
    #[inline(always)]
    pub fn fifoful(&self) -> FIFOFUL_R {
        FIFOFUL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Negates the DAC1 output"]
    #[inline(always)]
    pub fn negate(&self) -> NEGATE_R {
        NEGATE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enable sign output of DAC1 pattern generator"]
    #[inline(always)]
    pub fn signen(&self) -> SIGNEN_R {
        SIGNEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enable DAC1 service request interrupt generation"]
    #[inline(always)]
    pub fn sren(&self) -> SREN_R {
        SREN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - RUN indicates the current DAC1 operation status"]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - Integer Frequency Divider Value"]
    #[inline(always)]
    pub fn freq(&mut self) -> FREQ_W {
        FREQ_W { w: self }
    }
    #[doc = "Bits 20:22 - Enables and sets the Mode for DAC1"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 23 - Selects between signed and unsigned DAC1 mode"]
    #[inline(always)]
    pub fn sign(&mut self) -> SIGN_W {
        SIGN_W { w: self }
    }
    #[doc = "Bit 28 - Negates the DAC1 output"]
    #[inline(always)]
    pub fn negate(&mut self) -> NEGATE_W {
        NEGATE_W { w: self }
    }
    #[doc = "Bit 29 - Enable sign output of DAC1 pattern generator"]
    #[inline(always)]
    pub fn signen(&mut self) -> SIGNEN_W {
        SIGNEN_W { w: self }
    }
    #[doc = "Bit 30 - Enable DAC1 service request interrupt generation"]
    #[inline(always)]
    pub fn sren(&mut self) -> SREN_W {
        SREN_W { w: self }
    }
}
