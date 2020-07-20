#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "PARITY ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARERR_A {
    #[doc = "0: No Error"]
    VALUE1 = 0,
    #[doc = "1: Parity Error in User or Process RAM"]
    VALUE2 = 1,
}
impl From<PARERR_A> for bool {
    #[inline(always)]
    fn from(variant: PARERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PARERR`"]
pub type PARERR_R = crate::R<bool, PARERR_A>;
impl PARERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARERR_A {
        match self.bits {
            false => PARERR_A::VALUE1,
            true => PARERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PARERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PARERR_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - PARITY ERROR"]
    #[inline(always)]
    pub fn parerr(&self) -> PARERR_R {
        PARERR_R::new((self.bits & 0x01) != 0)
    }
}
