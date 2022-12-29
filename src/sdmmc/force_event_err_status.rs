#[doc = "Register `FORCE_EVENT_ERR_STATUS` writer"]
pub struct W(crate::W<FORCE_EVENT_ERR_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FORCE_EVENT_ERR_STATUS_SPEC>;
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
impl From<crate::W<FORCE_EVENT_ERR_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FORCE_EVENT_ERR_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Force Event for Command Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE_CMD_TIMEOUT_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1 = 0,
    #[doc = "1: Interrupt is generated"]
    VALUE2 = 1,
}
impl From<FE_CMD_TIMEOUT_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_CMD_TIMEOUT_ERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_CMD_TIMEOUT_ERR` writer - Force Event for Command Timeout Error"]
pub type FE_CMD_TIMEOUT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, FORCE_EVENT_ERR_STATUS_SPEC, FE_CMD_TIMEOUT_ERR_AW, O>;
impl<'a, const O: u8> FE_CMD_TIMEOUT_ERR_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_CMD_TIMEOUT_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_CMD_TIMEOUT_ERR_AW::VALUE2)
    }
}
#[doc = "Force Event for Command CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE_CMD_CRC_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1 = 0,
    #[doc = "1: Interrupt is generated"]
    VALUE2 = 1,
}
impl From<FE_CMD_CRC_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_CMD_CRC_ERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_CMD_CRC_ERR` writer - Force Event for Command CRC Error"]
pub type FE_CMD_CRC_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, FORCE_EVENT_ERR_STATUS_SPEC, FE_CMD_CRC_ERR_AW, O>;
impl<'a, const O: u8> FE_CMD_CRC_ERR_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_CMD_CRC_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_CMD_CRC_ERR_AW::VALUE2)
    }
}
#[doc = "Force Event for Command End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE_CMD_END_BIT_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1 = 0,
    #[doc = "1: Interrupt is generated"]
    VALUE2 = 1,
}
impl From<FE_CMD_END_BIT_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_CMD_END_BIT_ERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_CMD_END_BIT_ERR` writer - Force Event for Command End Bit Error"]
pub type FE_CMD_END_BIT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, FORCE_EVENT_ERR_STATUS_SPEC, FE_CMD_END_BIT_ERR_AW, O>;
impl<'a, const O: u8> FE_CMD_END_BIT_ERR_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_CMD_END_BIT_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_CMD_END_BIT_ERR_AW::VALUE2)
    }
}
#[doc = "Force Event for Command Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE_CMD_IND_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1 = 0,
    #[doc = "1: Interrupt is generated"]
    VALUE2 = 1,
}
impl From<FE_CMD_IND_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_CMD_IND_ERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_CMD_IND_ERR` writer - Force Event for Command Index Error"]
pub type FE_CMD_IND_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, FORCE_EVENT_ERR_STATUS_SPEC, FE_CMD_IND_ERR_AW, O>;
impl<'a, const O: u8> FE_CMD_IND_ERR_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_CMD_IND_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_CMD_IND_ERR_AW::VALUE2)
    }
}
#[doc = "Force Event for Data Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE_DATA_TIMEOUT_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1 = 0,
    #[doc = "1: Interrupt is generated"]
    VALUE2 = 1,
}
impl From<FE_DATA_TIMEOUT_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_DATA_TIMEOUT_ERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_DATA_TIMEOUT_ERR` writer - Force Event for Data Timeout Error"]
pub type FE_DATA_TIMEOUT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, FORCE_EVENT_ERR_STATUS_SPEC, FE_DATA_TIMEOUT_ERR_AW, O>;
impl<'a, const O: u8> FE_DATA_TIMEOUT_ERR_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_DATA_TIMEOUT_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_DATA_TIMEOUT_ERR_AW::VALUE2)
    }
}
#[doc = "Force Event for Data CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE_DATA_CRC_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1 = 0,
    #[doc = "1: Interrupt is generated"]
    VALUE2 = 1,
}
impl From<FE_DATA_CRC_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_DATA_CRC_ERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_DATA_CRC_ERR` writer - Force Event for Data CRC Error"]
pub type FE_DATA_CRC_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, FORCE_EVENT_ERR_STATUS_SPEC, FE_DATA_CRC_ERR_AW, O>;
impl<'a, const O: u8> FE_DATA_CRC_ERR_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_DATA_CRC_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_DATA_CRC_ERR_AW::VALUE2)
    }
}
#[doc = "Force Event for Data End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE_DATA_END_BIT_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1 = 0,
    #[doc = "1: Interrupt is generated"]
    VALUE2 = 1,
}
impl From<FE_DATA_END_BIT_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_DATA_END_BIT_ERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_DATA_END_BIT_ERR` writer - Force Event for Data End Bit Error"]
pub type FE_DATA_END_BIT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, FORCE_EVENT_ERR_STATUS_SPEC, FE_DATA_END_BIT_ERR_AW, O>;
impl<'a, const O: u8> FE_DATA_END_BIT_ERR_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_DATA_END_BIT_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_DATA_END_BIT_ERR_AW::VALUE2)
    }
}
#[doc = "Force Event for Current Limit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE_CURRENT_LIMIT_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1 = 0,
    #[doc = "1: Interrupt is generated"]
    VALUE2 = 1,
}
impl From<FE_CURRENT_LIMIT_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_CURRENT_LIMIT_ERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_CURRENT_LIMIT_ERR` writer - Force Event for Current Limit Error"]
pub type FE_CURRENT_LIMIT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, FORCE_EVENT_ERR_STATUS_SPEC, FE_CURRENT_LIMIT_ERR_AW, O>;
impl<'a, const O: u8> FE_CURRENT_LIMIT_ERR_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_CURRENT_LIMIT_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_CURRENT_LIMIT_ERR_AW::VALUE2)
    }
}
#[doc = "Force Event for Auto CMD Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE_ACMD12_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1 = 0,
    #[doc = "1: Interrupt is generated"]
    VALUE2 = 1,
}
impl From<FE_ACMD12_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_ACMD12_ERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_ACMD12_ERR` writer - Force Event for Auto CMD Error"]
pub type FE_ACMD12_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, FORCE_EVENT_ERR_STATUS_SPEC, FE_ACMD12_ERR_AW, O>;
impl<'a, const O: u8> FE_ACMD12_ERR_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_ACMD12_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_ACMD12_ERR_AW::VALUE2)
    }
}
#[doc = "Force event for Target Response Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE_TARGET_RESPONSE_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1 = 0,
    #[doc = "1: Interrupt is generated"]
    VALUE2 = 1,
}
impl From<FE_TARGET_RESPONSE_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_TARGET_RESPONSE_ERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_TARGET_RESPONSE_ERR` writer - Force event for Target Response Error"]
pub type FE_TARGET_RESPONSE_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, FORCE_EVENT_ERR_STATUS_SPEC, FE_TARGET_RESPONSE_ERR_AW, O>;
impl<'a, const O: u8> FE_TARGET_RESPONSE_ERR_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_TARGET_RESPONSE_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_TARGET_RESPONSE_ERR_AW::VALUE2)
    }
}
#[doc = "Force Event for Ceata Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE_CEATA_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1 = 0,
    #[doc = "1: Interrupt is generated"]
    VALUE2 = 1,
}
impl From<FE_CEATA_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_CEATA_ERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_CEATA_ERR` writer - Force Event for Ceata Error"]
pub type FE_CEATA_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, FORCE_EVENT_ERR_STATUS_SPEC, FE_CEATA_ERR_AW, O>;
impl<'a, const O: u8> FE_CEATA_ERR_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_CEATA_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_CEATA_ERR_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Force Event for Command Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_cmd_timeout_err(&mut self) -> FE_CMD_TIMEOUT_ERR_W<0> {
        FE_CMD_TIMEOUT_ERR_W::new(self)
    }
    #[doc = "Bit 1 - Force Event for Command CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_cmd_crc_err(&mut self) -> FE_CMD_CRC_ERR_W<1> {
        FE_CMD_CRC_ERR_W::new(self)
    }
    #[doc = "Bit 2 - Force Event for Command End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_cmd_end_bit_err(&mut self) -> FE_CMD_END_BIT_ERR_W<2> {
        FE_CMD_END_BIT_ERR_W::new(self)
    }
    #[doc = "Bit 3 - Force Event for Command Index Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_cmd_ind_err(&mut self) -> FE_CMD_IND_ERR_W<3> {
        FE_CMD_IND_ERR_W::new(self)
    }
    #[doc = "Bit 4 - Force Event for Data Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_data_timeout_err(&mut self) -> FE_DATA_TIMEOUT_ERR_W<4> {
        FE_DATA_TIMEOUT_ERR_W::new(self)
    }
    #[doc = "Bit 5 - Force Event for Data CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_data_crc_err(&mut self) -> FE_DATA_CRC_ERR_W<5> {
        FE_DATA_CRC_ERR_W::new(self)
    }
    #[doc = "Bit 6 - Force Event for Data End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_data_end_bit_err(&mut self) -> FE_DATA_END_BIT_ERR_W<6> {
        FE_DATA_END_BIT_ERR_W::new(self)
    }
    #[doc = "Bit 7 - Force Event for Current Limit Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_current_limit_err(&mut self) -> FE_CURRENT_LIMIT_ERR_W<7> {
        FE_CURRENT_LIMIT_ERR_W::new(self)
    }
    #[doc = "Bit 8 - Force Event for Auto CMD Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_acmd12_err(&mut self) -> FE_ACMD12_ERR_W<8> {
        FE_ACMD12_ERR_W::new(self)
    }
    #[doc = "Bit 12 - Force event for Target Response Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_target_response_err(&mut self) -> FE_TARGET_RESPONSE_ERR_W<12> {
        FE_TARGET_RESPONSE_ERR_W::new(self)
    }
    #[doc = "Bit 13 - Force Event for Ceata Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_ceata_err(&mut self) -> FE_CEATA_ERR_W<13> {
        FE_CEATA_ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Force Event Register for Error Interrupt Status\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [force_event_err_status](index.html) module"]
pub struct FORCE_EVENT_ERR_STATUS_SPEC;
impl crate::RegisterSpec for FORCE_EVENT_ERR_STATUS_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [force_event_err_status::W](W) writer structure"]
impl crate::Writable for FORCE_EVENT_ERR_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FORCE_EVENT_ERR_STATUS to value 0"]
impl crate::Resettable for FORCE_EVENT_ERR_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
