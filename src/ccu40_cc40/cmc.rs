#[doc = "Register `CMC` reader"]
pub type R = crate::R<CmcSpec>;
#[doc = "Register `CMC` writer"]
pub type W = crate::W<CmcSpec>;
#[doc = "External Start Functionality Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Strts {
    #[doc = "0: External Start Function deactivated"]
    Value1 = 0,
    #[doc = "1: External Start Function triggered by Event 0"]
    Value2 = 1,
    #[doc = "2: External Start Function triggered by Event 1"]
    Value3 = 2,
    #[doc = "3: External Start Function triggered by Event 2"]
    Value4 = 3,
}
impl From<Strts> for u8 {
    #[inline(always)]
    fn from(variant: Strts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Strts {
    type Ux = u8;
}
#[doc = "Field `STRTS` reader - External Start Functionality Selector"]
pub type StrtsR = crate::FieldReader<Strts>;
impl StrtsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Strts {
        match self.bits {
            0 => Strts::Value1,
            1 => Strts::Value2,
            2 => Strts::Value3,
            3 => Strts::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "External Start Function deactivated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Strts::Value1
    }
    #[doc = "External Start Function triggered by Event 0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Strts::Value2
    }
    #[doc = "External Start Function triggered by Event 1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Strts::Value3
    }
    #[doc = "External Start Function triggered by Event 2"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Strts::Value4
    }
}
#[doc = "Field `STRTS` writer - External Start Functionality Selector"]
pub type StrtsW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Strts>;
impl<'a, REG> StrtsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External Start Function deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Strts::Value1)
    }
    #[doc = "External Start Function triggered by Event 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Strts::Value2)
    }
    #[doc = "External Start Function triggered by Event 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Strts::Value3)
    }
    #[doc = "External Start Function triggered by Event 2"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Strts::Value4)
    }
}
#[doc = "External Stop Functionality Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ends {
    #[doc = "0: External Stop Function deactivated"]
    Value1 = 0,
    #[doc = "1: External Stop Function triggered by Event 0"]
    Value2 = 1,
    #[doc = "2: External Stop Function triggered by Event 1"]
    Value3 = 2,
    #[doc = "3: External Stop Function triggered by Event 2"]
    Value4 = 3,
}
impl From<Ends> for u8 {
    #[inline(always)]
    fn from(variant: Ends) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ends {
    type Ux = u8;
}
#[doc = "Field `ENDS` reader - External Stop Functionality Selector"]
pub type EndsR = crate::FieldReader<Ends>;
impl EndsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ends {
        match self.bits {
            0 => Ends::Value1,
            1 => Ends::Value2,
            2 => Ends::Value3,
            3 => Ends::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "External Stop Function deactivated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ends::Value1
    }
    #[doc = "External Stop Function triggered by Event 0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ends::Value2
    }
    #[doc = "External Stop Function triggered by Event 1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Ends::Value3
    }
    #[doc = "External Stop Function triggered by Event 2"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Ends::Value4
    }
}
#[doc = "Field `ENDS` writer - External Stop Functionality Selector"]
pub type EndsW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Ends>;
impl<'a, REG> EndsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External Stop Function deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ends::Value1)
    }
    #[doc = "External Stop Function triggered by Event 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ends::Value2)
    }
    #[doc = "External Stop Function triggered by Event 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Ends::Value3)
    }
    #[doc = "External Stop Function triggered by Event 2"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Ends::Value4)
    }
}
#[doc = "External Capture 0 Functionality Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cap0s {
    #[doc = "0: External Capture 0 Function deactivated"]
    Value1 = 0,
    #[doc = "1: External Capture 0 Function triggered by Event 0"]
    Value2 = 1,
    #[doc = "2: External Capture 0 Function triggered by Event 1"]
    Value3 = 2,
    #[doc = "3: External Capture 0 Function triggered by Event 2"]
    Value4 = 3,
}
impl From<Cap0s> for u8 {
    #[inline(always)]
    fn from(variant: Cap0s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cap0s {
    type Ux = u8;
}
#[doc = "Field `CAP0S` reader - External Capture 0 Functionality Selector"]
pub type Cap0sR = crate::FieldReader<Cap0s>;
impl Cap0sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cap0s {
        match self.bits {
            0 => Cap0s::Value1,
            1 => Cap0s::Value2,
            2 => Cap0s::Value3,
            3 => Cap0s::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "External Capture 0 Function deactivated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cap0s::Value1
    }
    #[doc = "External Capture 0 Function triggered by Event 0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cap0s::Value2
    }
    #[doc = "External Capture 0 Function triggered by Event 1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cap0s::Value3
    }
    #[doc = "External Capture 0 Function triggered by Event 2"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cap0s::Value4
    }
}
#[doc = "Field `CAP0S` writer - External Capture 0 Functionality Selector"]
pub type Cap0sW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Cap0s>;
impl<'a, REG> Cap0sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External Capture 0 Function deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cap0s::Value1)
    }
    #[doc = "External Capture 0 Function triggered by Event 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cap0s::Value2)
    }
    #[doc = "External Capture 0 Function triggered by Event 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cap0s::Value3)
    }
    #[doc = "External Capture 0 Function triggered by Event 2"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cap0s::Value4)
    }
}
#[doc = "External Capture 1 Functionality Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cap1s {
    #[doc = "0: External Capture 1 Function deactivated"]
    Value1 = 0,
    #[doc = "1: External Capture 1 Function triggered by Event 0"]
    Value2 = 1,
    #[doc = "2: External Capture 1 Function triggered by Event 1"]
    Value3 = 2,
    #[doc = "3: External Capture 1 Function triggered by Event 2"]
    Value4 = 3,
}
impl From<Cap1s> for u8 {
    #[inline(always)]
    fn from(variant: Cap1s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cap1s {
    type Ux = u8;
}
#[doc = "Field `CAP1S` reader - External Capture 1 Functionality Selector"]
pub type Cap1sR = crate::FieldReader<Cap1s>;
impl Cap1sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cap1s {
        match self.bits {
            0 => Cap1s::Value1,
            1 => Cap1s::Value2,
            2 => Cap1s::Value3,
            3 => Cap1s::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "External Capture 1 Function deactivated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cap1s::Value1
    }
    #[doc = "External Capture 1 Function triggered by Event 0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cap1s::Value2
    }
    #[doc = "External Capture 1 Function triggered by Event 1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cap1s::Value3
    }
    #[doc = "External Capture 1 Function triggered by Event 2"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cap1s::Value4
    }
}
#[doc = "Field `CAP1S` writer - External Capture 1 Functionality Selector"]
pub type Cap1sW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Cap1s>;
impl<'a, REG> Cap1sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External Capture 1 Function deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cap1s::Value1)
    }
    #[doc = "External Capture 1 Function triggered by Event 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cap1s::Value2)
    }
    #[doc = "External Capture 1 Function triggered by Event 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cap1s::Value3)
    }
    #[doc = "External Capture 1 Function triggered by Event 2"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cap1s::Value4)
    }
}
#[doc = "External Gate Functionality Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gates {
    #[doc = "0: External Gating Function deactivated"]
    Value1 = 0,
    #[doc = "1: External Gating Function triggered by Event 0"]
    Value2 = 1,
    #[doc = "2: External Gating Function triggered by Event 1"]
    Value3 = 2,
    #[doc = "3: External Gating Function triggered by Event 2"]
    Value4 = 3,
}
impl From<Gates> for u8 {
    #[inline(always)]
    fn from(variant: Gates) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gates {
    type Ux = u8;
}
#[doc = "Field `GATES` reader - External Gate Functionality Selector"]
pub type GatesR = crate::FieldReader<Gates>;
impl GatesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gates {
        match self.bits {
            0 => Gates::Value1,
            1 => Gates::Value2,
            2 => Gates::Value3,
            3 => Gates::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "External Gating Function deactivated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Gates::Value1
    }
    #[doc = "External Gating Function triggered by Event 0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Gates::Value2
    }
    #[doc = "External Gating Function triggered by Event 1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Gates::Value3
    }
    #[doc = "External Gating Function triggered by Event 2"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Gates::Value4
    }
}
#[doc = "Field `GATES` writer - External Gate Functionality Selector"]
pub type GatesW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gates>;
impl<'a, REG> GatesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External Gating Function deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Gates::Value1)
    }
    #[doc = "External Gating Function triggered by Event 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Gates::Value2)
    }
    #[doc = "External Gating Function triggered by Event 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Gates::Value3)
    }
    #[doc = "External Gating Function triggered by Event 2"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Gates::Value4)
    }
}
#[doc = "External Up/Down Functionality Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uds {
    #[doc = "0: External Up/Down Function deactivated"]
    Value1 = 0,
    #[doc = "1: External Up/Down Function triggered by Event 0"]
    Value2 = 1,
    #[doc = "2: External Up/Down Function triggered by Event 1"]
    Value3 = 2,
    #[doc = "3: External Up/Down Function triggered by Event 2"]
    Value4 = 3,
}
impl From<Uds> for u8 {
    #[inline(always)]
    fn from(variant: Uds) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uds {
    type Ux = u8;
}
#[doc = "Field `UDS` reader - External Up/Down Functionality Selector"]
pub type UdsR = crate::FieldReader<Uds>;
impl UdsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uds {
        match self.bits {
            0 => Uds::Value1,
            1 => Uds::Value2,
            2 => Uds::Value3,
            3 => Uds::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "External Up/Down Function deactivated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Uds::Value1
    }
    #[doc = "External Up/Down Function triggered by Event 0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Uds::Value2
    }
    #[doc = "External Up/Down Function triggered by Event 1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Uds::Value3
    }
    #[doc = "External Up/Down Function triggered by Event 2"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Uds::Value4
    }
}
#[doc = "Field `UDS` writer - External Up/Down Functionality Selector"]
pub type UdsW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Uds>;
impl<'a, REG> UdsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External Up/Down Function deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Uds::Value1)
    }
    #[doc = "External Up/Down Function triggered by Event 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Uds::Value2)
    }
    #[doc = "External Up/Down Function triggered by Event 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Uds::Value3)
    }
    #[doc = "External Up/Down Function triggered by Event 2"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Uds::Value4)
    }
}
#[doc = "Field `LDS` reader - External Timer Load Functionality Selector"]
pub type LdsR = crate::FieldReader;
#[doc = "Field `LDS` writer - External Timer Load Functionality Selector"]
pub type LdsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "External Count Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cnts {
    #[doc = "0: External Count Function deactivated"]
    Value1 = 0,
    #[doc = "1: External Count Function triggered by Event 0"]
    Value2 = 1,
    #[doc = "2: External Count Function triggered by Event 1"]
    Value3 = 2,
    #[doc = "3: External Count Function triggered by Event 2"]
    Value4 = 3,
}
impl From<Cnts> for u8 {
    #[inline(always)]
    fn from(variant: Cnts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cnts {
    type Ux = u8;
}
#[doc = "Field `CNTS` reader - External Count Selector"]
pub type CntsR = crate::FieldReader<Cnts>;
impl CntsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cnts {
        match self.bits {
            0 => Cnts::Value1,
            1 => Cnts::Value2,
            2 => Cnts::Value3,
            3 => Cnts::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "External Count Function deactivated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cnts::Value1
    }
    #[doc = "External Count Function triggered by Event 0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cnts::Value2
    }
    #[doc = "External Count Function triggered by Event 1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cnts::Value3
    }
    #[doc = "External Count Function triggered by Event 2"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cnts::Value4
    }
}
#[doc = "Field `CNTS` writer - External Count Selector"]
pub type CntsW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Cnts>;
impl<'a, REG> CntsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External Count Function deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cnts::Value1)
    }
    #[doc = "External Count Function triggered by Event 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cnts::Value2)
    }
    #[doc = "External Count Function triggered by Event 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cnts::Value3)
    }
    #[doc = "External Count Function triggered by Event 2"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cnts::Value4)
    }
}
#[doc = "Override Function Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ofs {
    #[doc = "0: Override functionality disabled"]
    Value1 = 0,
    #[doc = "1: Status bit trigger override connected to Event 1; Status bit value override connected to Event 2"]
    Value2 = 1,
}
impl From<Ofs> for bool {
    #[inline(always)]
    fn from(variant: Ofs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OFS` reader - Override Function Selector"]
pub type OfsR = crate::BitReader<Ofs>;
impl OfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ofs {
        match self.bits {
            false => Ofs::Value1,
            true => Ofs::Value2,
        }
    }
    #[doc = "Override functionality disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ofs::Value1
    }
    #[doc = "Status bit trigger override connected to Event 1; Status bit value override connected to Event 2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ofs::Value2
    }
}
#[doc = "Field `OFS` writer - Override Function Selector"]
pub type OfsW<'a, REG> = crate::BitWriter<'a, REG, Ofs>;
impl<'a, REG> OfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override functionality disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ofs::Value1)
    }
    #[doc = "Status bit trigger override connected to Event 1; Status bit value override connected to Event 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ofs::Value2)
    }
}
#[doc = "Trap Function Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ts {
    #[doc = "0: Trap function disabled"]
    Value1 = 0,
    #[doc = "1: TRAP function connected to Event 2"]
    Value2 = 1,
}
impl From<Ts> for bool {
    #[inline(always)]
    fn from(variant: Ts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS` reader - Trap Function Selector"]
pub type TsR = crate::BitReader<Ts>;
impl TsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ts {
        match self.bits {
            false => Ts::Value1,
            true => Ts::Value2,
        }
    }
    #[doc = "Trap function disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ts::Value1
    }
    #[doc = "TRAP function connected to Event 2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ts::Value2
    }
}
#[doc = "Field `TS` writer - Trap Function Selector"]
pub type TsW<'a, REG> = crate::BitWriter<'a, REG, Ts>;
impl<'a, REG> TsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trap function disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ts::Value1)
    }
    #[doc = "TRAP function connected to Event 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ts::Value2)
    }
}
#[doc = "Field `MOS` reader - External Modulation Functionality Selector"]
pub type MosR = crate::FieldReader;
#[doc = "Field `MOS` writer - External Modulation Functionality Selector"]
pub type MosW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Timer Concatenation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tce {
    #[doc = "0: Timer concatenation is disabled"]
    Value1 = 0,
    #[doc = "1: Timer concatenation is enabled"]
    Value2 = 1,
}
impl From<Tce> for bool {
    #[inline(always)]
    fn from(variant: Tce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCE` reader - Timer Concatenation Enable"]
pub type TceR = crate::BitReader<Tce>;
impl TceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tce {
        match self.bits {
            false => Tce::Value1,
            true => Tce::Value2,
        }
    }
    #[doc = "Timer concatenation is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tce::Value1
    }
    #[doc = "Timer concatenation is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tce::Value2
    }
}
#[doc = "Field `TCE` writer - Timer Concatenation Enable"]
pub type TceW<'a, REG> = crate::BitWriter<'a, REG, Tce>;
impl<'a, REG> TceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer concatenation is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tce::Value1)
    }
    #[doc = "Timer concatenation is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tce::Value2)
    }
}
impl R {
    #[doc = "Bits 0:1 - External Start Functionality Selector"]
    #[inline(always)]
    pub fn strts(&self) -> StrtsR {
        StrtsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - External Stop Functionality Selector"]
    #[inline(always)]
    pub fn ends(&self) -> EndsR {
        EndsR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - External Capture 0 Functionality Selector"]
    #[inline(always)]
    pub fn cap0s(&self) -> Cap0sR {
        Cap0sR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - External Capture 1 Functionality Selector"]
    #[inline(always)]
    pub fn cap1s(&self) -> Cap1sR {
        Cap1sR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - External Gate Functionality Selector"]
    #[inline(always)]
    pub fn gates(&self) -> GatesR {
        GatesR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - External Up/Down Functionality Selector"]
    #[inline(always)]
    pub fn uds(&self) -> UdsR {
        UdsR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - External Timer Load Functionality Selector"]
    #[inline(always)]
    pub fn lds(&self) -> LdsR {
        LdsR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - External Count Selector"]
    #[inline(always)]
    pub fn cnts(&self) -> CntsR {
        CntsR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Override Function Selector"]
    #[inline(always)]
    pub fn ofs(&self) -> OfsR {
        OfsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Trap Function Selector"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - External Modulation Functionality Selector"]
    #[inline(always)]
    pub fn mos(&self) -> MosR {
        MosR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Timer Concatenation Enable"]
    #[inline(always)]
    pub fn tce(&self) -> TceR {
        TceR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Start Functionality Selector"]
    #[inline(always)]
    #[must_use]
    pub fn strts(&mut self) -> StrtsW<CmcSpec> {
        StrtsW::new(self, 0)
    }
    #[doc = "Bits 2:3 - External Stop Functionality Selector"]
    #[inline(always)]
    #[must_use]
    pub fn ends(&mut self) -> EndsW<CmcSpec> {
        EndsW::new(self, 2)
    }
    #[doc = "Bits 4:5 - External Capture 0 Functionality Selector"]
    #[inline(always)]
    #[must_use]
    pub fn cap0s(&mut self) -> Cap0sW<CmcSpec> {
        Cap0sW::new(self, 4)
    }
    #[doc = "Bits 6:7 - External Capture 1 Functionality Selector"]
    #[inline(always)]
    #[must_use]
    pub fn cap1s(&mut self) -> Cap1sW<CmcSpec> {
        Cap1sW::new(self, 6)
    }
    #[doc = "Bits 8:9 - External Gate Functionality Selector"]
    #[inline(always)]
    #[must_use]
    pub fn gates(&mut self) -> GatesW<CmcSpec> {
        GatesW::new(self, 8)
    }
    #[doc = "Bits 10:11 - External Up/Down Functionality Selector"]
    #[inline(always)]
    #[must_use]
    pub fn uds(&mut self) -> UdsW<CmcSpec> {
        UdsW::new(self, 10)
    }
    #[doc = "Bits 12:13 - External Timer Load Functionality Selector"]
    #[inline(always)]
    #[must_use]
    pub fn lds(&mut self) -> LdsW<CmcSpec> {
        LdsW::new(self, 12)
    }
    #[doc = "Bits 14:15 - External Count Selector"]
    #[inline(always)]
    #[must_use]
    pub fn cnts(&mut self) -> CntsW<CmcSpec> {
        CntsW::new(self, 14)
    }
    #[doc = "Bit 16 - Override Function Selector"]
    #[inline(always)]
    #[must_use]
    pub fn ofs(&mut self) -> OfsW<CmcSpec> {
        OfsW::new(self, 16)
    }
    #[doc = "Bit 17 - Trap Function Selector"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TsW<CmcSpec> {
        TsW::new(self, 17)
    }
    #[doc = "Bits 18:19 - External Modulation Functionality Selector"]
    #[inline(always)]
    #[must_use]
    pub fn mos(&mut self) -> MosW<CmcSpec> {
        MosW::new(self, 18)
    }
    #[doc = "Bit 20 - Timer Concatenation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tce(&mut self) -> TceW<CmcSpec> {
        TceW::new(self, 20)
    }
}
#[doc = "Connection Matrix Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmcSpec;
impl crate::RegisterSpec for CmcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmc::R`](R) reader structure"]
impl crate::Readable for CmcSpec {}
#[doc = "`write(|w| ..)` method takes [`cmc::W`](W) writer structure"]
impl crate::Writable for CmcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMC to value 0"]
impl crate::Resettable for CmcSpec {
    const RESET_VALUE: u32 = 0;
}
