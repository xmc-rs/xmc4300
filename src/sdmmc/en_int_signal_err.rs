#[doc = "Register `EN_INT_SIGNAL_ERR` reader"]
pub struct R(crate::R<EN_INT_SIGNAL_ERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EN_INT_SIGNAL_ERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EN_INT_SIGNAL_ERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EN_INT_SIGNAL_ERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EN_INT_SIGNAL_ERR` writer"]
pub struct W(crate::W<EN_INT_SIGNAL_ERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EN_INT_SIGNAL_ERR_SPEC>;
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
impl From<crate::W<EN_INT_SIGNAL_ERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EN_INT_SIGNAL_ERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Ceata Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEATA_ERR_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<CEATA_ERR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CEATA_ERR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEATA_ERR_EN` reader - Ceata Error Signal Enable"]
pub struct CEATA_ERR_EN_R(crate::FieldReader<bool, CEATA_ERR_EN_A>);
impl CEATA_ERR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEATA_ERR_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEATA_ERR_EN_A {
        match self.bits {
            false => CEATA_ERR_EN_A::VALUE1,
            true => CEATA_ERR_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CEATA_ERR_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CEATA_ERR_EN_A::VALUE2
    }
}
impl core::ops::Deref for CEATA_ERR_EN_R {
    type Target = crate::FieldReader<bool, CEATA_ERR_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEATA_ERR_EN` writer - Ceata Error Signal Enable"]
pub struct CEATA_ERR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEATA_ERR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEATA_ERR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEATA_ERR_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEATA_ERR_EN_A::VALUE2)
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
#[doc = "Target Response Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TARGET_RESP_ERR_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<TARGET_RESP_ERR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TARGET_RESP_ERR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TARGET_RESP_ERR_EN` reader - Target Response Error Signal Enable"]
pub struct TARGET_RESP_ERR_EN_R(crate::FieldReader<bool, TARGET_RESP_ERR_EN_A>);
impl TARGET_RESP_ERR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TARGET_RESP_ERR_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TARGET_RESP_ERR_EN_A {
        match self.bits {
            false => TARGET_RESP_ERR_EN_A::VALUE1,
            true => TARGET_RESP_ERR_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TARGET_RESP_ERR_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TARGET_RESP_ERR_EN_A::VALUE2
    }
}
impl core::ops::Deref for TARGET_RESP_ERR_EN_R {
    type Target = crate::FieldReader<bool, TARGET_RESP_ERR_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TARGET_RESP_ERR_EN` writer - Target Response Error Signal Enable"]
pub struct TARGET_RESP_ERR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGET_RESP_ERR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TARGET_RESP_ERR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TARGET_RESP_ERR_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TARGET_RESP_ERR_EN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u16 & 0x01) << 12);
        self.w
    }
}
#[doc = "Auto CMD12 Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMD_ERR_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<ACMD_ERR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ACMD_ERR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD_ERR_EN` reader - Auto CMD12 Error Signal Enable"]
pub struct ACMD_ERR_EN_R(crate::FieldReader<bool, ACMD_ERR_EN_A>);
impl ACMD_ERR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACMD_ERR_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMD_ERR_EN_A {
        match self.bits {
            false => ACMD_ERR_EN_A::VALUE1,
            true => ACMD_ERR_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ACMD_ERR_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ACMD_ERR_EN_A::VALUE2
    }
}
impl core::ops::Deref for ACMD_ERR_EN_R {
    type Target = crate::FieldReader<bool, ACMD_ERR_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMD_ERR_EN` writer - Auto CMD12 Error Signal Enable"]
pub struct ACMD_ERR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMD_ERR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMD_ERR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ACMD_ERR_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ACMD_ERR_EN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Current Limit Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CURRENT_LIMIT_ERR_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<CURRENT_LIMIT_ERR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CURRENT_LIMIT_ERR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CURRENT_LIMIT_ERR_EN` reader - Current Limit Error Signal Enable"]
pub struct CURRENT_LIMIT_ERR_EN_R(crate::FieldReader<bool, CURRENT_LIMIT_ERR_EN_A>);
impl CURRENT_LIMIT_ERR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CURRENT_LIMIT_ERR_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CURRENT_LIMIT_ERR_EN_A {
        match self.bits {
            false => CURRENT_LIMIT_ERR_EN_A::VALUE1,
            true => CURRENT_LIMIT_ERR_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CURRENT_LIMIT_ERR_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CURRENT_LIMIT_ERR_EN_A::VALUE2
    }
}
impl core::ops::Deref for CURRENT_LIMIT_ERR_EN_R {
    type Target = crate::FieldReader<bool, CURRENT_LIMIT_ERR_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CURRENT_LIMIT_ERR_EN` writer - Current Limit Error Signal Enable"]
pub struct CURRENT_LIMIT_ERR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CURRENT_LIMIT_ERR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CURRENT_LIMIT_ERR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CURRENT_LIMIT_ERR_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CURRENT_LIMIT_ERR_EN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Data End Bit Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_END_BIT_ERR_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<DATA_END_BIT_ERR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_END_BIT_ERR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_END_BIT_ERR_EN` reader - Data End Bit Error Signal Enable"]
pub struct DATA_END_BIT_ERR_EN_R(crate::FieldReader<bool, DATA_END_BIT_ERR_EN_A>);
impl DATA_END_BIT_ERR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATA_END_BIT_ERR_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_END_BIT_ERR_EN_A {
        match self.bits {
            false => DATA_END_BIT_ERR_EN_A::VALUE1,
            true => DATA_END_BIT_ERR_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DATA_END_BIT_ERR_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DATA_END_BIT_ERR_EN_A::VALUE2
    }
}
impl core::ops::Deref for DATA_END_BIT_ERR_EN_R {
    type Target = crate::FieldReader<bool, DATA_END_BIT_ERR_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_END_BIT_ERR_EN` writer - Data End Bit Error Signal Enable"]
pub struct DATA_END_BIT_ERR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_END_BIT_ERR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_END_BIT_ERR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATA_END_BIT_ERR_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATA_END_BIT_ERR_EN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Data CRC Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_CRC_ERR_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<DATA_CRC_ERR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_CRC_ERR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_CRC_ERR_EN` reader - Data CRC Error Signal Enable"]
pub struct DATA_CRC_ERR_EN_R(crate::FieldReader<bool, DATA_CRC_ERR_EN_A>);
impl DATA_CRC_ERR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATA_CRC_ERR_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_CRC_ERR_EN_A {
        match self.bits {
            false => DATA_CRC_ERR_EN_A::VALUE1,
            true => DATA_CRC_ERR_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DATA_CRC_ERR_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DATA_CRC_ERR_EN_A::VALUE2
    }
}
impl core::ops::Deref for DATA_CRC_ERR_EN_R {
    type Target = crate::FieldReader<bool, DATA_CRC_ERR_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_CRC_ERR_EN` writer - Data CRC Error Signal Enable"]
pub struct DATA_CRC_ERR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_CRC_ERR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_CRC_ERR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATA_CRC_ERR_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATA_CRC_ERR_EN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Data Timeout Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_TIMEOUT_ERR_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<DATA_TIMEOUT_ERR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_TIMEOUT_ERR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_TIMEOUT_ERR_EN` reader - Data Timeout Error Signal Enable"]
pub struct DATA_TIMEOUT_ERR_EN_R(crate::FieldReader<bool, DATA_TIMEOUT_ERR_EN_A>);
impl DATA_TIMEOUT_ERR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATA_TIMEOUT_ERR_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_TIMEOUT_ERR_EN_A {
        match self.bits {
            false => DATA_TIMEOUT_ERR_EN_A::VALUE1,
            true => DATA_TIMEOUT_ERR_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DATA_TIMEOUT_ERR_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DATA_TIMEOUT_ERR_EN_A::VALUE2
    }
}
impl core::ops::Deref for DATA_TIMEOUT_ERR_EN_R {
    type Target = crate::FieldReader<bool, DATA_TIMEOUT_ERR_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_TIMEOUT_ERR_EN` writer - Data Timeout Error Signal Enable"]
pub struct DATA_TIMEOUT_ERR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_TIMEOUT_ERR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_TIMEOUT_ERR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATA_TIMEOUT_ERR_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATA_TIMEOUT_ERR_EN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Command Index Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_IND_ERR_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<CMD_IND_ERR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_IND_ERR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_IND_ERR_EN` reader - Command Index Error Signal Enable"]
pub struct CMD_IND_ERR_EN_R(crate::FieldReader<bool, CMD_IND_ERR_EN_A>);
impl CMD_IND_ERR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMD_IND_ERR_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_IND_ERR_EN_A {
        match self.bits {
            false => CMD_IND_ERR_EN_A::VALUE1,
            true => CMD_IND_ERR_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CMD_IND_ERR_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CMD_IND_ERR_EN_A::VALUE2
    }
}
impl core::ops::Deref for CMD_IND_ERR_EN_R {
    type Target = crate::FieldReader<bool, CMD_IND_ERR_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD_IND_ERR_EN` writer - Command Index Error Signal Enable"]
pub struct CMD_IND_ERR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_IND_ERR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_IND_ERR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_IND_ERR_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_IND_ERR_EN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Command End Bit Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_END_BIT_ERR_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<CMD_END_BIT_ERR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_END_BIT_ERR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_END_BIT_ERR_EN` reader - Command End Bit Error Signal Enable"]
pub struct CMD_END_BIT_ERR_EN_R(crate::FieldReader<bool, CMD_END_BIT_ERR_EN_A>);
impl CMD_END_BIT_ERR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMD_END_BIT_ERR_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_END_BIT_ERR_EN_A {
        match self.bits {
            false => CMD_END_BIT_ERR_EN_A::VALUE1,
            true => CMD_END_BIT_ERR_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CMD_END_BIT_ERR_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CMD_END_BIT_ERR_EN_A::VALUE2
    }
}
impl core::ops::Deref for CMD_END_BIT_ERR_EN_R {
    type Target = crate::FieldReader<bool, CMD_END_BIT_ERR_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD_END_BIT_ERR_EN` writer - Command End Bit Error Signal Enable"]
pub struct CMD_END_BIT_ERR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_END_BIT_ERR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_END_BIT_ERR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_END_BIT_ERR_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_END_BIT_ERR_EN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Command CRC Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_CRC_ERR_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<CMD_CRC_ERR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_CRC_ERR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_CRC_ERR_EN` reader - Command CRC Error Signal Enable"]
pub struct CMD_CRC_ERR_EN_R(crate::FieldReader<bool, CMD_CRC_ERR_EN_A>);
impl CMD_CRC_ERR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMD_CRC_ERR_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_CRC_ERR_EN_A {
        match self.bits {
            false => CMD_CRC_ERR_EN_A::VALUE1,
            true => CMD_CRC_ERR_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CMD_CRC_ERR_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CMD_CRC_ERR_EN_A::VALUE2
    }
}
impl core::ops::Deref for CMD_CRC_ERR_EN_R {
    type Target = crate::FieldReader<bool, CMD_CRC_ERR_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD_CRC_ERR_EN` writer - Command CRC Error Signal Enable"]
pub struct CMD_CRC_ERR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_CRC_ERR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_CRC_ERR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_CRC_ERR_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_CRC_ERR_EN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Command Timeout Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_TIMEOUT_ERR_EN_A {
    #[doc = "0: Masked"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<CMD_TIMEOUT_ERR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_TIMEOUT_ERR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_TIMEOUT_ERR_EN` reader - Command Timeout Error Signal Enable"]
pub struct CMD_TIMEOUT_ERR_EN_R(crate::FieldReader<bool, CMD_TIMEOUT_ERR_EN_A>);
impl CMD_TIMEOUT_ERR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMD_TIMEOUT_ERR_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_TIMEOUT_ERR_EN_A {
        match self.bits {
            false => CMD_TIMEOUT_ERR_EN_A::VALUE1,
            true => CMD_TIMEOUT_ERR_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CMD_TIMEOUT_ERR_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CMD_TIMEOUT_ERR_EN_A::VALUE2
    }
}
impl core::ops::Deref for CMD_TIMEOUT_ERR_EN_R {
    type Target = crate::FieldReader<bool, CMD_TIMEOUT_ERR_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD_TIMEOUT_ERR_EN` writer - Command Timeout Error Signal Enable"]
pub struct CMD_TIMEOUT_ERR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_TIMEOUT_ERR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_TIMEOUT_ERR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_TIMEOUT_ERR_EN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_TIMEOUT_ERR_EN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 13 - Ceata Error Signal Enable"]
    #[inline(always)]
    pub fn ceata_err_en(&self) -> CEATA_ERR_EN_R {
        CEATA_ERR_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Target Response Error Signal Enable"]
    #[inline(always)]
    pub fn target_resp_err_en(&self) -> TARGET_RESP_ERR_EN_R {
        TARGET_RESP_ERR_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Auto CMD12 Error Signal Enable"]
    #[inline(always)]
    pub fn acmd_err_en(&self) -> ACMD_ERR_EN_R {
        ACMD_ERR_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Current Limit Error Signal Enable"]
    #[inline(always)]
    pub fn current_limit_err_en(&self) -> CURRENT_LIMIT_ERR_EN_R {
        CURRENT_LIMIT_ERR_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Data End Bit Error Signal Enable"]
    #[inline(always)]
    pub fn data_end_bit_err_en(&self) -> DATA_END_BIT_ERR_EN_R {
        DATA_END_BIT_ERR_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Data CRC Error Signal Enable"]
    #[inline(always)]
    pub fn data_crc_err_en(&self) -> DATA_CRC_ERR_EN_R {
        DATA_CRC_ERR_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data Timeout Error Signal Enable"]
    #[inline(always)]
    pub fn data_timeout_err_en(&self) -> DATA_TIMEOUT_ERR_EN_R {
        DATA_TIMEOUT_ERR_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Command Index Error Signal Enable"]
    #[inline(always)]
    pub fn cmd_ind_err_en(&self) -> CMD_IND_ERR_EN_R {
        CMD_IND_ERR_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Command End Bit Error Signal Enable"]
    #[inline(always)]
    pub fn cmd_end_bit_err_en(&self) -> CMD_END_BIT_ERR_EN_R {
        CMD_END_BIT_ERR_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Command CRC Error Signal Enable"]
    #[inline(always)]
    pub fn cmd_crc_err_en(&self) -> CMD_CRC_ERR_EN_R {
        CMD_CRC_ERR_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Command Timeout Error Signal Enable"]
    #[inline(always)]
    pub fn cmd_timeout_err_en(&self) -> CMD_TIMEOUT_ERR_EN_R {
        CMD_TIMEOUT_ERR_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Ceata Error Signal Enable"]
    #[inline(always)]
    pub fn ceata_err_en(&mut self) -> CEATA_ERR_EN_W {
        CEATA_ERR_EN_W { w: self }
    }
    #[doc = "Bit 12 - Target Response Error Signal Enable"]
    #[inline(always)]
    pub fn target_resp_err_en(&mut self) -> TARGET_RESP_ERR_EN_W {
        TARGET_RESP_ERR_EN_W { w: self }
    }
    #[doc = "Bit 8 - Auto CMD12 Error Signal Enable"]
    #[inline(always)]
    pub fn acmd_err_en(&mut self) -> ACMD_ERR_EN_W {
        ACMD_ERR_EN_W { w: self }
    }
    #[doc = "Bit 7 - Current Limit Error Signal Enable"]
    #[inline(always)]
    pub fn current_limit_err_en(&mut self) -> CURRENT_LIMIT_ERR_EN_W {
        CURRENT_LIMIT_ERR_EN_W { w: self }
    }
    #[doc = "Bit 6 - Data End Bit Error Signal Enable"]
    #[inline(always)]
    pub fn data_end_bit_err_en(&mut self) -> DATA_END_BIT_ERR_EN_W {
        DATA_END_BIT_ERR_EN_W { w: self }
    }
    #[doc = "Bit 5 - Data CRC Error Signal Enable"]
    #[inline(always)]
    pub fn data_crc_err_en(&mut self) -> DATA_CRC_ERR_EN_W {
        DATA_CRC_ERR_EN_W { w: self }
    }
    #[doc = "Bit 4 - Data Timeout Error Signal Enable"]
    #[inline(always)]
    pub fn data_timeout_err_en(&mut self) -> DATA_TIMEOUT_ERR_EN_W {
        DATA_TIMEOUT_ERR_EN_W { w: self }
    }
    #[doc = "Bit 3 - Command Index Error Signal Enable"]
    #[inline(always)]
    pub fn cmd_ind_err_en(&mut self) -> CMD_IND_ERR_EN_W {
        CMD_IND_ERR_EN_W { w: self }
    }
    #[doc = "Bit 2 - Command End Bit Error Signal Enable"]
    #[inline(always)]
    pub fn cmd_end_bit_err_en(&mut self) -> CMD_END_BIT_ERR_EN_W {
        CMD_END_BIT_ERR_EN_W { w: self }
    }
    #[doc = "Bit 1 - Command CRC Error Signal Enable"]
    #[inline(always)]
    pub fn cmd_crc_err_en(&mut self) -> CMD_CRC_ERR_EN_W {
        CMD_CRC_ERR_EN_W { w: self }
    }
    #[doc = "Bit 0 - Command Timeout Error Signal Enable"]
    #[inline(always)]
    pub fn cmd_timeout_err_en(&mut self) -> CMD_TIMEOUT_ERR_EN_W {
        CMD_TIMEOUT_ERR_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Interrupt Signal Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en_int_signal_err](index.html) module"]
pub struct EN_INT_SIGNAL_ERR_SPEC;
impl crate::RegisterSpec for EN_INT_SIGNAL_ERR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [en_int_signal_err::R](R) reader structure"]
impl crate::Readable for EN_INT_SIGNAL_ERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [en_int_signal_err::W](W) writer structure"]
impl crate::Writable for EN_INT_SIGNAL_ERR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EN_INT_SIGNAL_ERR to value 0"]
impl crate::Resettable for EN_INT_SIGNAL_ERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
