#[doc = "Register `FORCE_EVENT_ERR_STATUS` writer"]
pub type W = crate::W<ForceEventErrStatusSpec>;
#[doc = "Force Event for Command Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeCmdTimeoutErr {
    #[doc = "0: No interrupt"]
    Value1 = 0,
    #[doc = "1: Interrupt is generated"]
    Value2 = 1,
}
impl From<FeCmdTimeoutErr> for bool {
    #[inline(always)]
    fn from(variant: FeCmdTimeoutErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_CMD_TIMEOUT_ERR` writer - Force Event for Command Timeout Error"]
pub type FeCmdTimeoutErrW<'a, REG> = crate::BitWriter<'a, REG, FeCmdTimeoutErr>;
impl<'a, REG> FeCmdTimeoutErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FeCmdTimeoutErr::Value1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FeCmdTimeoutErr::Value2)
    }
}
#[doc = "Force Event for Command CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeCmdCrcErr {
    #[doc = "0: No interrupt"]
    Value1 = 0,
    #[doc = "1: Interrupt is generated"]
    Value2 = 1,
}
impl From<FeCmdCrcErr> for bool {
    #[inline(always)]
    fn from(variant: FeCmdCrcErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_CMD_CRC_ERR` writer - Force Event for Command CRC Error"]
pub type FeCmdCrcErrW<'a, REG> = crate::BitWriter<'a, REG, FeCmdCrcErr>;
impl<'a, REG> FeCmdCrcErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FeCmdCrcErr::Value1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FeCmdCrcErr::Value2)
    }
}
#[doc = "Force Event for Command End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeCmdEndBitErr {
    #[doc = "0: No interrupt"]
    Value1 = 0,
    #[doc = "1: Interrupt is generated"]
    Value2 = 1,
}
impl From<FeCmdEndBitErr> for bool {
    #[inline(always)]
    fn from(variant: FeCmdEndBitErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_CMD_END_BIT_ERR` writer - Force Event for Command End Bit Error"]
pub type FeCmdEndBitErrW<'a, REG> = crate::BitWriter<'a, REG, FeCmdEndBitErr>;
impl<'a, REG> FeCmdEndBitErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FeCmdEndBitErr::Value1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FeCmdEndBitErr::Value2)
    }
}
#[doc = "Force Event for Command Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeCmdIndErr {
    #[doc = "0: No interrupt"]
    Value1 = 0,
    #[doc = "1: Interrupt is generated"]
    Value2 = 1,
}
impl From<FeCmdIndErr> for bool {
    #[inline(always)]
    fn from(variant: FeCmdIndErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_CMD_IND_ERR` writer - Force Event for Command Index Error"]
pub type FeCmdIndErrW<'a, REG> = crate::BitWriter<'a, REG, FeCmdIndErr>;
impl<'a, REG> FeCmdIndErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FeCmdIndErr::Value1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FeCmdIndErr::Value2)
    }
}
#[doc = "Force Event for Data Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeDataTimeoutErr {
    #[doc = "0: No interrupt"]
    Value1 = 0,
    #[doc = "1: Interrupt is generated"]
    Value2 = 1,
}
impl From<FeDataTimeoutErr> for bool {
    #[inline(always)]
    fn from(variant: FeDataTimeoutErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_DATA_TIMEOUT_ERR` writer - Force Event for Data Timeout Error"]
pub type FeDataTimeoutErrW<'a, REG> = crate::BitWriter<'a, REG, FeDataTimeoutErr>;
impl<'a, REG> FeDataTimeoutErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FeDataTimeoutErr::Value1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FeDataTimeoutErr::Value2)
    }
}
#[doc = "Force Event for Data CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeDataCrcErr {
    #[doc = "0: No interrupt"]
    Value1 = 0,
    #[doc = "1: Interrupt is generated"]
    Value2 = 1,
}
impl From<FeDataCrcErr> for bool {
    #[inline(always)]
    fn from(variant: FeDataCrcErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_DATA_CRC_ERR` writer - Force Event for Data CRC Error"]
pub type FeDataCrcErrW<'a, REG> = crate::BitWriter<'a, REG, FeDataCrcErr>;
impl<'a, REG> FeDataCrcErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FeDataCrcErr::Value1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FeDataCrcErr::Value2)
    }
}
#[doc = "Force Event for Data End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeDataEndBitErr {
    #[doc = "0: No interrupt"]
    Value1 = 0,
    #[doc = "1: Interrupt is generated"]
    Value2 = 1,
}
impl From<FeDataEndBitErr> for bool {
    #[inline(always)]
    fn from(variant: FeDataEndBitErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_DATA_END_BIT_ERR` writer - Force Event for Data End Bit Error"]
pub type FeDataEndBitErrW<'a, REG> = crate::BitWriter<'a, REG, FeDataEndBitErr>;
impl<'a, REG> FeDataEndBitErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FeDataEndBitErr::Value1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FeDataEndBitErr::Value2)
    }
}
#[doc = "Force Event for Current Limit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeCurrentLimitErr {
    #[doc = "0: No interrupt"]
    Value1 = 0,
    #[doc = "1: Interrupt is generated"]
    Value2 = 1,
}
impl From<FeCurrentLimitErr> for bool {
    #[inline(always)]
    fn from(variant: FeCurrentLimitErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_CURRENT_LIMIT_ERR` writer - Force Event for Current Limit Error"]
pub type FeCurrentLimitErrW<'a, REG> = crate::BitWriter<'a, REG, FeCurrentLimitErr>;
impl<'a, REG> FeCurrentLimitErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FeCurrentLimitErr::Value1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FeCurrentLimitErr::Value2)
    }
}
#[doc = "Force Event for Auto CMD Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeAcmd12Err {
    #[doc = "0: No interrupt"]
    Value1 = 0,
    #[doc = "1: Interrupt is generated"]
    Value2 = 1,
}
impl From<FeAcmd12Err> for bool {
    #[inline(always)]
    fn from(variant: FeAcmd12Err) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_ACMD12_ERR` writer - Force Event for Auto CMD Error"]
pub type FeAcmd12ErrW<'a, REG> = crate::BitWriter<'a, REG, FeAcmd12Err>;
impl<'a, REG> FeAcmd12ErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FeAcmd12Err::Value1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FeAcmd12Err::Value2)
    }
}
#[doc = "Force event for Target Response Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeTargetResponseErr {
    #[doc = "0: No interrupt"]
    Value1 = 0,
    #[doc = "1: Interrupt is generated"]
    Value2 = 1,
}
impl From<FeTargetResponseErr> for bool {
    #[inline(always)]
    fn from(variant: FeTargetResponseErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_TARGET_RESPONSE_ERR` writer - Force event for Target Response Error"]
pub type FeTargetResponseErrW<'a, REG> = crate::BitWriter<'a, REG, FeTargetResponseErr>;
impl<'a, REG> FeTargetResponseErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FeTargetResponseErr::Value1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FeTargetResponseErr::Value2)
    }
}
#[doc = "Force Event for Ceata Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeCeataErr {
    #[doc = "0: No interrupt"]
    Value1 = 0,
    #[doc = "1: Interrupt is generated"]
    Value2 = 1,
}
impl From<FeCeataErr> for bool {
    #[inline(always)]
    fn from(variant: FeCeataErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE_CEATA_ERR` writer - Force Event for Ceata Error"]
pub type FeCeataErrW<'a, REG> = crate::BitWriter<'a, REG, FeCeataErr>;
impl<'a, REG> FeCeataErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FeCeataErr::Value1)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FeCeataErr::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - Force Event for Command Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_cmd_timeout_err(&mut self) -> FeCmdTimeoutErrW<ForceEventErrStatusSpec> {
        FeCmdTimeoutErrW::new(self, 0)
    }
    #[doc = "Bit 1 - Force Event for Command CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_cmd_crc_err(&mut self) -> FeCmdCrcErrW<ForceEventErrStatusSpec> {
        FeCmdCrcErrW::new(self, 1)
    }
    #[doc = "Bit 2 - Force Event for Command End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_cmd_end_bit_err(&mut self) -> FeCmdEndBitErrW<ForceEventErrStatusSpec> {
        FeCmdEndBitErrW::new(self, 2)
    }
    #[doc = "Bit 3 - Force Event for Command Index Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_cmd_ind_err(&mut self) -> FeCmdIndErrW<ForceEventErrStatusSpec> {
        FeCmdIndErrW::new(self, 3)
    }
    #[doc = "Bit 4 - Force Event for Data Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_data_timeout_err(&mut self) -> FeDataTimeoutErrW<ForceEventErrStatusSpec> {
        FeDataTimeoutErrW::new(self, 4)
    }
    #[doc = "Bit 5 - Force Event for Data CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_data_crc_err(&mut self) -> FeDataCrcErrW<ForceEventErrStatusSpec> {
        FeDataCrcErrW::new(self, 5)
    }
    #[doc = "Bit 6 - Force Event for Data End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_data_end_bit_err(&mut self) -> FeDataEndBitErrW<ForceEventErrStatusSpec> {
        FeDataEndBitErrW::new(self, 6)
    }
    #[doc = "Bit 7 - Force Event for Current Limit Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_current_limit_err(&mut self) -> FeCurrentLimitErrW<ForceEventErrStatusSpec> {
        FeCurrentLimitErrW::new(self, 7)
    }
    #[doc = "Bit 8 - Force Event for Auto CMD Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_acmd12_err(&mut self) -> FeAcmd12ErrW<ForceEventErrStatusSpec> {
        FeAcmd12ErrW::new(self, 8)
    }
    #[doc = "Bit 12 - Force event for Target Response Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_target_response_err(&mut self) -> FeTargetResponseErrW<ForceEventErrStatusSpec> {
        FeTargetResponseErrW::new(self, 12)
    }
    #[doc = "Bit 13 - Force Event for Ceata Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe_ceata_err(&mut self) -> FeCeataErrW<ForceEventErrStatusSpec> {
        FeCeataErrW::new(self, 13)
    }
}
#[doc = "Force Event Register for Error Interrupt Status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`force_event_err_status::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ForceEventErrStatusSpec;
impl crate::RegisterSpec for ForceEventErrStatusSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`force_event_err_status::W`](W) writer structure"]
impl crate::Writable for ForceEventErrStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FORCE_EVENT_ERR_STATUS to value 0"]
impl crate::Resettable for ForceEventErrStatusSpec {
    const RESET_VALUE: u16 = 0;
}
