#[doc = "Reader of register DC_SYS_TIME_DIFF"]
pub type R = crate::R<u32, super::DC_SYS_TIME_DIFF>;
#[doc = "Reader of field `TIME_DIF`"]
pub type TIME_DIF_R = crate::R<u32, u32>;
#[doc = "Local copy of System Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPY_A {
    #[doc = "0: Greater than or equal received System Time"]
    VALUE1,
    #[doc = "1: Smaller than received System Time"]
    VALUE2,
}
impl From<CPY_A> for bool {
    #[inline(always)]
    fn from(variant: CPY_A) -> Self {
        match variant {
            CPY_A::VALUE1 => false,
            CPY_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CPY`"]
pub type CPY_R = crate::R<bool, CPY_A>;
impl CPY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPY_A {
        match self.bits {
            false => CPY_A::VALUE1,
            true => CPY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CPY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CPY_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:30 - Mean difference between local copy of System Time and received System Time values"]
    #[inline(always)]
    pub fn time_dif(&self) -> TIME_DIF_R {
        TIME_DIF_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Local copy of System Time"]
    #[inline(always)]
    pub fn cpy(&self) -> CPY_R {
        CPY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
