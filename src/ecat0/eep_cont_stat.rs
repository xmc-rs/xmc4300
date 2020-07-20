#[doc = "Reader of register EEP_CONT_STAT"]
pub type R = crate::R<u16, super::EEP_CONT_STAT>;
#[doc = "Writer for register EEP_CONT_STAT"]
pub type W = crate::W<u16, super::EEP_CONT_STAT>;
#[doc = "Register EEP_CONT_STAT `reset()`'s with value 0x9460"]
impl crate::ResetValue for super::EEP_CONT_STAT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x9460
    }
}
#[doc = "ECAT write enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum W_EN_A {
    #[doc = "0: Write requests are disabled"]
    VALUE1 = 0,
    #[doc = "1: Write requests are enabled"]
    VALUE2 = 1,
}
impl From<W_EN_A> for bool {
    #[inline(always)]
    fn from(variant: W_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `W_EN`"]
pub type W_EN_R = crate::R<bool, W_EN_A>;
impl W_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> W_EN_A {
        match self.bits {
            false => W_EN_A::VALUE1,
            true => W_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == W_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == W_EN_A::VALUE2
    }
}
#[doc = "EEPROM emulation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMUL_A {
    #[doc = "0: Normal operation (I2C interface used)"]
    VALUE1 = 0,
    #[doc = "1: PDI emulates EEPROM (I2C not used)"]
    VALUE2 = 1,
}
impl From<EMUL_A> for bool {
    #[inline(always)]
    fn from(variant: EMUL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EMUL`"]
pub type EMUL_R = crate::R<bool, EMUL_A>;
impl EMUL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMUL_A {
        match self.bits {
            false => EMUL_A::VALUE1,
            true => EMUL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EMUL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EMUL_A::VALUE2
    }
}
#[doc = "Supported number of EEPROM read bytes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYTES_A {
    #[doc = "0: 4 Bytes"]
    VALUE1 = 0,
    #[doc = "1: 8 Bytes"]
    VALUE2 = 1,
}
impl From<BYTES_A> for bool {
    #[inline(always)]
    fn from(variant: BYTES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BYTES`"]
pub type BYTES_R = crate::R<bool, BYTES_A>;
impl BYTES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYTES_A {
        match self.bits {
            false => BYTES_A::VALUE1,
            true => BYTES_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BYTES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BYTES_A::VALUE2
    }
}
#[doc = "Selected EEPROM Algorithm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALG_A {
    #[doc = "0: 1 address byte (1 KBit - 16 KBit EEPROMs)"]
    VALUE1 = 0,
    #[doc = "1: 2 address bytes (32 KBit - 4 MBit EEPROMs)"]
    VALUE2 = 1,
}
impl From<ALG_A> for bool {
    #[inline(always)]
    fn from(variant: ALG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALG`"]
pub type ALG_R = crate::R<bool, ALG_A>;
impl ALG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALG_A {
        match self.bits {
            false => ALG_A::VALUE1,
            true => ALG_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALG_A::VALUE2
    }
}
#[doc = "Command register\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_REG_A {
    #[doc = "0: No command/EEPROM idle (clear error bits)"]
    VALUE1 = 0,
    #[doc = "1: Read"]
    VALUE2 = 1,
    #[doc = "2: Write"]
    VALUE3 = 2,
    #[doc = "4: Reload"]
    VALUE4 = 4,
}
impl From<CMD_REG_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_REG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CMD_REG`"]
pub type CMD_REG_R = crate::R<u8, CMD_REG_A>;
impl CMD_REG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CMD_REG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CMD_REG_A::VALUE1),
            1 => Val(CMD_REG_A::VALUE2),
            2 => Val(CMD_REG_A::VALUE3),
            4 => Val(CMD_REG_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD_REG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD_REG_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CMD_REG_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CMD_REG_A::VALUE4
    }
}
#[doc = "Write proxy for field `CMD_REG`"]
pub struct CMD_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_REG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_REG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No command/EEPROM idle (clear error bits)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_REG_A::VALUE1)
    }
    #[doc = "Read"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_REG_A::VALUE2)
    }
    #[doc = "Write"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CMD_REG_A::VALUE3)
    }
    #[doc = "Reload"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CMD_REG_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u16) & 0x07) << 8);
        self.w
    }
}
#[doc = "Checksum Error at in ESC Configuration Area\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_A {
    #[doc = "0: Checksum OK"]
    VALUE1 = 0,
    #[doc = "1: Checksum error"]
    VALUE2 = 1,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERROR`"]
pub type ERROR_R = crate::R<bool, ERROR_A>;
impl ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_A {
        match self.bits {
            false => ERROR_A::VALUE1,
            true => ERROR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ERROR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ERROR_A::VALUE2
    }
}
#[doc = "EEPROM loading status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L_STAT_A {
    #[doc = "0: EEPROM loaded, device information OK"]
    VALUE1 = 0,
    #[doc = "1: EEPROM not loaded, device information not available (EEPROM loading in progress or finished with a failure)"]
    VALUE2 = 1,
}
impl From<L_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: L_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `L_STAT`"]
pub type L_STAT_R = crate::R<bool, L_STAT_A>;
impl L_STAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L_STAT_A {
        match self.bits {
            false => L_STAT_A::VALUE1,
            true => L_STAT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == L_STAT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == L_STAT_A::VALUE2
    }
}
#[doc = "Error Acknowledge/Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_AC_A {
    #[doc = "0: No error"]
    VALUE1 = 0,
    #[doc = "1: Missing EEPROM acknowledge or invalid command"]
    VALUE2 = 1,
}
impl From<ERROR_AC_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_AC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERROR_AC`"]
pub type ERROR_AC_R = crate::R<bool, ERROR_AC_A>;
impl ERROR_AC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_AC_A {
        match self.bits {
            false => ERROR_AC_A::VALUE1,
            true => ERROR_AC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ERROR_AC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ERROR_AC_A::VALUE2
    }
}
#[doc = "Write proxy for field `ERROR_AC`"]
pub struct ERROR_AC_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_AC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERROR_AC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ERROR_AC_A::VALUE1)
    }
    #[doc = "Missing EEPROM acknowledge or invalid command"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ERROR_AC_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "Error Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_WE_A {
    #[doc = "0: No error"]
    VALUE1 = 0,
    #[doc = "1: Write Command without Write enable"]
    VALUE2 = 1,
}
impl From<ERROR_WE_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_WE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERROR_WE`"]
pub type ERROR_WE_R = crate::R<bool, ERROR_WE_A>;
impl ERROR_WE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_WE_A {
        match self.bits {
            false => ERROR_WE_A::VALUE1,
            true => ERROR_WE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ERROR_WE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ERROR_WE_A::VALUE2
    }
}
#[doc = "Busy\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: EEPROM Interface is idle"]
    VALUE1 = 0,
    #[doc = "1: EEPROM Interface is busy"]
    VALUE2 = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, BUSY_A>;
impl BUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::VALUE1,
            true => BUSY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BUSY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BUSY_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - ECAT write enable"]
    #[inline(always)]
    pub fn w_en(&self) -> W_EN_R {
        W_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 5 - EEPROM emulation"]
    #[inline(always)]
    pub fn emul(&self) -> EMUL_R {
        EMUL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Supported number of EEPROM read bytes"]
    #[inline(always)]
    pub fn bytes(&self) -> BYTES_R {
        BYTES_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Selected EEPROM Algorithm"]
    #[inline(always)]
    pub fn alg(&self) -> ALG_R {
        ALG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Command register"]
    #[inline(always)]
    pub fn cmd_reg(&self) -> CMD_REG_R {
        CMD_REG_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Checksum Error at in ESC Configuration Area"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - EEPROM loading status"]
    #[inline(always)]
    pub fn l_stat(&self) -> L_STAT_R {
        L_STAT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Error Acknowledge/Command"]
    #[inline(always)]
    pub fn error_ac(&self) -> ERROR_AC_R {
        ERROR_AC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Error Write Enable"]
    #[inline(always)]
    pub fn error_we(&self) -> ERROR_WE_R {
        ERROR_WE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:10 - Command register"]
    #[inline(always)]
    pub fn cmd_reg(&mut self) -> CMD_REG_W {
        CMD_REG_W { w: self }
    }
    #[doc = "Bit 13 - Error Acknowledge/Command"]
    #[inline(always)]
    pub fn error_ac(&mut self) -> ERROR_AC_W {
        ERROR_AC_W { w: self }
    }
}
