#[doc = "Register `CAPABILITIES` reader"]
pub struct R(crate::R<CAPABILITIES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPABILITIES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPABILITIES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPABILITIES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Timeout Clock Frequency\n\nValue on reset: 48"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMEOUT_CLOCK_FREQ_A {
    #[doc = "48: 48 MHz"]
    VALUE1 = 48,
}
impl From<TIMEOUT_CLOCK_FREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUT_CLOCK_FREQ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMEOUT_CLOCK_FREQ` reader - Timeout Clock Frequency"]
pub struct TIMEOUT_CLOCK_FREQ_R(crate::FieldReader<u8, TIMEOUT_CLOCK_FREQ_A>);
impl TIMEOUT_CLOCK_FREQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        TIMEOUT_CLOCK_FREQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIMEOUT_CLOCK_FREQ_A> {
        match self.bits {
            48 => Some(TIMEOUT_CLOCK_FREQ_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TIMEOUT_CLOCK_FREQ_A::VALUE1
    }
}
impl core::ops::Deref for TIMEOUT_CLOCK_FREQ_R {
    type Target = crate::FieldReader<u8, TIMEOUT_CLOCK_FREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Timeout Clock Unit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUT_CLOCK_UNIT_A {
    #[doc = "1: MHz"]
    VALUE1 = 1,
}
impl From<TIMEOUT_CLOCK_UNIT_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUT_CLOCK_UNIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUT_CLOCK_UNIT` reader - Timeout Clock Unit"]
pub struct TIMEOUT_CLOCK_UNIT_R(crate::FieldReader<bool, TIMEOUT_CLOCK_UNIT_A>);
impl TIMEOUT_CLOCK_UNIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMEOUT_CLOCK_UNIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIMEOUT_CLOCK_UNIT_A> {
        match self.bits {
            true => Some(TIMEOUT_CLOCK_UNIT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TIMEOUT_CLOCK_UNIT_A::VALUE1
    }
}
impl core::ops::Deref for TIMEOUT_CLOCK_UNIT_R {
    type Target = crate::FieldReader<bool, TIMEOUT_CLOCK_UNIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Base Clock Frequency for SD Clock\n\nValue on reset: 48"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BASE_SD_CLOCK_FREQ_A {
    #[doc = "48: 48 MHz"]
    VALUE1 = 48,
}
impl From<BASE_SD_CLOCK_FREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: BASE_SD_CLOCK_FREQ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BASE_SD_CLOCK_FREQ` reader - Base Clock Frequency for SD Clock"]
pub struct BASE_SD_CLOCK_FREQ_R(crate::FieldReader<u8, BASE_SD_CLOCK_FREQ_A>);
impl BASE_SD_CLOCK_FREQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        BASE_SD_CLOCK_FREQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BASE_SD_CLOCK_FREQ_A> {
        match self.bits {
            48 => Some(BASE_SD_CLOCK_FREQ_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BASE_SD_CLOCK_FREQ_A::VALUE1
    }
}
impl core::ops::Deref for BASE_SD_CLOCK_FREQ_R {
    type Target = crate::FieldReader<u8, BASE_SD_CLOCK_FREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Max Block Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MAX_BLOCK_LENGTH_A {
    #[doc = "0: 512 byte"]
    VALUE1 = 0,
}
impl From<MAX_BLOCK_LENGTH_A> for u8 {
    #[inline(always)]
    fn from(variant: MAX_BLOCK_LENGTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MAX_BLOCK_LENGTH` reader - Max Block Length"]
pub struct MAX_BLOCK_LENGTH_R(crate::FieldReader<u8, MAX_BLOCK_LENGTH_A>);
impl MAX_BLOCK_LENGTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        MAX_BLOCK_LENGTH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MAX_BLOCK_LENGTH_A> {
        match self.bits {
            0 => Some(MAX_BLOCK_LENGTH_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MAX_BLOCK_LENGTH_A::VALUE1
    }
}
impl core::ops::Deref for MAX_BLOCK_LENGTH_R {
    type Target = crate::FieldReader<u8, MAX_BLOCK_LENGTH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Extended Media Bus Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXT_MEDIA_BUS_SUPPORT_A {
    #[doc = "0: Extended Media Bus not supported"]
    VALUE1 = 0,
}
impl From<EXT_MEDIA_BUS_SUPPORT_A> for bool {
    #[inline(always)]
    fn from(variant: EXT_MEDIA_BUS_SUPPORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXT_MEDIA_BUS_SUPPORT` reader - Extended Media Bus Support"]
pub struct EXT_MEDIA_BUS_SUPPORT_R(crate::FieldReader<bool, EXT_MEDIA_BUS_SUPPORT_A>);
impl EXT_MEDIA_BUS_SUPPORT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXT_MEDIA_BUS_SUPPORT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXT_MEDIA_BUS_SUPPORT_A> {
        match self.bits {
            false => Some(EXT_MEDIA_BUS_SUPPORT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EXT_MEDIA_BUS_SUPPORT_A::VALUE1
    }
}
impl core::ops::Deref for EXT_MEDIA_BUS_SUPPORT_R {
    type Target = crate::FieldReader<bool, EXT_MEDIA_BUS_SUPPORT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ADMA2 Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADMA2_SUPPORT_A {
    #[doc = "0: ADMA not supported"]
    VALUE1 = 0,
}
impl From<ADMA2_SUPPORT_A> for bool {
    #[inline(always)]
    fn from(variant: ADMA2_SUPPORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADMA2_SUPPORT` reader - ADMA2 Support"]
pub struct ADMA2_SUPPORT_R(crate::FieldReader<bool, ADMA2_SUPPORT_A>);
impl ADMA2_SUPPORT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADMA2_SUPPORT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADMA2_SUPPORT_A> {
        match self.bits {
            false => Some(ADMA2_SUPPORT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ADMA2_SUPPORT_A::VALUE1
    }
}
impl core::ops::Deref for ADMA2_SUPPORT_R {
    type Target = crate::FieldReader<bool, ADMA2_SUPPORT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "High Speed Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIGH_SPEED_SUPPORT_A {
    #[doc = "1: High Speed supported"]
    VALUE1 = 1,
}
impl From<HIGH_SPEED_SUPPORT_A> for bool {
    #[inline(always)]
    fn from(variant: HIGH_SPEED_SUPPORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIGH_SPEED_SUPPORT` reader - High Speed Support"]
pub struct HIGH_SPEED_SUPPORT_R(crate::FieldReader<bool, HIGH_SPEED_SUPPORT_A>);
impl HIGH_SPEED_SUPPORT_R {
    pub(crate) fn new(bits: bool) -> Self {
        HIGH_SPEED_SUPPORT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HIGH_SPEED_SUPPORT_A> {
        match self.bits {
            true => Some(HIGH_SPEED_SUPPORT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == HIGH_SPEED_SUPPORT_A::VALUE1
    }
}
impl core::ops::Deref for HIGH_SPEED_SUPPORT_R {
    type Target = crate::FieldReader<bool, HIGH_SPEED_SUPPORT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SDMA Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMA_SUPPORT_A {
    #[doc = "0: SDMA not supported"]
    VALUE1 = 0,
}
impl From<SDMA_SUPPORT_A> for bool {
    #[inline(always)]
    fn from(variant: SDMA_SUPPORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMA_SUPPORT` reader - SDMA Support"]
pub struct SDMA_SUPPORT_R(crate::FieldReader<bool, SDMA_SUPPORT_A>);
impl SDMA_SUPPORT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDMA_SUPPORT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SDMA_SUPPORT_A> {
        match self.bits {
            false => Some(SDMA_SUPPORT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SDMA_SUPPORT_A::VALUE1
    }
}
impl core::ops::Deref for SDMA_SUPPORT_R {
    type Target = crate::FieldReader<bool, SDMA_SUPPORT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Suspend / Resume Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSPEND_RESUME_SUPPORT_A {
    #[doc = "1: Supported"]
    VALUE1 = 1,
}
impl From<SUSPEND_RESUME_SUPPORT_A> for bool {
    #[inline(always)]
    fn from(variant: SUSPEND_RESUME_SUPPORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPEND_RESUME_SUPPORT` reader - Suspend / Resume Support"]
pub struct SUSPEND_RESUME_SUPPORT_R(crate::FieldReader<bool, SUSPEND_RESUME_SUPPORT_A>);
impl SUSPEND_RESUME_SUPPORT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUSPEND_RESUME_SUPPORT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SUSPEND_RESUME_SUPPORT_A> {
        match self.bits {
            true => Some(SUSPEND_RESUME_SUPPORT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SUSPEND_RESUME_SUPPORT_A::VALUE1
    }
}
impl core::ops::Deref for SUSPEND_RESUME_SUPPORT_R {
    type Target = crate::FieldReader<bool, SUSPEND_RESUME_SUPPORT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Voltage Support 3.3V\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VOLTAGE_SUPPORT_3_3V_A {
    #[doc = "1: 3.3V supported"]
    VALUE1 = 1,
}
impl From<VOLTAGE_SUPPORT_3_3V_A> for bool {
    #[inline(always)]
    fn from(variant: VOLTAGE_SUPPORT_3_3V_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOLTAGE_SUPPORT_3_3V` reader - Voltage Support 3.3V"]
pub struct VOLTAGE_SUPPORT_3_3V_R(crate::FieldReader<bool, VOLTAGE_SUPPORT_3_3V_A>);
impl VOLTAGE_SUPPORT_3_3V_R {
    pub(crate) fn new(bits: bool) -> Self {
        VOLTAGE_SUPPORT_3_3V_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VOLTAGE_SUPPORT_3_3V_A> {
        match self.bits {
            true => Some(VOLTAGE_SUPPORT_3_3V_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == VOLTAGE_SUPPORT_3_3V_A::VALUE1
    }
}
impl core::ops::Deref for VOLTAGE_SUPPORT_3_3V_R {
    type Target = crate::FieldReader<bool, VOLTAGE_SUPPORT_3_3V_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Voltage Support 3.0V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VOLTAGE_SUPPORT_3V_A {
    #[doc = "0: 3.0V not supported"]
    VALUE1 = 0,
}
impl From<VOLTAGE_SUPPORT_3V_A> for bool {
    #[inline(always)]
    fn from(variant: VOLTAGE_SUPPORT_3V_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOLTAGE_SUPPORT_3V` reader - Voltage Support 3.0V"]
pub struct VOLTAGE_SUPPORT_3V_R(crate::FieldReader<bool, VOLTAGE_SUPPORT_3V_A>);
impl VOLTAGE_SUPPORT_3V_R {
    pub(crate) fn new(bits: bool) -> Self {
        VOLTAGE_SUPPORT_3V_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VOLTAGE_SUPPORT_3V_A> {
        match self.bits {
            false => Some(VOLTAGE_SUPPORT_3V_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == VOLTAGE_SUPPORT_3V_A::VALUE1
    }
}
impl core::ops::Deref for VOLTAGE_SUPPORT_3V_R {
    type Target = crate::FieldReader<bool, VOLTAGE_SUPPORT_3V_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Voltage Support 1.8V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VOLTAGE_SUPPORT_1_8V_A {
    #[doc = "0: 1.8V not supported"]
    VALUE1 = 0,
}
impl From<VOLTAGE_SUPPORT_1_8V_A> for bool {
    #[inline(always)]
    fn from(variant: VOLTAGE_SUPPORT_1_8V_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOLTAGE_SUPPORT_1_8V` reader - Voltage Support 1.8V"]
pub struct VOLTAGE_SUPPORT_1_8V_R(crate::FieldReader<bool, VOLTAGE_SUPPORT_1_8V_A>);
impl VOLTAGE_SUPPORT_1_8V_R {
    pub(crate) fn new(bits: bool) -> Self {
        VOLTAGE_SUPPORT_1_8V_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VOLTAGE_SUPPORT_1_8V_A> {
        match self.bits {
            false => Some(VOLTAGE_SUPPORT_1_8V_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == VOLTAGE_SUPPORT_1_8V_A::VALUE1
    }
}
impl core::ops::Deref for VOLTAGE_SUPPORT_1_8V_R {
    type Target = crate::FieldReader<bool, VOLTAGE_SUPPORT_1_8V_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "64-bit System Bus Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSBUS_64_SUPPORT_A {
    #[doc = "0: Does not support 64-bit system address"]
    VALUE1 = 0,
}
impl From<SYSBUS_64_SUPPORT_A> for bool {
    #[inline(always)]
    fn from(variant: SYSBUS_64_SUPPORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSBUS_64_SUPPORT` reader - 64-bit System Bus Support"]
pub struct SYSBUS_64_SUPPORT_R(crate::FieldReader<bool, SYSBUS_64_SUPPORT_A>);
impl SYSBUS_64_SUPPORT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSBUS_64_SUPPORT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSBUS_64_SUPPORT_A> {
        match self.bits {
            false => Some(SYSBUS_64_SUPPORT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SYSBUS_64_SUPPORT_A::VALUE1
    }
}
impl core::ops::Deref for SYSBUS_64_SUPPORT_R {
    type Target = crate::FieldReader<bool, SYSBUS_64_SUPPORT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Asynchronous Interrupt Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASYNC_INT_SUPPORT_A {
    #[doc = "0: Asynchronous Interrupt not supported"]
    VALUE1 = 0,
}
impl From<ASYNC_INT_SUPPORT_A> for bool {
    #[inline(always)]
    fn from(variant: ASYNC_INT_SUPPORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASYNC_INT_SUPPORT` reader - Asynchronous Interrupt Support"]
pub struct ASYNC_INT_SUPPORT_R(crate::FieldReader<bool, ASYNC_INT_SUPPORT_A>);
impl ASYNC_INT_SUPPORT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASYNC_INT_SUPPORT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ASYNC_INT_SUPPORT_A> {
        match self.bits {
            false => Some(ASYNC_INT_SUPPORT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ASYNC_INT_SUPPORT_A::VALUE1
    }
}
impl core::ops::Deref for ASYNC_INT_SUPPORT_R {
    type Target = crate::FieldReader<bool, ASYNC_INT_SUPPORT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Slot Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SLOT_TYPE_A {
    #[doc = "0: Removable Card Slot"]
    VALUE1 = 0,
}
impl From<SLOT_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: SLOT_TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SLOT_TYPE` reader - Slot Type"]
pub struct SLOT_TYPE_R(crate::FieldReader<u8, SLOT_TYPE_A>);
impl SLOT_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SLOT_TYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SLOT_TYPE_A> {
        match self.bits {
            0 => Some(SLOT_TYPE_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SLOT_TYPE_A::VALUE1
    }
}
impl core::ops::Deref for SLOT_TYPE_R {
    type Target = crate::FieldReader<u8, SLOT_TYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5 - Timeout Clock Frequency"]
    #[inline(always)]
    pub fn timeout_clock_freq(&self) -> TIMEOUT_CLOCK_FREQ_R {
        TIMEOUT_CLOCK_FREQ_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Timeout Clock Unit"]
    #[inline(always)]
    pub fn timeout_clock_unit(&self) -> TIMEOUT_CLOCK_UNIT_R {
        TIMEOUT_CLOCK_UNIT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Base Clock Frequency for SD Clock"]
    #[inline(always)]
    pub fn base_sd_clock_freq(&self) -> BASE_SD_CLOCK_FREQ_R {
        BASE_SD_CLOCK_FREQ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Max Block Length"]
    #[inline(always)]
    pub fn max_block_length(&self) -> MAX_BLOCK_LENGTH_R {
        MAX_BLOCK_LENGTH_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Extended Media Bus Support"]
    #[inline(always)]
    pub fn ext_media_bus_support(&self) -> EXT_MEDIA_BUS_SUPPORT_R {
        EXT_MEDIA_BUS_SUPPORT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ADMA2 Support"]
    #[inline(always)]
    pub fn adma2_support(&self) -> ADMA2_SUPPORT_R {
        ADMA2_SUPPORT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - High Speed Support"]
    #[inline(always)]
    pub fn high_speed_support(&self) -> HIGH_SPEED_SUPPORT_R {
        HIGH_SPEED_SUPPORT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SDMA Support"]
    #[inline(always)]
    pub fn sdma_support(&self) -> SDMA_SUPPORT_R {
        SDMA_SUPPORT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Suspend / Resume Support"]
    #[inline(always)]
    pub fn suspend_resume_support(&self) -> SUSPEND_RESUME_SUPPORT_R {
        SUSPEND_RESUME_SUPPORT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Voltage Support 3.3V"]
    #[inline(always)]
    pub fn voltage_support_3_3v(&self) -> VOLTAGE_SUPPORT_3_3V_R {
        VOLTAGE_SUPPORT_3_3V_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Voltage Support 3.0V"]
    #[inline(always)]
    pub fn voltage_support_3v(&self) -> VOLTAGE_SUPPORT_3V_R {
        VOLTAGE_SUPPORT_3V_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Voltage Support 1.8V"]
    #[inline(always)]
    pub fn voltage_support_1_8v(&self) -> VOLTAGE_SUPPORT_1_8V_R {
        VOLTAGE_SUPPORT_1_8V_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 64-bit System Bus Support"]
    #[inline(always)]
    pub fn sysbus_64_support(&self) -> SYSBUS_64_SUPPORT_R {
        SYSBUS_64_SUPPORT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Asynchronous Interrupt Support"]
    #[inline(always)]
    pub fn async_int_support(&self) -> ASYNC_INT_SUPPORT_R {
        ASYNC_INT_SUPPORT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - Slot Type"]
    #[inline(always)]
    pub fn slot_type(&self) -> SLOT_TYPE_R {
        SLOT_TYPE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
#[doc = "Capabilities Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capabilities](index.html) module"]
pub struct CAPABILITIES_SPEC;
impl crate::RegisterSpec for CAPABILITIES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [capabilities::R](R) reader structure"]
impl crate::Readable for CAPABILITIES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAPABILITIES to value 0x01a0_30b0"]
impl crate::Resettable for CAPABILITIES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01a0_30b0
    }
}
