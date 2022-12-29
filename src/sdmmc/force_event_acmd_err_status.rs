#[doc = "Register `FORCE_EVENT_ACMD_ERR_STATUS` writer"]
pub struct W(crate::W<FORCE_EVENT_ACMD_ERR_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FORCE_EVENT_ACMD_ERR_STATUS_SPEC>;
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
impl From<crate::W<FORCE_EVENT_ACMD_ERR_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FORCE_EVENT_ACMD_ERR_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Force Event for Auto CMD12 NOT Executed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE_ACMD_NOT_EXEC_AW {
    #[doc = "0: No interrupt"]
    VALUE1 = 0,
    #[doc = "1: Interrupt is generated"]
    VALUE2 = 1,
}
impl From<FE_ACMD_NOT_EXEC_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_ACMD_NOT_EXEC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_ACMD_NOT_EXEC` writer - Force Event for Auto CMD12 NOT Executed"]
pub type FE_ACMD_NOT_EXEC_W<'a, const O: u8> = crate::BitWriter<'a, u16, FORCE_EVENT_ACMD_ERR_STATUS_SPEC, FE_ACMD_NOT_EXEC_AW, O>;
impl<'a, const O: u8> FE_ACMD_NOT_EXEC_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_ACMD_NOT_EXEC_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_ACMD_NOT_EXEC_AW::VALUE2)
    }
}
#[doc = "Force Event for Auto CMD timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE_ACMD_TIMEOUT_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1 = 0,
    #[doc = "1: Interrupt is generated"]
    VALUE2 = 1,
}
impl From<FE_ACMD_TIMEOUT_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_ACMD_TIMEOUT_ERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_ACMD_TIMEOUT_ERR` writer - Force Event for Auto CMD timeout Error"]
pub type FE_ACMD_TIMEOUT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, FORCE_EVENT_ACMD_ERR_STATUS_SPEC, FE_ACMD_TIMEOUT_ERR_AW, O>;
impl<'a, const O: u8> FE_ACMD_TIMEOUT_ERR_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_ACMD_TIMEOUT_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_ACMD_TIMEOUT_ERR_AW::VALUE2)
    }
}
#[doc = "Force Event for Auto CMD CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE_ACMD_CRC_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1 = 0,
    #[doc = "1: Interrupt is generated"]
    VALUE2 = 1,
}
impl From<FE_ACMD_CRC_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_ACMD_CRC_ERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_ACMD_CRC_ERR` writer - Force Event for Auto CMD CRC Error"]
pub type FE_ACMD_CRC_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, FORCE_EVENT_ACMD_ERR_STATUS_SPEC, FE_ACMD_CRC_ERR_AW, O>;
impl<'a, const O: u8> FE_ACMD_CRC_ERR_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_ACMD_CRC_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_ACMD_CRC_ERR_AW::VALUE2)
    }
}
#[doc = "Force Event for Auto CMD End bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE_ACMD_END_BIT_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1 = 0,
    #[doc = "1: Interrupt is generated"]
    VALUE2 = 1,
}
impl From<FE_ACMD_END_BIT_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_ACMD_END_BIT_ERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_ACMD_END_BIT_ERR` writer - Force Event for Auto CMD End bit Error"]
pub type FE_ACMD_END_BIT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, FORCE_EVENT_ACMD_ERR_STATUS_SPEC, FE_ACMD_END_BIT_ERR_AW, O>;
impl<'a, const O: u8> FE_ACMD_END_BIT_ERR_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_ACMD_END_BIT_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_ACMD_END_BIT_ERR_AW::VALUE2)
    }
}
#[doc = "Force Event for Auto CMD Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE_ACMD_IND_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1 = 0,
    #[doc = "1: Interrupt is generated"]
    VALUE2 = 1,
}
impl From<FE_ACMD_IND_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_ACMD_IND_ERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_ACMD_IND_ERR` writer - Force Event for Auto CMD Index Error"]
pub type FE_ACMD_IND_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, FORCE_EVENT_ACMD_ERR_STATUS_SPEC, FE_ACMD_IND_ERR_AW, O>;
impl<'a, const O: u8> FE_ACMD_IND_ERR_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_ACMD_IND_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_ACMD_IND_ERR_AW::VALUE2)
    }
}
#[doc = "Force Event for CMD not issued by Auto CMD12 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE_CMD_NOT_ISSUED_ACMD12_ERR_AW {
    #[doc = "0: No interrupt"]
    VALUE1 = 0,
    #[doc = "1: Interrupt is generated"]
    VALUE2 = 1,
}
impl From<FE_CMD_NOT_ISSUED_ACMD12_ERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FE_CMD_NOT_ISSUED_ACMD12_ERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_CMD_NOT_ISSUED_ACMD12_ERR` writer - Force Event for CMD not issued by Auto CMD12 Error"]
pub type FE_CMD_NOT_ISSUED_ACMD12_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, FORCE_EVENT_ACMD_ERR_STATUS_SPEC, FE_CMD_NOT_ISSUED_ACMD12_ERR_AW, O>;
impl<'a, const O: u8> FE_CMD_NOT_ISSUED_ACMD12_ERR_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_CMD_NOT_ISSUED_ACMD12_ERR_AW::VALUE1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_CMD_NOT_ISSUED_ACMD12_ERR_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Force Event for Auto CMD12 NOT Executed"]
    #[inline(always)]
    #[must_use]
    pub fn fe_acmd_not_exec(&mut self) -> FE_ACMD_NOT_EXEC_W<0> {
        FE_ACMD_NOT_EXEC_W::new(self)
    }
    #[doc = "Bit 1 - Force Event for Auto CMD timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_acmd_timeout_err(&mut self) -> FE_ACMD_TIMEOUT_ERR_W<1> {
        FE_ACMD_TIMEOUT_ERR_W::new(self)
    }
    #[doc = "Bit 2 - Force Event for Auto CMD CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_acmd_crc_err(&mut self) -> FE_ACMD_CRC_ERR_W<2> {
        FE_ACMD_CRC_ERR_W::new(self)
    }
    #[doc = "Bit 3 - Force Event for Auto CMD End bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_acmd_end_bit_err(&mut self) -> FE_ACMD_END_BIT_ERR_W<3> {
        FE_ACMD_END_BIT_ERR_W::new(self)
    }
    #[doc = "Bit 4 - Force Event for Auto CMD Index Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_acmd_ind_err(&mut self) -> FE_ACMD_IND_ERR_W<4> {
        FE_ACMD_IND_ERR_W::new(self)
    }
    #[doc = "Bit 7 - Force Event for CMD not issued by Auto CMD12 Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_cmd_not_issued_acmd12_err(&mut self) -> FE_CMD_NOT_ISSUED_ACMD12_ERR_W<7> {
        FE_CMD_NOT_ISSUED_ACMD12_ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Force Event Register for Auto CMD Error Status\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [force_event_acmd_err_status](index.html) module"]
pub struct FORCE_EVENT_ACMD_ERR_STATUS_SPEC;
impl crate::RegisterSpec for FORCE_EVENT_ACMD_ERR_STATUS_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [force_event_acmd_err_status::W](W) writer structure"]
impl crate::Writable for FORCE_EVENT_ACMD_ERR_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FORCE_EVENT_ACMD_ERR_STATUS to value 0"]
impl crate::Resettable for FORCE_EVENT_ACMD_ERR_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
