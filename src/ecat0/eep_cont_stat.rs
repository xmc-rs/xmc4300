#[doc = "Register `EEP_CONT_STAT` reader"]
pub type R = crate::R<EEP_CONT_STAT_SPEC>;
#[doc = "Register `EEP_CONT_STAT` writer"]
pub type W = crate::W<EEP_CONT_STAT_SPEC>;
#[doc = "Field `W_EN` reader - ECAT write enable"]
pub type W_EN_R = crate::BitReader<W_EN_A>;
#[doc = "ECAT write enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl W_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> W_EN_A {
        match self.bits {
            false => W_EN_A::VALUE1,
            true => W_EN_A::VALUE2,
        }
    }
    #[doc = "Write requests are disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == W_EN_A::VALUE1
    }
    #[doc = "Write requests are enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == W_EN_A::VALUE2
    }
}
#[doc = "Field `EMUL` reader - EEPROM emulation"]
pub type EMUL_R = crate::BitReader<EMUL_A>;
#[doc = "EEPROM emulation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl EMUL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EMUL_A {
        match self.bits {
            false => EMUL_A::VALUE1,
            true => EMUL_A::VALUE2,
        }
    }
    #[doc = "Normal operation (I2C interface used)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EMUL_A::VALUE1
    }
    #[doc = "PDI emulates EEPROM (I2C not used)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EMUL_A::VALUE2
    }
}
#[doc = "Field `BYTES` reader - Supported number of EEPROM read bytes"]
pub type BYTES_R = crate::BitReader<BYTES_A>;
#[doc = "Supported number of EEPROM read bytes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl BYTES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BYTES_A {
        match self.bits {
            false => BYTES_A::VALUE1,
            true => BYTES_A::VALUE2,
        }
    }
    #[doc = "4 Bytes"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BYTES_A::VALUE1
    }
    #[doc = "8 Bytes"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BYTES_A::VALUE2
    }
}
#[doc = "Field `ALG` reader - Selected EEPROM Algorithm"]
pub type ALG_R = crate::BitReader<ALG_A>;
#[doc = "Selected EEPROM Algorithm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ALG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALG_A {
        match self.bits {
            false => ALG_A::VALUE1,
            true => ALG_A::VALUE2,
        }
    }
    #[doc = "1 address byte (1 KBit - 16 KBit EEPROMs)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALG_A::VALUE1
    }
    #[doc = "2 address bytes (32 KBit - 4 MBit EEPROMs)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALG_A::VALUE2
    }
}
#[doc = "Field `CMD_REG` reader - Command register"]
pub type CMD_REG_R = crate::FieldReader<CMD_REG_A>;
#[doc = "Command register\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for CMD_REG_A {
    type Ux = u8;
}
impl CMD_REG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CMD_REG_A> {
        match self.bits {
            0 => Some(CMD_REG_A::VALUE1),
            1 => Some(CMD_REG_A::VALUE2),
            2 => Some(CMD_REG_A::VALUE3),
            4 => Some(CMD_REG_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "No command/EEPROM idle (clear error bits)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD_REG_A::VALUE1
    }
    #[doc = "Read"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD_REG_A::VALUE2
    }
    #[doc = "Write"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CMD_REG_A::VALUE3
    }
    #[doc = "Reload"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CMD_REG_A::VALUE4
    }
}
#[doc = "Field `CMD_REG` writer - Command register"]
pub type CMD_REG_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CMD_REG_A>;
impl<'a, REG> CMD_REG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No command/EEPROM idle (clear error bits)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_REG_A::VALUE1)
    }
    #[doc = "Read"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_REG_A::VALUE2)
    }
    #[doc = "Write"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_REG_A::VALUE3)
    }
    #[doc = "Reload"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_REG_A::VALUE4)
    }
}
#[doc = "Field `ERROR` reader - Checksum Error at in ESC Configuration Area"]
pub type ERROR_R = crate::BitReader<ERROR_A>;
#[doc = "Checksum Error at in ESC Configuration Area\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ERROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERROR_A {
        match self.bits {
            false => ERROR_A::VALUE1,
            true => ERROR_A::VALUE2,
        }
    }
    #[doc = "Checksum OK"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ERROR_A::VALUE1
    }
    #[doc = "Checksum error"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ERROR_A::VALUE2
    }
}
#[doc = "Field `L_STAT` reader - EEPROM loading status"]
pub type L_STAT_R = crate::BitReader<L_STAT_A>;
#[doc = "EEPROM loading status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl L_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L_STAT_A {
        match self.bits {
            false => L_STAT_A::VALUE1,
            true => L_STAT_A::VALUE2,
        }
    }
    #[doc = "EEPROM loaded, device information OK"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == L_STAT_A::VALUE1
    }
    #[doc = "EEPROM not loaded, device information not available (EEPROM loading in progress or finished with a failure)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == L_STAT_A::VALUE2
    }
}
#[doc = "Field `ERROR_AC` reader - Error Acknowledge/Command"]
pub type ERROR_AC_R = crate::BitReader<ERROR_AC_A>;
#[doc = "Error Acknowledge/Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ERROR_AC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERROR_AC_A {
        match self.bits {
            false => ERROR_AC_A::VALUE1,
            true => ERROR_AC_A::VALUE2,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ERROR_AC_A::VALUE1
    }
    #[doc = "Missing EEPROM acknowledge or invalid command"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ERROR_AC_A::VALUE2
    }
}
#[doc = "Field `ERROR_AC` writer - Error Acknowledge/Command"]
pub type ERROR_AC_W<'a, REG> = crate::BitWriter<'a, REG, ERROR_AC_A>;
impl<'a, REG> ERROR_AC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ERROR_AC_A::VALUE1)
    }
    #[doc = "Missing EEPROM acknowledge or invalid command"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ERROR_AC_A::VALUE2)
    }
}
#[doc = "Field `ERROR_WE` reader - Error Write Enable"]
pub type ERROR_WE_R = crate::BitReader<ERROR_WE_A>;
#[doc = "Error Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ERROR_WE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERROR_WE_A {
        match self.bits {
            false => ERROR_WE_A::VALUE1,
            true => ERROR_WE_A::VALUE2,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ERROR_WE_A::VALUE1
    }
    #[doc = "Write Command without Write enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ERROR_WE_A::VALUE2
    }
}
#[doc = "Field `BUSY` reader - Busy"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "Busy\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::VALUE1,
            true => BUSY_A::VALUE2,
        }
    }
    #[doc = "EEPROM Interface is idle"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BUSY_A::VALUE1
    }
    #[doc = "EEPROM Interface is busy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BUSY_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - ECAT write enable"]
    #[inline(always)]
    pub fn w_en(&self) -> W_EN_R {
        W_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - EEPROM emulation"]
    #[inline(always)]
    pub fn emul(&self) -> EMUL_R {
        EMUL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Supported number of EEPROM read bytes"]
    #[inline(always)]
    pub fn bytes(&self) -> BYTES_R {
        BYTES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selected EEPROM Algorithm"]
    #[inline(always)]
    pub fn alg(&self) -> ALG_R {
        ALG_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Command register"]
    #[inline(always)]
    pub fn cmd_reg(&self) -> CMD_REG_R {
        CMD_REG_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Checksum Error at in ESC Configuration Area"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EEPROM loading status"]
    #[inline(always)]
    pub fn l_stat(&self) -> L_STAT_R {
        L_STAT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Error Acknowledge/Command"]
    #[inline(always)]
    pub fn error_ac(&self) -> ERROR_AC_R {
        ERROR_AC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Error Write Enable"]
    #[inline(always)]
    pub fn error_we(&self) -> ERROR_WE_R {
        ERROR_WE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:10 - Command register"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_reg(&mut self) -> CMD_REG_W<EEP_CONT_STAT_SPEC> {
        CMD_REG_W::new(self, 8)
    }
    #[doc = "Bit 13 - Error Acknowledge/Command"]
    #[inline(always)]
    #[must_use]
    pub fn error_ac(&mut self) -> ERROR_AC_W<EEP_CONT_STAT_SPEC> {
        ERROR_AC_W::new(self, 13)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "EEPROM Control/Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eep_cont_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eep_cont_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EEP_CONT_STAT_SPEC;
impl crate::RegisterSpec for EEP_CONT_STAT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`eep_cont_stat::R`](R) reader structure"]
impl crate::Readable for EEP_CONT_STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eep_cont_stat::W`](W) writer structure"]
impl crate::Writable for EEP_CONT_STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EEP_CONT_STAT to value 0x9460"]
impl crate::Resettable for EEP_CONT_STAT_SPEC {
    const RESET_VALUE: u16 = 0x9460;
}
