#[doc = "Register `CFSR` reader"]
pub type R = crate::R<CfsrSpec>;
#[doc = "Register `CFSR` writer"]
pub type W = crate::W<CfsrSpec>;
#[doc = "Instruction access violation flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iaccviol {
    #[doc = "0: no instruction access violation fault"]
    Value1 = 0,
    #[doc = "1: the processor attempted an instruction fetch from a location that does not permit execution."]
    Value2 = 1,
}
impl From<Iaccviol> for bool {
    #[inline(always)]
    fn from(variant: Iaccviol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IACCVIOL` reader - Instruction access violation flag"]
pub type IaccviolR = crate::BitReader<Iaccviol>;
impl IaccviolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iaccviol {
        match self.bits {
            false => Iaccviol::Value1,
            true => Iaccviol::Value2,
        }
    }
    #[doc = "no instruction access violation fault"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Iaccviol::Value1
    }
    #[doc = "the processor attempted an instruction fetch from a location that does not permit execution."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Iaccviol::Value2
    }
}
#[doc = "Field `IACCVIOL` writer - Instruction access violation flag"]
pub type IaccviolW<'a, REG> = crate::BitWriter<'a, REG, Iaccviol>;
impl<'a, REG> IaccviolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no instruction access violation fault"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Iaccviol::Value1)
    }
    #[doc = "the processor attempted an instruction fetch from a location that does not permit execution."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Iaccviol::Value2)
    }
}
#[doc = "Data access violation flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Daccviol {
    #[doc = "0: no data access violation fault"]
    Value1 = 0,
    #[doc = "1: the processor attempted a load or store at a location that does not permit the operation."]
    Value2 = 1,
}
impl From<Daccviol> for bool {
    #[inline(always)]
    fn from(variant: Daccviol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACCVIOL` reader - Data access violation flag"]
pub type DaccviolR = crate::BitReader<Daccviol>;
impl DaccviolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Daccviol {
        match self.bits {
            false => Daccviol::Value1,
            true => Daccviol::Value2,
        }
    }
    #[doc = "no data access violation fault"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Daccviol::Value1
    }
    #[doc = "the processor attempted a load or store at a location that does not permit the operation."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Daccviol::Value2
    }
}
#[doc = "Field `DACCVIOL` writer - Data access violation flag"]
pub type DaccviolW<'a, REG> = crate::BitWriter<'a, REG, Daccviol>;
impl<'a, REG> DaccviolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no data access violation fault"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Daccviol::Value1)
    }
    #[doc = "the processor attempted a load or store at a location that does not permit the operation."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Daccviol::Value2)
    }
}
#[doc = "MemManage fault on unstacking for a return from exception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Munstkerr {
    #[doc = "0: no unstacking fault"]
    Value1 = 0,
    #[doc = "1: unstack for an exception return has caused one or more access violations."]
    Value2 = 1,
}
impl From<Munstkerr> for bool {
    #[inline(always)]
    fn from(variant: Munstkerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUNSTKERR` reader - MemManage fault on unstacking for a return from exception"]
pub type MunstkerrR = crate::BitReader<Munstkerr>;
impl MunstkerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Munstkerr {
        match self.bits {
            false => Munstkerr::Value1,
            true => Munstkerr::Value2,
        }
    }
    #[doc = "no unstacking fault"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Munstkerr::Value1
    }
    #[doc = "unstack for an exception return has caused one or more access violations."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Munstkerr::Value2
    }
}
#[doc = "Field `MUNSTKERR` writer - MemManage fault on unstacking for a return from exception"]
pub type MunstkerrW<'a, REG> = crate::BitWriter<'a, REG, Munstkerr>;
impl<'a, REG> MunstkerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no unstacking fault"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Munstkerr::Value1)
    }
    #[doc = "unstack for an exception return has caused one or more access violations."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Munstkerr::Value2)
    }
}
#[doc = "MemManage fault on stacking for exception entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstkerr {
    #[doc = "0: no stacking fault"]
    Value1 = 0,
    #[doc = "1: stacking for an exception entry has caused one or more access violations."]
    Value2 = 1,
}
impl From<Mstkerr> for bool {
    #[inline(always)]
    fn from(variant: Mstkerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTKERR` reader - MemManage fault on stacking for exception entry"]
pub type MstkerrR = crate::BitReader<Mstkerr>;
impl MstkerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstkerr {
        match self.bits {
            false => Mstkerr::Value1,
            true => Mstkerr::Value2,
        }
    }
    #[doc = "no stacking fault"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mstkerr::Value1
    }
    #[doc = "stacking for an exception entry has caused one or more access violations."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mstkerr::Value2
    }
}
#[doc = "Field `MSTKERR` writer - MemManage fault on stacking for exception entry"]
pub type MstkerrW<'a, REG> = crate::BitWriter<'a, REG, Mstkerr>;
impl<'a, REG> MstkerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no stacking fault"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstkerr::Value1)
    }
    #[doc = "stacking for an exception entry has caused one or more access violations."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mstkerr::Value2)
    }
}
#[doc = "MemManage fault during floating point lazy state preservation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mlsperr {
    #[doc = "0: No MemManage fault occurred during floating-point lazy state preservation"]
    Value1 = 0,
    #[doc = "1: A MemManage fault occurred during floating-point lazy state preservation"]
    Value2 = 1,
}
impl From<Mlsperr> for bool {
    #[inline(always)]
    fn from(variant: Mlsperr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MLSPERR` reader - MemManage fault during floating point lazy state preservation"]
pub type MlsperrR = crate::BitReader<Mlsperr>;
impl MlsperrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mlsperr {
        match self.bits {
            false => Mlsperr::Value1,
            true => Mlsperr::Value2,
        }
    }
    #[doc = "No MemManage fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mlsperr::Value1
    }
    #[doc = "A MemManage fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mlsperr::Value2
    }
}
#[doc = "Field `MLSPERR` writer - MemManage fault during floating point lazy state preservation"]
pub type MlsperrW<'a, REG> = crate::BitWriter<'a, REG, Mlsperr>;
impl<'a, REG> MlsperrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No MemManage fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mlsperr::Value1)
    }
    #[doc = "A MemManage fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mlsperr::Value2)
    }
}
#[doc = "MemManage Fault Address Register (MMFAR) valid flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mmarvalid {
    #[doc = "0: value in MMAR is not a valid fault address"]
    Value1 = 0,
    #[doc = "1: MMAR holds a valid fault address."]
    Value2 = 1,
}
impl From<Mmarvalid> for bool {
    #[inline(always)]
    fn from(variant: Mmarvalid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMARVALID` reader - MemManage Fault Address Register (MMFAR) valid flag"]
pub type MmarvalidR = crate::BitReader<Mmarvalid>;
impl MmarvalidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mmarvalid {
        match self.bits {
            false => Mmarvalid::Value1,
            true => Mmarvalid::Value2,
        }
    }
    #[doc = "value in MMAR is not a valid fault address"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mmarvalid::Value1
    }
    #[doc = "MMAR holds a valid fault address."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mmarvalid::Value2
    }
}
#[doc = "Field `MMARVALID` writer - MemManage Fault Address Register (MMFAR) valid flag"]
pub type MmarvalidW<'a, REG> = crate::BitWriter<'a, REG, Mmarvalid>;
impl<'a, REG> MmarvalidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "value in MMAR is not a valid fault address"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mmarvalid::Value1)
    }
    #[doc = "MMAR holds a valid fault address."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mmarvalid::Value2)
    }
}
#[doc = "Instruction bus error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ibuserr {
    #[doc = "0: no instruction bus error"]
    Value1 = 0,
    #[doc = "1: instruction bus error."]
    Value2 = 1,
}
impl From<Ibuserr> for bool {
    #[inline(always)]
    fn from(variant: Ibuserr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IBUSERR` reader - Instruction bus error"]
pub type IbuserrR = crate::BitReader<Ibuserr>;
impl IbuserrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ibuserr {
        match self.bits {
            false => Ibuserr::Value1,
            true => Ibuserr::Value2,
        }
    }
    #[doc = "no instruction bus error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ibuserr::Value1
    }
    #[doc = "instruction bus error."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ibuserr::Value2
    }
}
#[doc = "Field `IBUSERR` writer - Instruction bus error"]
pub type IbuserrW<'a, REG> = crate::BitWriter<'a, REG, Ibuserr>;
impl<'a, REG> IbuserrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no instruction bus error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ibuserr::Value1)
    }
    #[doc = "instruction bus error."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ibuserr::Value2)
    }
}
#[doc = "Precise data bus error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Preciserr {
    #[doc = "0: no precise data bus error"]
    Value1 = 0,
    #[doc = "1: a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault."]
    Value2 = 1,
}
impl From<Preciserr> for bool {
    #[inline(always)]
    fn from(variant: Preciserr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRECISERR` reader - Precise data bus error"]
pub type PreciserrR = crate::BitReader<Preciserr>;
impl PreciserrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Preciserr {
        match self.bits {
            false => Preciserr::Value1,
            true => Preciserr::Value2,
        }
    }
    #[doc = "no precise data bus error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Preciserr::Value1
    }
    #[doc = "a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Preciserr::Value2
    }
}
#[doc = "Field `PRECISERR` writer - Precise data bus error"]
pub type PreciserrW<'a, REG> = crate::BitWriter<'a, REG, Preciserr>;
impl<'a, REG> PreciserrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no precise data bus error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Preciserr::Value1)
    }
    #[doc = "a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Preciserr::Value2)
    }
}
#[doc = "Imprecise data bus error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Impreciserr {
    #[doc = "0: no imprecise data bus error"]
    Value1 = 0,
    #[doc = "1: a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error."]
    Value2 = 1,
}
impl From<Impreciserr> for bool {
    #[inline(always)]
    fn from(variant: Impreciserr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMPRECISERR` reader - Imprecise data bus error"]
pub type ImpreciserrR = crate::BitReader<Impreciserr>;
impl ImpreciserrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Impreciserr {
        match self.bits {
            false => Impreciserr::Value1,
            true => Impreciserr::Value2,
        }
    }
    #[doc = "no imprecise data bus error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Impreciserr::Value1
    }
    #[doc = "a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Impreciserr::Value2
    }
}
#[doc = "Field `IMPRECISERR` writer - Imprecise data bus error"]
pub type ImpreciserrW<'a, REG> = crate::BitWriter<'a, REG, Impreciserr>;
impl<'a, REG> ImpreciserrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no imprecise data bus error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Impreciserr::Value1)
    }
    #[doc = "a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Impreciserr::Value2)
    }
}
#[doc = "BusFault on unstacking for a return from exception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unstkerr {
    #[doc = "0: no unstacking fault"]
    Value1 = 0,
    #[doc = "1: stacking for an exception entry has caused one or more BusFaults."]
    Value2 = 1,
}
impl From<Unstkerr> for bool {
    #[inline(always)]
    fn from(variant: Unstkerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNSTKERR` reader - BusFault on unstacking for a return from exception"]
pub type UnstkerrR = crate::BitReader<Unstkerr>;
impl UnstkerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Unstkerr {
        match self.bits {
            false => Unstkerr::Value1,
            true => Unstkerr::Value2,
        }
    }
    #[doc = "no unstacking fault"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Unstkerr::Value1
    }
    #[doc = "stacking for an exception entry has caused one or more BusFaults."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Unstkerr::Value2
    }
}
#[doc = "Field `UNSTKERR` writer - BusFault on unstacking for a return from exception"]
pub type UnstkerrW<'a, REG> = crate::BitWriter<'a, REG, Unstkerr>;
impl<'a, REG> UnstkerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no unstacking fault"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Unstkerr::Value1)
    }
    #[doc = "stacking for an exception entry has caused one or more BusFaults."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Unstkerr::Value2)
    }
}
#[doc = "BusFault on stacking for exception entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stkerr {
    #[doc = "0: no stacking fault"]
    Value1 = 0,
    #[doc = "1: stacking for an exception entry has caused one or more BusFaults."]
    Value2 = 1,
}
impl From<Stkerr> for bool {
    #[inline(always)]
    fn from(variant: Stkerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STKERR` reader - BusFault on stacking for exception entry"]
pub type StkerrR = crate::BitReader<Stkerr>;
impl StkerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stkerr {
        match self.bits {
            false => Stkerr::Value1,
            true => Stkerr::Value2,
        }
    }
    #[doc = "no stacking fault"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Stkerr::Value1
    }
    #[doc = "stacking for an exception entry has caused one or more BusFaults."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Stkerr::Value2
    }
}
#[doc = "Field `STKERR` writer - BusFault on stacking for exception entry"]
pub type StkerrW<'a, REG> = crate::BitWriter<'a, REG, Stkerr>;
impl<'a, REG> StkerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no stacking fault"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Stkerr::Value1)
    }
    #[doc = "stacking for an exception entry has caused one or more BusFaults."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Stkerr::Value2)
    }
}
#[doc = "BusFault during floating point lazy state preservation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsperr {
    #[doc = "0: No bus fault occurred during floating-point lazy state preservation."]
    Value1 = 0,
    #[doc = "1: A bus fault occurred during floating-point lazy state preservation"]
    Value2 = 1,
}
impl From<Lsperr> for bool {
    #[inline(always)]
    fn from(variant: Lsperr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSPERR` reader - BusFault during floating point lazy state preservation"]
pub type LsperrR = crate::BitReader<Lsperr>;
impl LsperrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsperr {
        match self.bits {
            false => Lsperr::Value1,
            true => Lsperr::Value2,
        }
    }
    #[doc = "No bus fault occurred during floating-point lazy state preservation."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lsperr::Value1
    }
    #[doc = "A bus fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lsperr::Value2
    }
}
#[doc = "Field `LSPERR` writer - BusFault during floating point lazy state preservation"]
pub type LsperrW<'a, REG> = crate::BitWriter<'a, REG, Lsperr>;
impl<'a, REG> LsperrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No bus fault occurred during floating-point lazy state preservation."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lsperr::Value1)
    }
    #[doc = "A bus fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lsperr::Value2)
    }
}
#[doc = "BusFault Address Register (BFAR) valid flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfarvalid {
    #[doc = "0: value in BFAR is not a valid fault address"]
    Value1 = 0,
    #[doc = "1: BFAR holds a valid fault address."]
    Value2 = 1,
}
impl From<Bfarvalid> for bool {
    #[inline(always)]
    fn from(variant: Bfarvalid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFARVALID` reader - BusFault Address Register (BFAR) valid flag"]
pub type BfarvalidR = crate::BitReader<Bfarvalid>;
impl BfarvalidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfarvalid {
        match self.bits {
            false => Bfarvalid::Value1,
            true => Bfarvalid::Value2,
        }
    }
    #[doc = "value in BFAR is not a valid fault address"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfarvalid::Value1
    }
    #[doc = "BFAR holds a valid fault address."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfarvalid::Value2
    }
}
#[doc = "Field `BFARVALID` writer - BusFault Address Register (BFAR) valid flag"]
pub type BfarvalidW<'a, REG> = crate::BitWriter<'a, REG, Bfarvalid>;
impl<'a, REG> BfarvalidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "value in BFAR is not a valid fault address"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfarvalid::Value1)
    }
    #[doc = "BFAR holds a valid fault address."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfarvalid::Value2)
    }
}
#[doc = "Undefined instruction UsageFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Undefinstr {
    #[doc = "0: no undefined instruction UsageFault"]
    Value1 = 0,
    #[doc = "1: the processor has attempted to execute an undefined instruction."]
    Value2 = 1,
}
impl From<Undefinstr> for bool {
    #[inline(always)]
    fn from(variant: Undefinstr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNDEFINSTR` reader - Undefined instruction UsageFault"]
pub type UndefinstrR = crate::BitReader<Undefinstr>;
impl UndefinstrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Undefinstr {
        match self.bits {
            false => Undefinstr::Value1,
            true => Undefinstr::Value2,
        }
    }
    #[doc = "no undefined instruction UsageFault"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Undefinstr::Value1
    }
    #[doc = "the processor has attempted to execute an undefined instruction."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Undefinstr::Value2
    }
}
#[doc = "Field `UNDEFINSTR` writer - Undefined instruction UsageFault"]
pub type UndefinstrW<'a, REG> = crate::BitWriter<'a, REG, Undefinstr>;
impl<'a, REG> UndefinstrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no undefined instruction UsageFault"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Undefinstr::Value1)
    }
    #[doc = "the processor has attempted to execute an undefined instruction."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Undefinstr::Value2)
    }
}
#[doc = "Invalid state UsageFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Invstate {
    #[doc = "0: no invalid state UsageFault"]
    Value1 = 0,
    #[doc = "1: the processor has attempted to execute an instruction that makes illegal use of the EPSR."]
    Value2 = 1,
}
impl From<Invstate> for bool {
    #[inline(always)]
    fn from(variant: Invstate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVSTATE` reader - Invalid state UsageFault"]
pub type InvstateR = crate::BitReader<Invstate>;
impl InvstateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Invstate {
        match self.bits {
            false => Invstate::Value1,
            true => Invstate::Value2,
        }
    }
    #[doc = "no invalid state UsageFault"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Invstate::Value1
    }
    #[doc = "the processor has attempted to execute an instruction that makes illegal use of the EPSR."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Invstate::Value2
    }
}
#[doc = "Field `INVSTATE` writer - Invalid state UsageFault"]
pub type InvstateW<'a, REG> = crate::BitWriter<'a, REG, Invstate>;
impl<'a, REG> InvstateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no invalid state UsageFault"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Invstate::Value1)
    }
    #[doc = "the processor has attempted to execute an instruction that makes illegal use of the EPSR."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Invstate::Value2)
    }
}
#[doc = "Invalid PC load UsageFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Invpc {
    #[doc = "0: no invalid PC load UsageFault"]
    Value1 = 0,
    #[doc = "1: the processor has attempted an illegal load of EXC_RETURN to the PC, as a result of an invalid context, or an invalid EXC_RETURN value."]
    Value2 = 1,
}
impl From<Invpc> for bool {
    #[inline(always)]
    fn from(variant: Invpc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVPC` reader - Invalid PC load UsageFault"]
pub type InvpcR = crate::BitReader<Invpc>;
impl InvpcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Invpc {
        match self.bits {
            false => Invpc::Value1,
            true => Invpc::Value2,
        }
    }
    #[doc = "no invalid PC load UsageFault"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Invpc::Value1
    }
    #[doc = "the processor has attempted an illegal load of EXC_RETURN to the PC, as a result of an invalid context, or an invalid EXC_RETURN value."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Invpc::Value2
    }
}
#[doc = "Field `INVPC` writer - Invalid PC load UsageFault"]
pub type InvpcW<'a, REG> = crate::BitWriter<'a, REG, Invpc>;
impl<'a, REG> InvpcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no invalid PC load UsageFault"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Invpc::Value1)
    }
    #[doc = "the processor has attempted an illegal load of EXC_RETURN to the PC, as a result of an invalid context, or an invalid EXC_RETURN value."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Invpc::Value2)
    }
}
#[doc = "No coprocessor UsageFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nocp {
    #[doc = "0: no UsageFault caused by attempting to access a coprocessor"]
    Value1 = 0,
    #[doc = "1: the processor has attempted to access a coprocessor."]
    Value2 = 1,
}
impl From<Nocp> for bool {
    #[inline(always)]
    fn from(variant: Nocp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOCP` reader - No coprocessor UsageFault"]
pub type NocpR = crate::BitReader<Nocp>;
impl NocpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nocp {
        match self.bits {
            false => Nocp::Value1,
            true => Nocp::Value2,
        }
    }
    #[doc = "no UsageFault caused by attempting to access a coprocessor"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Nocp::Value1
    }
    #[doc = "the processor has attempted to access a coprocessor."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Nocp::Value2
    }
}
#[doc = "Field `NOCP` writer - No coprocessor UsageFault"]
pub type NocpW<'a, REG> = crate::BitWriter<'a, REG, Nocp>;
impl<'a, REG> NocpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no UsageFault caused by attempting to access a coprocessor"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Nocp::Value1)
    }
    #[doc = "the processor has attempted to access a coprocessor."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Nocp::Value2)
    }
}
#[doc = "Unaligned access UsageFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unaligned {
    #[doc = "0: no unaligned access fault, or unaligned access trapping not enabled"]
    Value1 = 0,
    #[doc = "1: the processor has made an unaligned memory access."]
    Value2 = 1,
}
impl From<Unaligned> for bool {
    #[inline(always)]
    fn from(variant: Unaligned) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNALIGNED` reader - Unaligned access UsageFault"]
pub type UnalignedR = crate::BitReader<Unaligned>;
impl UnalignedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Unaligned {
        match self.bits {
            false => Unaligned::Value1,
            true => Unaligned::Value2,
        }
    }
    #[doc = "no unaligned access fault, or unaligned access trapping not enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Unaligned::Value1
    }
    #[doc = "the processor has made an unaligned memory access."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Unaligned::Value2
    }
}
#[doc = "Field `UNALIGNED` writer - Unaligned access UsageFault"]
pub type UnalignedW<'a, REG> = crate::BitWriter<'a, REG, Unaligned>;
impl<'a, REG> UnalignedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no unaligned access fault, or unaligned access trapping not enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Unaligned::Value1)
    }
    #[doc = "the processor has made an unaligned memory access."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Unaligned::Value2)
    }
}
#[doc = "Divide by zero UsageFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Divbyzero {
    #[doc = "0: no divide by zero fault, or divide by zero trapping not enabled"]
    Value1 = 0,
    #[doc = "1: the processor has executed an SDIV or UDIV instruction with a divisor of 0"]
    Value2 = 1,
}
impl From<Divbyzero> for bool {
    #[inline(always)]
    fn from(variant: Divbyzero) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIVBYZERO` reader - Divide by zero UsageFault"]
pub type DivbyzeroR = crate::BitReader<Divbyzero>;
impl DivbyzeroR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Divbyzero {
        match self.bits {
            false => Divbyzero::Value1,
            true => Divbyzero::Value2,
        }
    }
    #[doc = "no divide by zero fault, or divide by zero trapping not enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Divbyzero::Value1
    }
    #[doc = "the processor has executed an SDIV or UDIV instruction with a divisor of 0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Divbyzero::Value2
    }
}
#[doc = "Field `DIVBYZERO` writer - Divide by zero UsageFault"]
pub type DivbyzeroW<'a, REG> = crate::BitWriter<'a, REG, Divbyzero>;
impl<'a, REG> DivbyzeroW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no divide by zero fault, or divide by zero trapping not enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Divbyzero::Value1)
    }
    #[doc = "the processor has executed an SDIV or UDIV instruction with a divisor of 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Divbyzero::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Instruction access violation flag"]
    #[inline(always)]
    pub fn iaccviol(&self) -> IaccviolR {
        IaccviolR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data access violation flag"]
    #[inline(always)]
    pub fn daccviol(&self) -> DaccviolR {
        DaccviolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - MemManage fault on unstacking for a return from exception"]
    #[inline(always)]
    pub fn munstkerr(&self) -> MunstkerrR {
        MunstkerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MemManage fault on stacking for exception entry"]
    #[inline(always)]
    pub fn mstkerr(&self) -> MstkerrR {
        MstkerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MemManage fault during floating point lazy state preservation"]
    #[inline(always)]
    pub fn mlsperr(&self) -> MlsperrR {
        MlsperrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - MemManage Fault Address Register (MMFAR) valid flag"]
    #[inline(always)]
    pub fn mmarvalid(&self) -> MmarvalidR {
        MmarvalidR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Instruction bus error"]
    #[inline(always)]
    pub fn ibuserr(&self) -> IbuserrR {
        IbuserrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Precise data bus error"]
    #[inline(always)]
    pub fn preciserr(&self) -> PreciserrR {
        PreciserrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Imprecise data bus error"]
    #[inline(always)]
    pub fn impreciserr(&self) -> ImpreciserrR {
        ImpreciserrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BusFault on unstacking for a return from exception"]
    #[inline(always)]
    pub fn unstkerr(&self) -> UnstkerrR {
        UnstkerrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BusFault on stacking for exception entry"]
    #[inline(always)]
    pub fn stkerr(&self) -> StkerrR {
        StkerrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - BusFault during floating point lazy state preservation"]
    #[inline(always)]
    pub fn lsperr(&self) -> LsperrR {
        LsperrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - BusFault Address Register (BFAR) valid flag"]
    #[inline(always)]
    pub fn bfarvalid(&self) -> BfarvalidR {
        BfarvalidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Undefined instruction UsageFault"]
    #[inline(always)]
    pub fn undefinstr(&self) -> UndefinstrR {
        UndefinstrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Invalid state UsageFault"]
    #[inline(always)]
    pub fn invstate(&self) -> InvstateR {
        InvstateR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Invalid PC load UsageFault"]
    #[inline(always)]
    pub fn invpc(&self) -> InvpcR {
        InvpcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - No coprocessor UsageFault"]
    #[inline(always)]
    pub fn nocp(&self) -> NocpR {
        NocpR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Unaligned access UsageFault"]
    #[inline(always)]
    pub fn unaligned(&self) -> UnalignedR {
        UnalignedR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Divide by zero UsageFault"]
    #[inline(always)]
    pub fn divbyzero(&self) -> DivbyzeroR {
        DivbyzeroR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Instruction access violation flag"]
    #[inline(always)]
    #[must_use]
    pub fn iaccviol(&mut self) -> IaccviolW<CfsrSpec> {
        IaccviolW::new(self, 0)
    }
    #[doc = "Bit 1 - Data access violation flag"]
    #[inline(always)]
    #[must_use]
    pub fn daccviol(&mut self) -> DaccviolW<CfsrSpec> {
        DaccviolW::new(self, 1)
    }
    #[doc = "Bit 3 - MemManage fault on unstacking for a return from exception"]
    #[inline(always)]
    #[must_use]
    pub fn munstkerr(&mut self) -> MunstkerrW<CfsrSpec> {
        MunstkerrW::new(self, 3)
    }
    #[doc = "Bit 4 - MemManage fault on stacking for exception entry"]
    #[inline(always)]
    #[must_use]
    pub fn mstkerr(&mut self) -> MstkerrW<CfsrSpec> {
        MstkerrW::new(self, 4)
    }
    #[doc = "Bit 5 - MemManage fault during floating point lazy state preservation"]
    #[inline(always)]
    #[must_use]
    pub fn mlsperr(&mut self) -> MlsperrW<CfsrSpec> {
        MlsperrW::new(self, 5)
    }
    #[doc = "Bit 7 - MemManage Fault Address Register (MMFAR) valid flag"]
    #[inline(always)]
    #[must_use]
    pub fn mmarvalid(&mut self) -> MmarvalidW<CfsrSpec> {
        MmarvalidW::new(self, 7)
    }
    #[doc = "Bit 8 - Instruction bus error"]
    #[inline(always)]
    #[must_use]
    pub fn ibuserr(&mut self) -> IbuserrW<CfsrSpec> {
        IbuserrW::new(self, 8)
    }
    #[doc = "Bit 9 - Precise data bus error"]
    #[inline(always)]
    #[must_use]
    pub fn preciserr(&mut self) -> PreciserrW<CfsrSpec> {
        PreciserrW::new(self, 9)
    }
    #[doc = "Bit 10 - Imprecise data bus error"]
    #[inline(always)]
    #[must_use]
    pub fn impreciserr(&mut self) -> ImpreciserrW<CfsrSpec> {
        ImpreciserrW::new(self, 10)
    }
    #[doc = "Bit 11 - BusFault on unstacking for a return from exception"]
    #[inline(always)]
    #[must_use]
    pub fn unstkerr(&mut self) -> UnstkerrW<CfsrSpec> {
        UnstkerrW::new(self, 11)
    }
    #[doc = "Bit 12 - BusFault on stacking for exception entry"]
    #[inline(always)]
    #[must_use]
    pub fn stkerr(&mut self) -> StkerrW<CfsrSpec> {
        StkerrW::new(self, 12)
    }
    #[doc = "Bit 13 - BusFault during floating point lazy state preservation"]
    #[inline(always)]
    #[must_use]
    pub fn lsperr(&mut self) -> LsperrW<CfsrSpec> {
        LsperrW::new(self, 13)
    }
    #[doc = "Bit 15 - BusFault Address Register (BFAR) valid flag"]
    #[inline(always)]
    #[must_use]
    pub fn bfarvalid(&mut self) -> BfarvalidW<CfsrSpec> {
        BfarvalidW::new(self, 15)
    }
    #[doc = "Bit 16 - Undefined instruction UsageFault"]
    #[inline(always)]
    #[must_use]
    pub fn undefinstr(&mut self) -> UndefinstrW<CfsrSpec> {
        UndefinstrW::new(self, 16)
    }
    #[doc = "Bit 17 - Invalid state UsageFault"]
    #[inline(always)]
    #[must_use]
    pub fn invstate(&mut self) -> InvstateW<CfsrSpec> {
        InvstateW::new(self, 17)
    }
    #[doc = "Bit 18 - Invalid PC load UsageFault"]
    #[inline(always)]
    #[must_use]
    pub fn invpc(&mut self) -> InvpcW<CfsrSpec> {
        InvpcW::new(self, 18)
    }
    #[doc = "Bit 19 - No coprocessor UsageFault"]
    #[inline(always)]
    #[must_use]
    pub fn nocp(&mut self) -> NocpW<CfsrSpec> {
        NocpW::new(self, 19)
    }
    #[doc = "Bit 24 - Unaligned access UsageFault"]
    #[inline(always)]
    #[must_use]
    pub fn unaligned(&mut self) -> UnalignedW<CfsrSpec> {
        UnalignedW::new(self, 24)
    }
    #[doc = "Bit 25 - Divide by zero UsageFault"]
    #[inline(always)]
    #[must_use]
    pub fn divbyzero(&mut self) -> DivbyzeroW<CfsrSpec> {
        DivbyzeroW::new(self, 25)
    }
}
#[doc = "Configurable Fault Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfsrSpec;
impl crate::RegisterSpec for CfsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfsr::R`](R) reader structure"]
impl crate::Readable for CfsrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfsr::W`](W) writer structure"]
impl crate::Writable for CfsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFSR to value 0"]
impl crate::Resettable for CfsrSpec {
    const RESET_VALUE: u32 = 0;
}
