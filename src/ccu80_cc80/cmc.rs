#[doc = "Register `CMC` reader"]
pub struct R(crate::R<CMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMC` writer"]
pub struct W(crate::W<CMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMC_SPEC>;
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
impl From<crate::W<CMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STRTS` reader - External Start Functionality Selector"]
pub type STRTS_R = crate::FieldReader<u8, STRTS_A>;
#[doc = "External Start Functionality Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STRTS_A {
    #[doc = "0: External Start Function deactivated"]
    VALUE1 = 0,
    #[doc = "1: External Start Function triggered by Event 0"]
    VALUE2 = 1,
    #[doc = "2: External Start Function triggered by Event 1"]
    VALUE3 = 2,
    #[doc = "3: External Start Function triggered by Event 2"]
    VALUE4 = 3,
}
impl From<STRTS_A> for u8 {
    #[inline(always)]
    fn from(variant: STRTS_A) -> Self {
        variant as _
    }
}
impl STRTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STRTS_A {
        match self.bits {
            0 => STRTS_A::VALUE1,
            1 => STRTS_A::VALUE2,
            2 => STRTS_A::VALUE3,
            3 => STRTS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STRTS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STRTS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == STRTS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == STRTS_A::VALUE4
    }
}
#[doc = "Field `STRTS` writer - External Start Functionality Selector"]
pub type STRTS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CMC_SPEC, u8, STRTS_A, 2, O>;
impl<'a, const O: u8> STRTS_W<'a, O> {
    #[doc = "External Start Function deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STRTS_A::VALUE1)
    }
    #[doc = "External Start Function triggered by Event 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STRTS_A::VALUE2)
    }
    #[doc = "External Start Function triggered by Event 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(STRTS_A::VALUE3)
    }
    #[doc = "External Start Function triggered by Event 2"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(STRTS_A::VALUE4)
    }
}
#[doc = "Field `ENDS` reader - External Stop Functionality Selector"]
pub type ENDS_R = crate::FieldReader<u8, ENDS_A>;
#[doc = "External Stop Functionality Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENDS_A {
    #[doc = "0: External Stop Function deactivated"]
    VALUE1 = 0,
    #[doc = "1: External Stop Function triggered by Event 0"]
    VALUE2 = 1,
    #[doc = "2: External Stop Function triggered by Event 1"]
    VALUE3 = 2,
    #[doc = "3: External Stop Function triggered by Event 2"]
    VALUE4 = 3,
}
impl From<ENDS_A> for u8 {
    #[inline(always)]
    fn from(variant: ENDS_A) -> Self {
        variant as _
    }
}
impl ENDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDS_A {
        match self.bits {
            0 => ENDS_A::VALUE1,
            1 => ENDS_A::VALUE2,
            2 => ENDS_A::VALUE3,
            3 => ENDS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENDS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENDS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ENDS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ENDS_A::VALUE4
    }
}
#[doc = "Field `ENDS` writer - External Stop Functionality Selector"]
pub type ENDS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CMC_SPEC, u8, ENDS_A, 2, O>;
impl<'a, const O: u8> ENDS_W<'a, O> {
    #[doc = "External Stop Function deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENDS_A::VALUE1)
    }
    #[doc = "External Stop Function triggered by Event 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENDS_A::VALUE2)
    }
    #[doc = "External Stop Function triggered by Event 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ENDS_A::VALUE3)
    }
    #[doc = "External Stop Function triggered by Event 2"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ENDS_A::VALUE4)
    }
}
#[doc = "Field `CAP0S` reader - External Capture 0 Functionality Selector"]
pub type CAP0S_R = crate::FieldReader<u8, CAP0S_A>;
#[doc = "External Capture 0 Functionality Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAP0S_A {
    #[doc = "0: External Capture 0 Function deactivated"]
    VALUE1 = 0,
    #[doc = "1: External Capture 0 Function triggered by Event 0"]
    VALUE2 = 1,
    #[doc = "2: External Capture 0 Function triggered by Event 1"]
    VALUE3 = 2,
    #[doc = "3: External Capture 0 Function triggered by Event 2"]
    VALUE4 = 3,
}
impl From<CAP0S_A> for u8 {
    #[inline(always)]
    fn from(variant: CAP0S_A) -> Self {
        variant as _
    }
}
impl CAP0S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP0S_A {
        match self.bits {
            0 => CAP0S_A::VALUE1,
            1 => CAP0S_A::VALUE2,
            2 => CAP0S_A::VALUE3,
            3 => CAP0S_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CAP0S_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CAP0S_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CAP0S_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CAP0S_A::VALUE4
    }
}
#[doc = "Field `CAP0S` writer - External Capture 0 Functionality Selector"]
pub type CAP0S_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CMC_SPEC, u8, CAP0S_A, 2, O>;
impl<'a, const O: u8> CAP0S_W<'a, O> {
    #[doc = "External Capture 0 Function deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CAP0S_A::VALUE1)
    }
    #[doc = "External Capture 0 Function triggered by Event 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CAP0S_A::VALUE2)
    }
    #[doc = "External Capture 0 Function triggered by Event 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CAP0S_A::VALUE3)
    }
    #[doc = "External Capture 0 Function triggered by Event 2"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CAP0S_A::VALUE4)
    }
}
#[doc = "Field `CAP1S` reader - External Capture 1 Functionality Selector"]
pub type CAP1S_R = crate::FieldReader<u8, CAP1S_A>;
#[doc = "External Capture 1 Functionality Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAP1S_A {
    #[doc = "0: External Capture 1 Function deactivated"]
    VALUE1 = 0,
    #[doc = "1: External Capture 1 Function triggered by Event 0"]
    VALUE2 = 1,
    #[doc = "2: External Capture 1 Function triggered by Event 1"]
    VALUE3 = 2,
    #[doc = "3: External Capture 1 Function triggered by Event 2"]
    VALUE4 = 3,
}
impl From<CAP1S_A> for u8 {
    #[inline(always)]
    fn from(variant: CAP1S_A) -> Self {
        variant as _
    }
}
impl CAP1S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP1S_A {
        match self.bits {
            0 => CAP1S_A::VALUE1,
            1 => CAP1S_A::VALUE2,
            2 => CAP1S_A::VALUE3,
            3 => CAP1S_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CAP1S_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CAP1S_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CAP1S_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CAP1S_A::VALUE4
    }
}
#[doc = "Field `CAP1S` writer - External Capture 1 Functionality Selector"]
pub type CAP1S_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CMC_SPEC, u8, CAP1S_A, 2, O>;
impl<'a, const O: u8> CAP1S_W<'a, O> {
    #[doc = "External Capture 1 Function deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CAP1S_A::VALUE1)
    }
    #[doc = "External Capture 1 Function triggered by Event 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CAP1S_A::VALUE2)
    }
    #[doc = "External Capture 1 Function triggered by Event 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CAP1S_A::VALUE3)
    }
    #[doc = "External Capture 1 Function triggered by Event 2"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CAP1S_A::VALUE4)
    }
}
#[doc = "Field `GATES` reader - External Gate Functionality Selector"]
pub type GATES_R = crate::FieldReader<u8, GATES_A>;
#[doc = "External Gate Functionality Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GATES_A {
    #[doc = "0: External Gating Function deactivated"]
    VALUE1 = 0,
    #[doc = "1: External Gating Function triggered by Event 0"]
    VALUE2 = 1,
    #[doc = "2: External Gating Function triggered by Event 1"]
    VALUE3 = 2,
    #[doc = "3: External Gating Function triggered by Event 2"]
    VALUE4 = 3,
}
impl From<GATES_A> for u8 {
    #[inline(always)]
    fn from(variant: GATES_A) -> Self {
        variant as _
    }
}
impl GATES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GATES_A {
        match self.bits {
            0 => GATES_A::VALUE1,
            1 => GATES_A::VALUE2,
            2 => GATES_A::VALUE3,
            3 => GATES_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GATES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GATES_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == GATES_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == GATES_A::VALUE4
    }
}
#[doc = "Field `GATES` writer - External Gate Functionality Selector"]
pub type GATES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CMC_SPEC, u8, GATES_A, 2, O>;
impl<'a, const O: u8> GATES_W<'a, O> {
    #[doc = "External Gating Function deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GATES_A::VALUE1)
    }
    #[doc = "External Gating Function triggered by Event 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GATES_A::VALUE2)
    }
    #[doc = "External Gating Function triggered by Event 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(GATES_A::VALUE3)
    }
    #[doc = "External Gating Function triggered by Event 2"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(GATES_A::VALUE4)
    }
}
#[doc = "Field `UDS` reader - External Up/Down Functionality Selector"]
pub type UDS_R = crate::FieldReader<u8, UDS_A>;
#[doc = "External Up/Down Functionality Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UDS_A {
    #[doc = "0: External Up/Down Function deactivated"]
    VALUE1 = 0,
    #[doc = "1: External Up/Down Function triggered by Event 0"]
    VALUE2 = 1,
    #[doc = "2: External Up/Down Function triggered by Event 1"]
    VALUE3 = 2,
    #[doc = "3: External Up/Down Function triggered by Event 2"]
    VALUE4 = 3,
}
impl From<UDS_A> for u8 {
    #[inline(always)]
    fn from(variant: UDS_A) -> Self {
        variant as _
    }
}
impl UDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UDS_A {
        match self.bits {
            0 => UDS_A::VALUE1,
            1 => UDS_A::VALUE2,
            2 => UDS_A::VALUE3,
            3 => UDS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == UDS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == UDS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == UDS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == UDS_A::VALUE4
    }
}
#[doc = "Field `UDS` writer - External Up/Down Functionality Selector"]
pub type UDS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CMC_SPEC, u8, UDS_A, 2, O>;
impl<'a, const O: u8> UDS_W<'a, O> {
    #[doc = "External Up/Down Function deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(UDS_A::VALUE1)
    }
    #[doc = "External Up/Down Function triggered by Event 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(UDS_A::VALUE2)
    }
    #[doc = "External Up/Down Function triggered by Event 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(UDS_A::VALUE3)
    }
    #[doc = "External Up/Down Function triggered by Event 2"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(UDS_A::VALUE4)
    }
}
#[doc = "Field `LDS` reader - External Timer Load Functionality Selector"]
pub type LDS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LDS` writer - External Timer Load Functionality Selector"]
pub type LDS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMC_SPEC, u8, u8, 2, O>;
#[doc = "Field `CNTS` reader - External Count Selector"]
pub type CNTS_R = crate::FieldReader<u8, CNTS_A>;
#[doc = "External Count Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CNTS_A {
    #[doc = "0: External Count Function deactivated"]
    VALUE1 = 0,
    #[doc = "1: External Count Function triggered by Event 0"]
    VALUE2 = 1,
    #[doc = "2: External Count Function triggered by Event 1"]
    VALUE3 = 2,
    #[doc = "3: External Count Function triggered by Event 2"]
    VALUE4 = 3,
}
impl From<CNTS_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTS_A) -> Self {
        variant as _
    }
}
impl CNTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTS_A {
        match self.bits {
            0 => CNTS_A::VALUE1,
            1 => CNTS_A::VALUE2,
            2 => CNTS_A::VALUE3,
            3 => CNTS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CNTS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CNTS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CNTS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CNTS_A::VALUE4
    }
}
#[doc = "Field `CNTS` writer - External Count Selector"]
pub type CNTS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CMC_SPEC, u8, CNTS_A, 2, O>;
impl<'a, const O: u8> CNTS_W<'a, O> {
    #[doc = "External Count Function deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CNTS_A::VALUE1)
    }
    #[doc = "External Count Function triggered by Event 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CNTS_A::VALUE2)
    }
    #[doc = "External Count Function triggered by Event 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CNTS_A::VALUE3)
    }
    #[doc = "External Count Function triggered by Event 2"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CNTS_A::VALUE4)
    }
}
#[doc = "Field `OFS` reader - Override Function Selector"]
pub type OFS_R = crate::BitReader<OFS_A>;
#[doc = "Override Function Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFS_A {
    #[doc = "0: Override functionality disabled"]
    VALUE1 = 0,
    #[doc = "1: Status bit trigger override connected to Event 1; Status bit value override connected to Event 2"]
    VALUE2 = 1,
}
impl From<OFS_A> for bool {
    #[inline(always)]
    fn from(variant: OFS_A) -> Self {
        variant as u8 != 0
    }
}
impl OFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFS_A {
        match self.bits {
            false => OFS_A::VALUE1,
            true => OFS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OFS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OFS_A::VALUE2
    }
}
#[doc = "Field `OFS` writer - Override Function Selector"]
pub type OFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMC_SPEC, OFS_A, O>;
impl<'a, const O: u8> OFS_W<'a, O> {
    #[doc = "Override functionality disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OFS_A::VALUE1)
    }
    #[doc = "Status bit trigger override connected to Event 1; Status bit value override connected to Event 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(OFS_A::VALUE2)
    }
}
#[doc = "Field `TS` reader - Trap Function Selector"]
pub type TS_R = crate::BitReader<TS_A>;
#[doc = "Trap Function Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS_A {
    #[doc = "0: Trap function disabled"]
    VALUE1 = 0,
    #[doc = "1: TRAP function connected to Event 2"]
    VALUE2 = 1,
}
impl From<TS_A> for bool {
    #[inline(always)]
    fn from(variant: TS_A) -> Self {
        variant as u8 != 0
    }
}
impl TS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TS_A {
        match self.bits {
            false => TS_A::VALUE1,
            true => TS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TS_A::VALUE2
    }
}
#[doc = "Field `TS` writer - Trap Function Selector"]
pub type TS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMC_SPEC, TS_A, O>;
impl<'a, const O: u8> TS_W<'a, O> {
    #[doc = "Trap function disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TS_A::VALUE1)
    }
    #[doc = "TRAP function connected to Event 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TS_A::VALUE2)
    }
}
#[doc = "Field `MOS` reader - External Modulation Functionality Selector"]
pub type MOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MOS` writer - External Modulation Functionality Selector"]
pub type MOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMC_SPEC, u8, u8, 2, O>;
#[doc = "Field `TCE` reader - Timer Concatenation Enable"]
pub type TCE_R = crate::BitReader<TCE_A>;
#[doc = "Timer Concatenation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCE_A {
    #[doc = "0: Timer concatenation is disabled"]
    VALUE1 = 0,
    #[doc = "1: Timer concatenation is enabled"]
    VALUE2 = 1,
}
impl From<TCE_A> for bool {
    #[inline(always)]
    fn from(variant: TCE_A) -> Self {
        variant as u8 != 0
    }
}
impl TCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCE_A {
        match self.bits {
            false => TCE_A::VALUE1,
            true => TCE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TCE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TCE_A::VALUE2
    }
}
#[doc = "Field `TCE` writer - Timer Concatenation Enable"]
pub type TCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMC_SPEC, TCE_A, O>;
impl<'a, const O: u8> TCE_W<'a, O> {
    #[doc = "Timer concatenation is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TCE_A::VALUE1)
    }
    #[doc = "Timer concatenation is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TCE_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:1 - External Start Functionality Selector"]
    #[inline(always)]
    pub fn strts(&self) -> STRTS_R {
        STRTS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - External Stop Functionality Selector"]
    #[inline(always)]
    pub fn ends(&self) -> ENDS_R {
        ENDS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - External Capture 0 Functionality Selector"]
    #[inline(always)]
    pub fn cap0s(&self) -> CAP0S_R {
        CAP0S_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - External Capture 1 Functionality Selector"]
    #[inline(always)]
    pub fn cap1s(&self) -> CAP1S_R {
        CAP1S_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - External Gate Functionality Selector"]
    #[inline(always)]
    pub fn gates(&self) -> GATES_R {
        GATES_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - External Up/Down Functionality Selector"]
    #[inline(always)]
    pub fn uds(&self) -> UDS_R {
        UDS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - External Timer Load Functionality Selector"]
    #[inline(always)]
    pub fn lds(&self) -> LDS_R {
        LDS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - External Count Selector"]
    #[inline(always)]
    pub fn cnts(&self) -> CNTS_R {
        CNTS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Override Function Selector"]
    #[inline(always)]
    pub fn ofs(&self) -> OFS_R {
        OFS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Trap Function Selector"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - External Modulation Functionality Selector"]
    #[inline(always)]
    pub fn mos(&self) -> MOS_R {
        MOS_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Timer Concatenation Enable"]
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Start Functionality Selector"]
    #[inline(always)]
    #[must_use]
    pub fn strts(&mut self) -> STRTS_W<0> {
        STRTS_W::new(self)
    }
    #[doc = "Bits 2:3 - External Stop Functionality Selector"]
    #[inline(always)]
    #[must_use]
    pub fn ends(&mut self) -> ENDS_W<2> {
        ENDS_W::new(self)
    }
    #[doc = "Bits 4:5 - External Capture 0 Functionality Selector"]
    #[inline(always)]
    #[must_use]
    pub fn cap0s(&mut self) -> CAP0S_W<4> {
        CAP0S_W::new(self)
    }
    #[doc = "Bits 6:7 - External Capture 1 Functionality Selector"]
    #[inline(always)]
    #[must_use]
    pub fn cap1s(&mut self) -> CAP1S_W<6> {
        CAP1S_W::new(self)
    }
    #[doc = "Bits 8:9 - External Gate Functionality Selector"]
    #[inline(always)]
    #[must_use]
    pub fn gates(&mut self) -> GATES_W<8> {
        GATES_W::new(self)
    }
    #[doc = "Bits 10:11 - External Up/Down Functionality Selector"]
    #[inline(always)]
    #[must_use]
    pub fn uds(&mut self) -> UDS_W<10> {
        UDS_W::new(self)
    }
    #[doc = "Bits 12:13 - External Timer Load Functionality Selector"]
    #[inline(always)]
    #[must_use]
    pub fn lds(&mut self) -> LDS_W<12> {
        LDS_W::new(self)
    }
    #[doc = "Bits 14:15 - External Count Selector"]
    #[inline(always)]
    #[must_use]
    pub fn cnts(&mut self) -> CNTS_W<14> {
        CNTS_W::new(self)
    }
    #[doc = "Bit 16 - Override Function Selector"]
    #[inline(always)]
    #[must_use]
    pub fn ofs(&mut self) -> OFS_W<16> {
        OFS_W::new(self)
    }
    #[doc = "Bit 17 - Trap Function Selector"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<17> {
        TS_W::new(self)
    }
    #[doc = "Bits 18:19 - External Modulation Functionality Selector"]
    #[inline(always)]
    #[must_use]
    pub fn mos(&mut self) -> MOS_W<18> {
        MOS_W::new(self)
    }
    #[doc = "Bit 20 - Timer Concatenation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tce(&mut self) -> TCE_W<20> {
        TCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection Matrix Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmc](index.html) module"]
pub struct CMC_SPEC;
impl crate::RegisterSpec for CMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmc::R](R) reader structure"]
impl crate::Readable for CMC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmc::W](W) writer structure"]
impl crate::Writable for CMC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMC to value 0"]
impl crate::Resettable for CMC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
