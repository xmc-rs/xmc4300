#[doc = "Reader of register WR_REG_PROTECT"]
pub type R = crate::R<u8, super::WR_REG_PROTECT>;
#[doc = "Write register protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WR_REG_P_A {
    #[doc = "0: Protection disabled"]
    VALUE1,
    #[doc = "1: Protection enabled"]
    VALUE2,
}
impl From<WR_REG_P_A> for bool {
    #[inline(always)]
    fn from(variant: WR_REG_P_A) -> Self {
        match variant {
            WR_REG_P_A::VALUE1 => false,
            WR_REG_P_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `WR_REG_P`"]
pub type WR_REG_P_R = crate::R<bool, WR_REG_P_A>;
impl WR_REG_P_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WR_REG_P_A {
        match self.bits {
            false => WR_REG_P_A::VALUE1,
            true => WR_REG_P_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WR_REG_P_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WR_REG_P_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Write register protection"]
    #[inline(always)]
    pub fn wr_reg_p(&self) -> WR_REG_P_R {
        WR_REG_P_R::new((self.bits & 0x01) != 0)
    }
}
