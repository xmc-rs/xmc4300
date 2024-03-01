#[doc = "Register `CAPABILITIES` reader"]
pub type R = crate::R<CapabilitiesSpec>;
#[doc = "Timeout Clock Frequency\n\nValue on reset: 48"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TimeoutClockFreq {
    #[doc = "48: 48 MHz"]
    Value1 = 48,
}
impl From<TimeoutClockFreq> for u8 {
    #[inline(always)]
    fn from(variant: TimeoutClockFreq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TimeoutClockFreq {
    type Ux = u8;
}
#[doc = "Field `TIMEOUT_CLOCK_FREQ` reader - Timeout Clock Frequency"]
pub type TimeoutClockFreqR = crate::FieldReader<TimeoutClockFreq>;
impl TimeoutClockFreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TimeoutClockFreq> {
        match self.bits {
            48 => Some(TimeoutClockFreq::Value1),
            _ => None,
        }
    }
    #[doc = "48 MHz"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TimeoutClockFreq::Value1
    }
}
#[doc = "Timeout Clock Unit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeoutClockUnit {
    #[doc = "1: MHz"]
    Value1 = 1,
}
impl From<TimeoutClockUnit> for bool {
    #[inline(always)]
    fn from(variant: TimeoutClockUnit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUT_CLOCK_UNIT` reader - Timeout Clock Unit"]
pub type TimeoutClockUnitR = crate::BitReader<TimeoutClockUnit>;
impl TimeoutClockUnitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TimeoutClockUnit> {
        match self.bits {
            true => Some(TimeoutClockUnit::Value1),
            _ => None,
        }
    }
    #[doc = "MHz"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TimeoutClockUnit::Value1
    }
}
#[doc = "Base Clock Frequency for SD Clock\n\nValue on reset: 48"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BaseSdClockFreq {
    #[doc = "48: 48 MHz"]
    Value1 = 48,
}
impl From<BaseSdClockFreq> for u8 {
    #[inline(always)]
    fn from(variant: BaseSdClockFreq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BaseSdClockFreq {
    type Ux = u8;
}
#[doc = "Field `BASE_SD_CLOCK_FREQ` reader - Base Clock Frequency for SD Clock"]
pub type BaseSdClockFreqR = crate::FieldReader<BaseSdClockFreq>;
impl BaseSdClockFreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BaseSdClockFreq> {
        match self.bits {
            48 => Some(BaseSdClockFreq::Value1),
            _ => None,
        }
    }
    #[doc = "48 MHz"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BaseSdClockFreq::Value1
    }
}
#[doc = "Max Block Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MaxBlockLength {
    #[doc = "0: 512 byte"]
    Value1 = 0,
}
impl From<MaxBlockLength> for u8 {
    #[inline(always)]
    fn from(variant: MaxBlockLength) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MaxBlockLength {
    type Ux = u8;
}
#[doc = "Field `MAX_BLOCK_LENGTH` reader - Max Block Length"]
pub type MaxBlockLengthR = crate::FieldReader<MaxBlockLength>;
impl MaxBlockLengthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MaxBlockLength> {
        match self.bits {
            0 => Some(MaxBlockLength::Value1),
            _ => None,
        }
    }
    #[doc = "512 byte"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MaxBlockLength::Value1
    }
}
#[doc = "Extended Media Bus Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExtMediaBusSupport {
    #[doc = "0: Extended Media Bus not supported"]
    Value1 = 0,
}
impl From<ExtMediaBusSupport> for bool {
    #[inline(always)]
    fn from(variant: ExtMediaBusSupport) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXT_MEDIA_BUS_SUPPORT` reader - Extended Media Bus Support"]
pub type ExtMediaBusSupportR = crate::BitReader<ExtMediaBusSupport>;
impl ExtMediaBusSupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ExtMediaBusSupport> {
        match self.bits {
            false => Some(ExtMediaBusSupport::Value1),
            _ => None,
        }
    }
    #[doc = "Extended Media Bus not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ExtMediaBusSupport::Value1
    }
}
#[doc = "ADMA2 Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adma2Support {
    #[doc = "0: ADMA not supported"]
    Value1 = 0,
}
impl From<Adma2Support> for bool {
    #[inline(always)]
    fn from(variant: Adma2Support) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADMA2_SUPPORT` reader - ADMA2 Support"]
pub type Adma2SupportR = crate::BitReader<Adma2Support>;
impl Adma2SupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Adma2Support> {
        match self.bits {
            false => Some(Adma2Support::Value1),
            _ => None,
        }
    }
    #[doc = "ADMA not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Adma2Support::Value1
    }
}
#[doc = "High Speed Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HighSpeedSupport {
    #[doc = "1: High Speed supported"]
    Value1 = 1,
}
impl From<HighSpeedSupport> for bool {
    #[inline(always)]
    fn from(variant: HighSpeedSupport) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIGH_SPEED_SUPPORT` reader - High Speed Support"]
pub type HighSpeedSupportR = crate::BitReader<HighSpeedSupport>;
impl HighSpeedSupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HighSpeedSupport> {
        match self.bits {
            true => Some(HighSpeedSupport::Value1),
            _ => None,
        }
    }
    #[doc = "High Speed supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HighSpeedSupport::Value1
    }
}
#[doc = "SDMA Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdmaSupport {
    #[doc = "0: SDMA not supported"]
    Value1 = 0,
}
impl From<SdmaSupport> for bool {
    #[inline(always)]
    fn from(variant: SdmaSupport) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMA_SUPPORT` reader - SDMA Support"]
pub type SdmaSupportR = crate::BitReader<SdmaSupport>;
impl SdmaSupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SdmaSupport> {
        match self.bits {
            false => Some(SdmaSupport::Value1),
            _ => None,
        }
    }
    #[doc = "SDMA not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SdmaSupport::Value1
    }
}
#[doc = "Suspend / Resume Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SuspendResumeSupport {
    #[doc = "1: Supported"]
    Value1 = 1,
}
impl From<SuspendResumeSupport> for bool {
    #[inline(always)]
    fn from(variant: SuspendResumeSupport) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPEND_RESUME_SUPPORT` reader - Suspend / Resume Support"]
pub type SuspendResumeSupportR = crate::BitReader<SuspendResumeSupport>;
impl SuspendResumeSupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SuspendResumeSupport> {
        match self.bits {
            true => Some(SuspendResumeSupport::Value1),
            _ => None,
        }
    }
    #[doc = "Supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SuspendResumeSupport::Value1
    }
}
#[doc = "Voltage Support 3.3V\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VoltageSupport3_3v {
    #[doc = "1: 3.3V supported"]
    Value1 = 1,
}
impl From<VoltageSupport3_3v> for bool {
    #[inline(always)]
    fn from(variant: VoltageSupport3_3v) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOLTAGE_SUPPORT_3_3V` reader - Voltage Support 3.3V"]
pub type VoltageSupport3_3vR = crate::BitReader<VoltageSupport3_3v>;
impl VoltageSupport3_3vR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VoltageSupport3_3v> {
        match self.bits {
            true => Some(VoltageSupport3_3v::Value1),
            _ => None,
        }
    }
    #[doc = "3.3V supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VoltageSupport3_3v::Value1
    }
}
#[doc = "Voltage Support 3.0V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VoltageSupport3v {
    #[doc = "0: 3.0V not supported"]
    Value1 = 0,
}
impl From<VoltageSupport3v> for bool {
    #[inline(always)]
    fn from(variant: VoltageSupport3v) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOLTAGE_SUPPORT_3V` reader - Voltage Support 3.0V"]
pub type VoltageSupport3vR = crate::BitReader<VoltageSupport3v>;
impl VoltageSupport3vR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VoltageSupport3v> {
        match self.bits {
            false => Some(VoltageSupport3v::Value1),
            _ => None,
        }
    }
    #[doc = "3.0V not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VoltageSupport3v::Value1
    }
}
#[doc = "Voltage Support 1.8V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VoltageSupport1_8v {
    #[doc = "0: 1.8V not supported"]
    Value1 = 0,
}
impl From<VoltageSupport1_8v> for bool {
    #[inline(always)]
    fn from(variant: VoltageSupport1_8v) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOLTAGE_SUPPORT_1_8V` reader - Voltage Support 1.8V"]
pub type VoltageSupport1_8vR = crate::BitReader<VoltageSupport1_8v>;
impl VoltageSupport1_8vR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VoltageSupport1_8v> {
        match self.bits {
            false => Some(VoltageSupport1_8v::Value1),
            _ => None,
        }
    }
    #[doc = "1.8V not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VoltageSupport1_8v::Value1
    }
}
#[doc = "64-bit System Bus Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sysbus64Support {
    #[doc = "0: Does not support 64-bit system address"]
    Value1 = 0,
}
impl From<Sysbus64Support> for bool {
    #[inline(always)]
    fn from(variant: Sysbus64Support) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSBUS_64_SUPPORT` reader - 64-bit System Bus Support"]
pub type Sysbus64SupportR = crate::BitReader<Sysbus64Support>;
impl Sysbus64SupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sysbus64Support> {
        match self.bits {
            false => Some(Sysbus64Support::Value1),
            _ => None,
        }
    }
    #[doc = "Does not support 64-bit system address"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sysbus64Support::Value1
    }
}
#[doc = "Asynchronous Interrupt Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AsyncIntSupport {
    #[doc = "0: Asynchronous Interrupt not supported"]
    Value1 = 0,
}
impl From<AsyncIntSupport> for bool {
    #[inline(always)]
    fn from(variant: AsyncIntSupport) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASYNC_INT_SUPPORT` reader - Asynchronous Interrupt Support"]
pub type AsyncIntSupportR = crate::BitReader<AsyncIntSupport>;
impl AsyncIntSupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AsyncIntSupport> {
        match self.bits {
            false => Some(AsyncIntSupport::Value1),
            _ => None,
        }
    }
    #[doc = "Asynchronous Interrupt not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AsyncIntSupport::Value1
    }
}
#[doc = "Slot Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SlotType {
    #[doc = "0: Removable Card Slot"]
    Value1 = 0,
}
impl From<SlotType> for u8 {
    #[inline(always)]
    fn from(variant: SlotType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SlotType {
    type Ux = u8;
}
#[doc = "Field `SLOT_TYPE` reader - Slot Type"]
pub type SlotTypeR = crate::FieldReader<SlotType>;
impl SlotTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SlotType> {
        match self.bits {
            0 => Some(SlotType::Value1),
            _ => None,
        }
    }
    #[doc = "Removable Card Slot"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SlotType::Value1
    }
}
impl R {
    #[doc = "Bits 0:5 - Timeout Clock Frequency"]
    #[inline(always)]
    pub fn timeout_clock_freq(&self) -> TimeoutClockFreqR {
        TimeoutClockFreqR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Timeout Clock Unit"]
    #[inline(always)]
    pub fn timeout_clock_unit(&self) -> TimeoutClockUnitR {
        TimeoutClockUnitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Base Clock Frequency for SD Clock"]
    #[inline(always)]
    pub fn base_sd_clock_freq(&self) -> BaseSdClockFreqR {
        BaseSdClockFreqR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Max Block Length"]
    #[inline(always)]
    pub fn max_block_length(&self) -> MaxBlockLengthR {
        MaxBlockLengthR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Extended Media Bus Support"]
    #[inline(always)]
    pub fn ext_media_bus_support(&self) -> ExtMediaBusSupportR {
        ExtMediaBusSupportR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADMA2 Support"]
    #[inline(always)]
    pub fn adma2_support(&self) -> Adma2SupportR {
        Adma2SupportR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - High Speed Support"]
    #[inline(always)]
    pub fn high_speed_support(&self) -> HighSpeedSupportR {
        HighSpeedSupportR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDMA Support"]
    #[inline(always)]
    pub fn sdma_support(&self) -> SdmaSupportR {
        SdmaSupportR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Suspend / Resume Support"]
    #[inline(always)]
    pub fn suspend_resume_support(&self) -> SuspendResumeSupportR {
        SuspendResumeSupportR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Voltage Support 3.3V"]
    #[inline(always)]
    pub fn voltage_support_3_3v(&self) -> VoltageSupport3_3vR {
        VoltageSupport3_3vR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Voltage Support 3.0V"]
    #[inline(always)]
    pub fn voltage_support_3v(&self) -> VoltageSupport3vR {
        VoltageSupport3vR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Voltage Support 1.8V"]
    #[inline(always)]
    pub fn voltage_support_1_8v(&self) -> VoltageSupport1_8vR {
        VoltageSupport1_8vR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - 64-bit System Bus Support"]
    #[inline(always)]
    pub fn sysbus_64_support(&self) -> Sysbus64SupportR {
        Sysbus64SupportR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Asynchronous Interrupt Support"]
    #[inline(always)]
    pub fn async_int_support(&self) -> AsyncIntSupportR {
        AsyncIntSupportR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Slot Type"]
    #[inline(always)]
    pub fn slot_type(&self) -> SlotTypeR {
        SlotTypeR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Capabilities Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capabilities::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapabilitiesSpec;
impl crate::RegisterSpec for CapabilitiesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capabilities::R`](R) reader structure"]
impl crate::Readable for CapabilitiesSpec {}
#[doc = "`reset()` method sets CAPABILITIES to value 0x01a0_30b0"]
impl crate::Resettable for CapabilitiesSpec {
    const RESET_VALUE: u32 = 0x01a0_30b0;
}
