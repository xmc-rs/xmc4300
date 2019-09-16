#[doc = "Reader of register C2V"]
pub type R = crate::R<u32, super::C2V>;
#[doc = "Reader of field `CAPTV`"]
pub type CAPTV_R = crate::R<u16, u16>;
#[doc = "Reader of field `FPCV`"]
pub type FPCV_R = crate::R<u8, u8>;
#[doc = "Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFL_A {
    #[doc = "0: No new value was captured into the specific capture register"]
    VALUE1,
    #[doc = "1: A new value was captured into the specific register"]
    VALUE2,
}
impl From<FFL_A> for bool {
    #[inline(always)]
    fn from(variant: FFL_A) -> Self {
        match variant {
            FFL_A::VALUE1 => false,
            FFL_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `FFL`"]
pub type FFL_R = crate::R<bool, FFL_A>;
impl FFL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFL_A {
        match self.bits {
            false => FFL_A::VALUE1,
            true => FFL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FFL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FFL_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:15 - Capture Value"]
    #[inline(always)]
    pub fn captv(&self) -> CAPTV_R {
        CAPTV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Prescaler Value"]
    #[inline(always)]
    pub fn fpcv(&self) -> FPCV_R {
        FPCV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Full Flag"]
    #[inline(always)]
    pub fn ffl(&self) -> FFL_R {
        FFL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
