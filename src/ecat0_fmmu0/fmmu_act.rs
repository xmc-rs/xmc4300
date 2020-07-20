#[doc = "Reader of register FMMU_ACT"]
pub type R = crate::R<u8, super::FMMU_ACT>;
#[doc = "FMMU Activation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACT_A {
    #[doc = "0: FMMU deactivated."]
    VALUE1 = 0,
    #[doc = "1: FMMU activated. FMMU checks logical addressed blocks to be mapped according to mapping configured"]
    VALUE2 = 1,
}
impl From<ACT_A> for bool {
    #[inline(always)]
    fn from(variant: ACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACT`"]
pub type ACT_R = crate::R<bool, ACT_A>;
impl ACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACT_A {
        match self.bits {
            false => ACT_A::VALUE1,
            true => ACT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ACT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ACT_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - FMMU Activation"]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new((self.bits & 0x01) != 0)
    }
}
