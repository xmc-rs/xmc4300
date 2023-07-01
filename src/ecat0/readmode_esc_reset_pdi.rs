#[doc = "Register `ESC_RESET_PDI` reader"]
pub struct R(crate::R<READMODE_ESC_RESET_PDI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READMODE_ESC_RESET_PDI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READMODE_ESC_RESET_PDI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READMODE_ESC_RESET_PDI_SPEC>) -> Self {
        R(reader)
    }
}
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
    pub fn variant(&self) -> Option<RESET_CMD_STATE_A> {
        match self.bits {
            1 => Some(RESET_CMD_STATE_A::VALUE1),
            2 => Some(RESET_CMD_STATE_A::VALUE2),
            0 => Some(RESET_CMD_STATE_A::VALUE3),
            _ => None,
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
        RESET_CMD_STATE_R::new(self.bits & 3)
    }
}
#[doc = "ESC Reset PDI \\[READ Mode\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readmode_esc_reset_pdi](index.html) module"]
pub struct READMODE_ESC_RESET_PDI_SPEC;
impl crate::RegisterSpec for READMODE_ESC_RESET_PDI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [readmode_esc_reset_pdi::R](R) reader structure"]
impl crate::Readable for READMODE_ESC_RESET_PDI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ESC_RESET_PDI to value 0"]
impl crate::Resettable for READMODE_ESC_RESET_PDI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
