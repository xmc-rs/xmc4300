#[doc = "Register `EXISEL` reader"]
pub struct R(crate::R<EXISEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXISEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXISEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXISEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXISEL` writer"]
pub struct W(crate::W<EXISEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXISEL_SPEC>;
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
impl From<crate::W<EXISEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXISEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Event Source Select for A0 (ERS0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXS0A_A {
    #[doc = "0: Input ERU_0A0 is selected"]
    VALUE1 = 0,
    #[doc = "1: Input ERU_0A1 is selected"]
    VALUE2 = 1,
    #[doc = "2: Input ERU_0A2 is selected"]
    VALUE3 = 2,
    #[doc = "3: Input ERU_0A3 is selected"]
    VALUE4 = 3,
}
impl From<EXS0A_A> for u8 {
    #[inline(always)]
    fn from(variant: EXS0A_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXS0A` reader - Event Source Select for A0 (ERS0)"]
pub struct EXS0A_R(crate::FieldReader<u8, EXS0A_A>);
impl EXS0A_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXS0A_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXS0A_A {
        match self.bits {
            0 => EXS0A_A::VALUE1,
            1 => EXS0A_A::VALUE2,
            2 => EXS0A_A::VALUE3,
            3 => EXS0A_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EXS0A_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EXS0A_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == EXS0A_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == EXS0A_A::VALUE4
    }
}
impl core::ops::Deref for EXS0A_R {
    type Target = crate::FieldReader<u8, EXS0A_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXS0A` writer - Event Source Select for A0 (ERS0)"]
pub struct EXS0A_W<'a> {
    w: &'a mut W,
}
impl<'a> EXS0A_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXS0A_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input ERU_0A0 is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXS0A_A::VALUE1)
    }
    #[doc = "Input ERU_0A1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXS0A_A::VALUE2)
    }
    #[doc = "Input ERU_0A2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(EXS0A_A::VALUE3)
    }
    #[doc = "Input ERU_0A3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(EXS0A_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Event Source Select for B0 (ERS0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXS0B_A {
    #[doc = "0: Input ERU_0B0 is selected"]
    VALUE1 = 0,
    #[doc = "1: Input ERU_0B1 is selected"]
    VALUE2 = 1,
    #[doc = "2: Input ERU_0B2 is selected"]
    VALUE3 = 2,
    #[doc = "3: Input ERU_0B3 is selected"]
    VALUE4 = 3,
}
impl From<EXS0B_A> for u8 {
    #[inline(always)]
    fn from(variant: EXS0B_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXS0B` reader - Event Source Select for B0 (ERS0)"]
pub struct EXS0B_R(crate::FieldReader<u8, EXS0B_A>);
impl EXS0B_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXS0B_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXS0B_A {
        match self.bits {
            0 => EXS0B_A::VALUE1,
            1 => EXS0B_A::VALUE2,
            2 => EXS0B_A::VALUE3,
            3 => EXS0B_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EXS0B_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EXS0B_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == EXS0B_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == EXS0B_A::VALUE4
    }
}
impl core::ops::Deref for EXS0B_R {
    type Target = crate::FieldReader<u8, EXS0B_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXS0B` writer - Event Source Select for B0 (ERS0)"]
pub struct EXS0B_W<'a> {
    w: &'a mut W,
}
impl<'a> EXS0B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXS0B_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input ERU_0B0 is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXS0B_A::VALUE1)
    }
    #[doc = "Input ERU_0B1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXS0B_A::VALUE2)
    }
    #[doc = "Input ERU_0B2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(EXS0B_A::VALUE3)
    }
    #[doc = "Input ERU_0B3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(EXS0B_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Event Source Select for A1 (ERS1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXS1A_A {
    #[doc = "0: Input ERU_1A0 is selected"]
    VALUE1 = 0,
    #[doc = "1: Input ERU_1A1 is selected"]
    VALUE2 = 1,
    #[doc = "2: Input ERU_1A2 is selected"]
    VALUE3 = 2,
    #[doc = "3: Input ERU_1A3 is selected"]
    VALUE4 = 3,
}
impl From<EXS1A_A> for u8 {
    #[inline(always)]
    fn from(variant: EXS1A_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXS1A` reader - Event Source Select for A1 (ERS1)"]
pub struct EXS1A_R(crate::FieldReader<u8, EXS1A_A>);
impl EXS1A_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXS1A_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXS1A_A {
        match self.bits {
            0 => EXS1A_A::VALUE1,
            1 => EXS1A_A::VALUE2,
            2 => EXS1A_A::VALUE3,
            3 => EXS1A_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EXS1A_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EXS1A_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == EXS1A_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == EXS1A_A::VALUE4
    }
}
impl core::ops::Deref for EXS1A_R {
    type Target = crate::FieldReader<u8, EXS1A_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXS1A` writer - Event Source Select for A1 (ERS1)"]
pub struct EXS1A_W<'a> {
    w: &'a mut W,
}
impl<'a> EXS1A_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXS1A_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input ERU_1A0 is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXS1A_A::VALUE1)
    }
    #[doc = "Input ERU_1A1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXS1A_A::VALUE2)
    }
    #[doc = "Input ERU_1A2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(EXS1A_A::VALUE3)
    }
    #[doc = "Input ERU_1A3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(EXS1A_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Event Source Select for B1 (ERS1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXS1B_A {
    #[doc = "0: Input ERU_1B0 is selected"]
    VALUE1 = 0,
    #[doc = "1: Input ERU_1B1 is selected"]
    VALUE2 = 1,
    #[doc = "2: Input ERU_1B2 is selected"]
    VALUE3 = 2,
    #[doc = "3: Input ERU_1B3 is selected"]
    VALUE4 = 3,
}
impl From<EXS1B_A> for u8 {
    #[inline(always)]
    fn from(variant: EXS1B_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXS1B` reader - Event Source Select for B1 (ERS1)"]
pub struct EXS1B_R(crate::FieldReader<u8, EXS1B_A>);
impl EXS1B_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXS1B_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXS1B_A {
        match self.bits {
            0 => EXS1B_A::VALUE1,
            1 => EXS1B_A::VALUE2,
            2 => EXS1B_A::VALUE3,
            3 => EXS1B_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EXS1B_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EXS1B_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == EXS1B_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == EXS1B_A::VALUE4
    }
}
impl core::ops::Deref for EXS1B_R {
    type Target = crate::FieldReader<u8, EXS1B_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXS1B` writer - Event Source Select for B1 (ERS1)"]
pub struct EXS1B_W<'a> {
    w: &'a mut W,
}
impl<'a> EXS1B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXS1B_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input ERU_1B0 is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXS1B_A::VALUE1)
    }
    #[doc = "Input ERU_1B1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXS1B_A::VALUE2)
    }
    #[doc = "Input ERU_1B2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(EXS1B_A::VALUE3)
    }
    #[doc = "Input ERU_1B3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(EXS1B_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Event Source Select for A2 (ERS2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXS2A_A {
    #[doc = "0: Input ERU_2A0 is selected"]
    VALUE1 = 0,
    #[doc = "1: Input ERU_2A1 is selected"]
    VALUE2 = 1,
    #[doc = "2: Input ERU_2A2 is selected"]
    VALUE3 = 2,
    #[doc = "3: Input ERU_2A3 is selected"]
    VALUE4 = 3,
}
impl From<EXS2A_A> for u8 {
    #[inline(always)]
    fn from(variant: EXS2A_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXS2A` reader - Event Source Select for A2 (ERS2)"]
pub struct EXS2A_R(crate::FieldReader<u8, EXS2A_A>);
impl EXS2A_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXS2A_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXS2A_A {
        match self.bits {
            0 => EXS2A_A::VALUE1,
            1 => EXS2A_A::VALUE2,
            2 => EXS2A_A::VALUE3,
            3 => EXS2A_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EXS2A_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EXS2A_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == EXS2A_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == EXS2A_A::VALUE4
    }
}
impl core::ops::Deref for EXS2A_R {
    type Target = crate::FieldReader<u8, EXS2A_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXS2A` writer - Event Source Select for A2 (ERS2)"]
pub struct EXS2A_W<'a> {
    w: &'a mut W,
}
impl<'a> EXS2A_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXS2A_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input ERU_2A0 is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXS2A_A::VALUE1)
    }
    #[doc = "Input ERU_2A1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXS2A_A::VALUE2)
    }
    #[doc = "Input ERU_2A2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(EXS2A_A::VALUE3)
    }
    #[doc = "Input ERU_2A3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(EXS2A_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Event Source Select for B2 (ERS2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXS2B_A {
    #[doc = "0: Input ERU_2B0 is selected"]
    VALUE1 = 0,
    #[doc = "1: Input ERU_2B1 is selected"]
    VALUE2 = 1,
    #[doc = "2: Input ERU_2B2 is selected"]
    VALUE3 = 2,
    #[doc = "3: Input ERU_2B3 is selected"]
    VALUE4 = 3,
}
impl From<EXS2B_A> for u8 {
    #[inline(always)]
    fn from(variant: EXS2B_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXS2B` reader - Event Source Select for B2 (ERS2)"]
pub struct EXS2B_R(crate::FieldReader<u8, EXS2B_A>);
impl EXS2B_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXS2B_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXS2B_A {
        match self.bits {
            0 => EXS2B_A::VALUE1,
            1 => EXS2B_A::VALUE2,
            2 => EXS2B_A::VALUE3,
            3 => EXS2B_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EXS2B_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EXS2B_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == EXS2B_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == EXS2B_A::VALUE4
    }
}
impl core::ops::Deref for EXS2B_R {
    type Target = crate::FieldReader<u8, EXS2B_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXS2B` writer - Event Source Select for B2 (ERS2)"]
pub struct EXS2B_W<'a> {
    w: &'a mut W,
}
impl<'a> EXS2B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXS2B_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input ERU_2B0 is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXS2B_A::VALUE1)
    }
    #[doc = "Input ERU_2B1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXS2B_A::VALUE2)
    }
    #[doc = "Input ERU_2B2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(EXS2B_A::VALUE3)
    }
    #[doc = "Input ERU_2B3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(EXS2B_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Event Source Select for A3 (ERS3)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXS3A_A {
    #[doc = "0: Input ERU_3A0 is selected"]
    VALUE1 = 0,
    #[doc = "1: Input ERU_3A1 is selected"]
    VALUE2 = 1,
    #[doc = "2: Input ERU_3A2 is selected"]
    VALUE3 = 2,
    #[doc = "3: Input ERU_3A3 is selected"]
    VALUE4 = 3,
}
impl From<EXS3A_A> for u8 {
    #[inline(always)]
    fn from(variant: EXS3A_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXS3A` reader - Event Source Select for A3 (ERS3)"]
pub struct EXS3A_R(crate::FieldReader<u8, EXS3A_A>);
impl EXS3A_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXS3A_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXS3A_A {
        match self.bits {
            0 => EXS3A_A::VALUE1,
            1 => EXS3A_A::VALUE2,
            2 => EXS3A_A::VALUE3,
            3 => EXS3A_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EXS3A_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EXS3A_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == EXS3A_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == EXS3A_A::VALUE4
    }
}
impl core::ops::Deref for EXS3A_R {
    type Target = crate::FieldReader<u8, EXS3A_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXS3A` writer - Event Source Select for A3 (ERS3)"]
pub struct EXS3A_W<'a> {
    w: &'a mut W,
}
impl<'a> EXS3A_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXS3A_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input ERU_3A0 is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXS3A_A::VALUE1)
    }
    #[doc = "Input ERU_3A1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXS3A_A::VALUE2)
    }
    #[doc = "Input ERU_3A2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(EXS3A_A::VALUE3)
    }
    #[doc = "Input ERU_3A3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(EXS3A_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Event Source Select for B3 (ERS3)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXS3B_A {
    #[doc = "0: Input ERU_3B0 is selected"]
    VALUE1 = 0,
    #[doc = "1: Input ERU_3B1 is selected"]
    VALUE2 = 1,
    #[doc = "2: Input ERU_3B2 is selected"]
    VALUE3 = 2,
    #[doc = "3: Input ERU_3B3 is selected"]
    VALUE4 = 3,
}
impl From<EXS3B_A> for u8 {
    #[inline(always)]
    fn from(variant: EXS3B_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXS3B` reader - Event Source Select for B3 (ERS3)"]
pub struct EXS3B_R(crate::FieldReader<u8, EXS3B_A>);
impl EXS3B_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXS3B_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXS3B_A {
        match self.bits {
            0 => EXS3B_A::VALUE1,
            1 => EXS3B_A::VALUE2,
            2 => EXS3B_A::VALUE3,
            3 => EXS3B_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EXS3B_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EXS3B_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == EXS3B_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == EXS3B_A::VALUE4
    }
}
impl core::ops::Deref for EXS3B_R {
    type Target = crate::FieldReader<u8, EXS3B_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXS3B` writer - Event Source Select for B3 (ERS3)"]
pub struct EXS3B_W<'a> {
    w: &'a mut W,
}
impl<'a> EXS3B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXS3B_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input ERU_3B0 is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXS3B_A::VALUE1)
    }
    #[doc = "Input ERU_3B1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXS3B_A::VALUE2)
    }
    #[doc = "Input ERU_3B2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(EXS3B_A::VALUE3)
    }
    #[doc = "Input ERU_3B3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(EXS3B_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Event Source Select for A0 (ERS0)"]
    #[inline(always)]
    pub fn exs0a(&self) -> EXS0A_R {
        EXS0A_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Event Source Select for B0 (ERS0)"]
    #[inline(always)]
    pub fn exs0b(&self) -> EXS0B_R {
        EXS0B_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Event Source Select for A1 (ERS1)"]
    #[inline(always)]
    pub fn exs1a(&self) -> EXS1A_R {
        EXS1A_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Event Source Select for B1 (ERS1)"]
    #[inline(always)]
    pub fn exs1b(&self) -> EXS1B_R {
        EXS1B_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Event Source Select for A2 (ERS2)"]
    #[inline(always)]
    pub fn exs2a(&self) -> EXS2A_R {
        EXS2A_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Event Source Select for B2 (ERS2)"]
    #[inline(always)]
    pub fn exs2b(&self) -> EXS2B_R {
        EXS2B_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Event Source Select for A3 (ERS3)"]
    #[inline(always)]
    pub fn exs3a(&self) -> EXS3A_R {
        EXS3A_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Event Source Select for B3 (ERS3)"]
    #[inline(always)]
    pub fn exs3b(&self) -> EXS3B_R {
        EXS3B_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Event Source Select for A0 (ERS0)"]
    #[inline(always)]
    pub fn exs0a(&mut self) -> EXS0A_W {
        EXS0A_W { w: self }
    }
    #[doc = "Bits 2:3 - Event Source Select for B0 (ERS0)"]
    #[inline(always)]
    pub fn exs0b(&mut self) -> EXS0B_W {
        EXS0B_W { w: self }
    }
    #[doc = "Bits 4:5 - Event Source Select for A1 (ERS1)"]
    #[inline(always)]
    pub fn exs1a(&mut self) -> EXS1A_W {
        EXS1A_W { w: self }
    }
    #[doc = "Bits 6:7 - Event Source Select for B1 (ERS1)"]
    #[inline(always)]
    pub fn exs1b(&mut self) -> EXS1B_W {
        EXS1B_W { w: self }
    }
    #[doc = "Bits 8:9 - Event Source Select for A2 (ERS2)"]
    #[inline(always)]
    pub fn exs2a(&mut self) -> EXS2A_W {
        EXS2A_W { w: self }
    }
    #[doc = "Bits 10:11 - Event Source Select for B2 (ERS2)"]
    #[inline(always)]
    pub fn exs2b(&mut self) -> EXS2B_W {
        EXS2B_W { w: self }
    }
    #[doc = "Bits 12:13 - Event Source Select for A3 (ERS3)"]
    #[inline(always)]
    pub fn exs3a(&mut self) -> EXS3A_W {
        EXS3A_W { w: self }
    }
    #[doc = "Bits 14:15 - Event Source Select for B3 (ERS3)"]
    #[inline(always)]
    pub fn exs3b(&mut self) -> EXS3B_W {
        EXS3B_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Input Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exisel](index.html) module"]
pub struct EXISEL_SPEC;
impl crate::RegisterSpec for EXISEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exisel::R](R) reader structure"]
impl crate::Readable for EXISEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exisel::W](W) writer structure"]
impl crate::Writable for EXISEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXISEL to value 0"]
impl crate::Resettable for EXISEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
