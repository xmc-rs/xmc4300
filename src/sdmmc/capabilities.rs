#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CAPABILITIES {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `TIMEOUT_CLOCK_FREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUT_CLOCK_FREQR {
    #[doc = "48 MHz"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TIMEOUT_CLOCK_FREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMEOUT_CLOCK_FREQR::VALUE1 => 48,
            TIMEOUT_CLOCK_FREQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMEOUT_CLOCK_FREQR {
        match value {
            48 => TIMEOUT_CLOCK_FREQR::VALUE1,
            i => TIMEOUT_CLOCK_FREQR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TIMEOUT_CLOCK_FREQR::VALUE1
    }
}
#[doc = "Possible values of the field `TIMEOUT_CLOCK_UNIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUT_CLOCK_UNITR {
    #[doc = "MHz"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TIMEOUT_CLOCK_UNITR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TIMEOUT_CLOCK_UNITR::VALUE1 => true,
            TIMEOUT_CLOCK_UNITR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMEOUT_CLOCK_UNITR {
        match value {
            true => TIMEOUT_CLOCK_UNITR::VALUE1,
            i => TIMEOUT_CLOCK_UNITR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TIMEOUT_CLOCK_UNITR::VALUE1
    }
}
#[doc = "Possible values of the field `BASE_SD_CLOCK_FREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BASE_SD_CLOCK_FREQR {
    #[doc = "48 MHz"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BASE_SD_CLOCK_FREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BASE_SD_CLOCK_FREQR::VALUE1 => 48,
            BASE_SD_CLOCK_FREQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BASE_SD_CLOCK_FREQR {
        match value {
            48 => BASE_SD_CLOCK_FREQR::VALUE1,
            i => BASE_SD_CLOCK_FREQR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BASE_SD_CLOCK_FREQR::VALUE1
    }
}
#[doc = "Possible values of the field `MAX_BLOCK_LENGTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAX_BLOCK_LENGTHR {
    #[doc = "512 byte"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MAX_BLOCK_LENGTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MAX_BLOCK_LENGTHR::VALUE1 => 0,
            MAX_BLOCK_LENGTHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MAX_BLOCK_LENGTHR {
        match value {
            0 => MAX_BLOCK_LENGTHR::VALUE1,
            i => MAX_BLOCK_LENGTHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MAX_BLOCK_LENGTHR::VALUE1
    }
}
#[doc = "Possible values of the field `EXT_MEDIA_BUS_SUPPORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXT_MEDIA_BUS_SUPPORTR {
    #[doc = "Extended Media Bus not supported"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl EXT_MEDIA_BUS_SUPPORTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EXT_MEDIA_BUS_SUPPORTR::VALUE1 => false,
            EXT_MEDIA_BUS_SUPPORTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXT_MEDIA_BUS_SUPPORTR {
        match value {
            false => EXT_MEDIA_BUS_SUPPORTR::VALUE1,
            i => EXT_MEDIA_BUS_SUPPORTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EXT_MEDIA_BUS_SUPPORTR::VALUE1
    }
}
#[doc = "Possible values of the field `ADMA2_SUPPORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADMA2_SUPPORTR {
    #[doc = "ADMA not supported"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADMA2_SUPPORTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ADMA2_SUPPORTR::VALUE1 => false,
            ADMA2_SUPPORTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADMA2_SUPPORTR {
        match value {
            false => ADMA2_SUPPORTR::VALUE1,
            i => ADMA2_SUPPORTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ADMA2_SUPPORTR::VALUE1
    }
}
#[doc = "Possible values of the field `HIGH_SPEED_SUPPORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIGH_SPEED_SUPPORTR {
    #[doc = "High Speed supported"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl HIGH_SPEED_SUPPORTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            HIGH_SPEED_SUPPORTR::VALUE1 => true,
            HIGH_SPEED_SUPPORTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIGH_SPEED_SUPPORTR {
        match value {
            true => HIGH_SPEED_SUPPORTR::VALUE1,
            i => HIGH_SPEED_SUPPORTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HIGH_SPEED_SUPPORTR::VALUE1
    }
}
#[doc = "Possible values of the field `SDMA_SUPPORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMA_SUPPORTR {
    #[doc = "SDMA not supported"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SDMA_SUPPORTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SDMA_SUPPORTR::VALUE1 => false,
            SDMA_SUPPORTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDMA_SUPPORTR {
        match value {
            false => SDMA_SUPPORTR::VALUE1,
            i => SDMA_SUPPORTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SDMA_SUPPORTR::VALUE1
    }
}
#[doc = "Possible values of the field `SUSPEND_RESUME_SUPPORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSPEND_RESUME_SUPPORTR {
    #[doc = "Supported"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SUSPEND_RESUME_SUPPORTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SUSPEND_RESUME_SUPPORTR::VALUE1 => true,
            SUSPEND_RESUME_SUPPORTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUSPEND_RESUME_SUPPORTR {
        match value {
            true => SUSPEND_RESUME_SUPPORTR::VALUE1,
            i => SUSPEND_RESUME_SUPPORTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SUSPEND_RESUME_SUPPORTR::VALUE1
    }
}
#[doc = "Possible values of the field `VOLTAGE_SUPPORT_3_3V`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VOLTAGE_SUPPORT_3_3VR {
    #[doc = "3.3V supported"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl VOLTAGE_SUPPORT_3_3VR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VOLTAGE_SUPPORT_3_3VR::VALUE1 => true,
            VOLTAGE_SUPPORT_3_3VR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VOLTAGE_SUPPORT_3_3VR {
        match value {
            true => VOLTAGE_SUPPORT_3_3VR::VALUE1,
            i => VOLTAGE_SUPPORT_3_3VR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VOLTAGE_SUPPORT_3_3VR::VALUE1
    }
}
#[doc = "Possible values of the field `VOLTAGE_SUPPORT_3V`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VOLTAGE_SUPPORT_3VR {
    #[doc = "3.0V not supported"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl VOLTAGE_SUPPORT_3VR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VOLTAGE_SUPPORT_3VR::VALUE1 => false,
            VOLTAGE_SUPPORT_3VR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VOLTAGE_SUPPORT_3VR {
        match value {
            false => VOLTAGE_SUPPORT_3VR::VALUE1,
            i => VOLTAGE_SUPPORT_3VR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VOLTAGE_SUPPORT_3VR::VALUE1
    }
}
#[doc = "Possible values of the field `VOLTAGE_SUPPORT_1_8V`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VOLTAGE_SUPPORT_1_8VR {
    #[doc = "1.8V not supported"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl VOLTAGE_SUPPORT_1_8VR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VOLTAGE_SUPPORT_1_8VR::VALUE1 => false,
            VOLTAGE_SUPPORT_1_8VR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VOLTAGE_SUPPORT_1_8VR {
        match value {
            false => VOLTAGE_SUPPORT_1_8VR::VALUE1,
            i => VOLTAGE_SUPPORT_1_8VR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VOLTAGE_SUPPORT_1_8VR::VALUE1
    }
}
#[doc = "Possible values of the field `SYSBUS_64_SUPPORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSBUS_64_SUPPORTR {
    #[doc = "Does not support 64-bit system address"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SYSBUS_64_SUPPORTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SYSBUS_64_SUPPORTR::VALUE1 => false,
            SYSBUS_64_SUPPORTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYSBUS_64_SUPPORTR {
        match value {
            false => SYSBUS_64_SUPPORTR::VALUE1,
            i => SYSBUS_64_SUPPORTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SYSBUS_64_SUPPORTR::VALUE1
    }
}
#[doc = "Possible values of the field `ASYNC_INT_SUPPORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASYNC_INT_SUPPORTR {
    #[doc = "Asynchronous Interrupt not supported"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ASYNC_INT_SUPPORTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ASYNC_INT_SUPPORTR::VALUE1 => false,
            ASYNC_INT_SUPPORTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASYNC_INT_SUPPORTR {
        match value {
            false => ASYNC_INT_SUPPORTR::VALUE1,
            i => ASYNC_INT_SUPPORTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASYNC_INT_SUPPORTR::VALUE1
    }
}
#[doc = "Possible values of the field `SLOT_TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOT_TYPER {
    #[doc = "Removable Card Slot"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SLOT_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SLOT_TYPER::VALUE1 => 0,
            SLOT_TYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SLOT_TYPER {
        match value {
            0 => SLOT_TYPER::VALUE1,
            i => SLOT_TYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SLOT_TYPER::VALUE1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - Timeout Clock Frequency"]
    #[inline]
    pub fn timeout_clock_freq(&self) -> TIMEOUT_CLOCK_FREQR {
        TIMEOUT_CLOCK_FREQR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Timeout Clock Unit"]
    #[inline]
    pub fn timeout_clock_unit(&self) -> TIMEOUT_CLOCK_UNITR {
        TIMEOUT_CLOCK_UNITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:15 - Base Clock Frequency for SD Clock"]
    #[inline]
    pub fn base_sd_clock_freq(&self) -> BASE_SD_CLOCK_FREQR {
        BASE_SD_CLOCK_FREQR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Max Block Length"]
    #[inline]
    pub fn max_block_length(&self) -> MAX_BLOCK_LENGTHR {
        MAX_BLOCK_LENGTHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - Extended Media Bus Support"]
    #[inline]
    pub fn ext_media_bus_support(&self) -> EXT_MEDIA_BUS_SUPPORTR {
        EXT_MEDIA_BUS_SUPPORTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - ADMA2 Support"]
    #[inline]
    pub fn adma2_support(&self) -> ADMA2_SUPPORTR {
        ADMA2_SUPPORTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - High Speed Support"]
    #[inline]
    pub fn high_speed_support(&self) -> HIGH_SPEED_SUPPORTR {
        HIGH_SPEED_SUPPORTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - SDMA Support"]
    #[inline]
    pub fn sdma_support(&self) -> SDMA_SUPPORTR {
        SDMA_SUPPORTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Suspend / Resume Support"]
    #[inline]
    pub fn suspend_resume_support(&self) -> SUSPEND_RESUME_SUPPORTR {
        SUSPEND_RESUME_SUPPORTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Voltage Support 3.3V"]
    #[inline]
    pub fn voltage_support_3_3v(&self) -> VOLTAGE_SUPPORT_3_3VR {
        VOLTAGE_SUPPORT_3_3VR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Voltage Support 3.0V"]
    #[inline]
    pub fn voltage_support_3v(&self) -> VOLTAGE_SUPPORT_3VR {
        VOLTAGE_SUPPORT_3VR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Voltage Support 1.8V"]
    #[inline]
    pub fn voltage_support_1_8v(&self) -> VOLTAGE_SUPPORT_1_8VR {
        VOLTAGE_SUPPORT_1_8VR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - 64-bit System Bus Support"]
    #[inline]
    pub fn sysbus_64_support(&self) -> SYSBUS_64_SUPPORTR {
        SYSBUS_64_SUPPORTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Asynchronous Interrupt Support"]
    #[inline]
    pub fn async_int_support(&self) -> ASYNC_INT_SUPPORTR {
        ASYNC_INT_SUPPORTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 30:31 - Slot Type"]
    #[inline]
    pub fn slot_type(&self) -> SLOT_TYPER {
        SLOT_TYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
