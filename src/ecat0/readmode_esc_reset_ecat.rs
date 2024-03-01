#[doc = "Register `ESC_RESET_ECAT` reader"]
pub type R = crate::R<ReadmodeEscResetEcatSpec>;
#[doc = "Progress of the reset procedure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ResetCmdState {
    #[doc = "1: after writing 0x52"]
    Value1 = 1,
    #[doc = "2: after writing 0x45 (if 0x52 was written before)"]
    Value2 = 2,
    #[doc = "0: default"]
    Value3 = 0,
}
impl From<ResetCmdState> for u8 {
    #[inline(always)]
    fn from(variant: ResetCmdState) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ResetCmdState {
    type Ux = u8;
}
#[doc = "Field `RESET_CMD_STATE` reader - Progress of the reset procedure"]
pub type ResetCmdStateR = crate::FieldReader<ResetCmdState>;
impl ResetCmdStateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ResetCmdState> {
        match self.bits {
            1 => Some(ResetCmdState::Value1),
            2 => Some(ResetCmdState::Value2),
            0 => Some(ResetCmdState::Value3),
            _ => None,
        }
    }
    #[doc = "after writing 0x52"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ResetCmdState::Value1
    }
    #[doc = "after writing 0x45 (if 0x52 was written before)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ResetCmdState::Value2
    }
    #[doc = "default"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ResetCmdState::Value3
    }
}
impl R {
    #[doc = "Bits 0:1 - Progress of the reset procedure"]
    #[inline(always)]
    pub fn reset_cmd_state(&self) -> ResetCmdStateR {
        ResetCmdStateR::new(self.bits & 3)
    }
}
#[doc = "ESC Reset ECAT \\[READ Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`readmode_esc_reset_ecat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadmodeEscResetEcatSpec;
impl crate::RegisterSpec for ReadmodeEscResetEcatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`readmode_esc_reset_ecat::R`](R) reader structure"]
impl crate::Readable for ReadmodeEscResetEcatSpec {}
#[doc = "`reset()` method sets ESC_RESET_ECAT to value 0"]
impl crate::Resettable for ReadmodeEscResetEcatSpec {
    const RESET_VALUE: u8 = 0;
}
