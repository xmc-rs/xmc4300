#[doc = "Register `AL_EVENT_MASK` reader"]
pub type R = crate::R<AlEventMaskSpec>;
#[doc = "Register `AL_EVENT_MASK` writer"]
pub type W = crate::W<AlEventMaskSpec>;
#[doc = "AL Control event\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlCeMask {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<AlCeMask> for bool {
    #[inline(always)]
    fn from(variant: AlCeMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AL_CE_MASK` reader - AL Control event"]
pub type AlCeMaskR = crate::BitReader<AlCeMask>;
impl AlCeMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AlCeMask {
        match self.bits {
            false => AlCeMask::Value1,
            true => AlCeMask::Value2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AlCeMask::Value1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AlCeMask::Value2
    }
}
#[doc = "Field `AL_CE_MASK` writer - AL Control event"]
pub type AlCeMaskW<'a, REG> = crate::BitWriter<'a, REG, AlCeMask>;
impl<'a, REG> AlCeMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AlCeMask::Value1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AlCeMask::Value2)
    }
}
#[doc = "DC Latch event\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DcLeMask {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<DcLeMask> for bool {
    #[inline(always)]
    fn from(variant: DcLeMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DC_LE_MASK` reader - DC Latch event"]
pub type DcLeMaskR = crate::BitReader<DcLeMask>;
impl DcLeMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DcLeMask {
        match self.bits {
            false => DcLeMask::Value1,
            true => DcLeMask::Value2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DcLeMask::Value1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DcLeMask::Value2
    }
}
#[doc = "Field `DC_LE_MASK` writer - DC Latch event"]
pub type DcLeMaskW<'a, REG> = crate::BitWriter<'a, REG, DcLeMask>;
impl<'a, REG> DcLeMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DcLeMask::Value1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DcLeMask::Value2)
    }
}
#[doc = "State of DC SYNC0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StS0Mask {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<StS0Mask> for bool {
    #[inline(always)]
    fn from(variant: StS0Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ST_S0_MASK` reader - State of DC SYNC0"]
pub type StS0MaskR = crate::BitReader<StS0Mask>;
impl StS0MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StS0Mask {
        match self.bits {
            false => StS0Mask::Value1,
            true => StS0Mask::Value2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == StS0Mask::Value1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == StS0Mask::Value2
    }
}
#[doc = "Field `ST_S0_MASK` writer - State of DC SYNC0"]
pub type StS0MaskW<'a, REG> = crate::BitWriter<'a, REG, StS0Mask>;
impl<'a, REG> StS0MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(StS0Mask::Value1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(StS0Mask::Value2)
    }
}
#[doc = "State of DC SYNC1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StS1Mask {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<StS1Mask> for bool {
    #[inline(always)]
    fn from(variant: StS1Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ST_S1_MASK` reader - State of DC SYNC1"]
pub type StS1MaskR = crate::BitReader<StS1Mask>;
impl StS1MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StS1Mask {
        match self.bits {
            false => StS1Mask::Value1,
            true => StS1Mask::Value2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == StS1Mask::Value1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == StS1Mask::Value2
    }
}
#[doc = "Field `ST_S1_MASK` writer - State of DC SYNC1"]
pub type StS1MaskW<'a, REG> = crate::BitWriter<'a, REG, StS1Mask>;
impl<'a, REG> StS1MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(StS1Mask::Value1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(StS1Mask::Value2)
    }
}
#[doc = "SyncManager activation register changed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmAMask {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<SmAMask> for bool {
    #[inline(always)]
    fn from(variant: SmAMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SM_A_MASK` reader - SyncManager activation register changed"]
pub type SmAMaskR = crate::BitReader<SmAMask>;
impl SmAMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SmAMask {
        match self.bits {
            false => SmAMask::Value1,
            true => SmAMask::Value2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SmAMask::Value1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SmAMask::Value2
    }
}
#[doc = "Field `SM_A_MASK` writer - SyncManager activation register changed"]
pub type SmAMaskW<'a, REG> = crate::BitWriter<'a, REG, SmAMask>;
impl<'a, REG> SmAMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SmAMask::Value1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SmAMask::Value2)
    }
}
#[doc = "EEPROM Emulation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EepEMask {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<EepEMask> for bool {
    #[inline(always)]
    fn from(variant: EepEMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEP_E_MASK` reader - EEPROM Emulation"]
pub type EepEMaskR = crate::BitReader<EepEMask>;
impl EepEMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EepEMask {
        match self.bits {
            false => EepEMask::Value1,
            true => EepEMask::Value2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EepEMask::Value1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EepEMask::Value2
    }
}
#[doc = "Field `EEP_E_MASK` writer - EEPROM Emulation"]
pub type EepEMaskW<'a, REG> = crate::BitWriter<'a, REG, EepEMask>;
impl<'a, REG> EepEMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EepEMask::Value1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EepEMask::Value2)
    }
}
#[doc = "Watchdog Process Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WpDMask {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<WpDMask> for bool {
    #[inline(always)]
    fn from(variant: WpDMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_D_MASK` reader - Watchdog Process Data"]
pub type WpDMaskR = crate::BitReader<WpDMask>;
impl WpDMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WpDMask {
        match self.bits {
            false => WpDMask::Value1,
            true => WpDMask::Value2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WpDMask::Value1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WpDMask::Value2
    }
}
#[doc = "Field `WP_D_MASK` writer - Watchdog Process Data"]
pub type WpDMaskW<'a, REG> = crate::BitWriter<'a, REG, WpDMask>;
impl<'a, REG> WpDMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WpDMask::Value1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WpDMask::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi0Mask {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<Smi0Mask> for bool {
    #[inline(always)]
    fn from(variant: Smi0Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_0_MASK` reader - SyncManager interrupt"]
pub type Smi0MaskR = crate::BitReader<Smi0Mask>;
impl Smi0MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi0Mask {
        match self.bits {
            false => Smi0Mask::Value1,
            true => Smi0Mask::Value2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi0Mask::Value1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi0Mask::Value2
    }
}
#[doc = "Field `SMI_0_MASK` writer - SyncManager interrupt"]
pub type Smi0MaskW<'a, REG> = crate::BitWriter<'a, REG, Smi0Mask>;
impl<'a, REG> Smi0MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi0Mask::Value1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi0Mask::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi1Mask {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<Smi1Mask> for bool {
    #[inline(always)]
    fn from(variant: Smi1Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_1_MASK` reader - SyncManager interrupt"]
pub type Smi1MaskR = crate::BitReader<Smi1Mask>;
impl Smi1MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi1Mask {
        match self.bits {
            false => Smi1Mask::Value1,
            true => Smi1Mask::Value2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi1Mask::Value1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi1Mask::Value2
    }
}
#[doc = "Field `SMI_1_MASK` writer - SyncManager interrupt"]
pub type Smi1MaskW<'a, REG> = crate::BitWriter<'a, REG, Smi1Mask>;
impl<'a, REG> Smi1MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi1Mask::Value1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi1Mask::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi2Mask {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<Smi2Mask> for bool {
    #[inline(always)]
    fn from(variant: Smi2Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_2_MASK` reader - SyncManager interrupt"]
pub type Smi2MaskR = crate::BitReader<Smi2Mask>;
impl Smi2MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi2Mask {
        match self.bits {
            false => Smi2Mask::Value1,
            true => Smi2Mask::Value2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi2Mask::Value1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi2Mask::Value2
    }
}
#[doc = "Field `SMI_2_MASK` writer - SyncManager interrupt"]
pub type Smi2MaskW<'a, REG> = crate::BitWriter<'a, REG, Smi2Mask>;
impl<'a, REG> Smi2MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi2Mask::Value1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi2Mask::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi3Mask {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<Smi3Mask> for bool {
    #[inline(always)]
    fn from(variant: Smi3Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_3_MASK` reader - SyncManager interrupt"]
pub type Smi3MaskR = crate::BitReader<Smi3Mask>;
impl Smi3MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi3Mask {
        match self.bits {
            false => Smi3Mask::Value1,
            true => Smi3Mask::Value2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi3Mask::Value1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi3Mask::Value2
    }
}
#[doc = "Field `SMI_3_MASK` writer - SyncManager interrupt"]
pub type Smi3MaskW<'a, REG> = crate::BitWriter<'a, REG, Smi3Mask>;
impl<'a, REG> Smi3MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi3Mask::Value1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi3Mask::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi4Mask {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<Smi4Mask> for bool {
    #[inline(always)]
    fn from(variant: Smi4Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_4_MASK` reader - SyncManager interrupt"]
pub type Smi4MaskR = crate::BitReader<Smi4Mask>;
impl Smi4MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi4Mask {
        match self.bits {
            false => Smi4Mask::Value1,
            true => Smi4Mask::Value2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi4Mask::Value1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi4Mask::Value2
    }
}
#[doc = "Field `SMI_4_MASK` writer - SyncManager interrupt"]
pub type Smi4MaskW<'a, REG> = crate::BitWriter<'a, REG, Smi4Mask>;
impl<'a, REG> Smi4MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi4Mask::Value1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi4Mask::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi5Mask {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<Smi5Mask> for bool {
    #[inline(always)]
    fn from(variant: Smi5Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_5_MASK` reader - SyncManager interrupt"]
pub type Smi5MaskR = crate::BitReader<Smi5Mask>;
impl Smi5MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi5Mask {
        match self.bits {
            false => Smi5Mask::Value1,
            true => Smi5Mask::Value2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi5Mask::Value1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi5Mask::Value2
    }
}
#[doc = "Field `SMI_5_MASK` writer - SyncManager interrupt"]
pub type Smi5MaskW<'a, REG> = crate::BitWriter<'a, REG, Smi5Mask>;
impl<'a, REG> Smi5MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi5Mask::Value1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi5Mask::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi6Mask {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<Smi6Mask> for bool {
    #[inline(always)]
    fn from(variant: Smi6Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_6_MASK` reader - SyncManager interrupt"]
pub type Smi6MaskR = crate::BitReader<Smi6Mask>;
impl Smi6MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi6Mask {
        match self.bits {
            false => Smi6Mask::Value1,
            true => Smi6Mask::Value2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi6Mask::Value1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi6Mask::Value2
    }
}
#[doc = "Field `SMI_6_MASK` writer - SyncManager interrupt"]
pub type Smi6MaskW<'a, REG> = crate::BitWriter<'a, REG, Smi6Mask>;
impl<'a, REG> Smi6MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi6Mask::Value1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi6Mask::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi7Mask {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<Smi7Mask> for bool {
    #[inline(always)]
    fn from(variant: Smi7Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_7_MASK` reader - SyncManager interrupt"]
pub type Smi7MaskR = crate::BitReader<Smi7Mask>;
impl Smi7MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi7Mask {
        match self.bits {
            false => Smi7Mask::Value1,
            true => Smi7Mask::Value2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi7Mask::Value1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi7Mask::Value2
    }
}
#[doc = "Field `SMI_7_MASK` writer - SyncManager interrupt"]
pub type Smi7MaskW<'a, REG> = crate::BitWriter<'a, REG, Smi7Mask>;
impl<'a, REG> Smi7MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi7Mask::Value1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi7Mask::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi8Mask {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<Smi8Mask> for bool {
    #[inline(always)]
    fn from(variant: Smi8Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_8_MASK` reader - SyncManager interrupt"]
pub type Smi8MaskR = crate::BitReader<Smi8Mask>;
impl Smi8MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi8Mask {
        match self.bits {
            false => Smi8Mask::Value1,
            true => Smi8Mask::Value2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi8Mask::Value1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi8Mask::Value2
    }
}
#[doc = "Field `SMI_8_MASK` writer - SyncManager interrupt"]
pub type Smi8MaskW<'a, REG> = crate::BitWriter<'a, REG, Smi8Mask>;
impl<'a, REG> Smi8MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi8Mask::Value1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi8Mask::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi9Mask {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<Smi9Mask> for bool {
    #[inline(always)]
    fn from(variant: Smi9Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_9_MASK` reader - SyncManager interrupt"]
pub type Smi9MaskR = crate::BitReader<Smi9Mask>;
impl Smi9MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi9Mask {
        match self.bits {
            false => Smi9Mask::Value1,
            true => Smi9Mask::Value2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi9Mask::Value1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi9Mask::Value2
    }
}
#[doc = "Field `SMI_9_MASK` writer - SyncManager interrupt"]
pub type Smi9MaskW<'a, REG> = crate::BitWriter<'a, REG, Smi9Mask>;
impl<'a, REG> Smi9MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi9Mask::Value1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi9Mask::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi10Mask {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<Smi10Mask> for bool {
    #[inline(always)]
    fn from(variant: Smi10Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_10_MASK` reader - SyncManager interrupt"]
pub type Smi10MaskR = crate::BitReader<Smi10Mask>;
impl Smi10MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi10Mask {
        match self.bits {
            false => Smi10Mask::Value1,
            true => Smi10Mask::Value2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi10Mask::Value1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi10Mask::Value2
    }
}
#[doc = "Field `SMI_10_MASK` writer - SyncManager interrupt"]
pub type Smi10MaskW<'a, REG> = crate::BitWriter<'a, REG, Smi10Mask>;
impl<'a, REG> Smi10MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi10Mask::Value1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi10Mask::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi11Mask {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<Smi11Mask> for bool {
    #[inline(always)]
    fn from(variant: Smi11Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_11_MASK` reader - SyncManager interrupt"]
pub type Smi11MaskR = crate::BitReader<Smi11Mask>;
impl Smi11MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi11Mask {
        match self.bits {
            false => Smi11Mask::Value1,
            true => Smi11Mask::Value2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi11Mask::Value1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi11Mask::Value2
    }
}
#[doc = "Field `SMI_11_MASK` writer - SyncManager interrupt"]
pub type Smi11MaskW<'a, REG> = crate::BitWriter<'a, REG, Smi11Mask>;
impl<'a, REG> Smi11MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi11Mask::Value1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi11Mask::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi12Mask {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<Smi12Mask> for bool {
    #[inline(always)]
    fn from(variant: Smi12Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_12_MASK` reader - SyncManager interrupt"]
pub type Smi12MaskR = crate::BitReader<Smi12Mask>;
impl Smi12MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi12Mask {
        match self.bits {
            false => Smi12Mask::Value1,
            true => Smi12Mask::Value2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi12Mask::Value1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi12Mask::Value2
    }
}
#[doc = "Field `SMI_12_MASK` writer - SyncManager interrupt"]
pub type Smi12MaskW<'a, REG> = crate::BitWriter<'a, REG, Smi12Mask>;
impl<'a, REG> Smi12MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi12Mask::Value1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi12Mask::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi13Mask {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<Smi13Mask> for bool {
    #[inline(always)]
    fn from(variant: Smi13Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_13_MASK` reader - SyncManager interrupt"]
pub type Smi13MaskR = crate::BitReader<Smi13Mask>;
impl Smi13MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi13Mask {
        match self.bits {
            false => Smi13Mask::Value1,
            true => Smi13Mask::Value2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi13Mask::Value1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi13Mask::Value2
    }
}
#[doc = "Field `SMI_13_MASK` writer - SyncManager interrupt"]
pub type Smi13MaskW<'a, REG> = crate::BitWriter<'a, REG, Smi13Mask>;
impl<'a, REG> Smi13MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi13Mask::Value1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi13Mask::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi14Mask {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<Smi14Mask> for bool {
    #[inline(always)]
    fn from(variant: Smi14Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_14_MASK` reader - SyncManager interrupt"]
pub type Smi14MaskR = crate::BitReader<Smi14Mask>;
impl Smi14MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi14Mask {
        match self.bits {
            false => Smi14Mask::Value1,
            true => Smi14Mask::Value2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi14Mask::Value1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi14Mask::Value2
    }
}
#[doc = "Field `SMI_14_MASK` writer - SyncManager interrupt"]
pub type Smi14MaskW<'a, REG> = crate::BitWriter<'a, REG, Smi14Mask>;
impl<'a, REG> Smi14MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi14Mask::Value1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi14Mask::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi15Mask {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<Smi15Mask> for bool {
    #[inline(always)]
    fn from(variant: Smi15Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_15_MASK` reader - SyncManager interrupt"]
pub type Smi15MaskR = crate::BitReader<Smi15Mask>;
impl Smi15MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi15Mask {
        match self.bits {
            false => Smi15Mask::Value1,
            true => Smi15Mask::Value2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi15Mask::Value1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi15Mask::Value2
    }
}
#[doc = "Field `SMI_15_MASK` writer - SyncManager interrupt"]
pub type Smi15MaskW<'a, REG> = crate::BitWriter<'a, REG, Smi15Mask>;
impl<'a, REG> Smi15MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi15Mask::Value1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi15Mask::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - AL Control event"]
    #[inline(always)]
    pub fn al_ce_mask(&self) -> AlCeMaskR {
        AlCeMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DC Latch event"]
    #[inline(always)]
    pub fn dc_le_mask(&self) -> DcLeMaskR {
        DcLeMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - State of DC SYNC0"]
    #[inline(always)]
    pub fn st_s0_mask(&self) -> StS0MaskR {
        StS0MaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - State of DC SYNC1"]
    #[inline(always)]
    pub fn st_s1_mask(&self) -> StS1MaskR {
        StS1MaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SyncManager activation register changed"]
    #[inline(always)]
    pub fn sm_a_mask(&self) -> SmAMaskR {
        SmAMaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EEPROM Emulation"]
    #[inline(always)]
    pub fn eep_e_mask(&self) -> EepEMaskR {
        EepEMaskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Watchdog Process Data"]
    #[inline(always)]
    pub fn wp_d_mask(&self) -> WpDMaskR {
        WpDMaskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_0_mask(&self) -> Smi0MaskR {
        Smi0MaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_1_mask(&self) -> Smi1MaskR {
        Smi1MaskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_2_mask(&self) -> Smi2MaskR {
        Smi2MaskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_3_mask(&self) -> Smi3MaskR {
        Smi3MaskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_4_mask(&self) -> Smi4MaskR {
        Smi4MaskR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_5_mask(&self) -> Smi5MaskR {
        Smi5MaskR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_6_mask(&self) -> Smi6MaskR {
        Smi6MaskR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_7_mask(&self) -> Smi7MaskR {
        Smi7MaskR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_8_mask(&self) -> Smi8MaskR {
        Smi8MaskR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_9_mask(&self) -> Smi9MaskR {
        Smi9MaskR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_10_mask(&self) -> Smi10MaskR {
        Smi10MaskR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_11_mask(&self) -> Smi11MaskR {
        Smi11MaskR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_12_mask(&self) -> Smi12MaskR {
        Smi12MaskR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_13_mask(&self) -> Smi13MaskR {
        Smi13MaskR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_14_mask(&self) -> Smi14MaskR {
        Smi14MaskR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_15_mask(&self) -> Smi15MaskR {
        Smi15MaskR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AL Control event"]
    #[inline(always)]
    #[must_use]
    pub fn al_ce_mask(&mut self) -> AlCeMaskW<AlEventMaskSpec> {
        AlCeMaskW::new(self, 0)
    }
    #[doc = "Bit 1 - DC Latch event"]
    #[inline(always)]
    #[must_use]
    pub fn dc_le_mask(&mut self) -> DcLeMaskW<AlEventMaskSpec> {
        DcLeMaskW::new(self, 1)
    }
    #[doc = "Bit 2 - State of DC SYNC0"]
    #[inline(always)]
    #[must_use]
    pub fn st_s0_mask(&mut self) -> StS0MaskW<AlEventMaskSpec> {
        StS0MaskW::new(self, 2)
    }
    #[doc = "Bit 3 - State of DC SYNC1"]
    #[inline(always)]
    #[must_use]
    pub fn st_s1_mask(&mut self) -> StS1MaskW<AlEventMaskSpec> {
        StS1MaskW::new(self, 3)
    }
    #[doc = "Bit 4 - SyncManager activation register changed"]
    #[inline(always)]
    #[must_use]
    pub fn sm_a_mask(&mut self) -> SmAMaskW<AlEventMaskSpec> {
        SmAMaskW::new(self, 4)
    }
    #[doc = "Bit 5 - EEPROM Emulation"]
    #[inline(always)]
    #[must_use]
    pub fn eep_e_mask(&mut self) -> EepEMaskW<AlEventMaskSpec> {
        EepEMaskW::new(self, 5)
    }
    #[doc = "Bit 6 - Watchdog Process Data"]
    #[inline(always)]
    #[must_use]
    pub fn wp_d_mask(&mut self) -> WpDMaskW<AlEventMaskSpec> {
        WpDMaskW::new(self, 6)
    }
    #[doc = "Bit 8 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_0_mask(&mut self) -> Smi0MaskW<AlEventMaskSpec> {
        Smi0MaskW::new(self, 8)
    }
    #[doc = "Bit 9 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_1_mask(&mut self) -> Smi1MaskW<AlEventMaskSpec> {
        Smi1MaskW::new(self, 9)
    }
    #[doc = "Bit 10 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_2_mask(&mut self) -> Smi2MaskW<AlEventMaskSpec> {
        Smi2MaskW::new(self, 10)
    }
    #[doc = "Bit 11 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_3_mask(&mut self) -> Smi3MaskW<AlEventMaskSpec> {
        Smi3MaskW::new(self, 11)
    }
    #[doc = "Bit 12 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_4_mask(&mut self) -> Smi4MaskW<AlEventMaskSpec> {
        Smi4MaskW::new(self, 12)
    }
    #[doc = "Bit 13 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_5_mask(&mut self) -> Smi5MaskW<AlEventMaskSpec> {
        Smi5MaskW::new(self, 13)
    }
    #[doc = "Bit 14 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_6_mask(&mut self) -> Smi6MaskW<AlEventMaskSpec> {
        Smi6MaskW::new(self, 14)
    }
    #[doc = "Bit 15 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_7_mask(&mut self) -> Smi7MaskW<AlEventMaskSpec> {
        Smi7MaskW::new(self, 15)
    }
    #[doc = "Bit 16 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_8_mask(&mut self) -> Smi8MaskW<AlEventMaskSpec> {
        Smi8MaskW::new(self, 16)
    }
    #[doc = "Bit 17 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_9_mask(&mut self) -> Smi9MaskW<AlEventMaskSpec> {
        Smi9MaskW::new(self, 17)
    }
    #[doc = "Bit 18 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_10_mask(&mut self) -> Smi10MaskW<AlEventMaskSpec> {
        Smi10MaskW::new(self, 18)
    }
    #[doc = "Bit 19 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_11_mask(&mut self) -> Smi11MaskW<AlEventMaskSpec> {
        Smi11MaskW::new(self, 19)
    }
    #[doc = "Bit 20 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_12_mask(&mut self) -> Smi12MaskW<AlEventMaskSpec> {
        Smi12MaskW::new(self, 20)
    }
    #[doc = "Bit 21 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_13_mask(&mut self) -> Smi13MaskW<AlEventMaskSpec> {
        Smi13MaskW::new(self, 21)
    }
    #[doc = "Bit 22 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_14_mask(&mut self) -> Smi14MaskW<AlEventMaskSpec> {
        Smi14MaskW::new(self, 22)
    }
    #[doc = "Bit 23 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_15_mask(&mut self) -> Smi15MaskW<AlEventMaskSpec> {
        Smi15MaskW::new(self, 23)
    }
}
#[doc = "PDI AL Event Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`al_event_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`al_event_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlEventMaskSpec;
impl crate::RegisterSpec for AlEventMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`al_event_mask::R`](R) reader structure"]
impl crate::Readable for AlEventMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`al_event_mask::W`](W) writer structure"]
impl crate::Writable for AlEventMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AL_EVENT_MASK to value 0x00ff_ff2f"]
impl crate::Resettable for AlEventMaskSpec {
    const RESET_VALUE: u32 = 0x00ff_ff2f;
}
