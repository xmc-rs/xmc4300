#[doc = "Register `GPCHK` reader"]
pub struct R(crate::R<GPCHK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPCHK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPCHK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPCHK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPCHK` writer"]
pub struct W(crate::W<GPCHK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPCHK_SPEC>;
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
impl From<crate::W<GPCHK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPCHK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PASE` reader - Parity Checker Automatic start/stop"]
pub struct PASE_R(crate::FieldReader<bool, bool>);
impl PASE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PASE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PASE` writer - Parity Checker Automatic start/stop"]
pub struct PASE_W<'a> {
    w: &'a mut W,
}
impl<'a> PASE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Parity Checker Automatic start/stop selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PACS_A {
    #[doc = "0: CC80"]
    VALUE1 = 0,
    #[doc = "1: CC81"]
    VALUE2 = 1,
    #[doc = "2: CC82"]
    VALUE3 = 2,
    #[doc = "3: CC83"]
    VALUE4 = 3,
}
impl From<PACS_A> for u8 {
    #[inline(always)]
    fn from(variant: PACS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PACS` reader - Parity Checker Automatic start/stop selector"]
pub struct PACS_R(crate::FieldReader<u8, PACS_A>);
impl PACS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PACS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PACS_A {
        match self.bits {
            0 => PACS_A::VALUE1,
            1 => PACS_A::VALUE2,
            2 => PACS_A::VALUE3,
            3 => PACS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PACS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PACS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PACS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == PACS_A::VALUE4
    }
}
impl core::ops::Deref for PACS_R {
    type Target = crate::FieldReader<u8, PACS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PACS` writer - Parity Checker Automatic start/stop selector"]
pub struct PACS_W<'a> {
    w: &'a mut W,
}
impl<'a> PACS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PACS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "CC80"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PACS_A::VALUE1)
    }
    #[doc = "CC81"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PACS_A::VALUE2)
    }
    #[doc = "CC82"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PACS_A::VALUE3)
    }
    #[doc = "CC83"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PACS_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Driver Input signal selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PISEL_A {
    #[doc = "0: CC8x.GP01 - driver output is connected to event 1 of slice 0"]
    VALUE1 = 0,
    #[doc = "1: CC8x.GP11 - drive output is connected to event 1 of slice 1"]
    VALUE2 = 1,
    #[doc = "2: CC8x.GP21 - driver output is connected to event 1 of slice 2"]
    VALUE3 = 2,
    #[doc = "3: CC8x.GP31 - driver output is connected to event 1 of slice 3"]
    VALUE4 = 3,
}
impl From<PISEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PISEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PISEL` reader - Driver Input signal selector"]
pub struct PISEL_R(crate::FieldReader<u8, PISEL_A>);
impl PISEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PISEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PISEL_A {
        match self.bits {
            0 => PISEL_A::VALUE1,
            1 => PISEL_A::VALUE2,
            2 => PISEL_A::VALUE3,
            3 => PISEL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PISEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PISEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PISEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == PISEL_A::VALUE4
    }
}
impl core::ops::Deref for PISEL_R {
    type Target = crate::FieldReader<u8, PISEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PISEL` writer - Driver Input signal selector"]
pub struct PISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PISEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PISEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "CC8x.GP01 - driver output is connected to event 1 of slice 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PISEL_A::VALUE1)
    }
    #[doc = "CC8x.GP11 - drive output is connected to event 1 of slice 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PISEL_A::VALUE2)
    }
    #[doc = "CC8x.GP21 - driver output is connected to event 1 of slice 2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PISEL_A::VALUE3)
    }
    #[doc = "CC8x.GP31 - driver output is connected to event 1 of slice 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PISEL_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Parity Checker Delay Input Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCDS_A {
    #[doc = "0: CCU8x.IGBTA"]
    VALUE1 = 0,
    #[doc = "1: CCU8x.IGBTB"]
    VALUE2 = 1,
    #[doc = "2: CCU8x.IGBTC"]
    VALUE3 = 2,
    #[doc = "3: CCU8x.IGBTD"]
    VALUE4 = 3,
}
impl From<PCDS_A> for u8 {
    #[inline(always)]
    fn from(variant: PCDS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PCDS` reader - Parity Checker Delay Input Selector"]
pub struct PCDS_R(crate::FieldReader<u8, PCDS_A>);
impl PCDS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCDS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCDS_A {
        match self.bits {
            0 => PCDS_A::VALUE1,
            1 => PCDS_A::VALUE2,
            2 => PCDS_A::VALUE3,
            3 => PCDS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PCDS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PCDS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PCDS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == PCDS_A::VALUE4
    }
}
impl core::ops::Deref for PCDS_R {
    type Target = crate::FieldReader<u8, PCDS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCDS` writer - Parity Checker Delay Input Selector"]
pub struct PCDS_W<'a> {
    w: &'a mut W,
}
impl<'a> PCDS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCDS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "CCU8x.IGBTA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PCDS_A::VALUE1)
    }
    #[doc = "CCU8x.IGBTB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PCDS_A::VALUE2)
    }
    #[doc = "CCU8x.IGBTC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PCDS_A::VALUE3)
    }
    #[doc = "CCU8x.IGBTD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PCDS_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Parity Checker type selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCTS_A {
    #[doc = "0: Even parity enabled"]
    VALUE1 = 0,
    #[doc = "1: Odd parity enabled"]
    VALUE2 = 1,
}
impl From<PCTS_A> for bool {
    #[inline(always)]
    fn from(variant: PCTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCTS` reader - Parity Checker type selector"]
pub struct PCTS_R(crate::FieldReader<bool, PCTS_A>);
impl PCTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCTS_A {
        match self.bits {
            false => PCTS_A::VALUE1,
            true => PCTS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PCTS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PCTS_A::VALUE2
    }
}
impl core::ops::Deref for PCTS_R {
    type Target = crate::FieldReader<bool, PCTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCTS` writer - Parity Checker type selector"]
pub struct PCTS_W<'a> {
    w: &'a mut W,
}
impl<'a> PCTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCTS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Even parity enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PCTS_A::VALUE1)
    }
    #[doc = "Odd parity enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PCTS_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `PCST` reader - Parity Checker XOR status"]
pub struct PCST_R(crate::FieldReader<bool, bool>);
impl PCST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL0` reader - Parity Checker Slice 0 output selection"]
pub struct PCSEL0_R(crate::FieldReader<u8, u8>);
impl PCSEL0_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCSEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSEL0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL0` writer - Parity Checker Slice 0 output selection"]
pub struct PCSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `PCSEL1` reader - Parity Checker Slice 1 output selection"]
pub struct PCSEL1_R(crate::FieldReader<u8, u8>);
impl PCSEL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCSEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSEL1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL1` writer - Parity Checker Slice 1 output selection"]
pub struct PCSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `PCSEL2` reader - Parity Checker Slice 2 output selection"]
pub struct PCSEL2_R(crate::FieldReader<u8, u8>);
impl PCSEL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCSEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSEL2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL2` writer - Parity Checker Slice 2 output selection"]
pub struct PCSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `PCSEL3` reader - Parity Checker Slice 3 output selection"]
pub struct PCSEL3_R(crate::FieldReader<u8, u8>);
impl PCSEL3_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCSEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSEL3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL3` writer - Parity Checker Slice 3 output selection"]
pub struct PCSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Parity Checker Automatic start/stop"]
    #[inline(always)]
    pub fn pase(&self) -> PASE_R {
        PASE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Parity Checker Automatic start/stop selector"]
    #[inline(always)]
    pub fn pacs(&self) -> PACS_R {
        PACS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - Driver Input signal selector"]
    #[inline(always)]
    pub fn pisel(&self) -> PISEL_R {
        PISEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:6 - Parity Checker Delay Input Selector"]
    #[inline(always)]
    pub fn pcds(&self) -> PCDS_R {
        PCDS_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Parity Checker type selector"]
    #[inline(always)]
    pub fn pcts(&self) -> PCTS_R {
        PCTS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Parity Checker XOR status"]
    #[inline(always)]
    pub fn pcst(&self) -> PCST_R {
        PCST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Parity Checker Slice 0 output selection"]
    #[inline(always)]
    pub fn pcsel0(&self) -> PCSEL0_R {
        PCSEL0_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Parity Checker Slice 1 output selection"]
    #[inline(always)]
    pub fn pcsel1(&self) -> PCSEL1_R {
        PCSEL1_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Parity Checker Slice 2 output selection"]
    #[inline(always)]
    pub fn pcsel2(&self) -> PCSEL2_R {
        PCSEL2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Parity Checker Slice 3 output selection"]
    #[inline(always)]
    pub fn pcsel3(&self) -> PCSEL3_R {
        PCSEL3_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Checker Automatic start/stop"]
    #[inline(always)]
    pub fn pase(&mut self) -> PASE_W {
        PASE_W { w: self }
    }
    #[doc = "Bits 1:2 - Parity Checker Automatic start/stop selector"]
    #[inline(always)]
    pub fn pacs(&mut self) -> PACS_W {
        PACS_W { w: self }
    }
    #[doc = "Bits 3:4 - Driver Input signal selector"]
    #[inline(always)]
    pub fn pisel(&mut self) -> PISEL_W {
        PISEL_W { w: self }
    }
    #[doc = "Bits 5:6 - Parity Checker Delay Input Selector"]
    #[inline(always)]
    pub fn pcds(&mut self) -> PCDS_W {
        PCDS_W { w: self }
    }
    #[doc = "Bit 7 - Parity Checker type selector"]
    #[inline(always)]
    pub fn pcts(&mut self) -> PCTS_W {
        PCTS_W { w: self }
    }
    #[doc = "Bits 16:19 - Parity Checker Slice 0 output selection"]
    #[inline(always)]
    pub fn pcsel0(&mut self) -> PCSEL0_W {
        PCSEL0_W { w: self }
    }
    #[doc = "Bits 20:23 - Parity Checker Slice 1 output selection"]
    #[inline(always)]
    pub fn pcsel1(&mut self) -> PCSEL1_W {
        PCSEL1_W { w: self }
    }
    #[doc = "Bits 24:27 - Parity Checker Slice 2 output selection"]
    #[inline(always)]
    pub fn pcsel2(&mut self) -> PCSEL2_W {
        PCSEL2_W { w: self }
    }
    #[doc = "Bits 28:31 - Parity Checker Slice 3 output selection"]
    #[inline(always)]
    pub fn pcsel3(&mut self) -> PCSEL3_W {
        PCSEL3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parity Checker Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpchk](index.html) module"]
pub struct GPCHK_SPEC;
impl crate::RegisterSpec for GPCHK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpchk::R](R) reader structure"]
impl crate::Readable for GPCHK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpchk::W](W) writer structure"]
impl crate::Writable for GPCHK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPCHK to value 0"]
impl crate::Resettable for GPCHK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
