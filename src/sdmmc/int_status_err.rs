#[doc = "Register `INT_STATUS_ERR` reader"]
pub type R = crate::R<INT_STATUS_ERR_SPEC>;
#[doc = "Register `INT_STATUS_ERR` writer"]
pub type W = crate::W<INT_STATUS_ERR_SPEC>;
#[doc = "Command Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMD_TIMEOUT_ERR_A {
    #[doc = "0: No Error"]
    VALUE1 = 0,
    #[doc = "1: Timeout"]
    VALUE2 = 1,
}
impl From<CMD_TIMEOUT_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_TIMEOUT_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_TIMEOUT_ERR` reader - Command Timeout Error"]
pub type CMD_TIMEOUT_ERR_R = crate::BitReader<CMD_TIMEOUT_ERR_A>;
impl CMD_TIMEOUT_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMD_TIMEOUT_ERR_A {
        match self.bits {
            false => CMD_TIMEOUT_ERR_A::VALUE1,
            true => CMD_TIMEOUT_ERR_A::VALUE2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD_TIMEOUT_ERR_A::VALUE1
    }
    #[doc = "Timeout"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD_TIMEOUT_ERR_A::VALUE2
    }
}
#[doc = "Field `CMD_TIMEOUT_ERR` writer - Command Timeout Error"]
pub type CMD_TIMEOUT_ERR_W<'a, REG> = crate::BitWriter<'a, REG, CMD_TIMEOUT_ERR_A>;
impl<'a, REG> CMD_TIMEOUT_ERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_TIMEOUT_ERR_A::VALUE1)
    }
    #[doc = "Timeout"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_TIMEOUT_ERR_A::VALUE2)
    }
}
#[doc = "Command CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMD_CRC_ERR_A {
    #[doc = "0: No Error"]
    VALUE1 = 0,
    #[doc = "1: CRC Error Generated"]
    VALUE2 = 1,
}
impl From<CMD_CRC_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_CRC_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_CRC_ERR` reader - Command CRC Error"]
pub type CMD_CRC_ERR_R = crate::BitReader<CMD_CRC_ERR_A>;
impl CMD_CRC_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMD_CRC_ERR_A {
        match self.bits {
            false => CMD_CRC_ERR_A::VALUE1,
            true => CMD_CRC_ERR_A::VALUE2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD_CRC_ERR_A::VALUE1
    }
    #[doc = "CRC Error Generated"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD_CRC_ERR_A::VALUE2
    }
}
#[doc = "Field `CMD_CRC_ERR` writer - Command CRC Error"]
pub type CMD_CRC_ERR_W<'a, REG> = crate::BitWriter<'a, REG, CMD_CRC_ERR_A>;
impl<'a, REG> CMD_CRC_ERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_CRC_ERR_A::VALUE1)
    }
    #[doc = "CRC Error Generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_CRC_ERR_A::VALUE2)
    }
}
#[doc = "Command End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMD_END_BIT_ERR_A {
    #[doc = "0: No Error"]
    VALUE1 = 0,
    #[doc = "1: End Bit Error Generated"]
    VALUE2 = 1,
}
impl From<CMD_END_BIT_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_END_BIT_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_END_BIT_ERR` reader - Command End Bit Error"]
pub type CMD_END_BIT_ERR_R = crate::BitReader<CMD_END_BIT_ERR_A>;
impl CMD_END_BIT_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMD_END_BIT_ERR_A {
        match self.bits {
            false => CMD_END_BIT_ERR_A::VALUE1,
            true => CMD_END_BIT_ERR_A::VALUE2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD_END_BIT_ERR_A::VALUE1
    }
    #[doc = "End Bit Error Generated"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD_END_BIT_ERR_A::VALUE2
    }
}
#[doc = "Field `CMD_END_BIT_ERR` writer - Command End Bit Error"]
pub type CMD_END_BIT_ERR_W<'a, REG> = crate::BitWriter<'a, REG, CMD_END_BIT_ERR_A>;
impl<'a, REG> CMD_END_BIT_ERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_END_BIT_ERR_A::VALUE1)
    }
    #[doc = "End Bit Error Generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_END_BIT_ERR_A::VALUE2)
    }
}
#[doc = "Command Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMD_IND_ERR_A {
    #[doc = "0: No Error"]
    VALUE1 = 0,
    #[doc = "1: Error"]
    VALUE2 = 1,
}
impl From<CMD_IND_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_IND_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_IND_ERR` reader - Command Index Error"]
pub type CMD_IND_ERR_R = crate::BitReader<CMD_IND_ERR_A>;
impl CMD_IND_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMD_IND_ERR_A {
        match self.bits {
            false => CMD_IND_ERR_A::VALUE1,
            true => CMD_IND_ERR_A::VALUE2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD_IND_ERR_A::VALUE1
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD_IND_ERR_A::VALUE2
    }
}
#[doc = "Field `CMD_IND_ERR` writer - Command Index Error"]
pub type CMD_IND_ERR_W<'a, REG> = crate::BitWriter<'a, REG, CMD_IND_ERR_A>;
impl<'a, REG> CMD_IND_ERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_IND_ERR_A::VALUE1)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_IND_ERR_A::VALUE2)
    }
}
#[doc = "Data Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATA_TIMEOUT_ERR_A {
    #[doc = "0: No Error"]
    VALUE1 = 0,
    #[doc = "1: Timeout"]
    VALUE2 = 1,
}
impl From<DATA_TIMEOUT_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_TIMEOUT_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_TIMEOUT_ERR` reader - Data Timeout Error"]
pub type DATA_TIMEOUT_ERR_R = crate::BitReader<DATA_TIMEOUT_ERR_A>;
impl DATA_TIMEOUT_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DATA_TIMEOUT_ERR_A {
        match self.bits {
            false => DATA_TIMEOUT_ERR_A::VALUE1,
            true => DATA_TIMEOUT_ERR_A::VALUE2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DATA_TIMEOUT_ERR_A::VALUE1
    }
    #[doc = "Timeout"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DATA_TIMEOUT_ERR_A::VALUE2
    }
}
#[doc = "Field `DATA_TIMEOUT_ERR` writer - Data Timeout Error"]
pub type DATA_TIMEOUT_ERR_W<'a, REG> = crate::BitWriter<'a, REG, DATA_TIMEOUT_ERR_A>;
impl<'a, REG> DATA_TIMEOUT_ERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_TIMEOUT_ERR_A::VALUE1)
    }
    #[doc = "Timeout"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_TIMEOUT_ERR_A::VALUE2)
    }
}
#[doc = "Data CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATA_CRC_ERR_A {
    #[doc = "0: No Error"]
    VALUE1 = 0,
    #[doc = "1: Error"]
    VALUE2 = 1,
}
impl From<DATA_CRC_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_CRC_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_CRC_ERR` reader - Data CRC Error"]
pub type DATA_CRC_ERR_R = crate::BitReader<DATA_CRC_ERR_A>;
impl DATA_CRC_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DATA_CRC_ERR_A {
        match self.bits {
            false => DATA_CRC_ERR_A::VALUE1,
            true => DATA_CRC_ERR_A::VALUE2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DATA_CRC_ERR_A::VALUE1
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DATA_CRC_ERR_A::VALUE2
    }
}
#[doc = "Field `DATA_CRC_ERR` writer - Data CRC Error"]
pub type DATA_CRC_ERR_W<'a, REG> = crate::BitWriter<'a, REG, DATA_CRC_ERR_A>;
impl<'a, REG> DATA_CRC_ERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_CRC_ERR_A::VALUE1)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_CRC_ERR_A::VALUE2)
    }
}
#[doc = "Data End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATA_END_BIT_ERR_A {
    #[doc = "0: No Error"]
    VALUE1 = 0,
    #[doc = "1: Error"]
    VALUE2 = 1,
}
impl From<DATA_END_BIT_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_END_BIT_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_END_BIT_ERR` reader - Data End Bit Error"]
pub type DATA_END_BIT_ERR_R = crate::BitReader<DATA_END_BIT_ERR_A>;
impl DATA_END_BIT_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DATA_END_BIT_ERR_A {
        match self.bits {
            false => DATA_END_BIT_ERR_A::VALUE1,
            true => DATA_END_BIT_ERR_A::VALUE2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DATA_END_BIT_ERR_A::VALUE1
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DATA_END_BIT_ERR_A::VALUE2
    }
}
#[doc = "Field `DATA_END_BIT_ERR` writer - Data End Bit Error"]
pub type DATA_END_BIT_ERR_W<'a, REG> = crate::BitWriter<'a, REG, DATA_END_BIT_ERR_A>;
impl<'a, REG> DATA_END_BIT_ERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_END_BIT_ERR_A::VALUE1)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_END_BIT_ERR_A::VALUE2)
    }
}
#[doc = "Current Limit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CURRENT_LIMIT_ERR_A {
    #[doc = "0: No Error"]
    VALUE1 = 0,
    #[doc = "1: Power Fail"]
    VALUE2 = 1,
}
impl From<CURRENT_LIMIT_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: CURRENT_LIMIT_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CURRENT_LIMIT_ERR` reader - Current Limit Error"]
pub type CURRENT_LIMIT_ERR_R = crate::BitReader<CURRENT_LIMIT_ERR_A>;
impl CURRENT_LIMIT_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CURRENT_LIMIT_ERR_A {
        match self.bits {
            false => CURRENT_LIMIT_ERR_A::VALUE1,
            true => CURRENT_LIMIT_ERR_A::VALUE2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CURRENT_LIMIT_ERR_A::VALUE1
    }
    #[doc = "Power Fail"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CURRENT_LIMIT_ERR_A::VALUE2
    }
}
#[doc = "Field `CURRENT_LIMIT_ERR` writer - Current Limit Error"]
pub type CURRENT_LIMIT_ERR_W<'a, REG> = crate::BitWriter<'a, REG, CURRENT_LIMIT_ERR_A>;
impl<'a, REG> CURRENT_LIMIT_ERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CURRENT_LIMIT_ERR_A::VALUE1)
    }
    #[doc = "Power Fail"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CURRENT_LIMIT_ERR_A::VALUE2)
    }
}
#[doc = "Auto CMD Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMD_ERR_A {
    #[doc = "0: No Error"]
    VALUE1 = 0,
    #[doc = "1: Error"]
    VALUE2 = 1,
}
impl From<ACMD_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ACMD_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD_ERR` reader - Auto CMD Error"]
pub type ACMD_ERR_R = crate::BitReader<ACMD_ERR_A>;
impl ACMD_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACMD_ERR_A {
        match self.bits {
            false => ACMD_ERR_A::VALUE1,
            true => ACMD_ERR_A::VALUE2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ACMD_ERR_A::VALUE1
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ACMD_ERR_A::VALUE2
    }
}
#[doc = "Field `ACMD_ERR` writer - Auto CMD Error"]
pub type ACMD_ERR_W<'a, REG> = crate::BitWriter<'a, REG, ACMD_ERR_A>;
impl<'a, REG> ACMD_ERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ACMD_ERR_A::VALUE1)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ACMD_ERR_A::VALUE2)
    }
}
#[doc = "Ceata Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEATA_ERR_A {
    #[doc = "0: no error"]
    VALUE1 = 0,
    #[doc = "1: error"]
    VALUE2 = 1,
}
impl From<CEATA_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: CEATA_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEATA_ERR` reader - Ceata Error Status"]
pub type CEATA_ERR_R = crate::BitReader<CEATA_ERR_A>;
impl CEATA_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CEATA_ERR_A {
        match self.bits {
            false => CEATA_ERR_A::VALUE1,
            true => CEATA_ERR_A::VALUE2,
        }
    }
    #[doc = "no error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEATA_ERR_A::VALUE1
    }
    #[doc = "error"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEATA_ERR_A::VALUE2
    }
}
#[doc = "Field `CEATA_ERR` writer - Ceata Error Status"]
pub type CEATA_ERR_W<'a, REG> = crate::BitWriter<'a, REG, CEATA_ERR_A>;
impl<'a, REG> CEATA_ERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no error"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEATA_ERR_A::VALUE1)
    }
    #[doc = "error"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEATA_ERR_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Command Timeout Error"]
    #[inline(always)]
    pub fn cmd_timeout_err(&self) -> CMD_TIMEOUT_ERR_R {
        CMD_TIMEOUT_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command CRC Error"]
    #[inline(always)]
    pub fn cmd_crc_err(&self) -> CMD_CRC_ERR_R {
        CMD_CRC_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command End Bit Error"]
    #[inline(always)]
    pub fn cmd_end_bit_err(&self) -> CMD_END_BIT_ERR_R {
        CMD_END_BIT_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Command Index Error"]
    #[inline(always)]
    pub fn cmd_ind_err(&self) -> CMD_IND_ERR_R {
        CMD_IND_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Timeout Error"]
    #[inline(always)]
    pub fn data_timeout_err(&self) -> DATA_TIMEOUT_ERR_R {
        DATA_TIMEOUT_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data CRC Error"]
    #[inline(always)]
    pub fn data_crc_err(&self) -> DATA_CRC_ERR_R {
        DATA_CRC_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data End Bit Error"]
    #[inline(always)]
    pub fn data_end_bit_err(&self) -> DATA_END_BIT_ERR_R {
        DATA_END_BIT_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Current Limit Error"]
    #[inline(always)]
    pub fn current_limit_err(&self) -> CURRENT_LIMIT_ERR_R {
        CURRENT_LIMIT_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Auto CMD Error"]
    #[inline(always)]
    pub fn acmd_err(&self) -> ACMD_ERR_R {
        ACMD_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 13 - Ceata Error Status"]
    #[inline(always)]
    pub fn ceata_err(&self) -> CEATA_ERR_R {
        CEATA_ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Timeout Error"]
    #[inline(always)]
    pub fn cmd_timeout_err(&mut self) -> CMD_TIMEOUT_ERR_W<INT_STATUS_ERR_SPEC> {
        CMD_TIMEOUT_ERR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Command CRC Error"]
    #[inline(always)]
    pub fn cmd_crc_err(&mut self) -> CMD_CRC_ERR_W<INT_STATUS_ERR_SPEC> {
        CMD_CRC_ERR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Command End Bit Error"]
    #[inline(always)]
    pub fn cmd_end_bit_err(&mut self) -> CMD_END_BIT_ERR_W<INT_STATUS_ERR_SPEC> {
        CMD_END_BIT_ERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Command Index Error"]
    #[inline(always)]
    pub fn cmd_ind_err(&mut self) -> CMD_IND_ERR_W<INT_STATUS_ERR_SPEC> {
        CMD_IND_ERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Data Timeout Error"]
    #[inline(always)]
    pub fn data_timeout_err(&mut self) -> DATA_TIMEOUT_ERR_W<INT_STATUS_ERR_SPEC> {
        DATA_TIMEOUT_ERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Data CRC Error"]
    #[inline(always)]
    pub fn data_crc_err(&mut self) -> DATA_CRC_ERR_W<INT_STATUS_ERR_SPEC> {
        DATA_CRC_ERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Data End Bit Error"]
    #[inline(always)]
    pub fn data_end_bit_err(&mut self) -> DATA_END_BIT_ERR_W<INT_STATUS_ERR_SPEC> {
        DATA_END_BIT_ERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Current Limit Error"]
    #[inline(always)]
    pub fn current_limit_err(&mut self) -> CURRENT_LIMIT_ERR_W<INT_STATUS_ERR_SPEC> {
        CURRENT_LIMIT_ERR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Auto CMD Error"]
    #[inline(always)]
    pub fn acmd_err(&mut self) -> ACMD_ERR_W<INT_STATUS_ERR_SPEC> {
        ACMD_ERR_W::new(self, 8)
    }
    #[doc = "Bit 13 - Ceata Error Status"]
    #[inline(always)]
    pub fn ceata_err(&mut self) -> CEATA_ERR_W<INT_STATUS_ERR_SPEC> {
        CEATA_ERR_W::new(self, 13)
    }
}
#[doc = "Error Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_status_err::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_status_err::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_STATUS_ERR_SPEC;
impl crate::RegisterSpec for INT_STATUS_ERR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`int_status_err::R`](R) reader structure"]
impl crate::Readable for INT_STATUS_ERR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_status_err::W`](W) writer structure"]
impl crate::Writable for INT_STATUS_ERR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets INT_STATUS_ERR to value 0"]
impl crate::Resettable for INT_STATUS_ERR_SPEC {
    const RESET_VALUE: u16 = 0;
}
