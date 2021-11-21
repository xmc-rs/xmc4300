#[doc = "Register `CFSR` reader"]
pub struct R(crate::R<CFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFSR` writer"]
pub struct W(crate::W<CFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFSR_SPEC>;
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
impl From<crate::W<CFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Instruction access violation flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IACCVIOL_A {
    #[doc = "0: no instruction access violation fault"]
    VALUE1 = 0,
    #[doc = "1: the processor attempted an instruction fetch from a location that does not permit execution."]
    VALUE2 = 1,
}
impl From<IACCVIOL_A> for bool {
    #[inline(always)]
    fn from(variant: IACCVIOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IACCVIOL` reader - Instruction access violation flag"]
pub struct IACCVIOL_R(crate::FieldReader<bool, IACCVIOL_A>);
impl IACCVIOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        IACCVIOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IACCVIOL_A {
        match self.bits {
            false => IACCVIOL_A::VALUE1,
            true => IACCVIOL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == IACCVIOL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == IACCVIOL_A::VALUE2
    }
}
impl core::ops::Deref for IACCVIOL_R {
    type Target = crate::FieldReader<bool, IACCVIOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IACCVIOL` writer - Instruction access violation flag"]
pub struct IACCVIOL_W<'a> {
    w: &'a mut W,
}
impl<'a> IACCVIOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IACCVIOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no instruction access violation fault"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IACCVIOL_A::VALUE1)
    }
    #[doc = "the processor attempted an instruction fetch from a location that does not permit execution."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(IACCVIOL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Data access violation flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACCVIOL_A {
    #[doc = "0: no data access violation fault"]
    VALUE1 = 0,
    #[doc = "1: the processor attempted a load or store at a location that does not permit the operation."]
    VALUE2 = 1,
}
impl From<DACCVIOL_A> for bool {
    #[inline(always)]
    fn from(variant: DACCVIOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACCVIOL` reader - Data access violation flag"]
pub struct DACCVIOL_R(crate::FieldReader<bool, DACCVIOL_A>);
impl DACCVIOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACCVIOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACCVIOL_A {
        match self.bits {
            false => DACCVIOL_A::VALUE1,
            true => DACCVIOL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DACCVIOL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DACCVIOL_A::VALUE2
    }
}
impl core::ops::Deref for DACCVIOL_R {
    type Target = crate::FieldReader<bool, DACCVIOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACCVIOL` writer - Data access violation flag"]
pub struct DACCVIOL_W<'a> {
    w: &'a mut W,
}
impl<'a> DACCVIOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACCVIOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no data access violation fault"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DACCVIOL_A::VALUE1)
    }
    #[doc = "the processor attempted a load or store at a location that does not permit the operation."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DACCVIOL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "MemManage fault on unstacking for a return from exception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUNSTKERR_A {
    #[doc = "0: no unstacking fault"]
    VALUE1 = 0,
    #[doc = "1: unstack for an exception return has caused one or more access violations."]
    VALUE2 = 1,
}
impl From<MUNSTKERR_A> for bool {
    #[inline(always)]
    fn from(variant: MUNSTKERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUNSTKERR` reader - MemManage fault on unstacking for a return from exception"]
pub struct MUNSTKERR_R(crate::FieldReader<bool, MUNSTKERR_A>);
impl MUNSTKERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUNSTKERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUNSTKERR_A {
        match self.bits {
            false => MUNSTKERR_A::VALUE1,
            true => MUNSTKERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MUNSTKERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MUNSTKERR_A::VALUE2
    }
}
impl core::ops::Deref for MUNSTKERR_R {
    type Target = crate::FieldReader<bool, MUNSTKERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUNSTKERR` writer - MemManage fault on unstacking for a return from exception"]
pub struct MUNSTKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> MUNSTKERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUNSTKERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no unstacking fault"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MUNSTKERR_A::VALUE1)
    }
    #[doc = "unstack for an exception return has caused one or more access violations."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MUNSTKERR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "MemManage fault on stacking for exception entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTKERR_A {
    #[doc = "0: no stacking fault"]
    VALUE1 = 0,
    #[doc = "1: stacking for an exception entry has caused one or more access violations."]
    VALUE2 = 1,
}
impl From<MSTKERR_A> for bool {
    #[inline(always)]
    fn from(variant: MSTKERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTKERR` reader - MemManage fault on stacking for exception entry"]
pub struct MSTKERR_R(crate::FieldReader<bool, MSTKERR_A>);
impl MSTKERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTKERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTKERR_A {
        match self.bits {
            false => MSTKERR_A::VALUE1,
            true => MSTKERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MSTKERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MSTKERR_A::VALUE2
    }
}
impl core::ops::Deref for MSTKERR_R {
    type Target = crate::FieldReader<bool, MSTKERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTKERR` writer - MemManage fault on stacking for exception entry"]
pub struct MSTKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTKERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTKERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no stacking fault"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSTKERR_A::VALUE1)
    }
    #[doc = "stacking for an exception entry has caused one or more access violations."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSTKERR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "MemManage fault during floating point lazy state preservation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MLSPERR_A {
    #[doc = "0: No MemManage fault occurred during floating-point lazy state preservation"]
    VALUE1 = 0,
    #[doc = "1: A MemManage fault occurred during floating-point lazy state preservation"]
    VALUE2 = 1,
}
impl From<MLSPERR_A> for bool {
    #[inline(always)]
    fn from(variant: MLSPERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MLSPERR` reader - MemManage fault during floating point lazy state preservation"]
pub struct MLSPERR_R(crate::FieldReader<bool, MLSPERR_A>);
impl MLSPERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        MLSPERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MLSPERR_A {
        match self.bits {
            false => MLSPERR_A::VALUE1,
            true => MLSPERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MLSPERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MLSPERR_A::VALUE2
    }
}
impl core::ops::Deref for MLSPERR_R {
    type Target = crate::FieldReader<bool, MLSPERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MLSPERR` writer - MemManage fault during floating point lazy state preservation"]
pub struct MLSPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> MLSPERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MLSPERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No MemManage fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MLSPERR_A::VALUE1)
    }
    #[doc = "A MemManage fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MLSPERR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "MemManage Fault Address Register (MMFAR) valid flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMARVALID_A {
    #[doc = "0: value in MMAR is not a valid fault address"]
    VALUE1 = 0,
    #[doc = "1: MMAR holds a valid fault address."]
    VALUE2 = 1,
}
impl From<MMARVALID_A> for bool {
    #[inline(always)]
    fn from(variant: MMARVALID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMARVALID` reader - MemManage Fault Address Register (MMFAR) valid flag"]
pub struct MMARVALID_R(crate::FieldReader<bool, MMARVALID_A>);
impl MMARVALID_R {
    pub(crate) fn new(bits: bool) -> Self {
        MMARVALID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMARVALID_A {
        match self.bits {
            false => MMARVALID_A::VALUE1,
            true => MMARVALID_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MMARVALID_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MMARVALID_A::VALUE2
    }
}
impl core::ops::Deref for MMARVALID_R {
    type Target = crate::FieldReader<bool, MMARVALID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMARVALID` writer - MemManage Fault Address Register (MMFAR) valid flag"]
pub struct MMARVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> MMARVALID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MMARVALID_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "value in MMAR is not a valid fault address"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MMARVALID_A::VALUE1)
    }
    #[doc = "MMAR holds a valid fault address."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MMARVALID_A::VALUE2)
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
#[doc = "Instruction bus error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBUSERR_A {
    #[doc = "0: no instruction bus error"]
    VALUE1 = 0,
    #[doc = "1: instruction bus error."]
    VALUE2 = 1,
}
impl From<IBUSERR_A> for bool {
    #[inline(always)]
    fn from(variant: IBUSERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IBUSERR` reader - Instruction bus error"]
pub struct IBUSERR_R(crate::FieldReader<bool, IBUSERR_A>);
impl IBUSERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        IBUSERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBUSERR_A {
        match self.bits {
            false => IBUSERR_A::VALUE1,
            true => IBUSERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == IBUSERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == IBUSERR_A::VALUE2
    }
}
impl core::ops::Deref for IBUSERR_R {
    type Target = crate::FieldReader<bool, IBUSERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBUSERR` writer - Instruction bus error"]
pub struct IBUSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> IBUSERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IBUSERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no instruction bus error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IBUSERR_A::VALUE1)
    }
    #[doc = "instruction bus error."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(IBUSERR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Precise data bus error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRECISERR_A {
    #[doc = "0: no precise data bus error"]
    VALUE1 = 0,
    #[doc = "1: a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault."]
    VALUE2 = 1,
}
impl From<PRECISERR_A> for bool {
    #[inline(always)]
    fn from(variant: PRECISERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRECISERR` reader - Precise data bus error"]
pub struct PRECISERR_R(crate::FieldReader<bool, PRECISERR_A>);
impl PRECISERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRECISERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRECISERR_A {
        match self.bits {
            false => PRECISERR_A::VALUE1,
            true => PRECISERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PRECISERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PRECISERR_A::VALUE2
    }
}
impl core::ops::Deref for PRECISERR_R {
    type Target = crate::FieldReader<bool, PRECISERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRECISERR` writer - Precise data bus error"]
pub struct PRECISERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRECISERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRECISERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no precise data bus error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRECISERR_A::VALUE1)
    }
    #[doc = "a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRECISERR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Imprecise data bus error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMPRECISERR_A {
    #[doc = "0: no imprecise data bus error"]
    VALUE1 = 0,
    #[doc = "1: a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error."]
    VALUE2 = 1,
}
impl From<IMPRECISERR_A> for bool {
    #[inline(always)]
    fn from(variant: IMPRECISERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMPRECISERR` reader - Imprecise data bus error"]
pub struct IMPRECISERR_R(crate::FieldReader<bool, IMPRECISERR_A>);
impl IMPRECISERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        IMPRECISERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMPRECISERR_A {
        match self.bits {
            false => IMPRECISERR_A::VALUE1,
            true => IMPRECISERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == IMPRECISERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == IMPRECISERR_A::VALUE2
    }
}
impl core::ops::Deref for IMPRECISERR_R {
    type Target = crate::FieldReader<bool, IMPRECISERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMPRECISERR` writer - Imprecise data bus error"]
pub struct IMPRECISERR_W<'a> {
    w: &'a mut W,
}
impl<'a> IMPRECISERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMPRECISERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no imprecise data bus error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IMPRECISERR_A::VALUE1)
    }
    #[doc = "a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(IMPRECISERR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "BusFault on unstacking for a return from exception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNSTKERR_A {
    #[doc = "0: no unstacking fault"]
    VALUE1 = 0,
    #[doc = "1: stacking for an exception entry has caused one or more BusFaults."]
    VALUE2 = 1,
}
impl From<UNSTKERR_A> for bool {
    #[inline(always)]
    fn from(variant: UNSTKERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNSTKERR` reader - BusFault on unstacking for a return from exception"]
pub struct UNSTKERR_R(crate::FieldReader<bool, UNSTKERR_A>);
impl UNSTKERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        UNSTKERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNSTKERR_A {
        match self.bits {
            false => UNSTKERR_A::VALUE1,
            true => UNSTKERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == UNSTKERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == UNSTKERR_A::VALUE2
    }
}
impl core::ops::Deref for UNSTKERR_R {
    type Target = crate::FieldReader<bool, UNSTKERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNSTKERR` writer - BusFault on unstacking for a return from exception"]
pub struct UNSTKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> UNSTKERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNSTKERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no unstacking fault"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(UNSTKERR_A::VALUE1)
    }
    #[doc = "stacking for an exception entry has caused one or more BusFaults."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(UNSTKERR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "BusFault on stacking for exception entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STKERR_A {
    #[doc = "0: no stacking fault"]
    VALUE1 = 0,
    #[doc = "1: stacking for an exception entry has caused one or more BusFaults."]
    VALUE2 = 1,
}
impl From<STKERR_A> for bool {
    #[inline(always)]
    fn from(variant: STKERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STKERR` reader - BusFault on stacking for exception entry"]
pub struct STKERR_R(crate::FieldReader<bool, STKERR_A>);
impl STKERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        STKERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STKERR_A {
        match self.bits {
            false => STKERR_A::VALUE1,
            true => STKERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == STKERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == STKERR_A::VALUE2
    }
}
impl core::ops::Deref for STKERR_R {
    type Target = crate::FieldReader<bool, STKERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STKERR` writer - BusFault on stacking for exception entry"]
pub struct STKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> STKERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STKERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no stacking fault"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STKERR_A::VALUE1)
    }
    #[doc = "stacking for an exception entry has caused one or more BusFaults."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STKERR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "BusFault during floating point lazy state preservation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSPERR_A {
    #[doc = "0: No bus fault occurred during floating-point lazy state preservation."]
    VALUE1 = 0,
    #[doc = "1: A bus fault occurred during floating-point lazy state preservation"]
    VALUE2 = 1,
}
impl From<LSPERR_A> for bool {
    #[inline(always)]
    fn from(variant: LSPERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSPERR` reader - BusFault during floating point lazy state preservation"]
pub struct LSPERR_R(crate::FieldReader<bool, LSPERR_A>);
impl LSPERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSPERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSPERR_A {
        match self.bits {
            false => LSPERR_A::VALUE1,
            true => LSPERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == LSPERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LSPERR_A::VALUE2
    }
}
impl core::ops::Deref for LSPERR_R {
    type Target = crate::FieldReader<bool, LSPERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSPERR` writer - BusFault during floating point lazy state preservation"]
pub struct LSPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> LSPERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSPERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No bus fault occurred during floating-point lazy state preservation."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LSPERR_A::VALUE1)
    }
    #[doc = "A bus fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LSPERR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "BusFault Address Register (BFAR) valid flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFARVALID_A {
    #[doc = "0: value in BFAR is not a valid fault address"]
    VALUE1 = 0,
    #[doc = "1: BFAR holds a valid fault address."]
    VALUE2 = 1,
}
impl From<BFARVALID_A> for bool {
    #[inline(always)]
    fn from(variant: BFARVALID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFARVALID` reader - BusFault Address Register (BFAR) valid flag"]
pub struct BFARVALID_R(crate::FieldReader<bool, BFARVALID_A>);
impl BFARVALID_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFARVALID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFARVALID_A {
        match self.bits {
            false => BFARVALID_A::VALUE1,
            true => BFARVALID_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFARVALID_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFARVALID_A::VALUE2
    }
}
impl core::ops::Deref for BFARVALID_R {
    type Target = crate::FieldReader<bool, BFARVALID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFARVALID` writer - BusFault Address Register (BFAR) valid flag"]
pub struct BFARVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> BFARVALID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFARVALID_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "value in BFAR is not a valid fault address"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFARVALID_A::VALUE1)
    }
    #[doc = "BFAR holds a valid fault address."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFARVALID_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Undefined instruction UsageFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNDEFINSTR_A {
    #[doc = "0: no undefined instruction UsageFault"]
    VALUE1 = 0,
    #[doc = "1: the processor has attempted to execute an undefined instruction."]
    VALUE2 = 1,
}
impl From<UNDEFINSTR_A> for bool {
    #[inline(always)]
    fn from(variant: UNDEFINSTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNDEFINSTR` reader - Undefined instruction UsageFault"]
pub struct UNDEFINSTR_R(crate::FieldReader<bool, UNDEFINSTR_A>);
impl UNDEFINSTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        UNDEFINSTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNDEFINSTR_A {
        match self.bits {
            false => UNDEFINSTR_A::VALUE1,
            true => UNDEFINSTR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == UNDEFINSTR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == UNDEFINSTR_A::VALUE2
    }
}
impl core::ops::Deref for UNDEFINSTR_R {
    type Target = crate::FieldReader<bool, UNDEFINSTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNDEFINSTR` writer - Undefined instruction UsageFault"]
pub struct UNDEFINSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDEFINSTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNDEFINSTR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no undefined instruction UsageFault"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(UNDEFINSTR_A::VALUE1)
    }
    #[doc = "the processor has attempted to execute an undefined instruction."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(UNDEFINSTR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Invalid state UsageFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVSTATE_A {
    #[doc = "0: no invalid state UsageFault"]
    VALUE1 = 0,
    #[doc = "1: the processor has attempted to execute an instruction that makes illegal use of the EPSR."]
    VALUE2 = 1,
}
impl From<INVSTATE_A> for bool {
    #[inline(always)]
    fn from(variant: INVSTATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVSTATE` reader - Invalid state UsageFault"]
pub struct INVSTATE_R(crate::FieldReader<bool, INVSTATE_A>);
impl INVSTATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        INVSTATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVSTATE_A {
        match self.bits {
            false => INVSTATE_A::VALUE1,
            true => INVSTATE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == INVSTATE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == INVSTATE_A::VALUE2
    }
}
impl core::ops::Deref for INVSTATE_R {
    type Target = crate::FieldReader<bool, INVSTATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVSTATE` writer - Invalid state UsageFault"]
pub struct INVSTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> INVSTATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVSTATE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no invalid state UsageFault"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(INVSTATE_A::VALUE1)
    }
    #[doc = "the processor has attempted to execute an instruction that makes illegal use of the EPSR."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(INVSTATE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Invalid PC load UsageFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVPC_A {
    #[doc = "0: no invalid PC load UsageFault"]
    VALUE1 = 0,
    #[doc = "1: the processor has attempted an illegal load of EXC_RETURN to the PC, as a result of an invalid context, or an invalid EXC_RETURN value."]
    VALUE2 = 1,
}
impl From<INVPC_A> for bool {
    #[inline(always)]
    fn from(variant: INVPC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVPC` reader - Invalid PC load UsageFault"]
pub struct INVPC_R(crate::FieldReader<bool, INVPC_A>);
impl INVPC_R {
    pub(crate) fn new(bits: bool) -> Self {
        INVPC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVPC_A {
        match self.bits {
            false => INVPC_A::VALUE1,
            true => INVPC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == INVPC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == INVPC_A::VALUE2
    }
}
impl core::ops::Deref for INVPC_R {
    type Target = crate::FieldReader<bool, INVPC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVPC` writer - Invalid PC load UsageFault"]
pub struct INVPC_W<'a> {
    w: &'a mut W,
}
impl<'a> INVPC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVPC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no invalid PC load UsageFault"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(INVPC_A::VALUE1)
    }
    #[doc = "the processor has attempted an illegal load of EXC_RETURN to the PC, as a result of an invalid context, or an invalid EXC_RETURN value."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(INVPC_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "No coprocessor UsageFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOCP_A {
    #[doc = "0: no UsageFault caused by attempting to access a coprocessor"]
    VALUE1 = 0,
    #[doc = "1: the processor has attempted to access a coprocessor."]
    VALUE2 = 1,
}
impl From<NOCP_A> for bool {
    #[inline(always)]
    fn from(variant: NOCP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOCP` reader - No coprocessor UsageFault"]
pub struct NOCP_R(crate::FieldReader<bool, NOCP_A>);
impl NOCP_R {
    pub(crate) fn new(bits: bool) -> Self {
        NOCP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOCP_A {
        match self.bits {
            false => NOCP_A::VALUE1,
            true => NOCP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == NOCP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == NOCP_A::VALUE2
    }
}
impl core::ops::Deref for NOCP_R {
    type Target = crate::FieldReader<bool, NOCP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOCP` writer - No coprocessor UsageFault"]
pub struct NOCP_W<'a> {
    w: &'a mut W,
}
impl<'a> NOCP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOCP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no UsageFault caused by attempting to access a coprocessor"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(NOCP_A::VALUE1)
    }
    #[doc = "the processor has attempted to access a coprocessor."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(NOCP_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Unaligned access UsageFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNALIGNED_A {
    #[doc = "0: no unaligned access fault, or unaligned access trapping not enabled"]
    VALUE1 = 0,
    #[doc = "1: the processor has made an unaligned memory access."]
    VALUE2 = 1,
}
impl From<UNALIGNED_A> for bool {
    #[inline(always)]
    fn from(variant: UNALIGNED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNALIGNED` reader - Unaligned access UsageFault"]
pub struct UNALIGNED_R(crate::FieldReader<bool, UNALIGNED_A>);
impl UNALIGNED_R {
    pub(crate) fn new(bits: bool) -> Self {
        UNALIGNED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNALIGNED_A {
        match self.bits {
            false => UNALIGNED_A::VALUE1,
            true => UNALIGNED_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == UNALIGNED_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == UNALIGNED_A::VALUE2
    }
}
impl core::ops::Deref for UNALIGNED_R {
    type Target = crate::FieldReader<bool, UNALIGNED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNALIGNED` writer - Unaligned access UsageFault"]
pub struct UNALIGNED_W<'a> {
    w: &'a mut W,
}
impl<'a> UNALIGNED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNALIGNED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no unaligned access fault, or unaligned access trapping not enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(UNALIGNED_A::VALUE1)
    }
    #[doc = "the processor has made an unaligned memory access."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(UNALIGNED_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Divide by zero UsageFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVBYZERO_A {
    #[doc = "0: no divide by zero fault, or divide by zero trapping not enabled"]
    VALUE1 = 0,
    #[doc = "1: the processor has executed an SDIV or UDIV instruction with a divisor of 0"]
    VALUE2 = 1,
}
impl From<DIVBYZERO_A> for bool {
    #[inline(always)]
    fn from(variant: DIVBYZERO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIVBYZERO` reader - Divide by zero UsageFault"]
pub struct DIVBYZERO_R(crate::FieldReader<bool, DIVBYZERO_A>);
impl DIVBYZERO_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIVBYZERO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVBYZERO_A {
        match self.bits {
            false => DIVBYZERO_A::VALUE1,
            true => DIVBYZERO_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DIVBYZERO_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DIVBYZERO_A::VALUE2
    }
}
impl core::ops::Deref for DIVBYZERO_R {
    type Target = crate::FieldReader<bool, DIVBYZERO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVBYZERO` writer - Divide by zero UsageFault"]
pub struct DIVBYZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVBYZERO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVBYZERO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no divide by zero fault, or divide by zero trapping not enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVBYZERO_A::VALUE1)
    }
    #[doc = "the processor has executed an SDIV or UDIV instruction with a divisor of 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVBYZERO_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Instruction access violation flag"]
    #[inline(always)]
    pub fn iaccviol(&self) -> IACCVIOL_R {
        IACCVIOL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data access violation flag"]
    #[inline(always)]
    pub fn daccviol(&self) -> DACCVIOL_R {
        DACCVIOL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MemManage fault on unstacking for a return from exception"]
    #[inline(always)]
    pub fn munstkerr(&self) -> MUNSTKERR_R {
        MUNSTKERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MemManage fault on stacking for exception entry"]
    #[inline(always)]
    pub fn mstkerr(&self) -> MSTKERR_R {
        MSTKERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MemManage fault during floating point lazy state preservation"]
    #[inline(always)]
    pub fn mlsperr(&self) -> MLSPERR_R {
        MLSPERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MemManage Fault Address Register (MMFAR) valid flag"]
    #[inline(always)]
    pub fn mmarvalid(&self) -> MMARVALID_R {
        MMARVALID_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Instruction bus error"]
    #[inline(always)]
    pub fn ibuserr(&self) -> IBUSERR_R {
        IBUSERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Precise data bus error"]
    #[inline(always)]
    pub fn preciserr(&self) -> PRECISERR_R {
        PRECISERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Imprecise data bus error"]
    #[inline(always)]
    pub fn impreciserr(&self) -> IMPRECISERR_R {
        IMPRECISERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BusFault on unstacking for a return from exception"]
    #[inline(always)]
    pub fn unstkerr(&self) -> UNSTKERR_R {
        UNSTKERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - BusFault on stacking for exception entry"]
    #[inline(always)]
    pub fn stkerr(&self) -> STKERR_R {
        STKERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - BusFault during floating point lazy state preservation"]
    #[inline(always)]
    pub fn lsperr(&self) -> LSPERR_R {
        LSPERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - BusFault Address Register (BFAR) valid flag"]
    #[inline(always)]
    pub fn bfarvalid(&self) -> BFARVALID_R {
        BFARVALID_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Undefined instruction UsageFault"]
    #[inline(always)]
    pub fn undefinstr(&self) -> UNDEFINSTR_R {
        UNDEFINSTR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Invalid state UsageFault"]
    #[inline(always)]
    pub fn invstate(&self) -> INVSTATE_R {
        INVSTATE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Invalid PC load UsageFault"]
    #[inline(always)]
    pub fn invpc(&self) -> INVPC_R {
        INVPC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - No coprocessor UsageFault"]
    #[inline(always)]
    pub fn nocp(&self) -> NOCP_R {
        NOCP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Unaligned access UsageFault"]
    #[inline(always)]
    pub fn unaligned(&self) -> UNALIGNED_R {
        UNALIGNED_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Divide by zero UsageFault"]
    #[inline(always)]
    pub fn divbyzero(&self) -> DIVBYZERO_R {
        DIVBYZERO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Instruction access violation flag"]
    #[inline(always)]
    pub fn iaccviol(&mut self) -> IACCVIOL_W {
        IACCVIOL_W { w: self }
    }
    #[doc = "Bit 1 - Data access violation flag"]
    #[inline(always)]
    pub fn daccviol(&mut self) -> DACCVIOL_W {
        DACCVIOL_W { w: self }
    }
    #[doc = "Bit 3 - MemManage fault on unstacking for a return from exception"]
    #[inline(always)]
    pub fn munstkerr(&mut self) -> MUNSTKERR_W {
        MUNSTKERR_W { w: self }
    }
    #[doc = "Bit 4 - MemManage fault on stacking for exception entry"]
    #[inline(always)]
    pub fn mstkerr(&mut self) -> MSTKERR_W {
        MSTKERR_W { w: self }
    }
    #[doc = "Bit 5 - MemManage fault during floating point lazy state preservation"]
    #[inline(always)]
    pub fn mlsperr(&mut self) -> MLSPERR_W {
        MLSPERR_W { w: self }
    }
    #[doc = "Bit 7 - MemManage Fault Address Register (MMFAR) valid flag"]
    #[inline(always)]
    pub fn mmarvalid(&mut self) -> MMARVALID_W {
        MMARVALID_W { w: self }
    }
    #[doc = "Bit 8 - Instruction bus error"]
    #[inline(always)]
    pub fn ibuserr(&mut self) -> IBUSERR_W {
        IBUSERR_W { w: self }
    }
    #[doc = "Bit 9 - Precise data bus error"]
    #[inline(always)]
    pub fn preciserr(&mut self) -> PRECISERR_W {
        PRECISERR_W { w: self }
    }
    #[doc = "Bit 10 - Imprecise data bus error"]
    #[inline(always)]
    pub fn impreciserr(&mut self) -> IMPRECISERR_W {
        IMPRECISERR_W { w: self }
    }
    #[doc = "Bit 11 - BusFault on unstacking for a return from exception"]
    #[inline(always)]
    pub fn unstkerr(&mut self) -> UNSTKERR_W {
        UNSTKERR_W { w: self }
    }
    #[doc = "Bit 12 - BusFault on stacking for exception entry"]
    #[inline(always)]
    pub fn stkerr(&mut self) -> STKERR_W {
        STKERR_W { w: self }
    }
    #[doc = "Bit 13 - BusFault during floating point lazy state preservation"]
    #[inline(always)]
    pub fn lsperr(&mut self) -> LSPERR_W {
        LSPERR_W { w: self }
    }
    #[doc = "Bit 15 - BusFault Address Register (BFAR) valid flag"]
    #[inline(always)]
    pub fn bfarvalid(&mut self) -> BFARVALID_W {
        BFARVALID_W { w: self }
    }
    #[doc = "Bit 16 - Undefined instruction UsageFault"]
    #[inline(always)]
    pub fn undefinstr(&mut self) -> UNDEFINSTR_W {
        UNDEFINSTR_W { w: self }
    }
    #[doc = "Bit 17 - Invalid state UsageFault"]
    #[inline(always)]
    pub fn invstate(&mut self) -> INVSTATE_W {
        INVSTATE_W { w: self }
    }
    #[doc = "Bit 18 - Invalid PC load UsageFault"]
    #[inline(always)]
    pub fn invpc(&mut self) -> INVPC_W {
        INVPC_W { w: self }
    }
    #[doc = "Bit 19 - No coprocessor UsageFault"]
    #[inline(always)]
    pub fn nocp(&mut self) -> NOCP_W {
        NOCP_W { w: self }
    }
    #[doc = "Bit 24 - Unaligned access UsageFault"]
    #[inline(always)]
    pub fn unaligned(&mut self) -> UNALIGNED_W {
        UNALIGNED_W { w: self }
    }
    #[doc = "Bit 25 - Divide by zero UsageFault"]
    #[inline(always)]
    pub fn divbyzero(&mut self) -> DIVBYZERO_W {
        DIVBYZERO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configurable Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfsr](index.html) module"]
pub struct CFSR_SPEC;
impl crate::RegisterSpec for CFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfsr::R](R) reader structure"]
impl crate::Readable for CFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfsr::W](W) writer structure"]
impl crate::Writable for CFSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFSR to value 0"]
impl crate::Resettable for CFSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
