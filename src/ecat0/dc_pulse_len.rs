#[doc = "Reader of register DC_PULSE_LEN"]
pub type R = crate::R<u16, super::DC_PULSE_LEN>;
#[doc = "Pulse length of SyncSignals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `PULS_LENGTH`"]
pub type PULS_LENGTH_R = crate::R<u16, PULS_LENGTH_A>;
impl PULS_LENGTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, PULS_LENGTH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PULS_LENGTH_A::VALUE1),
            i => Res(i),
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
        PULS_LENGTH_R::new((self.bits & 0xffff) as u16)
    }
}
