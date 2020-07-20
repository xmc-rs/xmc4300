#[doc = "Reader of register RSTSTAT"]
pub type R = crate::R<u32, super::RSTSTAT>;
#[doc = "Reset Status Information\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSTSTAT_A {
    #[doc = "1: PORST reset"]
    CONST_00000001 = 1,
    #[doc = "2: SWD reset"]
    CONST_00000010 = 2,
    #[doc = "4: PV reset"]
    CONST_00000100 = 4,
    #[doc = "8: CPU system reset"]
    CONST_00001000 = 8,
    #[doc = "16: CPU lockup reset"]
    CONST_00010000 = 16,
    #[doc = "32: WDT reset"]
    CONST_00100000 = 32,
    #[doc = "128: Parity Error reset"]
    CONST_10000000 = 128,
}
impl From<RSTSTAT_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTSTAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RSTSTAT`"]
pub type RSTSTAT_R = crate::R<u8, RSTSTAT_A>;
impl RSTSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RSTSTAT_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(RSTSTAT_A::CONST_00000001),
            2 => Val(RSTSTAT_A::CONST_00000010),
            4 => Val(RSTSTAT_A::CONST_00000100),
            8 => Val(RSTSTAT_A::CONST_00001000),
            16 => Val(RSTSTAT_A::CONST_00010000),
            32 => Val(RSTSTAT_A::CONST_00100000),
            128 => Val(RSTSTAT_A::CONST_10000000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00000001`"]
    #[inline(always)]
    pub fn is_const_00000001(&self) -> bool {
        *self == RSTSTAT_A::CONST_00000001
    }
    #[doc = "Checks if the value of the field is `CONST_00000010`"]
    #[inline(always)]
    pub fn is_const_00000010(&self) -> bool {
        *self == RSTSTAT_A::CONST_00000010
    }
    #[doc = "Checks if the value of the field is `CONST_00000100`"]
    #[inline(always)]
    pub fn is_const_00000100(&self) -> bool {
        *self == RSTSTAT_A::CONST_00000100
    }
    #[doc = "Checks if the value of the field is `CONST_00001000`"]
    #[inline(always)]
    pub fn is_const_00001000(&self) -> bool {
        *self == RSTSTAT_A::CONST_00001000
    }
    #[doc = "Checks if the value of the field is `CONST_00010000`"]
    #[inline(always)]
    pub fn is_const_00010000(&self) -> bool {
        *self == RSTSTAT_A::CONST_00010000
    }
    #[doc = "Checks if the value of the field is `CONST_00100000`"]
    #[inline(always)]
    pub fn is_const_00100000(&self) -> bool {
        *self == RSTSTAT_A::CONST_00100000
    }
    #[doc = "Checks if the value of the field is `CONST_10000000`"]
    #[inline(always)]
    pub fn is_const_10000000(&self) -> bool {
        *self == RSTSTAT_A::CONST_10000000
    }
}
#[doc = "Hibernate Wake-up Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBWK_A {
    #[doc = "0: No Wake-up"]
    CONST_0 = 0,
    #[doc = "1: Wake-up event"]
    CONST_1 = 1,
}
impl From<HIBWK_A> for bool {
    #[inline(always)]
    fn from(variant: HIBWK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HIBWK`"]
pub type HIBWK_R = crate::R<bool, HIBWK_A>;
impl HIBWK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIBWK_A {
        match self.bits {
            false => HIBWK_A::CONST_0,
            true => HIBWK_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == HIBWK_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == HIBWK_A::CONST_1
    }
}
#[doc = "Hibernate Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBRS_A {
    #[doc = "0: Reset de-asserted"]
    CONST_0 = 0,
    #[doc = "1: Reset asserted"]
    CONST_1 = 1,
}
impl From<HIBRS_A> for bool {
    #[inline(always)]
    fn from(variant: HIBRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HIBRS`"]
pub type HIBRS_R = crate::R<bool, HIBRS_A>;
impl HIBRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIBRS_A {
        match self.bits {
            false => HIBRS_A::CONST_0,
            true => HIBRS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == HIBRS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == HIBRS_A::CONST_1
    }
}
#[doc = "Enable Lockup Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCKEN_A {
    #[doc = "0: Reset by Lockup disabled"]
    CONST_0 = 0,
    #[doc = "1: Reset by Lockup enabled"]
    CONST_1 = 1,
}
impl From<LCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: LCKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LCKEN`"]
pub type LCKEN_R = crate::R<bool, LCKEN_A>;
impl LCKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCKEN_A {
        match self.bits {
            false => LCKEN_A::CONST_0,
            true => LCKEN_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == LCKEN_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == LCKEN_A::CONST_1
    }
}
#[doc = "ECAT0 Reset Status Information\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECAT0RS_A {
    #[doc = "0: Reset did not occur"]
    CONST_0 = 0,
    #[doc = "1: Reset occurred"]
    CONST_1 = 1,
}
impl From<ECAT0RS_A> for bool {
    #[inline(always)]
    fn from(variant: ECAT0RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ECAT0RS`"]
pub type ECAT0RS_R = crate::R<bool, ECAT0RS_A>;
impl ECAT0RS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECAT0RS_A {
        match self.bits {
            false => ECAT0RS_A::CONST_0,
            true => ECAT0RS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ECAT0RS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ECAT0RS_A::CONST_1
    }
}
impl R {
    #[doc = "Bits 0:7 - Reset Status Information"]
    #[inline(always)]
    pub fn rststat(&self) -> RSTSTAT_R {
        RSTSTAT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Hibernate Wake-up Status"]
    #[inline(always)]
    pub fn hibwk(&self) -> HIBWK_R {
        HIBWK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Hibernate Reset Status"]
    #[inline(always)]
    pub fn hibrs(&self) -> HIBRS_R {
        HIBRS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable Lockup Status"]
    #[inline(always)]
    pub fn lcken(&self) -> LCKEN_R {
        LCKEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ECAT0 Reset Status Information"]
    #[inline(always)]
    pub fn ecat0rs(&self) -> ECAT0RS_R {
        ECAT0RS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
