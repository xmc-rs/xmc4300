#[doc = "Register `FORCE_EVENT_ACMD_ERR_STATUS` writer"]
pub type W = crate::W<ForceEventAcmdErrStatusSpec>;
#[doc = "Force Event for Auto CMD12 NOT Executed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeAcmdNotExec {
    #[doc = "0: No interrupt"]
    Value1 = 0,
    #[doc = "1: Interrupt is generated"]
    Value2 = 1,
}
impl From<FeAcmdNotExec> for bool {
    #[inline(always)]
    fn from(variant: FeAcmdNotExec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_ACMD_NOT_EXEC` writer - Force Event for Auto CMD12 NOT Executed"]
pub type FeAcmdNotExecW<'a, REG> = crate::BitWriter<'a, REG, FeAcmdNotExec>;
impl<'a, REG> FeAcmdNotExecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FeAcmdNotExec::Value1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FeAcmdNotExec::Value2)
    }
}
#[doc = "Force Event for Auto CMD timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeAcmdTimeoutErr {
    #[doc = "0: No interrupt"]
    Value1 = 0,
    #[doc = "1: Interrupt is generated"]
    Value2 = 1,
}
impl From<FeAcmdTimeoutErr> for bool {
    #[inline(always)]
    fn from(variant: FeAcmdTimeoutErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_ACMD_TIMEOUT_ERR` writer - Force Event for Auto CMD timeout Error"]
pub type FeAcmdTimeoutErrW<'a, REG> = crate::BitWriter<'a, REG, FeAcmdTimeoutErr>;
impl<'a, REG> FeAcmdTimeoutErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FeAcmdTimeoutErr::Value1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FeAcmdTimeoutErr::Value2)
    }
}
#[doc = "Force Event for Auto CMD CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeAcmdCrcErr {
    #[doc = "0: No interrupt"]
    Value1 = 0,
    #[doc = "1: Interrupt is generated"]
    Value2 = 1,
}
impl From<FeAcmdCrcErr> for bool {
    #[inline(always)]
    fn from(variant: FeAcmdCrcErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_ACMD_CRC_ERR` writer - Force Event for Auto CMD CRC Error"]
pub type FeAcmdCrcErrW<'a, REG> = crate::BitWriter<'a, REG, FeAcmdCrcErr>;
impl<'a, REG> FeAcmdCrcErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FeAcmdCrcErr::Value1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FeAcmdCrcErr::Value2)
    }
}
#[doc = "Force Event for Auto CMD End bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeAcmdEndBitErr {
    #[doc = "0: No interrupt"]
    Value1 = 0,
    #[doc = "1: Interrupt is generated"]
    Value2 = 1,
}
impl From<FeAcmdEndBitErr> for bool {
    #[inline(always)]
    fn from(variant: FeAcmdEndBitErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_ACMD_END_BIT_ERR` writer - Force Event for Auto CMD End bit Error"]
pub type FeAcmdEndBitErrW<'a, REG> = crate::BitWriter<'a, REG, FeAcmdEndBitErr>;
impl<'a, REG> FeAcmdEndBitErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FeAcmdEndBitErr::Value1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FeAcmdEndBitErr::Value2)
    }
}
#[doc = "Force Event for Auto CMD Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeAcmdIndErr {
    #[doc = "0: No interrupt"]
    Value1 = 0,
    #[doc = "1: Interrupt is generated"]
    Value2 = 1,
}
impl From<FeAcmdIndErr> for bool {
    #[inline(always)]
    fn from(variant: FeAcmdIndErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_ACMD_IND_ERR` writer - Force Event for Auto CMD Index Error"]
pub type FeAcmdIndErrW<'a, REG> = crate::BitWriter<'a, REG, FeAcmdIndErr>;
impl<'a, REG> FeAcmdIndErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FeAcmdIndErr::Value1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FeAcmdIndErr::Value2)
    }
}
#[doc = "Force Event for CMD not issued by Auto CMD12 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeCmdNotIssuedAcmd12Err {
    #[doc = "0: No interrupt"]
    Value1 = 0,
    #[doc = "1: Interrupt is generated"]
    Value2 = 1,
}
impl From<FeCmdNotIssuedAcmd12Err> for bool {
    #[inline(always)]
    fn from(variant: FeCmdNotIssuedAcmd12Err) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_CMD_NOT_ISSUED_ACMD12_ERR` writer - Force Event for CMD not issued by Auto CMD12 Error"]
pub type FeCmdNotIssuedAcmd12ErrW<'a, REG> = crate::BitWriter<'a, REG, FeCmdNotIssuedAcmd12Err>;
impl<'a, REG> FeCmdNotIssuedAcmd12ErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FeCmdNotIssuedAcmd12Err::Value1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FeCmdNotIssuedAcmd12Err::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - Force Event for Auto CMD12 NOT Executed"]
    #[inline(always)]
    #[must_use]
    pub fn fe_acmd_not_exec(&mut self) -> FeAcmdNotExecW<ForceEventAcmdErrStatusSpec> {
        FeAcmdNotExecW::new(self, 0)
    }
    #[doc = "Bit 1 - Force Event for Auto CMD timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_acmd_timeout_err(&mut self) -> FeAcmdTimeoutErrW<ForceEventAcmdErrStatusSpec> {
        FeAcmdTimeoutErrW::new(self, 1)
    }
    #[doc = "Bit 2 - Force Event for Auto CMD CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_acmd_crc_err(&mut self) -> FeAcmdCrcErrW<ForceEventAcmdErrStatusSpec> {
        FeAcmdCrcErrW::new(self, 2)
    }
    #[doc = "Bit 3 - Force Event for Auto CMD End bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_acmd_end_bit_err(&mut self) -> FeAcmdEndBitErrW<ForceEventAcmdErrStatusSpec> {
        FeAcmdEndBitErrW::new(self, 3)
    }
    #[doc = "Bit 4 - Force Event for Auto CMD Index Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_acmd_ind_err(&mut self) -> FeAcmdIndErrW<ForceEventAcmdErrStatusSpec> {
        FeAcmdIndErrW::new(self, 4)
    }
    #[doc = "Bit 7 - Force Event for CMD not issued by Auto CMD12 Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_cmd_not_issued_acmd12_err(&mut self) -> FeCmdNotIssuedAcmd12ErrW<ForceEventAcmdErrStatusSpec> {
        FeCmdNotIssuedAcmd12ErrW::new(self, 7)
    }
}
#[doc = "Force Event Register for Auto CMD Error Status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`force_event_acmd_err_status::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ForceEventAcmdErrStatusSpec;
impl crate::RegisterSpec for ForceEventAcmdErrStatusSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`force_event_acmd_err_status::W`](W) writer structure"]
impl crate::Writable for ForceEventAcmdErrStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FORCE_EVENT_ACMD_ERR_STATUS to value 0"]
impl crate::Resettable for ForceEventAcmdErrStatusSpec {
    const RESET_VALUE: u16 = 0;
}
