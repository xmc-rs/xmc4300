#[doc = "Register `FCON` reader"]
pub type R = crate::R<FconSpec>;
#[doc = "Register `FCON` writer"]
pub type W = crate::W<FconSpec>;
#[doc = "Wait States for read access to PFLASH\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wspflash {
    #[doc = "0: PFLASH access in one clock cycle"]
    Value1 = 0,
    #[doc = "1: PFLASH access in one clock cycle"]
    Value2 = 1,
    #[doc = "2: PFLASH access in two clock cycles"]
    Value3 = 2,
    #[doc = "3: PFLASH access in three clock cycles"]
    Value4 = 3,
    #[doc = "15: PFLASH access in fifteen clock cycles."]
    Value5 = 15,
}
impl From<Wspflash> for u8 {
    #[inline(always)]
    fn from(variant: Wspflash) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wspflash {
    type Ux = u8;
}
impl crate::IsEnum for Wspflash {}
#[doc = "Field `WSPFLASH` reader - Wait States for read access to PFLASH"]
pub type WspflashR = crate::FieldReader<Wspflash>;
impl WspflashR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wspflash> {
        match self.bits {
            0 => Some(Wspflash::Value1),
            1 => Some(Wspflash::Value2),
            2 => Some(Wspflash::Value3),
            3 => Some(Wspflash::Value4),
            15 => Some(Wspflash::Value5),
            _ => None,
        }
    }
    #[doc = "PFLASH access in one clock cycle"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wspflash::Value1
    }
    #[doc = "PFLASH access in one clock cycle"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wspflash::Value2
    }
    #[doc = "PFLASH access in two clock cycles"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Wspflash::Value3
    }
    #[doc = "PFLASH access in three clock cycles"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Wspflash::Value4
    }
    #[doc = "PFLASH access in fifteen clock cycles."]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Wspflash::Value5
    }
}
#[doc = "Field `WSPFLASH` writer - Wait States for read access to PFLASH"]
pub type WspflashW<'a, REG> = crate::FieldWriter<'a, REG, 4, Wspflash>;
impl<'a, REG> WspflashW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PFLASH access in one clock cycle"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wspflash::Value1)
    }
    #[doc = "PFLASH access in one clock cycle"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wspflash::Value2)
    }
    #[doc = "PFLASH access in two clock cycles"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Wspflash::Value3)
    }
    #[doc = "PFLASH access in three clock cycles"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Wspflash::Value4)
    }
    #[doc = "PFLASH access in fifteen clock cycles."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Wspflash::Value5)
    }
}
#[doc = "Wait State for Error Correction of PFLASH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wsecpf {
    #[doc = "0: No additional wait state for error correction"]
    Value1 = 0,
    #[doc = "1: One additional wait state for error correction during read access to Program Flash. If enabled, this wait state is only used for the first transfer of a burst transfer."]
    Value2 = 1,
}
impl From<Wsecpf> for bool {
    #[inline(always)]
    fn from(variant: Wsecpf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WSECPF` reader - Wait State for Error Correction of PFLASH"]
pub type WsecpfR = crate::BitReader<Wsecpf>;
impl WsecpfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wsecpf {
        match self.bits {
            false => Wsecpf::Value1,
            true => Wsecpf::Value2,
        }
    }
    #[doc = "No additional wait state for error correction"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wsecpf::Value1
    }
    #[doc = "One additional wait state for error correction during read access to Program Flash. If enabled, this wait state is only used for the first transfer of a burst transfer."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wsecpf::Value2
    }
}
#[doc = "Field `WSECPF` writer - Wait State for Error Correction of PFLASH"]
pub type WsecpfW<'a, REG> = crate::BitWriter<'a, REG, Wsecpf>;
impl<'a, REG> WsecpfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No additional wait state for error correction"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wsecpf::Value1)
    }
    #[doc = "One additional wait state for error correction during read access to Program Flash. If enabled, this wait state is only used for the first transfer of a burst transfer."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wsecpf::Value2)
    }
}
#[doc = "Dynamic Flash Idle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idle {
    #[doc = "0: Normal/standard Flash read operation"]
    Value1 = 0,
    #[doc = "1: Dynamic idle of Program Flash enabled for power saving; static prefetching disabled"]
    Value2 = 1,
}
impl From<Idle> for bool {
    #[inline(always)]
    fn from(variant: Idle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE` reader - Dynamic Flash Idle"]
pub type IdleR = crate::BitReader<Idle>;
impl IdleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idle {
        match self.bits {
            false => Idle::Value1,
            true => Idle::Value2,
        }
    }
    #[doc = "Normal/standard Flash read operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Idle::Value1
    }
    #[doc = "Dynamic idle of Program Flash enabled for power saving; static prefetching disabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Idle::Value2
    }
}
#[doc = "Field `IDLE` writer - Dynamic Flash Idle"]
pub type IdleW<'a, REG> = crate::BitWriter<'a, REG, Idle>;
impl<'a, REG> IdleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal/standard Flash read operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Idle::Value1)
    }
    #[doc = "Dynamic idle of Program Flash enabled for power saving; static prefetching disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Idle::Value2)
    }
}
#[doc = "External Sleep Request Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Esldis {
    #[doc = "0: External sleep request signal input is enabled"]
    Value1 = 0,
    #[doc = "1: Externally requested Flash sleep is disabled"]
    Value2 = 1,
}
impl From<Esldis> for bool {
    #[inline(always)]
    fn from(variant: Esldis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESLDIS` reader - External Sleep Request Disable"]
pub type EsldisR = crate::BitReader<Esldis>;
impl EsldisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Esldis {
        match self.bits {
            false => Esldis::Value1,
            true => Esldis::Value2,
        }
    }
    #[doc = "External sleep request signal input is enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Esldis::Value1
    }
    #[doc = "Externally requested Flash sleep is disabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Esldis::Value2
    }
}
#[doc = "Field `ESLDIS` writer - External Sleep Request Disable"]
pub type EsldisW<'a, REG> = crate::BitWriter<'a, REG, Esldis>;
impl<'a, REG> EsldisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External sleep request signal input is enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Esldis::Value1)
    }
    #[doc = "Externally requested Flash sleep is disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Esldis::Value2)
    }
}
#[doc = "Flash SLEEP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sleep {
    #[doc = "0: Normal state or wake-up"]
    Value1 = 0,
    #[doc = "1: Flash sleep mode is requested"]
    Value2 = 1,
}
impl From<Sleep> for bool {
    #[inline(always)]
    fn from(variant: Sleep) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEP` reader - Flash SLEEP"]
pub type SleepR = crate::BitReader<Sleep>;
impl SleepR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sleep {
        match self.bits {
            false => Sleep::Value1,
            true => Sleep::Value2,
        }
    }
    #[doc = "Normal state or wake-up"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sleep::Value1
    }
    #[doc = "Flash sleep mode is requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sleep::Value2
    }
}
#[doc = "Field `SLEEP` writer - Flash SLEEP"]
pub type SleepW<'a, REG> = crate::BitWriter<'a, REG, Sleep>;
impl<'a, REG> SleepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal state or wake-up"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sleep::Value1)
    }
    #[doc = "Flash sleep mode is requested"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sleep::Value2)
    }
}
#[doc = "Read Protection Activated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpa {
    #[doc = "0: The Flash-internal read protection is not activated. Bits DCF, DDF are not taken into account. Bits DCF, DDFx can be cleared"]
    Value1 = 0,
    #[doc = "1: The Flash-internal read protection is activated. Bits DCF, DDF are enabled and evaluated."]
    Value2 = 1,
}
impl From<Rpa> for bool {
    #[inline(always)]
    fn from(variant: Rpa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPA` reader - Read Protection Activated"]
pub type RpaR = crate::BitReader<Rpa>;
impl RpaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rpa {
        match self.bits {
            false => Rpa::Value1,
            true => Rpa::Value2,
        }
    }
    #[doc = "The Flash-internal read protection is not activated. Bits DCF, DDF are not taken into account. Bits DCF, DDFx can be cleared"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rpa::Value1
    }
    #[doc = "The Flash-internal read protection is activated. Bits DCF, DDF are enabled and evaluated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rpa::Value2
    }
}
#[doc = "Disable Code Fetch from Flash Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcf {
    #[doc = "0: Code fetching from the Flash memory area is allowed."]
    Value1 = 0,
    #[doc = "1: Code fetching from the Flash memory area is not allowed. This bit is not taken into account while RPA='0'."]
    Value2 = 1,
}
impl From<Dcf> for bool {
    #[inline(always)]
    fn from(variant: Dcf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCF` reader - Disable Code Fetch from Flash Memory"]
pub type DcfR = crate::BitReader<Dcf>;
impl DcfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcf {
        match self.bits {
            false => Dcf::Value1,
            true => Dcf::Value2,
        }
    }
    #[doc = "Code fetching from the Flash memory area is allowed."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dcf::Value1
    }
    #[doc = "Code fetching from the Flash memory area is not allowed. This bit is not taken into account while RPA='0'."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dcf::Value2
    }
}
#[doc = "Field `DCF` writer - Disable Code Fetch from Flash Memory"]
pub type DcfW<'a, REG> = crate::BitWriter<'a, REG, Dcf>;
impl<'a, REG> DcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Code fetching from the Flash memory area is allowed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcf::Value1)
    }
    #[doc = "Code fetching from the Flash memory area is not allowed. This bit is not taken into account while RPA='0'."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dcf::Value2)
    }
}
#[doc = "Disable Any Data Fetch from Flash\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddf {
    #[doc = "0: Data read access to the Flash memory area is allowed."]
    Value1 = 0,
    #[doc = "1: Data read access to the Flash memory area is not allowed. This bit is not taken into account while RPA='0'."]
    Value2 = 1,
}
impl From<Ddf> for bool {
    #[inline(always)]
    fn from(variant: Ddf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDF` reader - Disable Any Data Fetch from Flash"]
pub type DdfR = crate::BitReader<Ddf>;
impl DdfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddf {
        match self.bits {
            false => Ddf::Value1,
            true => Ddf::Value2,
        }
    }
    #[doc = "Data read access to the Flash memory area is allowed."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ddf::Value1
    }
    #[doc = "Data read access to the Flash memory area is not allowed. This bit is not taken into account while RPA='0'."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ddf::Value2
    }
}
#[doc = "Field `DDF` writer - Disable Any Data Fetch from Flash"]
pub type DdfW<'a, REG> = crate::BitWriter<'a, REG, Ddf>;
impl<'a, REG> DdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data read access to the Flash memory area is allowed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ddf::Value1)
    }
    #[doc = "Data read access to the Flash memory area is not allowed. This bit is not taken into account while RPA='0'."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ddf::Value2)
    }
}
#[doc = "Verify and Operation Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Voperm {
    #[doc = "0: Interrupt not enabled"]
    Value1 = 0,
    #[doc = "1: Flash interrupt because of Verify Error or Operation Error in Flash array (FSI) is enabled"]
    Value2 = 1,
}
impl From<Voperm> for bool {
    #[inline(always)]
    fn from(variant: Voperm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOPERM` reader - Verify and Operation Error Interrupt Mask"]
pub type VopermR = crate::BitReader<Voperm>;
impl VopermR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Voperm {
        match self.bits {
            false => Voperm::Value1,
            true => Voperm::Value2,
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Voperm::Value1
    }
    #[doc = "Flash interrupt because of Verify Error or Operation Error in Flash array (FSI) is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Voperm::Value2
    }
}
#[doc = "Field `VOPERM` writer - Verify and Operation Error Interrupt Mask"]
pub type VopermW<'a, REG> = crate::BitWriter<'a, REG, Voperm>;
impl<'a, REG> VopermW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Voperm::Value1)
    }
    #[doc = "Flash interrupt because of Verify Error or Operation Error in Flash array (FSI) is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Voperm::Value2)
    }
}
#[doc = "Command Sequence Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sqerm {
    #[doc = "0: Interrupt not enabled"]
    Value1 = 0,
    #[doc = "1: Flash interrupt because of Sequence Error is enabled"]
    Value2 = 1,
}
impl From<Sqerm> for bool {
    #[inline(always)]
    fn from(variant: Sqerm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SQERM` reader - Command Sequence Error Interrupt Mask"]
pub type SqermR = crate::BitReader<Sqerm>;
impl SqermR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sqerm {
        match self.bits {
            false => Sqerm::Value1,
            true => Sqerm::Value2,
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sqerm::Value1
    }
    #[doc = "Flash interrupt because of Sequence Error is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sqerm::Value2
    }
}
#[doc = "Field `SQERM` writer - Command Sequence Error Interrupt Mask"]
pub type SqermW<'a, REG> = crate::BitWriter<'a, REG, Sqerm>;
impl<'a, REG> SqermW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sqerm::Value1)
    }
    #[doc = "Flash interrupt because of Sequence Error is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sqerm::Value2)
    }
}
#[doc = "Protection Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Proerm {
    #[doc = "0: Interrupt not enabled"]
    Value1 = 0,
    #[doc = "1: Flash interrupt because of Protection Error is enabled"]
    Value2 = 1,
}
impl From<Proerm> for bool {
    #[inline(always)]
    fn from(variant: Proerm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROERM` reader - Protection Error Interrupt Mask"]
pub type ProermR = crate::BitReader<Proerm>;
impl ProermR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Proerm {
        match self.bits {
            false => Proerm::Value1,
            true => Proerm::Value2,
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Proerm::Value1
    }
    #[doc = "Flash interrupt because of Protection Error is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Proerm::Value2
    }
}
#[doc = "Field `PROERM` writer - Protection Error Interrupt Mask"]
pub type ProermW<'a, REG> = crate::BitWriter<'a, REG, Proerm>;
impl<'a, REG> ProermW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Proerm::Value1)
    }
    #[doc = "Flash interrupt because of Protection Error is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Proerm::Value2)
    }
}
#[doc = "PFLASH Single-Bit Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pfsberm {
    #[doc = "0: No Single-Bit Error interrupt enabled"]
    Value1 = 0,
    #[doc = "1: Single-Bit Error interrupt enabled for PFLASH"]
    Value2 = 1,
}
impl From<Pfsberm> for bool {
    #[inline(always)]
    fn from(variant: Pfsberm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFSBERM` reader - PFLASH Single-Bit Error Interrupt Mask"]
pub type PfsbermR = crate::BitReader<Pfsberm>;
impl PfsbermR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pfsberm {
        match self.bits {
            false => Pfsberm::Value1,
            true => Pfsberm::Value2,
        }
    }
    #[doc = "No Single-Bit Error interrupt enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pfsberm::Value1
    }
    #[doc = "Single-Bit Error interrupt enabled for PFLASH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pfsberm::Value2
    }
}
#[doc = "Field `PFSBERM` writer - PFLASH Single-Bit Error Interrupt Mask"]
pub type PfsbermW<'a, REG> = crate::BitWriter<'a, REG, Pfsberm>;
impl<'a, REG> PfsbermW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Single-Bit Error interrupt enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pfsberm::Value1)
    }
    #[doc = "Single-Bit Error interrupt enabled for PFLASH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pfsberm::Value2)
    }
}
#[doc = "PFLASH Double-Bit Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pfdberm {
    #[doc = "0: Double-Bit Error interrupt for PFLASH not enabled"]
    Value1 = 0,
    #[doc = "1: Double-Bit Error interrupt for PFLASH enabled. Especially intended for margin check"]
    Value2 = 1,
}
impl From<Pfdberm> for bool {
    #[inline(always)]
    fn from(variant: Pfdberm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFDBERM` reader - PFLASH Double-Bit Error Interrupt Mask"]
pub type PfdbermR = crate::BitReader<Pfdberm>;
impl PfdbermR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pfdberm {
        match self.bits {
            false => Pfdberm::Value1,
            true => Pfdberm::Value2,
        }
    }
    #[doc = "Double-Bit Error interrupt for PFLASH not enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pfdberm::Value1
    }
    #[doc = "Double-Bit Error interrupt for PFLASH enabled. Especially intended for margin check"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pfdberm::Value2
    }
}
#[doc = "Field `PFDBERM` writer - PFLASH Double-Bit Error Interrupt Mask"]
pub type PfdbermW<'a, REG> = crate::BitWriter<'a, REG, Pfdberm>;
impl<'a, REG> PfdbermW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Double-Bit Error interrupt for PFLASH not enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pfdberm::Value1)
    }
    #[doc = "Double-Bit Error interrupt for PFLASH enabled. Especially intended for margin check"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pfdberm::Value2)
    }
}
#[doc = "End of Busy Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eobm {
    #[doc = "0: Interrupt not enabled"]
    Value1 = 0,
    #[doc = "1: EOB interrupt is enabled"]
    Value2 = 1,
}
impl From<Eobm> for bool {
    #[inline(always)]
    fn from(variant: Eobm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOBM` reader - End of Busy Interrupt Mask"]
pub type EobmR = crate::BitReader<Eobm>;
impl EobmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eobm {
        match self.bits {
            false => Eobm::Value1,
            true => Eobm::Value2,
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Eobm::Value1
    }
    #[doc = "EOB interrupt is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Eobm::Value2
    }
}
#[doc = "Field `EOBM` writer - End of Busy Interrupt Mask"]
pub type EobmW<'a, REG> = crate::BitWriter<'a, REG, Eobm>;
impl<'a, REG> EobmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Eobm::Value1)
    }
    #[doc = "EOB interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Eobm::Value2)
    }
}
impl R {
    #[doc = "Bits 0:3 - Wait States for read access to PFLASH"]
    #[inline(always)]
    pub fn wspflash(&self) -> WspflashR {
        WspflashR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Wait State for Error Correction of PFLASH"]
    #[inline(always)]
    pub fn wsecpf(&self) -> WsecpfR {
        WsecpfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 13 - Dynamic Flash Idle"]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - External Sleep Request Disable"]
    #[inline(always)]
    pub fn esldis(&self) -> EsldisR {
        EsldisR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Flash SLEEP"]
    #[inline(always)]
    pub fn sleep(&self) -> SleepR {
        SleepR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Read Protection Activated"]
    #[inline(always)]
    pub fn rpa(&self) -> RpaR {
        RpaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Disable Code Fetch from Flash Memory"]
    #[inline(always)]
    pub fn dcf(&self) -> DcfR {
        DcfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Disable Any Data Fetch from Flash"]
    #[inline(always)]
    pub fn ddf(&self) -> DdfR {
        DdfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Verify and Operation Error Interrupt Mask"]
    #[inline(always)]
    pub fn voperm(&self) -> VopermR {
        VopermR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Command Sequence Error Interrupt Mask"]
    #[inline(always)]
    pub fn sqerm(&self) -> SqermR {
        SqermR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Protection Error Interrupt Mask"]
    #[inline(always)]
    pub fn proerm(&self) -> ProermR {
        ProermR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PFLASH Single-Bit Error Interrupt Mask"]
    #[inline(always)]
    pub fn pfsberm(&self) -> PfsbermR {
        PfsbermR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - PFLASH Double-Bit Error Interrupt Mask"]
    #[inline(always)]
    pub fn pfdberm(&self) -> PfdbermR {
        PfdbermR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - End of Busy Interrupt Mask"]
    #[inline(always)]
    pub fn eobm(&self) -> EobmR {
        EobmR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Wait States for read access to PFLASH"]
    #[inline(always)]
    #[must_use]
    pub fn wspflash(&mut self) -> WspflashW<FconSpec> {
        WspflashW::new(self, 0)
    }
    #[doc = "Bit 4 - Wait State for Error Correction of PFLASH"]
    #[inline(always)]
    #[must_use]
    pub fn wsecpf(&mut self) -> WsecpfW<FconSpec> {
        WsecpfW::new(self, 4)
    }
    #[doc = "Bit 13 - Dynamic Flash Idle"]
    #[inline(always)]
    #[must_use]
    pub fn idle(&mut self) -> IdleW<FconSpec> {
        IdleW::new(self, 13)
    }
    #[doc = "Bit 14 - External Sleep Request Disable"]
    #[inline(always)]
    #[must_use]
    pub fn esldis(&mut self) -> EsldisW<FconSpec> {
        EsldisW::new(self, 14)
    }
    #[doc = "Bit 15 - Flash SLEEP"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SleepW<FconSpec> {
        SleepW::new(self, 15)
    }
    #[doc = "Bit 17 - Disable Code Fetch from Flash Memory"]
    #[inline(always)]
    #[must_use]
    pub fn dcf(&mut self) -> DcfW<FconSpec> {
        DcfW::new(self, 17)
    }
    #[doc = "Bit 18 - Disable Any Data Fetch from Flash"]
    #[inline(always)]
    #[must_use]
    pub fn ddf(&mut self) -> DdfW<FconSpec> {
        DdfW::new(self, 18)
    }
    #[doc = "Bit 24 - Verify and Operation Error Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn voperm(&mut self) -> VopermW<FconSpec> {
        VopermW::new(self, 24)
    }
    #[doc = "Bit 25 - Command Sequence Error Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sqerm(&mut self) -> SqermW<FconSpec> {
        SqermW::new(self, 25)
    }
    #[doc = "Bit 26 - Protection Error Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn proerm(&mut self) -> ProermW<FconSpec> {
        ProermW::new(self, 26)
    }
    #[doc = "Bit 27 - PFLASH Single-Bit Error Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn pfsberm(&mut self) -> PfsbermW<FconSpec> {
        PfsbermW::new(self, 27)
    }
    #[doc = "Bit 29 - PFLASH Double-Bit Error Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn pfdberm(&mut self) -> PfdbermW<FconSpec> {
        PfdbermW::new(self, 29)
    }
    #[doc = "Bit 31 - End of Busy Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn eobm(&mut self) -> EobmW<FconSpec> {
        EobmW::new(self, 31)
    }
}
#[doc = "Flash Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FconSpec;
impl crate::RegisterSpec for FconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcon::R`](R) reader structure"]
impl crate::Readable for FconSpec {}
#[doc = "`write(|w| ..)` method takes [`fcon::W`](W) writer structure"]
impl crate::Writable for FconSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCON to value 0x06"]
impl crate::Resettable for FconSpec {
    const RESET_VALUE: u32 = 0x06;
}
