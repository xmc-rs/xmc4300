#[doc = "Register `ACMD_ERR_STATUS` reader"]
pub type R = crate::R<AcmdErrStatusSpec>;
#[doc = "Auto CMD12 Not Executed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmd12NotExecErr {
    #[doc = "0: Executed"]
    Value1 = 0,
    #[doc = "1: Not Executed"]
    Value2 = 1,
}
impl From<Acmd12NotExecErr> for bool {
    #[inline(always)]
    fn from(variant: Acmd12NotExecErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD12_NOT_EXEC_ERR` reader - Auto CMD12 Not Executed"]
pub type Acmd12NotExecErrR = crate::BitReader<Acmd12NotExecErr>;
impl Acmd12NotExecErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acmd12NotExecErr {
        match self.bits {
            false => Acmd12NotExecErr::Value1,
            true => Acmd12NotExecErr::Value2,
        }
    }
    #[doc = "Executed"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Acmd12NotExecErr::Value1
    }
    #[doc = "Not Executed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Acmd12NotExecErr::Value2
    }
}
#[doc = "Auto CMD Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AcmdTimeoutErr {
    #[doc = "0: No Error"]
    Value1 = 0,
    #[doc = "1: Timeout"]
    Value2 = 1,
}
impl From<AcmdTimeoutErr> for bool {
    #[inline(always)]
    fn from(variant: AcmdTimeoutErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD_TIMEOUT_ERR` reader - Auto CMD Timeout Error"]
pub type AcmdTimeoutErrR = crate::BitReader<AcmdTimeoutErr>;
impl AcmdTimeoutErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AcmdTimeoutErr {
        match self.bits {
            false => AcmdTimeoutErr::Value1,
            true => AcmdTimeoutErr::Value2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AcmdTimeoutErr::Value1
    }
    #[doc = "Timeout"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AcmdTimeoutErr::Value2
    }
}
#[doc = "Auto CMD CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AcmdCrcErr {
    #[doc = "0: No Error"]
    Value1 = 0,
    #[doc = "1: CRC Error Generated"]
    Value2 = 1,
}
impl From<AcmdCrcErr> for bool {
    #[inline(always)]
    fn from(variant: AcmdCrcErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD_CRC_ERR` reader - Auto CMD CRC Error"]
pub type AcmdCrcErrR = crate::BitReader<AcmdCrcErr>;
impl AcmdCrcErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AcmdCrcErr {
        match self.bits {
            false => AcmdCrcErr::Value1,
            true => AcmdCrcErr::Value2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AcmdCrcErr::Value1
    }
    #[doc = "CRC Error Generated"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AcmdCrcErr::Value2
    }
}
#[doc = "Auto CMD End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AcmdEndBitErr {
    #[doc = "0: No Error"]
    Value1 = 0,
    #[doc = "1: End Bit Error Generated"]
    Value2 = 1,
}
impl From<AcmdEndBitErr> for bool {
    #[inline(always)]
    fn from(variant: AcmdEndBitErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD_END_BIT_ERR` reader - Auto CMD End Bit Error"]
pub type AcmdEndBitErrR = crate::BitReader<AcmdEndBitErr>;
impl AcmdEndBitErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AcmdEndBitErr {
        match self.bits {
            false => AcmdEndBitErr::Value1,
            true => AcmdEndBitErr::Value2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AcmdEndBitErr::Value1
    }
    #[doc = "End Bit Error Generated"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AcmdEndBitErr::Value2
    }
}
#[doc = "Auto CMD Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AcmdIndErr {
    #[doc = "0: No Error"]
    Value1 = 0,
    #[doc = "1: Error"]
    Value2 = 1,
}
impl From<AcmdIndErr> for bool {
    #[inline(always)]
    fn from(variant: AcmdIndErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD_IND_ERR` reader - Auto CMD Index Error"]
pub type AcmdIndErrR = crate::BitReader<AcmdIndErr>;
impl AcmdIndErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AcmdIndErr {
        match self.bits {
            false => AcmdIndErr::Value1,
            true => AcmdIndErr::Value2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AcmdIndErr::Value1
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AcmdIndErr::Value2
    }
}
#[doc = "Command Not Issued By Auto CMD12 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmdNotIssuedByAcmd12Err {
    #[doc = "0: No Error"]
    Value1 = 0,
    #[doc = "1: Not Issued"]
    Value2 = 1,
}
impl From<CmdNotIssuedByAcmd12Err> for bool {
    #[inline(always)]
    fn from(variant: CmdNotIssuedByAcmd12Err) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_NOT_ISSUED_BY_ACMD12_ERR` reader - Command Not Issued By Auto CMD12 Error"]
pub type CmdNotIssuedByAcmd12ErrR = crate::BitReader<CmdNotIssuedByAcmd12Err>;
impl CmdNotIssuedByAcmd12ErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CmdNotIssuedByAcmd12Err {
        match self.bits {
            false => CmdNotIssuedByAcmd12Err::Value1,
            true => CmdNotIssuedByAcmd12Err::Value2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CmdNotIssuedByAcmd12Err::Value1
    }
    #[doc = "Not Issued"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CmdNotIssuedByAcmd12Err::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Auto CMD12 Not Executed"]
    #[inline(always)]
    pub fn acmd12_not_exec_err(&self) -> Acmd12NotExecErrR {
        Acmd12NotExecErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto CMD Timeout Error"]
    #[inline(always)]
    pub fn acmd_timeout_err(&self) -> AcmdTimeoutErrR {
        AcmdTimeoutErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auto CMD CRC Error"]
    #[inline(always)]
    pub fn acmd_crc_err(&self) -> AcmdCrcErrR {
        AcmdCrcErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Auto CMD End Bit Error"]
    #[inline(always)]
    pub fn acmd_end_bit_err(&self) -> AcmdEndBitErrR {
        AcmdEndBitErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Auto CMD Index Error"]
    #[inline(always)]
    pub fn acmd_ind_err(&self) -> AcmdIndErrR {
        AcmdIndErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Command Not Issued By Auto CMD12 Error"]
    #[inline(always)]
    pub fn cmd_not_issued_by_acmd12_err(&self) -> CmdNotIssuedByAcmd12ErrR {
        CmdNotIssuedByAcmd12ErrR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Auto CMD Error Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acmd_err_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcmdErrStatusSpec;
impl crate::RegisterSpec for AcmdErrStatusSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`acmd_err_status::R`](R) reader structure"]
impl crate::Readable for AcmdErrStatusSpec {}
#[doc = "`reset()` method sets ACMD_ERR_STATUS to value 0"]
impl crate::Resettable for AcmdErrStatusSpec {
    const RESET_VALUE: u16 = 0;
}
