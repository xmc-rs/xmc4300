#[doc = "Register `ACMD_ERR_STATUS` reader"]
pub struct R(crate::R<ACMD_ERR_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACMD_ERR_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACMD_ERR_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACMD_ERR_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Command Not Issued By Auto CMD12 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_NOT_ISSUED_BY_ACMD12_ERR_A {
    #[doc = "0: No Error"]
    VALUE1 = 0,
    #[doc = "1: Not Issued"]
    VALUE2 = 1,
}
impl From<CMD_NOT_ISSUED_BY_ACMD12_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_NOT_ISSUED_BY_ACMD12_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_NOT_ISSUED_BY_ACMD12_ERR` reader - Command Not Issued By Auto CMD12 Error"]
pub struct CMD_NOT_ISSUED_BY_ACMD12_ERR_R(crate::FieldReader<bool, CMD_NOT_ISSUED_BY_ACMD12_ERR_A>);
impl CMD_NOT_ISSUED_BY_ACMD12_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMD_NOT_ISSUED_BY_ACMD12_ERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_NOT_ISSUED_BY_ACMD12_ERR_A {
        match self.bits {
            false => CMD_NOT_ISSUED_BY_ACMD12_ERR_A::VALUE1,
            true => CMD_NOT_ISSUED_BY_ACMD12_ERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CMD_NOT_ISSUED_BY_ACMD12_ERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CMD_NOT_ISSUED_BY_ACMD12_ERR_A::VALUE2
    }
}
impl core::ops::Deref for CMD_NOT_ISSUED_BY_ACMD12_ERR_R {
    type Target = crate::FieldReader<bool, CMD_NOT_ISSUED_BY_ACMD12_ERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Auto CMD Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMD_IND_ERR_A {
    #[doc = "0: No Error"]
    VALUE1 = 0,
    #[doc = "1: Error"]
    VALUE2 = 1,
}
impl From<ACMD_IND_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ACMD_IND_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD_IND_ERR` reader - Auto CMD Index Error"]
pub struct ACMD_IND_ERR_R(crate::FieldReader<bool, ACMD_IND_ERR_A>);
impl ACMD_IND_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACMD_IND_ERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMD_IND_ERR_A {
        match self.bits {
            false => ACMD_IND_ERR_A::VALUE1,
            true => ACMD_IND_ERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ACMD_IND_ERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ACMD_IND_ERR_A::VALUE2
    }
}
impl core::ops::Deref for ACMD_IND_ERR_R {
    type Target = crate::FieldReader<bool, ACMD_IND_ERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Auto CMD End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMD_END_BIT_ERR_A {
    #[doc = "0: No Error"]
    VALUE1 = 0,
    #[doc = "1: End Bit Error Generated"]
    VALUE2 = 1,
}
impl From<ACMD_END_BIT_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ACMD_END_BIT_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD_END_BIT_ERR` reader - Auto CMD End Bit Error"]
pub struct ACMD_END_BIT_ERR_R(crate::FieldReader<bool, ACMD_END_BIT_ERR_A>);
impl ACMD_END_BIT_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACMD_END_BIT_ERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMD_END_BIT_ERR_A {
        match self.bits {
            false => ACMD_END_BIT_ERR_A::VALUE1,
            true => ACMD_END_BIT_ERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ACMD_END_BIT_ERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ACMD_END_BIT_ERR_A::VALUE2
    }
}
impl core::ops::Deref for ACMD_END_BIT_ERR_R {
    type Target = crate::FieldReader<bool, ACMD_END_BIT_ERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Auto CMD CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMD_CRC_ERR_A {
    #[doc = "0: No Error"]
    VALUE1 = 0,
    #[doc = "1: CRC Error Generated"]
    VALUE2 = 1,
}
impl From<ACMD_CRC_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ACMD_CRC_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD_CRC_ERR` reader - Auto CMD CRC Error"]
pub struct ACMD_CRC_ERR_R(crate::FieldReader<bool, ACMD_CRC_ERR_A>);
impl ACMD_CRC_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACMD_CRC_ERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMD_CRC_ERR_A {
        match self.bits {
            false => ACMD_CRC_ERR_A::VALUE1,
            true => ACMD_CRC_ERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ACMD_CRC_ERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ACMD_CRC_ERR_A::VALUE2
    }
}
impl core::ops::Deref for ACMD_CRC_ERR_R {
    type Target = crate::FieldReader<bool, ACMD_CRC_ERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Auto CMD Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMD_TIMEOUT_ERR_A {
    #[doc = "0: No Error"]
    VALUE1 = 0,
    #[doc = "1: Timeout"]
    VALUE2 = 1,
}
impl From<ACMD_TIMEOUT_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ACMD_TIMEOUT_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD_TIMEOUT_ERR` reader - Auto CMD Timeout Error"]
pub struct ACMD_TIMEOUT_ERR_R(crate::FieldReader<bool, ACMD_TIMEOUT_ERR_A>);
impl ACMD_TIMEOUT_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACMD_TIMEOUT_ERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMD_TIMEOUT_ERR_A {
        match self.bits {
            false => ACMD_TIMEOUT_ERR_A::VALUE1,
            true => ACMD_TIMEOUT_ERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ACMD_TIMEOUT_ERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ACMD_TIMEOUT_ERR_A::VALUE2
    }
}
impl core::ops::Deref for ACMD_TIMEOUT_ERR_R {
    type Target = crate::FieldReader<bool, ACMD_TIMEOUT_ERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Auto CMD12 Not Executed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMD12_NOT_EXEC_ERR_A {
    #[doc = "0: Executed"]
    VALUE1 = 0,
    #[doc = "1: Not Executed"]
    VALUE2 = 1,
}
impl From<ACMD12_NOT_EXEC_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ACMD12_NOT_EXEC_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD12_NOT_EXEC_ERR` reader - Auto CMD12 Not Executed"]
pub struct ACMD12_NOT_EXEC_ERR_R(crate::FieldReader<bool, ACMD12_NOT_EXEC_ERR_A>);
impl ACMD12_NOT_EXEC_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACMD12_NOT_EXEC_ERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMD12_NOT_EXEC_ERR_A {
        match self.bits {
            false => ACMD12_NOT_EXEC_ERR_A::VALUE1,
            true => ACMD12_NOT_EXEC_ERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ACMD12_NOT_EXEC_ERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ACMD12_NOT_EXEC_ERR_A::VALUE2
    }
}
impl core::ops::Deref for ACMD12_NOT_EXEC_ERR_R {
    type Target = crate::FieldReader<bool, ACMD12_NOT_EXEC_ERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 7 - Command Not Issued By Auto CMD12 Error"]
    #[inline(always)]
    pub fn cmd_not_issued_by_acmd12_err(&self) -> CMD_NOT_ISSUED_BY_ACMD12_ERR_R {
        CMD_NOT_ISSUED_BY_ACMD12_ERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Auto CMD Index Error"]
    #[inline(always)]
    pub fn acmd_ind_err(&self) -> ACMD_IND_ERR_R {
        ACMD_IND_ERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Auto CMD End Bit Error"]
    #[inline(always)]
    pub fn acmd_end_bit_err(&self) -> ACMD_END_BIT_ERR_R {
        ACMD_END_BIT_ERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Auto CMD CRC Error"]
    #[inline(always)]
    pub fn acmd_crc_err(&self) -> ACMD_CRC_ERR_R {
        ACMD_CRC_ERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Auto CMD Timeout Error"]
    #[inline(always)]
    pub fn acmd_timeout_err(&self) -> ACMD_TIMEOUT_ERR_R {
        ACMD_TIMEOUT_ERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Auto CMD12 Not Executed"]
    #[inline(always)]
    pub fn acmd12_not_exec_err(&self) -> ACMD12_NOT_EXEC_ERR_R {
        ACMD12_NOT_EXEC_ERR_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Auto CMD Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acmd_err_status](index.html) module"]
pub struct ACMD_ERR_STATUS_SPEC;
impl crate::RegisterSpec for ACMD_ERR_STATUS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [acmd_err_status::R](R) reader structure"]
impl crate::Readable for ACMD_ERR_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACMD_ERR_STATUS to value 0"]
impl crate::Resettable for ACMD_ERR_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
