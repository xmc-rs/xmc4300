#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::EEP_CONT_STAT {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `W_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum W_ENR {
    #[doc = "Write requests are disabled"]
    VALUE1,
    #[doc = "Write requests are enabled"]
    VALUE2,
}
impl W_ENR {
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
            W_ENR::VALUE1 => false,
            W_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> W_ENR {
        match value {
            false => W_ENR::VALUE1,
            true => W_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == W_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == W_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `EMUL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMULR {
    #[doc = "Normal operation (I2C interface used)"]
    VALUE1,
    #[doc = "PDI emulates EEPROM (I2C not used)"]
    VALUE2,
}
impl EMULR {
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
            EMULR::VALUE1 => false,
            EMULR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EMULR {
        match value {
            false => EMULR::VALUE1,
            true => EMULR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EMULR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EMULR::VALUE2
    }
}
#[doc = "Possible values of the field `BYTES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYTESR {
    #[doc = "4 Bytes"]
    VALUE1,
    #[doc = "8 Bytes"]
    VALUE2,
}
impl BYTESR {
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
            BYTESR::VALUE1 => false,
            BYTESR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BYTESR {
        match value {
            false => BYTESR::VALUE1,
            true => BYTESR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BYTESR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BYTESR::VALUE2
    }
}
#[doc = "Possible values of the field `ALG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALGR {
    #[doc = "1 address byte (1 KBit - 16 KBit EEPROMs)"]
    VALUE1,
    #[doc = "2 address bytes (32 KBit - 4 MBit EEPROMs)"]
    VALUE2,
}
impl ALGR {
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
            ALGR::VALUE1 => false,
            ALGR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALGR {
        match value {
            false => ALGR::VALUE1,
            true => ALGR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ALGR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ALGR::VALUE2
    }
}
#[doc = "Possible values of the field `CMD_REG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_REGR {
    #[doc = "No command/EEPROM idle (clear error bits)"]
    VALUE1,
    #[doc = "Read"]
    VALUE2,
    #[doc = "Write"]
    VALUE3,
    #[doc = "Reload"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMD_REGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMD_REGR::VALUE1 => 0,
            CMD_REGR::VALUE2 => 1,
            CMD_REGR::VALUE3 => 2,
            CMD_REGR::VALUE4 => 4,
            CMD_REGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMD_REGR {
        match value {
            0 => CMD_REGR::VALUE1,
            1 => CMD_REGR::VALUE2,
            2 => CMD_REGR::VALUE3,
            4 => CMD_REGR::VALUE4,
            i => CMD_REGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMD_REGR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMD_REGR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CMD_REGR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CMD_REGR::VALUE4
    }
}
#[doc = "Possible values of the field `ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRORR {
    #[doc = "Checksum OK"]
    VALUE1,
    #[doc = "Checksum error"]
    VALUE2,
}
impl ERRORR {
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
            ERRORR::VALUE1 => false,
            ERRORR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRORR {
        match value {
            false => ERRORR::VALUE1,
            true => ERRORR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ERRORR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ERRORR::VALUE2
    }
}
#[doc = "Possible values of the field `L_STAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L_STATR {
    #[doc = "EEPROM loaded, device information OK"]
    VALUE1,
    #[doc = "EEPROM not loaded, device information not available (EEPROM loading in progress or finished with a failure)"]
    VALUE2,
}
impl L_STATR {
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
            L_STATR::VALUE1 => false,
            L_STATR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> L_STATR {
        match value {
            false => L_STATR::VALUE1,
            true => L_STATR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == L_STATR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == L_STATR::VALUE2
    }
}
#[doc = "Possible values of the field `ERROR_AC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_ACR {
    #[doc = "No error"]
    VALUE1,
    #[doc = "Missing EEPROM acknowledge or invalid command"]
    VALUE2,
}
impl ERROR_ACR {
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
            ERROR_ACR::VALUE1 => false,
            ERROR_ACR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERROR_ACR {
        match value {
            false => ERROR_ACR::VALUE1,
            true => ERROR_ACR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ERROR_ACR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ERROR_ACR::VALUE2
    }
}
#[doc = "Possible values of the field `ERROR_WE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_WER {
    #[doc = "No error"]
    VALUE1,
    #[doc = "Write Command without Write enable"]
    VALUE2,
}
impl ERROR_WER {
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
            ERROR_WER::VALUE1 => false,
            ERROR_WER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERROR_WER {
        match value {
            false => ERROR_WER::VALUE1,
            true => ERROR_WER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ERROR_WER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ERROR_WER::VALUE2
    }
}
#[doc = "Possible values of the field `BUSY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSYR {
    #[doc = "EEPROM Interface is idle"]
    VALUE1,
    #[doc = "EEPROM Interface is busy"]
    VALUE2,
}
impl BUSYR {
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
            BUSYR::VALUE1 => false,
            BUSYR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUSYR {
        match value {
            false => BUSYR::VALUE1,
            true => BUSYR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BUSYR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BUSYR::VALUE2
    }
}
#[doc = "Values that can be written to the field `CMD_REG`"]
pub enum CMD_REGW {
    #[doc = "No command/EEPROM idle (clear error bits)"]
    VALUE1,
    #[doc = "Read"]
    VALUE2,
    #[doc = "Write"]
    VALUE3,
    #[doc = "Reload"]
    VALUE4,
}
impl CMD_REGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMD_REGW::VALUE1 => 0,
            CMD_REGW::VALUE2 => 1,
            CMD_REGW::VALUE3 => 2,
            CMD_REGW::VALUE4 => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD_REGW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_REGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD_REGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No command/EEPROM idle (clear error bits)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_REGW::VALUE1)
    }
    #[doc = "Read"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_REGW::VALUE2)
    }
    #[doc = "Write"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CMD_REGW::VALUE3)
    }
    #[doc = "Reload"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CMD_REGW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERROR_AC`"]
