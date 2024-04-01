#[doc = "Register `DC_PULSE_LEN` reader"]
pub type R = crate::R<DcPulseLenSpec>;
#[doc = "Pulse length of SyncSignals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PulsLength {
    #[doc = "0: Acknowledge mode: SyncSignal will be cleared by reading SYNC\\[1:0\\]
Status register"]
    Value1 = 0,
}
impl From<PulsLength> for u16 {
    #[inline(always)]
    fn from(variant: PulsLength) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PulsLength {
    type Ux = u16;
}
impl crate::IsEnum for PulsLength {}
#[doc = "Field `PULS_LENGTH` reader - Pulse length of SyncSignals"]
pub type PulsLengthR = crate::FieldReader<PulsLength>;
impl PulsLengthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PulsLength> {
        match self.bits {
            0 => Some(PulsLength::Value1),
            _ => None,
        }
    }
    #[doc = "Acknowledge mode: SyncSignal will be cleared by reading SYNC\\[1:0\\]
Status register"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PulsLength::Value1
    }
}
impl R {
    #[doc = "Bits 0:15 - Pulse length of SyncSignals"]
    #[inline(always)]
    pub fn puls_length(&self) -> PulsLengthR {
        PulsLengthR::new(self.bits)
    }
}
#[doc = "Pulse Length of SyncSignals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_pulse_len::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcPulseLenSpec;
impl crate::RegisterSpec for DcPulseLenSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dc_pulse_len::R`](R) reader structure"]
impl crate::Readable for DcPulseLenSpec {}
#[doc = "`reset()` method sets DC_PULSE_LEN to value 0"]
impl crate::Resettable for DcPulseLenSpec {
    const RESET_VALUE: u16 = 0;
}
