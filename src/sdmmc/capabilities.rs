#[doc = "Register `CAPABILITIES` reader"]
pub type R = crate::R<CAPABILITIES_SPEC>;
#[doc = "Field `TIMEOUT_CLOCK_FREQ` reader - Timeout Clock Frequency"]
pub type TIMEOUT_CLOCK_FREQ_R = crate::FieldReader<TIMEOUT_CLOCK_FREQ_A>;
#[doc = "Timeout Clock Frequency\n\nValue on reset: 48"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for TIMEOUT_CLOCK_FREQ_A {
    type Ux = u8;
}
impl TIMEOUT_CLOCK_FREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TIMEOUT_CLOCK_FREQ_A> {
        match self.bits {
            48 => Some(TIMEOUT_CLOCK_FREQ_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "48 MHz"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TIMEOUT_CLOCK_FREQ_A::VALUE1
    }
}
#[doc = "Field `TIMEOUT_CLOCK_UNIT` reader - Timeout Clock Unit"]
pub type TIMEOUT_CLOCK_UNIT_R = crate::BitReader<TIMEOUT_CLOCK_UNIT_A>;
#[doc = "Timeout Clock Unit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TIMEOUT_CLOCK_UNIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TIMEOUT_CLOCK_UNIT_A> {
        match self.bits {
            true => Some(TIMEOUT_CLOCK_UNIT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "MHz"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TIMEOUT_CLOCK_UNIT_A::VALUE1
    }
}
#[doc = "Field `BASE_SD_CLOCK_FREQ` reader - Base Clock Frequency for SD Clock"]
pub type BASE_SD_CLOCK_FREQ_R = crate::FieldReader<BASE_SD_CLOCK_FREQ_A>;
#[doc = "Base Clock Frequency for SD Clock\n\nValue on reset: 48"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for BASE_SD_CLOCK_FREQ_A {
    type Ux = u8;
}
impl BASE_SD_CLOCK_FREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BASE_SD_CLOCK_FREQ_A> {
        match self.bits {
            48 => Some(BASE_SD_CLOCK_FREQ_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "48 MHz"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BASE_SD_CLOCK_FREQ_A::VALUE1
    }
}
#[doc = "Field `MAX_BLOCK_LENGTH` reader - Max Block Length"]
pub type MAX_BLOCK_LENGTH_R = crate::FieldReader<MAX_BLOCK_LENGTH_A>;
#[doc = "Max Block Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for MAX_BLOCK_LENGTH_A {
    type Ux = u8;
}
impl MAX_BLOCK_LENGTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MAX_BLOCK_LENGTH_A> {
        match self.bits {
            0 => Some(MAX_BLOCK_LENGTH_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "512 byte"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MAX_BLOCK_LENGTH_A::VALUE1
    }
}
#[doc = "Field `EXT_MEDIA_BUS_SUPPORT` reader - Extended Media Bus Support"]
pub type EXT_MEDIA_BUS_SUPPORT_R = crate::BitReader<EXT_MEDIA_BUS_SUPPORT_A>;
#[doc = "Extended Media Bus Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl EXT_MEDIA_BUS_SUPPORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXT_MEDIA_BUS_SUPPORT_A> {
        match self.bits {
            false => Some(EXT_MEDIA_BUS_SUPPORT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Extended Media Bus not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EXT_MEDIA_BUS_SUPPORT_A::VALUE1
    }
}
#[doc = "Field `ADMA2_SUPPORT` reader - ADMA2 Support"]
pub type ADMA2_SUPPORT_R = crate::BitReader<ADMA2_SUPPORT_A>;
#[doc = "ADMA2 Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ADMA2_SUPPORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADMA2_SUPPORT_A> {
        match self.bits {
            false => Some(ADMA2_SUPPORT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "ADMA not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ADMA2_SUPPORT_A::VALUE1
    }
}
#[doc = "Field `HIGH_SPEED_SUPPORT` reader - High Speed Support"]
pub type HIGH_SPEED_SUPPORT_R = crate::BitReader<HIGH_SPEED_SUPPORT_A>;
#[doc = "High Speed Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl HIGH_SPEED_SUPPORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HIGH_SPEED_SUPPORT_A> {
        match self.bits {
            true => Some(HIGH_SPEED_SUPPORT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "High Speed supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HIGH_SPEED_SUPPORT_A::VALUE1
    }
}
#[doc = "Field `SDMA_SUPPORT` reader - SDMA Support"]
pub type SDMA_SUPPORT_R = crate::BitReader<SDMA_SUPPORT_A>;
#[doc = "SDMA Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SDMA_SUPPORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SDMA_SUPPORT_A> {
        match self.bits {
            false => Some(SDMA_SUPPORT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "SDMA not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SDMA_SUPPORT_A::VALUE1
    }
}
#[doc = "Field `SUSPEND_RESUME_SUPPORT` reader - Suspend / Resume Support"]
pub type SUSPEND_RESUME_SUPPORT_R = crate::BitReader<SUSPEND_RESUME_SUPPORT_A>;
#[doc = "Suspend / Resume Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SUSPEND_RESUME_SUPPORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SUSPEND_RESUME_SUPPORT_A> {
        match self.bits {
            true => Some(SUSPEND_RESUME_SUPPORT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SUSPEND_RESUME_SUPPORT_A::VALUE1
    }
}
#[doc = "Field `VOLTAGE_SUPPORT_3_3V` reader - Voltage Support 3.3V"]
pub type VOLTAGE_SUPPORT_3_3V_R = crate::BitReader<VOLTAGE_SUPPORT_3_3V_A>;
#[doc = "Voltage Support 3.3V\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl VOLTAGE_SUPPORT_3_3V_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VOLTAGE_SUPPORT_3_3V_A> {
        match self.bits {
            true => Some(VOLTAGE_SUPPORT_3_3V_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "3.3V supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VOLTAGE_SUPPORT_3_3V_A::VALUE1
    }
}
#[doc = "Field `VOLTAGE_SUPPORT_3V` reader - Voltage Support 3.0V"]
pub type VOLTAGE_SUPPORT_3V_R = crate::BitReader<VOLTAGE_SUPPORT_3V_A>;
#[doc = "Voltage Support 3.0V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl VOLTAGE_SUPPORT_3V_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VOLTAGE_SUPPORT_3V_A> {
        match self.bits {
            false => Some(VOLTAGE_SUPPORT_3V_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "3.0V not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VOLTAGE_SUPPORT_3V_A::VALUE1
    }
}
#[doc = "Field `VOLTAGE_SUPPORT_1_8V` reader - Voltage Support 1.8V"]
pub type VOLTAGE_SUPPORT_1_8V_R = crate::BitReader<VOLTAGE_SUPPORT_1_8V_A>;
#[doc = "Voltage Support 1.8V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl VOLTAGE_SUPPORT_1_8V_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VOLTAGE_SUPPORT_1_8V_A> {
        match self.bits {
            false => Some(VOLTAGE_SUPPORT_1_8V_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "1.8V not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VOLTAGE_SUPPORT_1_8V_A::VALUE1
    }
}
#[doc = "Field `SYSBUS_64_SUPPORT` reader - 64-bit System Bus Support"]
pub type SYSBUS_64_SUPPORT_R = crate::BitReader<SYSBUS_64_SUPPORT_A>;
#[doc = "64-bit System Bus Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SYSBUS_64_SUPPORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSBUS_64_SUPPORT_A> {
        match self.bits {
            false => Some(SYSBUS_64_SUPPORT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Does not support 64-bit system address"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYSBUS_64_SUPPORT_A::VALUE1
    }
}
#[doc = "Field `ASYNC_INT_SUPPORT` reader - Asynchronous Interrupt Support"]
pub type ASYNC_INT_SUPPORT_R = crate::BitReader<ASYNC_INT_SUPPORT_A>;
#[doc = "Asynchronous Interrupt Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ASYNC_INT_SUPPORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ASYNC_INT_SUPPORT_A> {
        match self.bits {
            false => Some(ASYNC_INT_SUPPORT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Asynchronous Interrupt not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASYNC_INT_SUPPORT_A::VALUE1
    }
}
#[doc = "Field `SLOT_TYPE` reader - Slot Type"]
pub type SLOT_TYPE_R = crate::FieldReader<SLOT_TYPE_A>;
#[doc = "Slot Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for SLOT_TYPE_A {
    type Ux = u8;
}
impl SLOT_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SLOT_TYPE_A> {
        match self.bits {
            0 => Some(SLOT_TYPE_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Removable Card Slot"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SLOT_TYPE_A::VALUE1
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
        TIMEOUT_CLOCK_UNIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Base Clock Frequency for SD Clock"]
    #[inline(always)]
    pub fn base_sd_clock_freq(&self) -> BASE_SD_CLOCK_FREQ_R {
        BASE_SD_CLOCK_FREQ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Max Block Length"]
    #[inline(always)]
    pub fn max_block_length(&self) -> MAX_BLOCK_LENGTH_R {
        MAX_BLOCK_LENGTH_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Extended Media Bus Support"]
    #[inline(always)]
    pub fn ext_media_bus_support(&self) -> EXT_MEDIA_BUS_SUPPORT_R {
        EXT_MEDIA_BUS_SUPPORT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADMA2 Support"]
    #[inline(always)]
    pub fn adma2_support(&self) -> ADMA2_SUPPORT_R {
        ADMA2_SUPPORT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - High Speed Support"]
    #[inline(always)]
    pub fn high_speed_support(&self) -> HIGH_SPEED_SUPPORT_R {
        HIGH_SPEED_SUPPORT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDMA Support"]
    #[inline(always)]
    pub fn sdma_support(&self) -> SDMA_SUPPORT_R {
        SDMA_SUPPORT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Suspend / Resume Support"]
    #[inline(always)]
    pub fn suspend_resume_support(&self) -> SUSPEND_RESUME_SUPPORT_R {
        SUSPEND_RESUME_SUPPORT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Voltage Support 3.3V"]
    #[inline(always)]
    pub fn voltage_support_3_3v(&self) -> VOLTAGE_SUPPORT_3_3V_R {
        VOLTAGE_SUPPORT_3_3V_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Voltage Support 3.0V"]
    #[inline(always)]
    pub fn voltage_support_3v(&self) -> VOLTAGE_SUPPORT_3V_R {
        VOLTAGE_SUPPORT_3V_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Voltage Support 1.8V"]
    #[inline(always)]
    pub fn voltage_support_1_8v(&self) -> VOLTAGE_SUPPORT_1_8V_R {
        VOLTAGE_SUPPORT_1_8V_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - 64-bit System Bus Support"]
    #[inline(always)]
    pub fn sysbus_64_support(&self) -> SYSBUS_64_SUPPORT_R {
        SYSBUS_64_SUPPORT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Asynchronous Interrupt Support"]
    #[inline(always)]
    pub fn async_int_support(&self) -> ASYNC_INT_SUPPORT_R {
        ASYNC_INT_SUPPORT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Slot Type"]
    #[inline(always)]
    pub fn slot_type(&self) -> SLOT_TYPE_R {
        SLOT_TYPE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Capabilities Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capabilities::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAPABILITIES_SPEC;
impl crate::RegisterSpec for CAPABILITIES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capabilities::R`](R) reader structure"]
impl crate::Readable for CAPABILITIES_SPEC {}
#[doc = "`reset()` method sets CAPABILITIES to value 0x01a0_30b0"]
impl crate::Resettable for CAPABILITIES_SPEC {
    const RESET_VALUE: Self::Ux = 0x01a0_30b0;
}
