#[doc = "Reader of register CAPABILITIES"]
pub type R = crate::R<u32, super::CAPABILITIES>;
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
#[doc = "Reader of field `TIMEOUT_CLOCK_FREQ`"]
pub type TIMEOUT_CLOCK_FREQ_R = crate::R<u8, TIMEOUT_CLOCK_FREQ_A>;
impl TIMEOUT_CLOCK_FREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIMEOUT_CLOCK_FREQ_A> {
        use crate::Variant::*;
        match self.bits {
            48 => Val(TIMEOUT_CLOCK_FREQ_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TIMEOUT_CLOCK_FREQ_A::VALUE1
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
#[doc = "Reader of field `TIMEOUT_CLOCK_UNIT`"]
pub type TIMEOUT_CLOCK_UNIT_R = crate::R<bool, TIMEOUT_CLOCK_UNIT_A>;
impl TIMEOUT_CLOCK_UNIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, TIMEOUT_CLOCK_UNIT_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(TIMEOUT_CLOCK_UNIT_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TIMEOUT_CLOCK_UNIT_A::VALUE1
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
#[doc = "Reader of field `BASE_SD_CLOCK_FREQ`"]
pub type BASE_SD_CLOCK_FREQ_R = crate::R<u8, BASE_SD_CLOCK_FREQ_A>;
impl BASE_SD_CLOCK_FREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BASE_SD_CLOCK_FREQ_A> {
        use crate::Variant::*;
        match self.bits {
            48 => Val(BASE_SD_CLOCK_FREQ_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BASE_SD_CLOCK_FREQ_A::VALUE1
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
#[doc = "Reader of field `MAX_BLOCK_LENGTH`"]
pub type MAX_BLOCK_LENGTH_R = crate::R<u8, MAX_BLOCK_LENGTH_A>;
impl MAX_BLOCK_LENGTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MAX_BLOCK_LENGTH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MAX_BLOCK_LENGTH_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MAX_BLOCK_LENGTH_A::VALUE1
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
#[doc = "Reader of field `EXT_MEDIA_BUS_SUPPORT`"]
pub type EXT_MEDIA_BUS_SUPPORT_R = crate::R<bool, EXT_MEDIA_BUS_SUPPORT_A>;
impl EXT_MEDIA_BUS_SUPPORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, EXT_MEDIA_BUS_SUPPORT_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(EXT_MEDIA_BUS_SUPPORT_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EXT_MEDIA_BUS_SUPPORT_A::VALUE1
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
#[doc = "Reader of field `ADMA2_SUPPORT`"]
pub type ADMA2_SUPPORT_R = crate::R<bool, ADMA2_SUPPORT_A>;
impl ADMA2_SUPPORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ADMA2_SUPPORT_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(ADMA2_SUPPORT_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ADMA2_SUPPORT_A::VALUE1
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
#[doc = "Reader of field `HIGH_SPEED_SUPPORT`"]
pub type HIGH_SPEED_SUPPORT_R = crate::R<bool, HIGH_SPEED_SUPPORT_A>;
impl HIGH_SPEED_SUPPORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, HIGH_SPEED_SUPPORT_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(HIGH_SPEED_SUPPORT_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HIGH_SPEED_SUPPORT_A::VALUE1
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
#[doc = "Reader of field `SDMA_SUPPORT`"]
pub type SDMA_SUPPORT_R = crate::R<bool, SDMA_SUPPORT_A>;
impl SDMA_SUPPORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SDMA_SUPPORT_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(SDMA_SUPPORT_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SDMA_SUPPORT_A::VALUE1
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
#[doc = "Reader of field `SUSPEND_RESUME_SUPPORT`"]
pub type SUSPEND_RESUME_SUPPORT_R = crate::R<bool, SUSPEND_RESUME_SUPPORT_A>;
impl SUSPEND_RESUME_SUPPORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SUSPEND_RESUME_SUPPORT_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SUSPEND_RESUME_SUPPORT_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SUSPEND_RESUME_SUPPORT_A::VALUE1
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
#[doc = "Reader of field `VOLTAGE_SUPPORT_3_3V`"]
pub type VOLTAGE_SUPPORT_3_3V_R = crate::R<bool, VOLTAGE_SUPPORT_3_3V_A>;
impl VOLTAGE_SUPPORT_3_3V_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, VOLTAGE_SUPPORT_3_3V_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(VOLTAGE_SUPPORT_3_3V_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VOLTAGE_SUPPORT_3_3V_A::VALUE1
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
#[doc = "Reader of field `VOLTAGE_SUPPORT_3V`"]
pub type VOLTAGE_SUPPORT_3V_R = crate::R<bool, VOLTAGE_SUPPORT_3V_A>;
impl VOLTAGE_SUPPORT_3V_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, VOLTAGE_SUPPORT_3V_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(VOLTAGE_SUPPORT_3V_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VOLTAGE_SUPPORT_3V_A::VALUE1
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
#[doc = "Reader of field `VOLTAGE_SUPPORT_1_8V`"]
pub type VOLTAGE_SUPPORT_1_8V_R = crate::R<bool, VOLTAGE_SUPPORT_1_8V_A>;
impl VOLTAGE_SUPPORT_1_8V_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, VOLTAGE_SUPPORT_1_8V_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(VOLTAGE_SUPPORT_1_8V_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VOLTAGE_SUPPORT_1_8V_A::VALUE1
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
#[doc = "Reader of field `SYSBUS_64_SUPPORT`"]
pub type SYSBUS_64_SUPPORT_R = crate::R<bool, SYSBUS_64_SUPPORT_A>;
impl SYSBUS_64_SUPPORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SYSBUS_64_SUPPORT_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(SYSBUS_64_SUPPORT_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYSBUS_64_SUPPORT_A::VALUE1
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
#[doc = "Reader of field `ASYNC_INT_SUPPORT`"]
pub type ASYNC_INT_SUPPORT_R = crate::R<bool, ASYNC_INT_SUPPORT_A>;
impl ASYNC_INT_SUPPORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ASYNC_INT_SUPPORT_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(ASYNC_INT_SUPPORT_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASYNC_INT_SUPPORT_A::VALUE1
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
#[doc = "Reader of field `SLOT_TYPE`"]
pub type SLOT_TYPE_R = crate::R<u8, SLOT_TYPE_A>;
impl SLOT_TYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SLOT_TYPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SLOT_TYPE_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
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
