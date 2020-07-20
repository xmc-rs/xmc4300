#[doc = "Reader of register ESC_RESET_PDI"]
pub type R = crate::R<u8, super::ESC_RESET_PDI>;
#[doc = "Progress of the reset procedure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RESET_CMD_STATE_A {
    #[doc = "1: after writing 0x52"]
    VALUE1 = 1,
    #[doc = "2: after writing 0x45 (if 0x52 was written before)"]
    VALUE2 = 2,
    #[doc = "0: default"]
    VALUE3 = 0,
}
impl From<RESET_CMD_STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: RESET_CMD_STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RESET_CMD_STATE`"]
pub type RESET_CMD_STATE_R = crate::R<u8, RESET_CMD_STATE_A>;
impl RESET_CMD_STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RESET_CMD_STATE_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(RESET_CMD_STATE_A::VALUE1),
            2 => Val(RESET_CMD_STATE_A::VALUE2),
            0 => Val(RESET_CMD_STATE_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESET_CMD_STATE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESET_CMD_STATE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RESET_CMD_STATE_A::VALUE3
    }
}
impl R {
    #[doc = "Bits 0:1 - Progress of the reset procedure"]
    #[inline(always)]
    pub fn reset_cmd_state(&self) -> RESET_CMD_STATE_R {
        RESET_CMD_STATE_R::new((self.bits & 0x03) as u8)
    }
}
