#[doc = "Register `COMMAND` reader"]
pub type R = crate::R<CommandSpec>;
#[doc = "Register `COMMAND` writer"]
pub type W = crate::W<CommandSpec>;
#[doc = "Response Type Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RespTypeSelect {
    #[doc = "0: No Response"]
    Value1 = 0,
    #[doc = "1: Response length 136"]
    Value2 = 1,
    #[doc = "2: Response length 48"]
    Value3 = 2,
    #[doc = "3: Response length 48 check Busy after response"]
    Value4 = 3,
}
impl From<RespTypeSelect> for u8 {
    #[inline(always)]
    fn from(variant: RespTypeSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RespTypeSelect {
    type Ux = u8;
}
#[doc = "Field `RESP_TYPE_SELECT` reader - Response Type Select"]
pub type RespTypeSelectR = crate::FieldReader<RespTypeSelect>;
impl RespTypeSelectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RespTypeSelect {
        match self.bits {
            0 => RespTypeSelect::Value1,
            1 => RespTypeSelect::Value2,
            2 => RespTypeSelect::Value3,
            3 => RespTypeSelect::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "No Response"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RespTypeSelect::Value1
    }
    #[doc = "Response length 136"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RespTypeSelect::Value2
    }
    #[doc = "Response length 48"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RespTypeSelect::Value3
    }
    #[doc = "Response length 48 check Busy after response"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RespTypeSelect::Value4
    }
}
#[doc = "Field `RESP_TYPE_SELECT` writer - Response Type Select"]
pub type RespTypeSelectW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RespTypeSelect>;
impl<'a, REG> RespTypeSelectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Response"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RespTypeSelect::Value1)
    }
    #[doc = "Response length 136"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RespTypeSelect::Value2)
    }
    #[doc = "Response length 48"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(RespTypeSelect::Value3)
    }
    #[doc = "Response length 48 check Busy after response"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(RespTypeSelect::Value4)
    }
}
#[doc = "Command CRC Check Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmdCrcCheckEn {
    #[doc = "0: Disable"]
    Value1 = 0,
    #[doc = "1: Enable"]
    Value2 = 1,
}
impl From<CmdCrcCheckEn> for bool {
    #[inline(always)]
    fn from(variant: CmdCrcCheckEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_CRC_CHECK_EN` reader - Command CRC Check Enable"]
pub type CmdCrcCheckEnR = crate::BitReader<CmdCrcCheckEn>;
impl CmdCrcCheckEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CmdCrcCheckEn {
        match self.bits {
            false => CmdCrcCheckEn::Value1,
            true => CmdCrcCheckEn::Value2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CmdCrcCheckEn::Value1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CmdCrcCheckEn::Value2
    }
}
#[doc = "Field `CMD_CRC_CHECK_EN` writer - Command CRC Check Enable"]
pub type CmdCrcCheckEnW<'a, REG> = crate::BitWriter<'a, REG, CmdCrcCheckEn>;
impl<'a, REG> CmdCrcCheckEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CmdCrcCheckEn::Value1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CmdCrcCheckEn::Value2)
    }
}
#[doc = "Command Index Check Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmdIndCheckEn {
    #[doc = "0: Disable"]
    Value1 = 0,
    #[doc = "1: Enable"]
    Value2 = 1,
}
impl From<CmdIndCheckEn> for bool {
    #[inline(always)]
    fn from(variant: CmdIndCheckEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_IND_CHECK_EN` reader - Command Index Check Enable"]
pub type CmdIndCheckEnR = crate::BitReader<CmdIndCheckEn>;
impl CmdIndCheckEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CmdIndCheckEn {
        match self.bits {
            false => CmdIndCheckEn::Value1,
            true => CmdIndCheckEn::Value2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CmdIndCheckEn::Value1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CmdIndCheckEn::Value2
    }
}
#[doc = "Field `CMD_IND_CHECK_EN` writer - Command Index Check Enable"]
pub type CmdIndCheckEnW<'a, REG> = crate::BitWriter<'a, REG, CmdIndCheckEn>;
impl<'a, REG> CmdIndCheckEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CmdIndCheckEn::Value1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CmdIndCheckEn::Value2)
    }
}
#[doc = "Data Present Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataPresentSelect {
    #[doc = "0: No Data Present"]
    Value1 = 0,
    #[doc = "1: Data Present"]
    Value2 = 1,
}
impl From<DataPresentSelect> for bool {
    #[inline(always)]
    fn from(variant: DataPresentSelect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_PRESENT_SELECT` reader - Data Present Select"]
pub type DataPresentSelectR = crate::BitReader<DataPresentSelect>;
impl DataPresentSelectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataPresentSelect {
        match self.bits {
            false => DataPresentSelect::Value1,
            true => DataPresentSelect::Value2,
        }
    }
    #[doc = "No Data Present"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DataPresentSelect::Value1
    }
    #[doc = "Data Present"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DataPresentSelect::Value2
    }
}
#[doc = "Field `DATA_PRESENT_SELECT` writer - Data Present Select"]
pub type DataPresentSelectW<'a, REG> = crate::BitWriter<'a, REG, DataPresentSelect>;
impl<'a, REG> DataPresentSelectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Data Present"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DataPresentSelect::Value1)
    }
    #[doc = "Data Present"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DataPresentSelect::Value2)
    }
}
#[doc = "Command Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CmdType {
    #[doc = "0: Normal"]
    Value1 = 0,
    #[doc = "1: Suspend"]
    Value2 = 1,
    #[doc = "2: Resume"]
    Value3 = 2,
    #[doc = "3: Abort"]
    Value4 = 3,
}
impl From<CmdType> for u8 {
    #[inline(always)]
    fn from(variant: CmdType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CmdType {
    type Ux = u8;
}
#[doc = "Field `CMD_TYPE` reader - Command Type"]
pub type CmdTypeR = crate::FieldReader<CmdType>;
impl CmdTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CmdType {
        match self.bits {
            0 => CmdType::Value1,
            1 => CmdType::Value2,
            2 => CmdType::Value3,
            3 => CmdType::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CmdType::Value1
    }
    #[doc = "Suspend"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CmdType::Value2
    }
    #[doc = "Resume"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CmdType::Value3
    }
    #[doc = "Abort"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CmdType::Value4
    }
}
#[doc = "Field `CMD_TYPE` writer - Command Type"]
pub type CmdTypeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CmdType>;
impl<'a, REG> CmdTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CmdType::Value1)
    }
    #[doc = "Suspend"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CmdType::Value2)
    }
    #[doc = "Resume"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CmdType::Value3)
    }
    #[doc = "Abort"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CmdType::Value4)
    }
}
#[doc = "Field `CMD_IND` reader - Command Index"]
pub type CmdIndR = crate::FieldReader;
#[doc = "Field `CMD_IND` writer - Command Index"]
pub type CmdIndW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - Response Type Select"]
    #[inline(always)]
    pub fn resp_type_select(&self) -> RespTypeSelectR {
        RespTypeSelectR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Command CRC Check Enable"]
    #[inline(always)]
    pub fn cmd_crc_check_en(&self) -> CmdCrcCheckEnR {
        CmdCrcCheckEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Command Index Check Enable"]
    #[inline(always)]
    pub fn cmd_ind_check_en(&self) -> CmdIndCheckEnR {
        CmdIndCheckEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Present Select"]
    #[inline(always)]
    pub fn data_present_select(&self) -> DataPresentSelectR {
        DataPresentSelectR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Command Type"]
    #[inline(always)]
    pub fn cmd_type(&self) -> CmdTypeR {
        CmdTypeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:13 - Command Index"]
    #[inline(always)]
    pub fn cmd_ind(&self) -> CmdIndR {
        CmdIndR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Response Type Select"]
    #[inline(always)]
    #[must_use]
    pub fn resp_type_select(&mut self) -> RespTypeSelectW<CommandSpec> {
        RespTypeSelectW::new(self, 0)
    }
    #[doc = "Bit 3 - Command CRC Check Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crc_check_en(&mut self) -> CmdCrcCheckEnW<CommandSpec> {
        CmdCrcCheckEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Command Index Check Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_ind_check_en(&mut self) -> CmdIndCheckEnW<CommandSpec> {
        CmdIndCheckEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Data Present Select"]
    #[inline(always)]
    #[must_use]
    pub fn data_present_select(&mut self) -> DataPresentSelectW<CommandSpec> {
        DataPresentSelectW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Command Type"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_type(&mut self) -> CmdTypeW<CommandSpec> {
        CmdTypeW::new(self, 6)
    }
    #[doc = "Bits 8:13 - Command Index"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_ind(&mut self) -> CmdIndW<CommandSpec> {
        CmdIndW::new(self, 8)
    }
}
#[doc = "Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`command::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`command::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CommandSpec;
impl crate::RegisterSpec for CommandSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`command::R`](R) reader structure"]
impl crate::Readable for CommandSpec {}
#[doc = "`write(|w| ..)` method takes [`command::W`](W) writer structure"]
impl crate::Writable for CommandSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets COMMAND to value 0"]
impl crate::Resettable for CommandSpec {
    const RESET_VALUE: u16 = 0;
}
