#[doc = "Register `COMMAND` reader"]
pub struct R(crate::R<COMMAND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMMAND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMMAND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMMAND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMMAND` writer"]
pub struct W(crate::W<COMMAND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMMAND_SPEC>;
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
impl From<crate::W<COMMAND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMMAND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESP_TYPE_SELECT` reader - Response Type Select"]
pub type RESP_TYPE_SELECT_R = crate::FieldReader<u8, RESP_TYPE_SELECT_A>;
#[doc = "Response Type Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RESP_TYPE_SELECT_A {
    #[doc = "0: No Response"]
    VALUE1 = 0,
    #[doc = "1: Response length 136"]
    VALUE2 = 1,
    #[doc = "2: Response length 48"]
    VALUE3 = 2,
    #[doc = "3: Response length 48 check Busy after response"]
    VALUE4 = 3,
}
impl From<RESP_TYPE_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RESP_TYPE_SELECT_A) -> Self {
        variant as _
    }
}
impl RESP_TYPE_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESP_TYPE_SELECT_A {
        match self.bits {
            0 => RESP_TYPE_SELECT_A::VALUE1,
            1 => RESP_TYPE_SELECT_A::VALUE2,
            2 => RESP_TYPE_SELECT_A::VALUE3,
            3 => RESP_TYPE_SELECT_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESP_TYPE_SELECT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESP_TYPE_SELECT_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RESP_TYPE_SELECT_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RESP_TYPE_SELECT_A::VALUE4
    }
}
#[doc = "Field `RESP_TYPE_SELECT` writer - Response Type Select"]
pub type RESP_TYPE_SELECT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, COMMAND_SPEC, u8, RESP_TYPE_SELECT_A, 2, O>;
impl<'a, const O: u8> RESP_TYPE_SELECT_W<'a, O> {
    #[doc = "No Response"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESP_TYPE_SELECT_A::VALUE1)
    }
    #[doc = "Response length 136"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESP_TYPE_SELECT_A::VALUE2)
    }
    #[doc = "Response length 48"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(RESP_TYPE_SELECT_A::VALUE3)
    }
    #[doc = "Response length 48 check Busy after response"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(RESP_TYPE_SELECT_A::VALUE4)
    }
}
#[doc = "Field `CMD_CRC_CHECK_EN` reader - Command CRC Check Enable"]
pub type CMD_CRC_CHECK_EN_R = crate::BitReader<CMD_CRC_CHECK_EN_A>;
#[doc = "Command CRC Check Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMD_CRC_CHECK_EN_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<CMD_CRC_CHECK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_CRC_CHECK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CMD_CRC_CHECK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_CRC_CHECK_EN_A {
        match self.bits {
            false => CMD_CRC_CHECK_EN_A::VALUE1,
            true => CMD_CRC_CHECK_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD_CRC_CHECK_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD_CRC_CHECK_EN_A::VALUE2
    }
}
#[doc = "Field `CMD_CRC_CHECK_EN` writer - Command CRC Check Enable"]
pub type CMD_CRC_CHECK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, COMMAND_SPEC, CMD_CRC_CHECK_EN_A, O>;
impl<'a, const O: u8> CMD_CRC_CHECK_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_CRC_CHECK_EN_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_CRC_CHECK_EN_A::VALUE2)
    }
}
#[doc = "Field `CMD_IND_CHECK_EN` reader - Command Index Check Enable"]
pub type CMD_IND_CHECK_EN_R = crate::BitReader<CMD_IND_CHECK_EN_A>;
#[doc = "Command Index Check Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMD_IND_CHECK_EN_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<CMD_IND_CHECK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_IND_CHECK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CMD_IND_CHECK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_IND_CHECK_EN_A {
        match self.bits {
            false => CMD_IND_CHECK_EN_A::VALUE1,
            true => CMD_IND_CHECK_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD_IND_CHECK_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD_IND_CHECK_EN_A::VALUE2
    }
}
#[doc = "Field `CMD_IND_CHECK_EN` writer - Command Index Check Enable"]
pub type CMD_IND_CHECK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, COMMAND_SPEC, CMD_IND_CHECK_EN_A, O>;
impl<'a, const O: u8> CMD_IND_CHECK_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_IND_CHECK_EN_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_IND_CHECK_EN_A::VALUE2)
    }
}
#[doc = "Field `DATA_PRESENT_SELECT` reader - Data Present Select"]
pub type DATA_PRESENT_SELECT_R = crate::BitReader<DATA_PRESENT_SELECT_A>;
#[doc = "Data Present Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATA_PRESENT_SELECT_A {
    #[doc = "0: No Data Present"]
    VALUE1 = 0,
    #[doc = "1: Data Present"]
    VALUE2 = 1,
}
impl From<DATA_PRESENT_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_PRESENT_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DATA_PRESENT_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_PRESENT_SELECT_A {
        match self.bits {
            false => DATA_PRESENT_SELECT_A::VALUE1,
            true => DATA_PRESENT_SELECT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DATA_PRESENT_SELECT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DATA_PRESENT_SELECT_A::VALUE2
    }
}
#[doc = "Field `DATA_PRESENT_SELECT` writer - Data Present Select"]
pub type DATA_PRESENT_SELECT_W<'a, const O: u8> = crate::BitWriter<'a, u16, COMMAND_SPEC, DATA_PRESENT_SELECT_A, O>;
impl<'a, const O: u8> DATA_PRESENT_SELECT_W<'a, O> {
    #[doc = "No Data Present"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATA_PRESENT_SELECT_A::VALUE1)
    }
    #[doc = "Data Present"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATA_PRESENT_SELECT_A::VALUE2)
    }
}
#[doc = "Field `CMD_TYPE` reader - Command Type"]
pub type CMD_TYPE_R = crate::FieldReader<u8, CMD_TYPE_A>;
#[doc = "Command Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMD_TYPE_A {
    #[doc = "0: Normal"]
    VALUE1 = 0,
    #[doc = "1: Suspend"]
    VALUE2 = 1,
    #[doc = "2: Resume"]
    VALUE3 = 2,
    #[doc = "3: Abort"]
    VALUE4 = 3,
}
impl From<CMD_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_TYPE_A) -> Self {
        variant as _
    }
}
impl CMD_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_TYPE_A {
        match self.bits {
            0 => CMD_TYPE_A::VALUE1,
            1 => CMD_TYPE_A::VALUE2,
            2 => CMD_TYPE_A::VALUE3,
            3 => CMD_TYPE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD_TYPE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD_TYPE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CMD_TYPE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CMD_TYPE_A::VALUE4
    }
}
#[doc = "Field `CMD_TYPE` writer - Command Type"]
pub type CMD_TYPE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, COMMAND_SPEC, u8, CMD_TYPE_A, 2, O>;
impl<'a, const O: u8> CMD_TYPE_W<'a, O> {
    #[doc = "Normal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_TYPE_A::VALUE1)
    }
    #[doc = "Suspend"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_TYPE_A::VALUE2)
    }
    #[doc = "Resume"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CMD_TYPE_A::VALUE3)
    }
    #[doc = "Abort"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CMD_TYPE_A::VALUE4)
    }
}
#[doc = "Field `CMD_IND` reader - Command Index"]
pub type CMD_IND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMD_IND` writer - Command Index"]
pub type CMD_IND_W<'a, const O: u8> = crate::FieldWriter<'a, u16, COMMAND_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:1 - Response Type Select"]
    #[inline(always)]
    pub fn resp_type_select(&self) -> RESP_TYPE_SELECT_R {
        RESP_TYPE_SELECT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Command CRC Check Enable"]
    #[inline(always)]
    pub fn cmd_crc_check_en(&self) -> CMD_CRC_CHECK_EN_R {
        CMD_CRC_CHECK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Command Index Check Enable"]
    #[inline(always)]
    pub fn cmd_ind_check_en(&self) -> CMD_IND_CHECK_EN_R {
        CMD_IND_CHECK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Present Select"]
    #[inline(always)]
    pub fn data_present_select(&self) -> DATA_PRESENT_SELECT_R {
        DATA_PRESENT_SELECT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Command Type"]
    #[inline(always)]
    pub fn cmd_type(&self) -> CMD_TYPE_R {
        CMD_TYPE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:13 - Command Index"]
    #[inline(always)]
    pub fn cmd_ind(&self) -> CMD_IND_R {
        CMD_IND_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Response Type Select"]
    #[inline(always)]
    #[must_use]
    pub fn resp_type_select(&mut self) -> RESP_TYPE_SELECT_W<0> {
        RESP_TYPE_SELECT_W::new(self)
    }
    #[doc = "Bit 3 - Command CRC Check Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crc_check_en(&mut self) -> CMD_CRC_CHECK_EN_W<3> {
        CMD_CRC_CHECK_EN_W::new(self)
    }
    #[doc = "Bit 4 - Command Index Check Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_ind_check_en(&mut self) -> CMD_IND_CHECK_EN_W<4> {
        CMD_IND_CHECK_EN_W::new(self)
    }
    #[doc = "Bit 5 - Data Present Select"]
    #[inline(always)]
    #[must_use]
    pub fn data_present_select(&mut self) -> DATA_PRESENT_SELECT_W<5> {
        DATA_PRESENT_SELECT_W::new(self)
    }
    #[doc = "Bits 6:7 - Command Type"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_type(&mut self) -> CMD_TYPE_W<6> {
        CMD_TYPE_W::new(self)
    }
    #[doc = "Bits 8:13 - Command Index"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_ind(&mut self) -> CMD_IND_W<8> {
        CMD_IND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [command](index.html) module"]
pub struct COMMAND_SPEC;
impl crate::RegisterSpec for COMMAND_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [command::R](R) reader structure"]
impl crate::Readable for COMMAND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [command::W](W) writer structure"]
impl crate::Writable for COMMAND_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMMAND to value 0"]
impl crate::Resettable for COMMAND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
