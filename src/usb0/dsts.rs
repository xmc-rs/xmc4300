#[doc = "Reader of register DSTS"]
pub type R = crate::R<u32, super::DSTS>;
#[doc = "Reader of field `SuspSts`"]
pub type SUSPSTS_R = crate::R<bool, bool>;
#[doc = "Enumerated Speed\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENUMSPD_A {
    #[doc = "3: Full speed (PHY clock is running at 48 MHz)"]
    VALUE4,
}
impl From<ENUMSPD_A> for u8 {
    #[inline(always)]
    fn from(variant: ENUMSPD_A) -> Self {
        match variant {
            ENUMSPD_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `EnumSpd`"]
pub type ENUMSPD_R = crate::R<u8, ENUMSPD_A>;
impl ENUMSPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ENUMSPD_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(ENUMSPD_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ENUMSPD_A::VALUE4
    }
}
#[doc = "Reader of field `ErrticErr`"]
pub type ERRTICERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SOFFN`"]
pub type SOFFN_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bit 0 - Suspend Status"]
    #[inline(always)]
    pub fn susp_sts(&self) -> SUSPSTS_R {
        SUSPSTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Enumerated Speed"]
    #[inline(always)]
    pub fn enum_spd(&self) -> ENUMSPD_R {
        ENUMSPD_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Erratic Error"]
    #[inline(always)]
    pub fn errtic_err(&self) -> ERRTICERR_R {
        ERRTICERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:21 - Frame Number of the Received SOF"]
    #[inline(always)]
    pub fn soffn(&self) -> SOFFN_R {
        SOFFN_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
