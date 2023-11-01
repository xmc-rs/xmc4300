#[doc = "Register `ESC_RESET_PDI` reader"]
pub type R = crate::R<READMODE_ESC_RESET_PDI_SPEC>;
#[doc = "Field `RESET_CMD_STATE` reader - Progress of the reset procedure"]
pub type RESET_CMD_STATE_R = crate::FieldReader<RESET_CMD_STATE_A>;
#[doc = "Progress of the reset procedure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for RESET_CMD_STATE_A {
    type Ux = u8;
}
impl RESET_CMD_STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RESET_CMD_STATE_A> {
        match self.bits {
            1 => Some(RESET_CMD_STATE_A::VALUE1),
            2 => Some(RESET_CMD_STATE_A::VALUE2),
            0 => Some(RESET_CMD_STATE_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "after writing 0x52"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESET_CMD_STATE_A::VALUE1
    }
    #[doc = "after writing 0x45 (if 0x52 was written before)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESET_CMD_STATE_A::VALUE2
    }
    #[doc = "default"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RESET_CMD_STATE_A::VALUE3
    }
}
impl R {
    #[doc = "Bits 0:1 - Progress of the reset procedure"]
    #[inline(always)]
    pub fn reset_cmd_state(&self) -> RESET_CMD_STATE_R {
        RESET_CMD_STATE_R::new(self.bits & 3)
    }
}
#[doc = "ESC Reset PDI \\[READ Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`readmode_esc_reset_pdi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct READMODE_ESC_RESET_PDI_SPEC;
impl crate::RegisterSpec for READMODE_ESC_RESET_PDI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`readmode_esc_reset_pdi::R`](R) reader structure"]
impl crate::Readable for READMODE_ESC_RESET_PDI_SPEC {}
#[doc = "`reset()` method sets ESC_RESET_PDI to value 0"]
impl crate::Resettable for READMODE_ESC_RESET_PDI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
