#[doc = "Reader of register CLKMXSTAT"]
pub type R = crate::R<u32, super::CLKMXSTAT>;
#[doc = "Status of System Clock Multiplexing Upon Source Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCLKMUX_A {
    #[doc = "1: fOFI clock active"]
    CONST_X1,
    #[doc = "2: fPLL clock active"]
    CONST_1X,
}
impl From<SYSCLKMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCLKMUX_A) -> Self {
        match variant {
            SYSCLKMUX_A::CONST_X1 => 1,
            SYSCLKMUX_A::CONST_1X => 2,
        }
    }
}
#[doc = "Reader of field `SYSCLKMUX`"]
pub type SYSCLKMUX_R = crate::R<u8, SYSCLKMUX_A>;
impl SYSCLKMUX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYSCLKMUX_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SYSCLKMUX_A::CONST_X1),
            2 => Val(SYSCLKMUX_A::CONST_1X),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_X1`"]
    #[inline(always)]
    pub fn is_const_x1(&self) -> bool {
        *self == SYSCLKMUX_A::CONST_X1
    }
    #[doc = "Checks if the value of the field is `CONST_1X`"]
    #[inline(always)]
    pub fn is_const_1x(&self) -> bool {
        *self == SYSCLKMUX_A::CONST_1X
    }
}
impl R {
    #[doc = "Bits 0:1 - Status of System Clock Multiplexing Upon Source Switching"]
    #[inline(always)]
    pub fn sysclkmux(&self) -> SYSCLKMUX_R {
        SYSCLKMUX_R::new((self.bits & 0x03) as u8)
    }
}
