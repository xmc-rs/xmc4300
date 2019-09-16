#[doc = "Reader of register ESC_WR_PROTECT"]
pub type R = crate::R<u8, super::ESC_WR_PROTECT>;
#[doc = "Write protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESC_WR_PROT_A {
    #[doc = "0: Protection disabled"]
    VALUE1,
    #[doc = "1: Protection enabled"]
    VALUE2,
}
impl From<ESC_WR_PROT_A> for bool {
    #[inline(always)]
    fn from(variant: ESC_WR_PROT_A) -> Self {
        match variant {
            ESC_WR_PROT_A::VALUE1 => false,
            ESC_WR_PROT_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ESC_WR_PROT`"]
pub type ESC_WR_PROT_R = crate::R<bool, ESC_WR_PROT_A>;
impl ESC_WR_PROT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESC_WR_PROT_A {
        match self.bits {
            false => ESC_WR_PROT_A::VALUE1,
            true => ESC_WR_PROT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ESC_WR_PROT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ESC_WR_PROT_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Write protect"]
    #[inline(always)]
    pub fn esc_wr_prot(&self) -> ESC_WR_PROT_R {
        ESC_WR_PROT_R::new((self.bits & 0x01) != 0)
    }
}
