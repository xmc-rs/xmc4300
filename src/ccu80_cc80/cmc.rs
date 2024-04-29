#[doc = "Register `CMC` reader"]
pub type R = crate::R<CMC_SPEC>;
#[doc = "Register `CMC` writer"]
pub type W = crate::W<CMC_SPEC>;
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
impl crate::FieldSpec for STRTS_A {
    type Ux = u8;
}
impl crate::IsEnum for STRTS_A {}
#[doc = "Field `STRTS` reader - External Start Functionality Selector"]
pub type STRTS_R = crate::FieldReader<STRTS_A>;
impl STRTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STRTS_A {
        match self.bits {
            0 => STRTS_A::VALUE1,
            1 => STRTS_A::VALUE2,
            2 => STRTS_A::VALUE3,
            3 => STRTS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "External Start Function deactivated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STRTS_A::VALUE1
    }
    #[doc = "External Start Function triggered by Event 0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STRTS_A::VALUE2
    }
    #[doc = "External Start Function triggered by Event 1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == STRTS_A::VALUE3
    }
    #[doc = "External Start Function triggered by Event 2"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == STRTS_A::VALUE4
    }
}
#[doc = "Field `STRTS` writer - External Start Functionality Selector"]
pub type STRTS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STRTS_A, crate::Safe>;
impl<'a, REG> STRTS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External Start Function deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(STRTS_A::VALUE1)
    }
    #[doc = "External Start Function triggered by Event 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(STRTS_A::VALUE2)
    }
    #[doc = "External Start Function triggered by Event 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(STRTS_A::VALUE3)
    }
    #[doc = "External Start Function triggered by Event 2"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(STRTS_A::VALUE4)
    }
}
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
impl crate::FieldSpec for ENDS_A {
    type Ux = u8;
}
impl crate::IsEnum for ENDS_A {}
#[doc = "Field `ENDS` reader - External Stop Functionality Selector"]
pub type ENDS_R = crate::FieldReader<ENDS_A>;
impl ENDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENDS_A {
        match self.bits {
            0 => ENDS_A::VALUE1,
            1 => ENDS_A::VALUE2,
            2 => ENDS_A::VALUE3,
            3 => ENDS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "External Stop Function deactivated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENDS_A::VALUE1
    }
    #[doc = "External Stop Function triggered by Event 0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENDS_A::VALUE2
    }
    #[doc = "External Stop Function triggered by Event 1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ENDS_A::VALUE3
    }
    #[doc = "External Stop Function triggered by Event 2"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ENDS_A::VALUE4
    }
}
#[doc = "Field `ENDS` writer - External Stop Functionality Selector"]
pub type ENDS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ENDS_A, crate::Safe>;
impl<'a, REG> ENDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External Stop Function deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ENDS_A::VALUE1)
    }
    #[doc = "External Stop Function triggered by Event 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ENDS_A::VALUE2)
    }
    #[doc = "External Stop Function triggered by Event 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(ENDS_A::VALUE3)
    }
    #[doc = "External Stop Function triggered by Event 2"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(ENDS_A::VALUE4)
    }
}
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
impl crate::FieldSpec for CAP0S_A {
    type Ux = u8;
}
impl crate::IsEnum for CAP0S_A {}
#[doc = "Field `CAP0S` reader - External Capture 0 Functionality Selector"]
pub type CAP0S_R = crate::FieldReader<CAP0S_A>;
impl CAP0S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAP0S_A {
        match self.bits {
            0 => CAP0S_A::VALUE1,
            1 => CAP0S_A::VALUE2,
            2 => CAP0S_A::VALUE3,
            3 => CAP0S_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "External Capture 0 Function deactivated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CAP0S_A::VALUE1
    }
    #[doc = "External Capture 0 Function triggered by Event 0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CAP0S_A::VALUE2
    }
    #[doc = "External Capture 0 Function triggered by Event 1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CAP0S_A::VALUE3
    }
    #[doc = "External Capture 0 Function triggered by Event 2"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CAP0S_A::VALUE4
    }
}
#[doc = "Field `CAP0S` writer - External Capture 0 Functionality Selector"]
pub type CAP0S_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CAP0S_A, crate::Safe>;
impl<'a, REG> CAP0S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External Capture 0 Function deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CAP0S_A::VALUE1)
    }
    #[doc = "External Capture 0 Function triggered by Event 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CAP0S_A::VALUE2)
    }
    #[doc = "External Capture 0 Function triggered by Event 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CAP0S_A::VALUE3)
    }
    #[doc = "External Capture 0 Function triggered by Event 2"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CAP0S_A::VALUE4)
    }
}
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
impl crate::FieldSpec for CAP1S_A {
    type Ux = u8;
}
impl crate::IsEnum for CAP1S_A {}
#[doc = "Field `CAP1S` reader - External Capture 1 Functionality Selector"]
pub type CAP1S_R = crate::FieldReader<CAP1S_A>;
impl CAP1S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAP1S_A {
        match self.bits {
            0 => CAP1S_A::VALUE1,
            1 => CAP1S_A::VALUE2,
            2 => CAP1S_A::VALUE3,
            3 => CAP1S_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "External Capture 1 Function deactivated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CAP1S_A::VALUE1
    }
    #[doc = "External Capture 1 Function triggered by Event 0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CAP1S_A::VALUE2
    }
    #[doc = "External Capture 1 Function triggered by Event 1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CAP1S_A::VALUE3
    }
    #[doc = "External Capture 1 Function triggered by Event 2"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CAP1S_A::VALUE4
    }
}
#[doc = "Field `CAP1S` writer - External Capture 1 Functionality Selector"]
pub type CAP1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CAP1S_A, crate::Safe>;
impl<'a, REG> CAP1S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External Capture 1 Function deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CAP1S_A::VALUE1)
    }
    #[doc = "External Capture 1 Function triggered by Event 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CAP1S_A::VALUE2)
    }
    #[doc = "External Capture 1 Function triggered by Event 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CAP1S_A::VALUE3)
    }
    #[doc = "External Capture 1 Function triggered by Event 2"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CAP1S_A::VALUE4)
    }
}
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
impl crate::FieldSpec for GATES_A {
    type Ux = u8;
}
impl crate::IsEnum for GATES_A {}
#[doc = "Field `GATES` reader - External Gate Functionality Selector"]
pub type GATES_R = crate::FieldReader<GATES_A>;
impl GATES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GATES_A {
        match self.bits {
            0 => GATES_A::VALUE1,
            1 => GATES_A::VALUE2,
            2 => GATES_A::VALUE3,
            3 => GATES_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "External Gating Function deactivated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GATES_A::VALUE1
    }
    #[doc = "External Gating Function triggered by Event 0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GATES_A::VALUE2
    }
    #[doc = "External Gating Function triggered by Event 1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == GATES_A::VALUE3
    }
    #[doc = "External Gating Function triggered by Event 2"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == GATES_A::VALUE4
    }
}
#[doc = "Field `GATES` writer - External Gate Functionality Selector"]
pub type GATES_W<'a, REG> = crate::FieldWriter<'a, REG, 2, GATES_A, crate::Safe>;
impl<'a, REG> GATES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External Gating Function deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(GATES_A::VALUE1)
    }
    #[doc = "External Gating Function triggered by Event 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(GATES_A::VALUE2)
    }
    #[doc = "External Gating Function triggered by Event 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(GATES_A::VALUE3)
    }
    #[doc = "External Gating Function triggered by Event 2"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(GATES_A::VALUE4)
    }
}
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
impl crate::FieldSpec for UDS_A {
    type Ux = u8;
}
impl crate::IsEnum for UDS_A {}
#[doc = "Field `UDS` reader - External Up/Down Functionality Selector"]
pub type UDS_R = crate::FieldReader<UDS_A>;
impl UDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UDS_A {
        match self.bits {
            0 => UDS_A::VALUE1,
            1 => UDS_A::VALUE2,
            2 => UDS_A::VALUE3,
            3 => UDS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "External Up/Down Function deactivated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == UDS_A::VALUE1
    }
    #[doc = "External Up/Down Function triggered by Event 0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == UDS_A::VALUE2
    }
    #[doc = "External Up/Down Function triggered by Event 1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == UDS_A::VALUE3
    }
    #[doc = "External Up/Down Function triggered by Event 2"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == UDS_A::VALUE4
    }
}
#[doc = "Field `UDS` writer - External Up/Down Functionality Selector"]
pub type UDS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UDS_A, crate::Safe>;
impl<'a, REG> UDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External Up/Down Function deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(UDS_A::VALUE1)
    }
    #[doc = "External Up/Down Function triggered by Event 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(UDS_A::VALUE2)
    }
    #[doc = "External Up/Down Function triggered by Event 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(UDS_A::VALUE3)
    }
    #[doc = "External Up/Down Function triggered by Event 2"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(UDS_A::VALUE4)
    }
}
#[doc = "Field `LDS` reader - External Timer Load Functionality Selector"]
pub type LDS_R = crate::FieldReader;
#[doc = "Field `LDS` writer - External Timer Load Functionality Selector"]
pub type LDS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
impl crate::FieldSpec for CNTS_A {
    type Ux = u8;
}
impl crate::IsEnum for CNTS_A {}
#[doc = "Field `CNTS` reader - External Count Selector"]
pub type CNTS_R = crate::FieldReader<CNTS_A>;
impl CNTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CNTS_A {
        match self.bits {
            0 => CNTS_A::VALUE1,
            1 => CNTS_A::VALUE2,
            2 => CNTS_A::VALUE3,
            3 => CNTS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "External Count Function deactivated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CNTS_A::VALUE1
    }
    #[doc = "External Count Function triggered by Event 0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CNTS_A::VALUE2
    }
    #[doc = "External Count Function triggered by Event 1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CNTS_A::VALUE3
    }
    #[doc = "External Count Function triggered by Event 2"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CNTS_A::VALUE4
    }
}
#[doc = "Field `CNTS` writer - External Count Selector"]
pub type CNTS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CNTS_A, crate::Safe>;
impl<'a, REG> CNTS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External Count Function deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CNTS_A::VALUE1)
    }
    #[doc = "External Count Function triggered by Event 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CNTS_A::VALUE2)
    }
    #[doc = "External Count Function triggered by Event 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CNTS_A::VALUE3)
    }
    #[doc = "External Count Function triggered by Event 2"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CNTS_A::VALUE4)
    }
}
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
#[doc = "Field `OFS` reader - Override Function Selector"]
pub type OFS_R = crate::BitReader<OFS_A>;
impl OFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OFS_A {
        match self.bits {
            false => OFS_A::VALUE1,
            true => OFS_A::VALUE2,
        }
    }
    #[doc = "Override functionality disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OFS_A::VALUE1
    }
    #[doc = "Status bit trigger override connected to Event 1; Status bit value override connected to Event 2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OFS_A::VALUE2
    }
}
#[doc = "Field `OFS` writer - Override Function Selector"]
pub type OFS_W<'a, REG> = crate::BitWriter<'a, REG, OFS_A>;
impl<'a, REG> OFS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override functionality disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(OFS_A::VALUE1)
    }
    #[doc = "Status bit trigger override connected to Event 1; Status bit value override connected to Event 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(OFS_A::VALUE2)
    }
}
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
#[doc = "Field `TS` reader - Trap Function Selector"]
pub type TS_R = crate::BitReader<TS_A>;
impl TS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS_A {
        match self.bits {
            false => TS_A::VALUE1,
            true => TS_A::VALUE2,
        }
    }
    #[doc = "Trap function disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TS_A::VALUE1
    }
    #[doc = "TRAP function connected to Event 2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TS_A::VALUE2
    }
}
#[doc = "Field `TS` writer - Trap Function Selector"]
pub type TS_W<'a, REG> = crate::BitWriter<'a, REG, TS_A>;
impl<'a, REG> TS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trap function disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TS_A::VALUE1)
    }
    #[doc = "TRAP function connected to Event 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TS_A::VALUE2)
    }
}
#[doc = "Field `MOS` reader - External Modulation Functionality Selector"]
pub type MOS_R = crate::FieldReader;
#[doc = "Field `MOS` writer - External Modulation Functionality Selector"]
pub type MOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
#[doc = "Field `TCE` reader - Timer Concatenation Enable"]
pub type TCE_R = crate::BitReader<TCE_A>;
impl TCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCE_A {
        match self.bits {
            false => TCE_A::VALUE1,
            true => TCE_A::VALUE2,
        }
    }
    #[doc = "Timer concatenation is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TCE_A::VALUE1
    }
    #[doc = "Timer concatenation is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TCE_A::VALUE2
    }
}
#[doc = "Field `TCE` writer - Timer Concatenation Enable"]
pub type TCE_W<'a, REG> = crate::BitWriter<'a, REG, TCE_A>;
impl<'a, REG> TCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer concatenation is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TCE_A::VALUE1)
    }
    #[doc = "Timer concatenation is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
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
    pub fn strts(&mut self) -> STRTS_W<CMC_SPEC> {
        STRTS_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - External Stop Functionality Selector"]
    #[inline(always)]
    #[must_use]
    pub fn ends(&mut self) -> ENDS_W<CMC_SPEC> {
        ENDS_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - External Capture 0 Functionality Selector"]
    #[inline(always)]
    #[must_use]
    pub fn cap0s(&mut self) -> CAP0S_W<CMC_SPEC> {
        CAP0S_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - External Capture 1 Functionality Selector"]
    #[inline(always)]
    #[must_use]
    pub fn cap1s(&mut self) -> CAP1S_W<CMC_SPEC> {
        CAP1S_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - External Gate Functionality Selector"]
    #[inline(always)]
    #[must_use]
    pub fn gates(&mut self) -> GATES_W<CMC_SPEC> {
        GATES_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - External Up/Down Functionality Selector"]
    #[inline(always)]
    #[must_use]
    pub fn uds(&mut self) -> UDS_W<CMC_SPEC> {
        UDS_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - External Timer Load Functionality Selector"]
    #[inline(always)]
    #[must_use]
    pub fn lds(&mut self) -> LDS_W<CMC_SPEC> {
        LDS_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - External Count Selector"]
    #[inline(always)]
    #[must_use]
    pub fn cnts(&mut self) -> CNTS_W<CMC_SPEC> {
        CNTS_W::new(self, 14)
    }
    #[doc = "Bit 16 - Override Function Selector"]
    #[inline(always)]
    #[must_use]
    pub fn ofs(&mut self) -> OFS_W<CMC_SPEC> {
        OFS_W::new(self, 16)
    }
    #[doc = "Bit 17 - Trap Function Selector"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<CMC_SPEC> {
        TS_W::new(self, 17)
    }
    #[doc = "Bits 18:19 - External Modulation Functionality Selector"]
    #[inline(always)]
    #[must_use]
    pub fn mos(&mut self) -> MOS_W<CMC_SPEC> {
        MOS_W::new(self, 18)
    }
    #[doc = "Bit 20 - Timer Concatenation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tce(&mut self) -> TCE_W<CMC_SPEC> {
        TCE_W::new(self, 20)
    }
}
#[doc = "Connection Matrix Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMC_SPEC;
impl crate::RegisterSpec for CMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmc::R`](R) reader structure"]
impl crate::Readable for CMC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmc::W`](W) writer structure"]
impl crate::Writable for CMC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMC to value 0"]
impl crate::Resettable for CMC_SPEC {
    const RESET_VALUE: u32 = 0;
}
