#[doc = "Reader of register ECRD1"]
pub type R = crate::R<u32, super::ECRD1>;
#[doc = "Reader of field `CAPV`"]
pub type CAPV_R = crate::R<u16, u16>;
#[doc = "Reader of field `FPCV`"]
pub type FPCV_R = crate::R<u8, u8>;
#[doc = "Slice pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPTR_A {
    #[doc = "0: CC40"]
    VALUE1,
    #[doc = "1: CC41"]
    VALUE2,
    #[doc = "2: CC42"]
    VALUE3,
    #[doc = "3: CC43"]
    VALUE4,
}
impl From<SPTR_A> for u8 {
    #[inline(always)]
    fn from(variant: SPTR_A) -> Self {
        match variant {
            SPTR_A::VALUE1 => 0,
            SPTR_A::VALUE2 => 1,
            SPTR_A::VALUE3 => 2,
            SPTR_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `SPTR`"]
pub type SPTR_R = crate::R<u8, SPTR_A>;
impl SPTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPTR_A {
        match self.bits {
            0 => SPTR_A::VALUE1,
            1 => SPTR_A::VALUE2,
            2 => SPTR_A::VALUE3,
            3 => SPTR_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SPTR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SPTR_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SPTR_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SPTR_A::VALUE4
    }
}
#[doc = "Capture register pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VPTR_A {
    #[doc = "0: Capture register 0"]
    VALUE1,
    #[doc = "1: Capture register 1"]
    VALUE2,
    #[doc = "2: Capture register 2"]
    VALUE3,
    #[doc = "3: Capture register 3"]
    VALUE4,
}
impl From<VPTR_A> for u8 {
    #[inline(always)]
    fn from(variant: VPTR_A) -> Self {
        match variant {
            VPTR_A::VALUE1 => 0,
            VPTR_A::VALUE2 => 1,
            VPTR_A::VALUE3 => 2,
            VPTR_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `VPTR`"]
pub type VPTR_R = crate::R<u8, VPTR_A>;
impl VPTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VPTR_A {
        match self.bits {
            0 => VPTR_A::VALUE1,
            1 => VPTR_A::VALUE2,
            2 => VPTR_A::VALUE3,
            3 => VPTR_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VPTR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VPTR_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == VPTR_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == VPTR_A::VALUE4
    }
}
#[doc = "Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFL_A {
    #[doc = "0: No new value was captured into this register"]
    VALUE1,
    #[doc = "1: A new value has been captured into this register"]
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
#[doc = "Lost Capture Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCV_A {
    #[doc = "0: No capture was lost"]
    VALUE1,
    #[doc = "1: A capture was lost"]
    VALUE2,
}
impl From<LCV_A> for bool {
    #[inline(always)]
    fn from(variant: LCV_A) -> Self {
        match variant {
            LCV_A::VALUE1 => false,
            LCV_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `LCV`"]
pub type LCV_R = crate::R<bool, LCV_A>;
impl LCV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCV_A {
        match self.bits {
            false => LCV_A::VALUE1,
            true => LCV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LCV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LCV_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:15 - Timer Capture Value"]
    #[inline(always)]
    pub fn capv(&self) -> CAPV_R {
        CAPV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Prescaler Capture value"]
    #[inline(always)]
    pub fn fpcv(&self) -> FPCV_R {
        FPCV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Slice pointer"]
    #[inline(always)]
    pub fn sptr(&self) -> SPTR_R {
        SPTR_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Capture register pointer"]
    #[inline(always)]
    pub fn vptr(&self) -> VPTR_R {
        VPTR_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 24 - Full Flag"]
    #[inline(always)]
    pub fn ffl(&self) -> FFL_R {
        FFL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Lost Capture Value"]
    #[inline(always)]
    pub fn lcv(&self) -> LCV_R {
        LCV_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
