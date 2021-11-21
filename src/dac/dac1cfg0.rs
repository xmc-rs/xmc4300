#[doc = "Register `DAC1CFG0` reader"]
pub struct R(crate::R<DAC1CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC1CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC1CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC1CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC1CFG0` writer"]
pub struct W(crate::W<DAC1CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC1CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DAC1CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC1CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FREQ` reader - Integer Frequency Divider Value"]
pub struct FREQ_R(crate::FieldReader<u32, u32>);
impl FREQ_R {
    pub(crate) fn new(bits: u32) -> Self {
        FREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREQ_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQ` writer - Integer Frequency Divider Value"]
pub struct FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
#[doc = "Enables and sets the Mode for DAC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: disable/switch-off DAC"]
    VALUE1 = 0,
    #[doc = "1: Single Value Mode"]
    VALUE2 = 1,
    #[doc = "2: Data Mode"]
    VALUE3 = 2,
    #[doc = "3: Patgen Mode"]
    VALUE4 = 3,
    #[doc = "4: Noise Mode"]
    VALUE5 = 4,
    #[doc = "5: Ramp Mode"]
    VALUE6 = 5,
    #[doc = "6: na"]
    VALUE7 = 6,
    #[doc = "7: na"]
    VALUE8 = 7,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - Enables and sets the Mode for DAC1"]
pub struct MODE_R(crate::FieldReader<u8, MODE_A>);
impl MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
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
        **self == MODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MODE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == MODE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == MODE_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == MODE_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        **self == MODE_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        **self == MODE_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        **self == MODE_A::VALUE8
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Enables and sets the Mode for DAC1"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Selects between signed and unsigned DAC1 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIGN_A {
    #[doc = "0: DAC expects unsigned input data"]
    VALUE1 = 0,
    #[doc = "1: DAC expects signed input data"]
    VALUE2 = 1,
}
impl From<SIGN_A> for bool {
    #[inline(always)]
    fn from(variant: SIGN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIGN` reader - Selects between signed and unsigned DAC1 mode"]
pub struct SIGN_R(crate::FieldReader<bool, SIGN_A>);
impl SIGN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIGN_R(crate::FieldReader::new(bits))
    }
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
        **self == SIGN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SIGN_A::VALUE2
    }
}
impl core::ops::Deref for SIGN_R {
    type Target = crate::FieldReader<bool, SIGN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIGN` writer - Selects between signed and unsigned DAC1 mode"]
pub struct SIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIGN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `FIFOIND` reader - Current write position inside the data FIFO"]
pub struct FIFOIND_R(crate::FieldReader<u8, u8>);
impl FIFOIND_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIFOIND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOIND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Indicate if the FIFO is empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOEMP_A {
    #[doc = "0: FIFO not empty"]
    VALUE1 = 0,
    #[doc = "1: FIFO empty"]
    VALUE2 = 1,
}
impl From<FIFOEMP_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOEMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOEMP` reader - Indicate if the FIFO is empty"]
pub struct FIFOEMP_R(crate::FieldReader<bool, FIFOEMP_A>);
impl FIFOEMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFOEMP_R(crate::FieldReader::new(bits))
    }
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
        **self == FIFOEMP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FIFOEMP_A::VALUE2
    }
}
impl core::ops::Deref for FIFOEMP_R {
    type Target = crate::FieldReader<bool, FIFOEMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Indicate if the FIFO is full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOFUL_A {
    #[doc = "0: FIFO not full"]
    VALUE1 = 0,
    #[doc = "1: FIFO full"]
    VALUE2 = 1,
}
impl From<FIFOFUL_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOFUL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOFUL` reader - Indicate if the FIFO is full"]
pub struct FIFOFUL_R(crate::FieldReader<bool, FIFOFUL_A>);
impl FIFOFUL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFOFUL_R(crate::FieldReader::new(bits))
    }
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
        **self == FIFOFUL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FIFOFUL_A::VALUE2
    }
}
impl core::ops::Deref for FIFOFUL_R {
    type Target = crate::FieldReader<bool, FIFOFUL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Negates the DAC1 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEGATE_A {
    #[doc = "0: DAC output not negated"]
    VALUE1 = 0,
    #[doc = "1: DAC output negated"]
    VALUE2 = 1,
}
impl From<NEGATE_A> for bool {
    #[inline(always)]
    fn from(variant: NEGATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NEGATE` reader - Negates the DAC1 output"]
pub struct NEGATE_R(crate::FieldReader<bool, NEGATE_A>);
impl NEGATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NEGATE_R(crate::FieldReader::new(bits))
    }
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
        **self == NEGATE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == NEGATE_A::VALUE2
    }
}
impl core::ops::Deref for NEGATE_R {
    type Target = crate::FieldReader<bool, NEGATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NEGATE` writer - Negates the DAC1 output"]
pub struct NEGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> NEGATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NEGATE_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Enable sign output of DAC1 pattern generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIGNEN_A {
    #[doc = "0: disable"]
    VALUE1 = 0,
    #[doc = "1: enable"]
    VALUE2 = 1,
}
impl From<SIGNEN_A> for bool {
    #[inline(always)]
    fn from(variant: SIGNEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIGNEN` reader - Enable sign output of DAC1 pattern generator"]
pub struct SIGNEN_R(crate::FieldReader<bool, SIGNEN_A>);
impl SIGNEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIGNEN_R(crate::FieldReader::new(bits))
    }
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
        **self == SIGNEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SIGNEN_A::VALUE2
    }
}
impl core::ops::Deref for SIGNEN_R {
    type Target = crate::FieldReader<bool, SIGNEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIGNEN` writer - Enable sign output of DAC1 pattern generator"]
pub struct SIGNEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGNEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIGNEN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Enable DAC1 service request interrupt generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SREN_A {
    #[doc = "0: disable"]
    VALUE1 = 0,
    #[doc = "1: enable"]
    VALUE2 = 1,
}
impl From<SREN_A> for bool {
    #[inline(always)]
    fn from(variant: SREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SREN` reader - Enable DAC1 service request interrupt generation"]
pub struct SREN_R(crate::FieldReader<bool, SREN_A>);
impl SREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SREN_R(crate::FieldReader::new(bits))
    }
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
        **self == SREN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SREN_A::VALUE2
    }
}
impl core::ops::Deref for SREN_R {
    type Target = crate::FieldReader<bool, SREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SREN` writer - Enable DAC1 service request interrupt generation"]
pub struct SREN_W<'a> {
    w: &'a mut W,
}
impl<'a> SREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SREN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "RUN indicates the current DAC1 operation status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUN_A {
    #[doc = "0: DAC1 channel disabled"]
    VALUE1 = 0,
    #[doc = "1: DAC1 channel in operation"]
    VALUE2 = 1,
}
impl From<RUN_A> for bool {
    #[inline(always)]
    fn from(variant: RUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUN` reader - RUN indicates the current DAC1 operation status"]
pub struct RUN_R(crate::FieldReader<bool, RUN_A>);
impl RUN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RUN_R(crate::FieldReader::new(bits))
    }
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
        **self == RUN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RUN_A::VALUE2
    }
}
impl core::ops::Deref for RUN_R {
    type Target = crate::FieldReader<bool, RUN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC1 Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac1cfg0](index.html) module"]
pub struct DAC1CFG0_SPEC;
impl crate::RegisterSpec for DAC1CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac1cfg0::R](R) reader structure"]
impl crate::Readable for DAC1CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac1cfg0::W](W) writer structure"]
impl crate::Writable for DAC1CFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC1CFG0 to value 0"]
impl crate::Resettable for DAC1CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
