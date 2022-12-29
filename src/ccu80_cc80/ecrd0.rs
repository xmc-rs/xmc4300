#[doc = "Register `ECRD0` reader"]
pub struct R(crate::R<ECRD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECRD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECRD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECRD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAPV` reader - Timer Capture Value"]
pub type CAPV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FPCV` reader - Prescaler Capture value"]
pub type FPCV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPTR` reader - Slice pointer"]
pub type SPTR_R = crate::FieldReader<u8, SPTR_A>;
#[doc = "Slice pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPTR_A {
    #[doc = "0: CC80"]
    VALUE1 = 0,
    #[doc = "1: CC81"]
    VALUE2 = 1,
    #[doc = "2: CC82"]
    VALUE3 = 2,
    #[doc = "3: CC83"]
    VALUE4 = 3,
}
impl From<SPTR_A> for u8 {
    #[inline(always)]
    fn from(variant: SPTR_A) -> Self {
        variant as _
    }
}
impl SPTR_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `VPTR` reader - Capture register pointer"]
pub type VPTR_R = crate::FieldReader<u8, VPTR_A>;
#[doc = "Capture register pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VPTR_A {
    #[doc = "0: Capture register 0"]
    VALUE1 = 0,
    #[doc = "1: Capture register 1"]
    VALUE2 = 1,
    #[doc = "2: Capture register 2"]
    VALUE3 = 2,
    #[doc = "3: Capture register 3"]
    VALUE4 = 3,
}
impl From<VPTR_A> for u8 {
    #[inline(always)]
    fn from(variant: VPTR_A) -> Self {
        variant as _
    }
}
impl VPTR_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `FFL` reader - Full Flag"]
pub type FFL_R = crate::BitReader<FFL_A>;
#[doc = "Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FFL_A {
    #[doc = "0: No new value was captured into this register"]
    VALUE1 = 0,
    #[doc = "1: A new value has been captured into this register"]
    VALUE2 = 1,
}
impl From<FFL_A> for bool {
    #[inline(always)]
    fn from(variant: FFL_A) -> Self {
        variant as u8 != 0
    }
}
impl FFL_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `LCV` reader - Lost Capture Value"]
pub type LCV_R = crate::BitReader<LCV_A>;
#[doc = "Lost Capture Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCV_A {
    #[doc = "0: No capture was lost"]
    VALUE1 = 0,
    #[doc = "1: A capture was lost"]
    VALUE2 = 1,
}
impl From<LCV_A> for bool {
    #[inline(always)]
    fn from(variant: LCV_A) -> Self {
        variant as u8 != 0
    }
}
impl LCV_R {
    #[doc = "Get enumerated values variant"]
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
        SPTR_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Capture register pointer"]
    #[inline(always)]
    pub fn vptr(&self) -> VPTR_R {
        VPTR_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Full Flag"]
    #[inline(always)]
    pub fn ffl(&self) -> FFL_R {
        FFL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Lost Capture Value"]
    #[inline(always)]
    pub fn lcv(&self) -> LCV_R {
        LCV_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Extended Read Back 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecrd0](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct ECRD0_SPEC;
impl crate::RegisterSpec for ECRD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecrd0::R](R) reader structure"]
impl crate::Readable for ECRD0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ECRD0 to value 0"]
impl crate::Resettable for ECRD0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
