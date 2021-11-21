#[doc = "Register `EEP_CONT_STAT` reader"]
pub struct R(crate::R<EEP_CONT_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEP_CONT_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEP_CONT_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEP_CONT_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEP_CONT_STAT` writer"]
pub struct W(crate::W<EEP_CONT_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEP_CONT_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EEP_CONT_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEP_CONT_STAT_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `W_EN` reader - ECAT write enable"]
pub struct W_EN_R(crate::FieldReader<bool, W_EN_A>);
impl W_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        W_EN_R(crate::FieldReader::new(bits))
    }
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
        **self == W_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == W_EN_A::VALUE2
    }
}
impl core::ops::Deref for W_EN_R {
    type Target = crate::FieldReader<bool, W_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `EMUL` reader - EEPROM emulation"]
pub struct EMUL_R(crate::FieldReader<bool, EMUL_A>);
impl EMUL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EMUL_R(crate::FieldReader::new(bits))
    }
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
        **self == EMUL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EMUL_A::VALUE2
    }
}
impl core::ops::Deref for EMUL_R {
    type Target = crate::FieldReader<bool, EMUL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `BYTES` reader - Supported number of EEPROM read bytes"]
pub struct BYTES_R(crate::FieldReader<bool, BYTES_A>);
impl BYTES_R {
    pub(crate) fn new(bits: bool) -> Self {
        BYTES_R(crate::FieldReader::new(bits))
    }
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
        **self == BYTES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BYTES_A::VALUE2
    }
}
impl core::ops::Deref for BYTES_R {
    type Target = crate::FieldReader<bool, BYTES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `ALG` reader - Selected EEPROM Algorithm"]
pub struct ALG_R(crate::FieldReader<bool, ALG_A>);
impl ALG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALG_R(crate::FieldReader::new(bits))
    }
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
        **self == ALG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ALG_A::VALUE2
    }
}
impl core::ops::Deref for ALG_R {
    type Target = crate::FieldReader<bool, ALG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `CMD_REG` reader - Command register"]
pub struct CMD_REG_R(crate::FieldReader<u8, CMD_REG_A>);
impl CMD_REG_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMD_REG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMD_REG_A> {
        match self.bits {
            0 => Some(CMD_REG_A::VALUE1),
            1 => Some(CMD_REG_A::VALUE2),
            2 => Some(CMD_REG_A::VALUE3),
            4 => Some(CMD_REG_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CMD_REG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CMD_REG_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == CMD_REG_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == CMD_REG_A::VALUE4
    }
}
impl core::ops::Deref for CMD_REG_R {
    type Target = crate::FieldReader<u8, CMD_REG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD_REG` writer - Command register"]
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
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u16 & 0x07) << 8);
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
#[doc = "Field `ERROR` reader - Checksum Error at in ESC Configuration Area"]
pub struct ERROR_R(crate::FieldReader<bool, ERROR_A>);
impl ERROR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERROR_R(crate::FieldReader::new(bits))
    }
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
        **self == ERROR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ERROR_A::VALUE2
    }
}
impl core::ops::Deref for ERROR_R {
    type Target = crate::FieldReader<bool, ERROR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `L_STAT` reader - EEPROM loading status"]
pub struct L_STAT_R(crate::FieldReader<bool, L_STAT_A>);
impl L_STAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        L_STAT_R(crate::FieldReader::new(bits))
    }
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
        **self == L_STAT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == L_STAT_A::VALUE2
    }
}
impl core::ops::Deref for L_STAT_R {
    type Target = crate::FieldReader<bool, L_STAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `ERROR_AC` reader - Error Acknowledge/Command"]
pub struct ERROR_AC_R(crate::FieldReader<bool, ERROR_AC_A>);
impl ERROR_AC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERROR_AC_R(crate::FieldReader::new(bits))
    }
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
        **self == ERROR_AC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ERROR_AC_A::VALUE2
    }
}
impl core::ops::Deref for ERROR_AC_R {
    type Target = crate::FieldReader<bool, ERROR_AC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROR_AC` writer - Error Acknowledge/Command"]
pub struct ERROR_AC_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_AC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERROR_AC_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u16 & 0x01) << 13);
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
#[doc = "Field `ERROR_WE` reader - Error Write Enable"]
pub struct ERROR_WE_R(crate::FieldReader<bool, ERROR_WE_A>);
impl ERROR_WE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERROR_WE_R(crate::FieldReader::new(bits))
    }
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
        **self == ERROR_WE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ERROR_WE_A::VALUE2
    }
}
impl core::ops::Deref for ERROR_WE_R {
    type Target = crate::FieldReader<bool, ERROR_WE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `BUSY` reader - Busy"]
pub struct BUSY_R(crate::FieldReader<bool, BUSY_A>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
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
        **self == BUSY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BUSY_A::VALUE2
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, BUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Control/Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eep_cont_stat](index.html) module"]
pub struct EEP_CONT_STAT_SPEC;
impl crate::RegisterSpec for EEP_CONT_STAT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [eep_cont_stat::R](R) reader structure"]
impl crate::Readable for EEP_CONT_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eep_cont_stat::W](W) writer structure"]
impl crate::Writable for EEP_CONT_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEP_CONT_STAT to value 0x9460"]
impl crate::Resettable for EEP_CONT_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x9460
    }
}
