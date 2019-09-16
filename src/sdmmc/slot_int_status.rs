#[doc = "Reader of register SLOT_INT_STATUS"]
pub type R = crate::R<u16, super::SLOT_INT_STATUS>;
#[doc = "Interrupt Signal for Card Slot\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOT_INT_STATUS_A {
    #[doc = "0: Slot 1"]
    VALUE1,
}
impl From<SLOT_INT_STATUS_A> for u8 {
    #[inline(always)]
    fn from(variant: SLOT_INT_STATUS_A) -> Self {
        match variant {
            SLOT_INT_STATUS_A::VALUE1 => 0,
        }
    }
}
#[doc = "Reader of field `SLOT_INT_STATUS`"]
pub type SLOT_INT_STATUS_R = crate::R<u8, SLOT_INT_STATUS_A>;
impl SLOT_INT_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SLOT_INT_STATUS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SLOT_INT_STATUS_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SLOT_INT_STATUS_A::VALUE1
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt Signal for Card Slot"]
    #[inline(always)]
    pub fn slot_int_status(&self) -> SLOT_INT_STATUS_R {
        SLOT_INT_STATUS_R::new((self.bits & 0xff) as u8)
    }
}
