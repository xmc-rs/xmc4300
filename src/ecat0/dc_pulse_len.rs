#[doc = "Register `DC_PULSE_LEN` reader"]
pub struct R(crate::R<DC_PULSE_LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_PULSE_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_PULSE_LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_PULSE_LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PULS_LENGTH` reader - Pulse length of SyncSignals"]
pub type PULS_LENGTH_R = crate::FieldReader<PULS_LENGTH_A>;
#[doc = "Pulse length of SyncSignals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PULS_LENGTH_A {
    #[doc = "0: Acknowledge mode: SyncSignal will be cleared by reading SYNC\\[1:0\\]
Status register"]
    VALUE1 = 0,
}
impl From<PULS_LENGTH_A> for u16 {
    #[inline(always)]
    fn from(variant: PULS_LENGTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PULS_LENGTH_A {
    type Ux = u16;
}
impl PULS_LENGTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PULS_LENGTH_A> {
        match self.bits {
            0 => Some(PULS_LENGTH_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PULS_LENGTH_A::VALUE1
    }
}
impl R {
    #[doc = "Bits 0:15 - Pulse length of SyncSignals"]
    #[inline(always)]
    pub fn puls_length(&self) -> PULS_LENGTH_R {
        PULS_LENGTH_R::new(self.bits)
    }
}
#[doc = "Pulse Length of SyncSignals\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_pulse_len](index.html) module"]
pub struct DC_PULSE_LEN_SPEC;
impl crate::RegisterSpec for DC_PULSE_LEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dc_pulse_len::R](R) reader structure"]
impl crate::Readable for DC_PULSE_LEN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DC_PULSE_LEN to value 0"]
impl crate::Resettable for DC_PULSE_LEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
