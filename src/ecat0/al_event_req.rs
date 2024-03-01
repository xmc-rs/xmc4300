#[doc = "Register `AL_EVENT_REQ` reader"]
pub type R = crate::R<AlEventReqSpec>;
#[doc = "Register `AL_EVENT_REQ` writer"]
pub type W = crate::W<AlEventReqSpec>;
#[doc = "AL Control event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlCe {
    #[doc = "0: No AL Control Register change"]
    Value1 = 0,
    #[doc = "1: AL Control Register has been written"]
    Value2 = 1,
}
impl From<AlCe> for bool {
    #[inline(always)]
    fn from(variant: AlCe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AL_CE` reader - AL Control event"]
pub type AlCeR = crate::BitReader<AlCe>;
impl AlCeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AlCe {
        match self.bits {
            false => AlCe::Value1,
            true => AlCe::Value2,
        }
    }
    #[doc = "No AL Control Register change"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AlCe::Value1
    }
    #[doc = "AL Control Register has been written"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AlCe::Value2
    }
}
#[doc = "DC Latch event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DcLe {
    #[doc = "0: No change on DC Latch Inputs"]
    Value1 = 0,
    #[doc = "1: At least one change on DC Latch Inputs"]
    Value2 = 1,
}
impl From<DcLe> for bool {
    #[inline(always)]
    fn from(variant: DcLe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DC_LE` reader - DC Latch event"]
pub type DcLeR = crate::BitReader<DcLe>;
impl DcLeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DcLe {
        match self.bits {
            false => DcLe::Value1,
            true => DcLe::Value2,
        }
    }
    #[doc = "No change on DC Latch Inputs"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DcLe::Value1
    }
    #[doc = "At least one change on DC Latch Inputs"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DcLe::Value2
    }
}
#[doc = "Field `ST_S0` reader - State of DC SYNC0"]
pub type StS0R = crate::BitReader;
#[doc = "Field `ST_S1` reader - State of DC SYNC1"]
pub type StS1R = crate::BitReader;
#[doc = "SyncManager activation register changed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmA {
    #[doc = "0: No change in any SyncManager"]
    Value1 = 0,
    #[doc = "1: At least one change on DC Latch Inputs"]
    Value2 = 1,
}
impl From<SmA> for bool {
    #[inline(always)]
    fn from(variant: SmA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SM_A` reader - SyncManager activation register changed"]
pub type SmAR = crate::BitReader<SmA>;
impl SmAR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SmA {
        match self.bits {
            false => SmA::Value1,
            true => SmA::Value2,
        }
    }
    #[doc = "No change in any SyncManager"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SmA::Value1
    }
    #[doc = "At least one change on DC Latch Inputs"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SmA::Value2
    }
}
#[doc = "EEPROM Emulation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EepE {
    #[doc = "0: No command pending"]
    Value1 = 0,
    #[doc = "1: EEPROM command pending"]
    Value2 = 1,
}
impl From<EepE> for bool {
    #[inline(always)]
    fn from(variant: EepE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEP_E` reader - EEPROM Emulation"]
pub type EepER = crate::BitReader<EepE>;
impl EepER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EepE {
        match self.bits {
            false => EepE::Value1,
            true => EepE::Value2,
        }
    }
    #[doc = "No command pending"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EepE::Value1
    }
    #[doc = "EEPROM command pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EepE::Value2
    }
}
#[doc = "Watchdog Process Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WpD {
    #[doc = "0: Has not expired"]
    Value1 = 0,
    #[doc = "1: Has expired"]
    Value2 = 1,
}
impl From<WpD> for bool {
    #[inline(always)]
    fn from(variant: WpD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_D` reader - Watchdog Process Data"]
pub type WpDR = crate::BitReader<WpD>;
impl WpDR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WpD {
        match self.bits {
            false => WpD::Value1,
            true => WpD::Value2,
        }
    }
    #[doc = "Has not expired"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WpD::Value1
    }
    #[doc = "Has expired"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WpD::Value2
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi0 {
    #[doc = "0: No SyncManager 0 interrupt"]
    Value1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    Value2 = 1,
}
impl From<Smi0> for bool {
    #[inline(always)]
    fn from(variant: Smi0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_0` reader - SyncManager interrupt"]
pub type Smi0R = crate::BitReader<Smi0>;
impl Smi0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi0 {
        match self.bits {
            false => Smi0::Value1,
            true => Smi0::Value2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi0::Value1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi0::Value2
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi1 {
    #[doc = "0: No SyncManager 0 interrupt"]
    Value1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    Value2 = 1,
}
impl From<Smi1> for bool {
    #[inline(always)]
    fn from(variant: Smi1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_1` reader - SyncManager interrupt"]
pub type Smi1R = crate::BitReader<Smi1>;
impl Smi1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi1 {
        match self.bits {
            false => Smi1::Value1,
            true => Smi1::Value2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi1::Value1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi1::Value2
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi2 {
    #[doc = "0: No SyncManager 0 interrupt"]
    Value1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    Value2 = 1,
}
impl From<Smi2> for bool {
    #[inline(always)]
    fn from(variant: Smi2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_2` reader - SyncManager interrupt"]
pub type Smi2R = crate::BitReader<Smi2>;
impl Smi2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi2 {
        match self.bits {
            false => Smi2::Value1,
            true => Smi2::Value2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi2::Value1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi2::Value2
    }
}
#[doc = "Field `SMI_2` writer - SyncManager interrupt"]
pub type Smi2W<'a, REG> = crate::BitWriter<'a, REG, Smi2>;
impl<'a, REG> Smi2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi2::Value1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi2::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi3 {
    #[doc = "0: No SyncManager 0 interrupt"]
    Value1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    Value2 = 1,
}
impl From<Smi3> for bool {
    #[inline(always)]
    fn from(variant: Smi3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_3` reader - SyncManager interrupt"]
pub type Smi3R = crate::BitReader<Smi3>;
impl Smi3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi3 {
        match self.bits {
            false => Smi3::Value1,
            true => Smi3::Value2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi3::Value1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi3::Value2
    }
}
#[doc = "Field `SMI_3` writer - SyncManager interrupt"]
pub type Smi3W<'a, REG> = crate::BitWriter<'a, REG, Smi3>;
impl<'a, REG> Smi3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi3::Value1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi3::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi4 {
    #[doc = "0: No SyncManager 0 interrupt"]
    Value1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    Value2 = 1,
}
impl From<Smi4> for bool {
    #[inline(always)]
    fn from(variant: Smi4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_4` reader - SyncManager interrupt"]
pub type Smi4R = crate::BitReader<Smi4>;
impl Smi4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi4 {
        match self.bits {
            false => Smi4::Value1,
            true => Smi4::Value2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi4::Value1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi4::Value2
    }
}
#[doc = "Field `SMI_4` writer - SyncManager interrupt"]
pub type Smi4W<'a, REG> = crate::BitWriter<'a, REG, Smi4>;
impl<'a, REG> Smi4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi4::Value1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi4::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi5 {
    #[doc = "0: No SyncManager 0 interrupt"]
    Value1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    Value2 = 1,
}
impl From<Smi5> for bool {
    #[inline(always)]
    fn from(variant: Smi5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_5` reader - SyncManager interrupt"]
pub type Smi5R = crate::BitReader<Smi5>;
impl Smi5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi5 {
        match self.bits {
            false => Smi5::Value1,
            true => Smi5::Value2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi5::Value1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi5::Value2
    }
}
#[doc = "Field `SMI_5` writer - SyncManager interrupt"]
pub type Smi5W<'a, REG> = crate::BitWriter<'a, REG, Smi5>;
impl<'a, REG> Smi5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi5::Value1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi5::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi6 {
    #[doc = "0: No SyncManager 0 interrupt"]
    Value1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    Value2 = 1,
}
impl From<Smi6> for bool {
    #[inline(always)]
    fn from(variant: Smi6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_6` reader - SyncManager interrupt"]
pub type Smi6R = crate::BitReader<Smi6>;
impl Smi6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi6 {
        match self.bits {
            false => Smi6::Value1,
            true => Smi6::Value2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi6::Value1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi6::Value2
    }
}
#[doc = "Field `SMI_6` writer - SyncManager interrupt"]
pub type Smi6W<'a, REG> = crate::BitWriter<'a, REG, Smi6>;
impl<'a, REG> Smi6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi6::Value1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi6::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi7 {
    #[doc = "0: No SyncManager 0 interrupt"]
    Value1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    Value2 = 1,
}
impl From<Smi7> for bool {
    #[inline(always)]
    fn from(variant: Smi7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_7` reader - SyncManager interrupt"]
pub type Smi7R = crate::BitReader<Smi7>;
impl Smi7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi7 {
        match self.bits {
            false => Smi7::Value1,
            true => Smi7::Value2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi7::Value1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi7::Value2
    }
}
#[doc = "Field `SMI_7` writer - SyncManager interrupt"]
pub type Smi7W<'a, REG> = crate::BitWriter<'a, REG, Smi7>;
impl<'a, REG> Smi7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi7::Value1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi7::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi8 {
    #[doc = "0: No SyncManager 0 interrupt"]
    Value1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    Value2 = 1,
}
impl From<Smi8> for bool {
    #[inline(always)]
    fn from(variant: Smi8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_8` reader - SyncManager interrupt"]
pub type Smi8R = crate::BitReader<Smi8>;
impl Smi8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi8 {
        match self.bits {
            false => Smi8::Value1,
            true => Smi8::Value2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi8::Value1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi8::Value2
    }
}
#[doc = "Field `SMI_8` writer - SyncManager interrupt"]
pub type Smi8W<'a, REG> = crate::BitWriter<'a, REG, Smi8>;
impl<'a, REG> Smi8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi8::Value1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi8::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi9 {
    #[doc = "0: No SyncManager 0 interrupt"]
    Value1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    Value2 = 1,
}
impl From<Smi9> for bool {
    #[inline(always)]
    fn from(variant: Smi9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_9` reader - SyncManager interrupt"]
pub type Smi9R = crate::BitReader<Smi9>;
impl Smi9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi9 {
        match self.bits {
            false => Smi9::Value1,
            true => Smi9::Value2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi9::Value1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi9::Value2
    }
}
#[doc = "Field `SMI_9` writer - SyncManager interrupt"]
pub type Smi9W<'a, REG> = crate::BitWriter<'a, REG, Smi9>;
impl<'a, REG> Smi9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi9::Value1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi9::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi10 {
    #[doc = "0: No SyncManager 0 interrupt"]
    Value1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    Value2 = 1,
}
impl From<Smi10> for bool {
    #[inline(always)]
    fn from(variant: Smi10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_10` reader - SyncManager interrupt"]
pub type Smi10R = crate::BitReader<Smi10>;
impl Smi10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi10 {
        match self.bits {
            false => Smi10::Value1,
            true => Smi10::Value2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi10::Value1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi10::Value2
    }
}
#[doc = "Field `SMI_10` writer - SyncManager interrupt"]
pub type Smi10W<'a, REG> = crate::BitWriter<'a, REG, Smi10>;
impl<'a, REG> Smi10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi10::Value1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi10::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi11 {
    #[doc = "0: No SyncManager 0 interrupt"]
    Value1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    Value2 = 1,
}
impl From<Smi11> for bool {
    #[inline(always)]
    fn from(variant: Smi11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_11` reader - SyncManager interrupt"]
pub type Smi11R = crate::BitReader<Smi11>;
impl Smi11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi11 {
        match self.bits {
            false => Smi11::Value1,
            true => Smi11::Value2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi11::Value1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi11::Value2
    }
}
#[doc = "Field `SMI_11` writer - SyncManager interrupt"]
pub type Smi11W<'a, REG> = crate::BitWriter<'a, REG, Smi11>;
impl<'a, REG> Smi11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi11::Value1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi11::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi12 {
    #[doc = "0: No SyncManager 0 interrupt"]
    Value1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    Value2 = 1,
}
impl From<Smi12> for bool {
    #[inline(always)]
    fn from(variant: Smi12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_12` reader - SyncManager interrupt"]
pub type Smi12R = crate::BitReader<Smi12>;
impl Smi12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi12 {
        match self.bits {
            false => Smi12::Value1,
            true => Smi12::Value2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi12::Value1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi12::Value2
    }
}
#[doc = "Field `SMI_12` writer - SyncManager interrupt"]
pub type Smi12W<'a, REG> = crate::BitWriter<'a, REG, Smi12>;
impl<'a, REG> Smi12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi12::Value1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi12::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi13 {
    #[doc = "0: No SyncManager 0 interrupt"]
    Value1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    Value2 = 1,
}
impl From<Smi13> for bool {
    #[inline(always)]
    fn from(variant: Smi13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_13` reader - SyncManager interrupt"]
pub type Smi13R = crate::BitReader<Smi13>;
impl Smi13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi13 {
        match self.bits {
            false => Smi13::Value1,
            true => Smi13::Value2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi13::Value1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi13::Value2
    }
}
#[doc = "Field `SMI_13` writer - SyncManager interrupt"]
pub type Smi13W<'a, REG> = crate::BitWriter<'a, REG, Smi13>;
impl<'a, REG> Smi13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi13::Value1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi13::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi14 {
    #[doc = "0: No SyncManager 0 interrupt"]
    Value1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    Value2 = 1,
}
impl From<Smi14> for bool {
    #[inline(always)]
    fn from(variant: Smi14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_14` reader - SyncManager interrupt"]
pub type Smi14R = crate::BitReader<Smi14>;
impl Smi14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi14 {
        match self.bits {
            false => Smi14::Value1,
            true => Smi14::Value2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi14::Value1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi14::Value2
    }
}
#[doc = "Field `SMI_14` writer - SyncManager interrupt"]
pub type Smi14W<'a, REG> = crate::BitWriter<'a, REG, Smi14>;
impl<'a, REG> Smi14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Smi14::Value1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Smi14::Value2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi15 {
    #[doc = "0: No SyncManager 0 interrupt"]
    Value1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    Value2 = 1,
}
impl From<Smi15> for bool {
    #[inline(always)]
    fn from(variant: Smi15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_15` reader - SyncManager interrupt"]
pub type Smi15R = crate::BitReader<Smi15>;
impl Smi15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi15 {
        match self.bits {
            false => Smi15::Value1,
            true => Smi15::Value2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Smi15::Value1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Smi15::Value2
    }
}
impl R {
    #[doc = "Bit 0 - AL Control event"]
    #[inline(always)]
    pub fn al_ce(&self) -> AlCeR {
        AlCeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DC Latch event"]
    #[inline(always)]
    pub fn dc_le(&self) -> DcLeR {
        DcLeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - State of DC SYNC0"]
    #[inline(always)]
    pub fn st_s0(&self) -> StS0R {
        StS0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - State of DC SYNC1"]
    #[inline(always)]
    pub fn st_s1(&self) -> StS1R {
        StS1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SyncManager activation register changed"]
    #[inline(always)]
    pub fn sm_a(&self) -> SmAR {
        SmAR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EEPROM Emulation"]
    #[inline(always)]
    pub fn eep_e(&self) -> EepER {
        EepER::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Watchdog Process Data"]
    #[inline(always)]
    pub fn wp_d(&self) -> WpDR {
        WpDR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_0(&self) -> Smi0R {
        Smi0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_1(&self) -> Smi1R {
        Smi1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_2(&self) -> Smi2R {
        Smi2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_3(&self) -> Smi3R {
        Smi3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_4(&self) -> Smi4R {
        Smi4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_5(&self) -> Smi5R {
        Smi5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_6(&self) -> Smi6R {
        Smi6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_7(&self) -> Smi7R {
        Smi7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_8(&self) -> Smi8R {
        Smi8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_9(&self) -> Smi9R {
        Smi9R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_10(&self) -> Smi10R {
        Smi10R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_11(&self) -> Smi11R {
        Smi11R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_12(&self) -> Smi12R {
        Smi12R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_13(&self) -> Smi13R {
        Smi13R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_14(&self) -> Smi14R {
        Smi14R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_15(&self) -> Smi15R {
        Smi15R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_2(&mut self) -> Smi2W<AlEventReqSpec> {
        Smi2W::new(self, 10)
    }
    #[doc = "Bit 11 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_3(&mut self) -> Smi3W<AlEventReqSpec> {
        Smi3W::new(self, 11)
    }
    #[doc = "Bit 12 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_4(&mut self) -> Smi4W<AlEventReqSpec> {
        Smi4W::new(self, 12)
    }
    #[doc = "Bit 13 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_5(&mut self) -> Smi5W<AlEventReqSpec> {
        Smi5W::new(self, 13)
    }
    #[doc = "Bit 14 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_6(&mut self) -> Smi6W<AlEventReqSpec> {
        Smi6W::new(self, 14)
    }
    #[doc = "Bit 15 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_7(&mut self) -> Smi7W<AlEventReqSpec> {
        Smi7W::new(self, 15)
    }
    #[doc = "Bit 16 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_8(&mut self) -> Smi8W<AlEventReqSpec> {
        Smi8W::new(self, 16)
    }
    #[doc = "Bit 17 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_9(&mut self) -> Smi9W<AlEventReqSpec> {
        Smi9W::new(self, 17)
    }
    #[doc = "Bit 18 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_10(&mut self) -> Smi10W<AlEventReqSpec> {
        Smi10W::new(self, 18)
    }
    #[doc = "Bit 19 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_11(&mut self) -> Smi11W<AlEventReqSpec> {
        Smi11W::new(self, 19)
    }
    #[doc = "Bit 20 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_12(&mut self) -> Smi12W<AlEventReqSpec> {
        Smi12W::new(self, 20)
    }
    #[doc = "Bit 21 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_13(&mut self) -> Smi13W<AlEventReqSpec> {
        Smi13W::new(self, 21)
    }
    #[doc = "Bit 22 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_14(&mut self) -> Smi14W<AlEventReqSpec> {
        Smi14W::new(self, 22)
    }
}
#[doc = "AL Event Request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`al_event_req::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`al_event_req::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlEventReqSpec;
impl crate::RegisterSpec for AlEventReqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`al_event_req::R`](R) reader structure"]
impl crate::Readable for AlEventReqSpec {}
#[doc = "`write(|w| ..)` method takes [`al_event_req::W`](W) writer structure"]
impl crate::Writable for AlEventReqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AL_EVENT_REQ to value 0x20"]
impl crate::Resettable for AlEventReqSpec {
    const RESET_VALUE: u32 = 0x20;
}