pub enum ERROR_ACW {
    #[doc = "No error"]
    VALUE1,
    #[doc = "Missing EEPROM acknowledge or invalid command"]
    VALUE2,
}
impl ERROR_ACW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERROR_ACW::VALUE1 => false,
            ERROR_ACW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERROR_ACW<'a> {
    w: &'a mut W,
}
impl<'a> _ERROR_ACW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERROR_ACW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ERROR_ACW::VALUE1)
    }
    #[doc = "Missing EEPROM acknowledge or invalid command"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ERROR_ACW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - ECAT write enable"]
    #[inline]
    pub fn w_en(&self) -> W_ENR {
        W_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - EEPROM emulation"]
    #[inline]
    pub fn emul(&self) -> EMULR {
        EMULR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - Supported number of EEPROM read bytes"]
    #[inline]
    pub fn bytes(&self) -> BYTESR {
        BYTESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Selected EEPROM Algorithm"]
    #[inline]
    pub fn alg(&self) -> ALGR {
        ALGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 8:10 - Command register"]
    #[inline]
    pub fn cmd_reg(&self) -> CMD_REGR {
        CMD_REGR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 11 - Checksum Error at in ESC Configuration Area"]
    #[inline]
    pub fn error(&self) -> ERRORR {
        ERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 12 - EEPROM loading status"]
    #[inline]
    pub fn l_stat(&self) -> L_STATR {
        L_STATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 13 - Error Acknowledge/Command"]
    #[inline]
    pub fn error_ac(&self) -> ERROR_ACR {
        ERROR_ACR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 14 - Error Write Enable"]
    #[inline]
    pub fn error_we(&self) -> ERROR_WER {
        ERROR_WER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 15 - Busy"]
    #[inline]
    pub fn busy(&self) -> BUSYR {
        BUSYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 37984 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 8:10 - Command register"]
    #[inline]
    pub fn cmd_reg(&mut self) -> _CMD_REGW {
        _CMD_REGW { w: self }
    }
    #[doc = "Bit 13 - Error Acknowledge/Command"]
    #[inline]
    pub fn error_ac(&mut self) -> _ERROR_ACW {
        _ERROR_ACW { w: self }
    }
}
