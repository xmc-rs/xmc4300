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
#[doc = "Field `IACCVIOL` reader - Instruction access violation flag"]
pub type IACCVIOL_R = crate::BitReader<IACCVIOL_A>;
#[doc = "Instruction access violation flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl IACCVIOL_R {
    #[doc = "Get enumerated values variant"]
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
        *self == IACCVIOL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IACCVIOL_A::VALUE2
    }
}
#[doc = "Field `IACCVIOL` writer - Instruction access violation flag"]
pub type IACCVIOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, IACCVIOL_A, O>;
impl<'a, const O: u8> IACCVIOL_W<'a, O> {
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
}
#[doc = "Field `DACCVIOL` reader - Data access violation flag"]
pub type DACCVIOL_R = crate::BitReader<DACCVIOL_A>;
#[doc = "Data access violation flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DACCVIOL_R {
    #[doc = "Get enumerated values variant"]
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
        *self == DACCVIOL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DACCVIOL_A::VALUE2
    }
}
#[doc = "Field `DACCVIOL` writer - Data access violation flag"]
pub type DACCVIOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, DACCVIOL_A, O>;
impl<'a, const O: u8> DACCVIOL_W<'a, O> {
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
}
#[doc = "Field `MUNSTKERR` reader - MemManage fault on unstacking for a return from exception"]
pub type MUNSTKERR_R = crate::BitReader<MUNSTKERR_A>;
#[doc = "MemManage fault on unstacking for a return from exception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MUNSTKERR_R {
    #[doc = "Get enumerated values variant"]
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
        *self == MUNSTKERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MUNSTKERR_A::VALUE2
    }
}
#[doc = "Field `MUNSTKERR` writer - MemManage fault on unstacking for a return from exception"]
pub type MUNSTKERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, MUNSTKERR_A, O>;
impl<'a, const O: u8> MUNSTKERR_W<'a, O> {
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
}
#[doc = "Field `MSTKERR` reader - MemManage fault on stacking for exception entry"]
pub type MSTKERR_R = crate::BitReader<MSTKERR_A>;
#[doc = "MemManage fault on stacking for exception entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MSTKERR_R {
    #[doc = "Get enumerated values variant"]
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
        *self == MSTKERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSTKERR_A::VALUE2
    }
}
#[doc = "Field `MSTKERR` writer - MemManage fault on stacking for exception entry"]
pub type MSTKERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, MSTKERR_A, O>;
impl<'a, const O: u8> MSTKERR_W<'a, O> {
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
}
#[doc = "Field `MLSPERR` reader - MemManage fault during floating point lazy state preservation"]
pub type MLSPERR_R = crate::BitReader<MLSPERR_A>;
#[doc = "MemManage fault during floating point lazy state preservation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MLSPERR_R {
    #[doc = "Get enumerated values variant"]
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
        *self == MLSPERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MLSPERR_A::VALUE2
    }
}
#[doc = "Field `MLSPERR` writer - MemManage fault during floating point lazy state preservation"]
pub type MLSPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, MLSPERR_A, O>;
impl<'a, const O: u8> MLSPERR_W<'a, O> {
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
}
#[doc = "Field `MMARVALID` reader - MemManage Fault Address Register (MMFAR) valid flag"]
pub type MMARVALID_R = crate::BitReader<MMARVALID_A>;
#[doc = "MemManage Fault Address Register (MMFAR) valid flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MMARVALID_R {
    #[doc = "Get enumerated values variant"]
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
        *self == MMARVALID_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MMARVALID_A::VALUE2
    }
}
#[doc = "Field `MMARVALID` writer - MemManage Fault Address Register (MMFAR) valid flag"]
pub type MMARVALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, MMARVALID_A, O>;
impl<'a, const O: u8> MMARVALID_W<'a, O> {
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
}
#[doc = "Field `IBUSERR` reader - Instruction bus error"]
pub type IBUSERR_R = crate::BitReader<IBUSERR_A>;
#[doc = "Instruction bus error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl IBUSERR_R {
    #[doc = "Get enumerated values variant"]
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
        *self == IBUSERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IBUSERR_A::VALUE2
    }
}
#[doc = "Field `IBUSERR` writer - Instruction bus error"]
pub type IBUSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, IBUSERR_A, O>;
impl<'a, const O: u8> IBUSERR_W<'a, O> {
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
}
#[doc = "Field `PRECISERR` reader - Precise data bus error"]
pub type PRECISERR_R = crate::BitReader<PRECISERR_A>;
#[doc = "Precise data bus error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PRECISERR_R {
    #[doc = "Get enumerated values variant"]
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
        *self == PRECISERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRECISERR_A::VALUE2
    }
}
#[doc = "Field `PRECISERR` writer - Precise data bus error"]
pub type PRECISERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, PRECISERR_A, O>;
impl<'a, const O: u8> PRECISERR_W<'a, O> {
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
}
#[doc = "Field `IMPRECISERR` reader - Imprecise data bus error"]
pub type IMPRECISERR_R = crate::BitReader<IMPRECISERR_A>;
#[doc = "Imprecise data bus error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl IMPRECISERR_R {
    #[doc = "Get enumerated values variant"]
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
        *self == IMPRECISERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IMPRECISERR_A::VALUE2
    }
}
#[doc = "Field `IMPRECISERR` writer - Imprecise data bus error"]
pub type IMPRECISERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, IMPRECISERR_A, O>;
impl<'a, const O: u8> IMPRECISERR_W<'a, O> {
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
}
#[doc = "Field `UNSTKERR` reader - BusFault on unstacking for a return from exception"]
pub type UNSTKERR_R = crate::BitReader<UNSTKERR_A>;
#[doc = "BusFault on unstacking for a return from exception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl UNSTKERR_R {
    #[doc = "Get enumerated values variant"]
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
        *self == UNSTKERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == UNSTKERR_A::VALUE2
    }
}
#[doc = "Field `UNSTKERR` writer - BusFault on unstacking for a return from exception"]
pub type UNSTKERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, UNSTKERR_A, O>;
impl<'a, const O: u8> UNSTKERR_W<'a, O> {
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
}
#[doc = "Field `STKERR` reader - BusFault on stacking for exception entry"]
pub type STKERR_R = crate::BitReader<STKERR_A>;
#[doc = "BusFault on stacking for exception entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl STKERR_R {
    #[doc = "Get enumerated values variant"]
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
        *self == STKERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STKERR_A::VALUE2
    }
}
#[doc = "Field `STKERR` writer - BusFault on stacking for exception entry"]
pub type STKERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, STKERR_A, O>;
impl<'a, const O: u8> STKERR_W<'a, O> {
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
}
#[doc = "Field `LSPERR` reader - BusFault during floating point lazy state preservation"]
pub type LSPERR_R = crate::BitReader<LSPERR_A>;
#[doc = "BusFault during floating point lazy state preservation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl LSPERR_R {
    #[doc = "Get enumerated values variant"]
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
        *self == LSPERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LSPERR_A::VALUE2
    }
}
#[doc = "Field `LSPERR` writer - BusFault during floating point lazy state preservation"]
pub type LSPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, LSPERR_A, O>;
impl<'a, const O: u8> LSPERR_W<'a, O> {
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
}
#[doc = "Field `BFARVALID` reader - BusFault Address Register (BFAR) valid flag"]
pub type BFARVALID_R = crate::BitReader<BFARVALID_A>;
#[doc = "BusFault Address Register (BFAR) valid flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl BFARVALID_R {
    #[doc = "Get enumerated values variant"]
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
        *self == BFARVALID_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFARVALID_A::VALUE2
    }
}
#[doc = "Field `BFARVALID` writer - BusFault Address Register (BFAR) valid flag"]
pub type BFARVALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, BFARVALID_A, O>;
impl<'a, const O: u8> BFARVALID_W<'a, O> {
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
}
#[doc = "Field `UNDEFINSTR` reader - Undefined instruction UsageFault"]
pub type UNDEFINSTR_R = crate::BitReader<UNDEFINSTR_A>;
#[doc = "Undefined instruction UsageFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl UNDEFINSTR_R {
    #[doc = "Get enumerated values variant"]
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
        *self == UNDEFINSTR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == UNDEFINSTR_A::VALUE2
    }
}
#[doc = "Field `UNDEFINSTR` writer - Undefined instruction UsageFault"]
pub type UNDEFINSTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, UNDEFINSTR_A, O>;
impl<'a, const O: u8> UNDEFINSTR_W<'a, O> {
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
}
#[doc = "Field `INVSTATE` reader - Invalid state UsageFault"]
pub type INVSTATE_R = crate::BitReader<INVSTATE_A>;
#[doc = "Invalid state UsageFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl INVSTATE_R {
    #[doc = "Get enumerated values variant"]
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
        *self == INVSTATE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INVSTATE_A::VALUE2
    }
}
#[doc = "Field `INVSTATE` writer - Invalid state UsageFault"]
pub type INVSTATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, INVSTATE_A, O>;
impl<'a, const O: u8> INVSTATE_W<'a, O> {
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
}
#[doc = "Field `INVPC` reader - Invalid PC load UsageFault"]
pub type INVPC_R = crate::BitReader<INVPC_A>;
#[doc = "Invalid PC load UsageFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl INVPC_R {
    #[doc = "Get enumerated values variant"]
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
        *self == INVPC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INVPC_A::VALUE2
    }
}
#[doc = "Field `INVPC` writer - Invalid PC load UsageFault"]
pub type INVPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, INVPC_A, O>;
impl<'a, const O: u8> INVPC_W<'a, O> {
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
}
#[doc = "Field `NOCP` reader - No coprocessor UsageFault"]
pub type NOCP_R = crate::BitReader<NOCP_A>;
#[doc = "No coprocessor UsageFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl NOCP_R {
    #[doc = "Get enumerated values variant"]
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
        *self == NOCP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NOCP_A::VALUE2
    }
}
#[doc = "Field `NOCP` writer - No coprocessor UsageFault"]
pub type NOCP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, NOCP_A, O>;
impl<'a, const O: u8> NOCP_W<'a, O> {
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
}
#[doc = "Field `UNALIGNED` reader - Unaligned access UsageFault"]
pub type UNALIGNED_R = crate::BitReader<UNALIGNED_A>;
#[doc = "Unaligned access UsageFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl UNALIGNED_R {
    #[doc = "Get enumerated values variant"]
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
        *self == UNALIGNED_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == UNALIGNED_A::VALUE2
    }
}
#[doc = "Field `UNALIGNED` writer - Unaligned access UsageFault"]
pub type UNALIGNED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, UNALIGNED_A, O>;
impl<'a, const O: u8> UNALIGNED_W<'a, O> {
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
}
#[doc = "Field `DIVBYZERO` reader - Divide by zero UsageFault"]
pub type DIVBYZERO_R = crate::BitReader<DIVBYZERO_A>;
#[doc = "Divide by zero UsageFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DIVBYZERO_R {
    #[doc = "Get enumerated values variant"]
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
        *self == DIVBYZERO_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIVBYZERO_A::VALUE2
    }
}
#[doc = "Field `DIVBYZERO` writer - Divide by zero UsageFault"]
pub type DIVBYZERO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, DIVBYZERO_A, O>;
impl<'a, const O: u8> DIVBYZERO_W<'a, O> {
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
}
impl R {
    #[doc = "Bit 0 - Instruction access violation flag"]
    #[inline(always)]
    pub fn iaccviol(&self) -> IACCVIOL_R {
        IACCVIOL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data access violation flag"]
    #[inline(always)]
    pub fn daccviol(&self) -> DACCVIOL_R {
        DACCVIOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - MemManage fault on unstacking for a return from exception"]
    #[inline(always)]
    pub fn munstkerr(&self) -> MUNSTKERR_R {
        MUNSTKERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MemManage fault on stacking for exception entry"]
    #[inline(always)]
    pub fn mstkerr(&self) -> MSTKERR_R {
        MSTKERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MemManage fault during floating point lazy state preservation"]
    #[inline(always)]
    pub fn mlsperr(&self) -> MLSPERR_R {
        MLSPERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - MemManage Fault Address Register (MMFAR) valid flag"]
    #[inline(always)]
    pub fn mmarvalid(&self) -> MMARVALID_R {
        MMARVALID_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Instruction bus error"]
    #[inline(always)]
    pub fn ibuserr(&self) -> IBUSERR_R {
        IBUSERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Precise data bus error"]
    #[inline(always)]
    pub fn preciserr(&self) -> PRECISERR_R {
        PRECISERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Imprecise data bus error"]
    #[inline(always)]
    pub fn impreciserr(&self) -> IMPRECISERR_R {
        IMPRECISERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BusFault on unstacking for a return from exception"]
    #[inline(always)]
    pub fn unstkerr(&self) -> UNSTKERR_R {
        UNSTKERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BusFault on stacking for exception entry"]
    #[inline(always)]
    pub fn stkerr(&self) -> STKERR_R {
        STKERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - BusFault during floating point lazy state preservation"]
    #[inline(always)]
    pub fn lsperr(&self) -> LSPERR_R {
        LSPERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - BusFault Address Register (BFAR) valid flag"]
    #[inline(always)]
    pub fn bfarvalid(&self) -> BFARVALID_R {
        BFARVALID_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Undefined instruction UsageFault"]
    #[inline(always)]
    pub fn undefinstr(&self) -> UNDEFINSTR_R {
        UNDEFINSTR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Invalid state UsageFault"]
    #[inline(always)]
    pub fn invstate(&self) -> INVSTATE_R {
        INVSTATE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Invalid PC load UsageFault"]
    #[inline(always)]
    pub fn invpc(&self) -> INVPC_R {
        INVPC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - No coprocessor UsageFault"]
    #[inline(always)]
    pub fn nocp(&self) -> NOCP_R {
        NOCP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Unaligned access UsageFault"]
    #[inline(always)]
    pub fn unaligned(&self) -> UNALIGNED_R {
        UNALIGNED_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Divide by zero UsageFault"]
    #[inline(always)]
    pub fn divbyzero(&self) -> DIVBYZERO_R {
        DIVBYZERO_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Instruction access violation flag"]
    #[inline(always)]
    #[must_use]
    pub fn iaccviol(&mut self) -> IACCVIOL_W<0> {
        IACCVIOL_W::new(self)
    }
    #[doc = "Bit 1 - Data access violation flag"]
    #[inline(always)]
    #[must_use]
    pub fn daccviol(&mut self) -> DACCVIOL_W<1> {
        DACCVIOL_W::new(self)
    }
    #[doc = "Bit 3 - MemManage fault on unstacking for a return from exception"]
    #[inline(always)]
    #[must_use]
    pub fn munstkerr(&mut self) -> MUNSTKERR_W<3> {
        MUNSTKERR_W::new(self)
    }
    #[doc = "Bit 4 - MemManage fault on stacking for exception entry"]
    #[inline(always)]
    #[must_use]
    pub fn mstkerr(&mut self) -> MSTKERR_W<4> {
        MSTKERR_W::new(self)
    }
    #[doc = "Bit 5 - MemManage fault during floating point lazy state preservation"]
    #[inline(always)]
    #[must_use]
    pub fn mlsperr(&mut self) -> MLSPERR_W<5> {
        MLSPERR_W::new(self)
    }
    #[doc = "Bit 7 - MemManage Fault Address Register (MMFAR) valid flag"]
    #[inline(always)]
    #[must_use]
    pub fn mmarvalid(&mut self) -> MMARVALID_W<7> {
        MMARVALID_W::new(self)
    }
    #[doc = "Bit 8 - Instruction bus error"]
    #[inline(always)]
    #[must_use]
    pub fn ibuserr(&mut self) -> IBUSERR_W<8> {
        IBUSERR_W::new(self)
    }
    #[doc = "Bit 9 - Precise data bus error"]
    #[inline(always)]
    #[must_use]
    pub fn preciserr(&mut self) -> PRECISERR_W<9> {
        PRECISERR_W::new(self)
    }
    #[doc = "Bit 10 - Imprecise data bus error"]
    #[inline(always)]
    #[must_use]
    pub fn impreciserr(&mut self) -> IMPRECISERR_W<10> {
        IMPRECISERR_W::new(self)
    }
    #[doc = "Bit 11 - BusFault on unstacking for a return from exception"]
    #[inline(always)]
    #[must_use]
    pub fn unstkerr(&mut self) -> UNSTKERR_W<11> {
        UNSTKERR_W::new(self)
    }
    #[doc = "Bit 12 - BusFault on stacking for exception entry"]
    #[inline(always)]
    #[must_use]
    pub fn stkerr(&mut self) -> STKERR_W<12> {
        STKERR_W::new(self)
    }
    #[doc = "Bit 13 - BusFault during floating point lazy state preservation"]
    #[inline(always)]
    #[must_use]
    pub fn lsperr(&mut self) -> LSPERR_W<13> {
        LSPERR_W::new(self)
    }
    #[doc = "Bit 15 - BusFault Address Register (BFAR) valid flag"]
    #[inline(always)]
    #[must_use]
    pub fn bfarvalid(&mut self) -> BFARVALID_W<15> {
        BFARVALID_W::new(self)
    }
    #[doc = "Bit 16 - Undefined instruction UsageFault"]
    #[inline(always)]
    #[must_use]
    pub fn undefinstr(&mut self) -> UNDEFINSTR_W<16> {
        UNDEFINSTR_W::new(self)
    }
    #[doc = "Bit 17 - Invalid state UsageFault"]
    #[inline(always)]
    #[must_use]
    pub fn invstate(&mut self) -> INVSTATE_W<17> {
        INVSTATE_W::new(self)
    }
    #[doc = "Bit 18 - Invalid PC load UsageFault"]
    #[inline(always)]
    #[must_use]
    pub fn invpc(&mut self) -> INVPC_W<18> {
        INVPC_W::new(self)
    }
    #[doc = "Bit 19 - No coprocessor UsageFault"]
    #[inline(always)]
    #[must_use]
    pub fn nocp(&mut self) -> NOCP_W<19> {
        NOCP_W::new(self)
    }
    #[doc = "Bit 24 - Unaligned access UsageFault"]
    #[inline(always)]
    #[must_use]
    pub fn unaligned(&mut self) -> UNALIGNED_W<24> {
        UNALIGNED_W::new(self)
    }
    #[doc = "Bit 25 - Divide by zero UsageFault"]
    #[inline(always)]
    #[must_use]
    pub fn divbyzero(&mut self) -> DIVBYZERO_W<25> {
        DIVBYZERO_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFSR to value 0"]
impl crate::Resettable for CFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
