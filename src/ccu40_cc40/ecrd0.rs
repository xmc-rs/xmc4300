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
pub struct CAPV_R(crate::FieldReader<u16, u16>);
impl CAPV_R {
    pub(crate) fn new(bits: u16) -> Self {
        CAPV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPCV` reader - Prescaler Capture value"]
pub struct FPCV_R(crate::FieldReader<u8, u8>);
impl FPCV_R {
    pub(crate) fn new(bits: u8) -> Self {
        FPCV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPCV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Slice pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPTR_A {
    #[doc = "0: CC40"]
    VALUE1 = 0,
    #[doc = "1: CC41"]
    VALUE2 = 1,
    #[doc = "2: CC42"]
    VALUE3 = 2,
    #[doc = "3: CC43"]
    VALUE4 = 3,
}
impl From<SPTR_A> for u8 {
    #[inline(always)]
    fn from(variant: SPTR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPTR` reader - Slice pointer"]
pub struct SPTR_R(crate::FieldReader<u8, SPTR_A>);
impl SPTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPTR_R(crate::FieldReader::new(bits))
    }
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
        **self == SPTR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SPTR_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == SPTR_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == SPTR_A::VALUE4
    }
}
impl core::ops::Deref for SPTR_R {
    type Target = crate::FieldReader<u8, SPTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Capture register pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `VPTR` reader - Capture register pointer"]
pub struct VPTR_R(crate::FieldReader<u8, VPTR_A>);
impl VPTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        VPTR_R(crate::FieldReader::new(bits))
    }
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
        **self == VPTR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == VPTR_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == VPTR_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == VPTR_A::VALUE4
    }
}
impl core::ops::Deref for VPTR_R {
    type Target = crate::FieldReader<u8, VPTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `FFL` reader - Full Flag"]
pub struct FFL_R(crate::FieldReader<bool, FFL_A>);
impl FFL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FFL_R(crate::FieldReader::new(bits))
    }
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
        **self == FFL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FFL_A::VALUE2
    }
}
impl core::ops::Deref for FFL_R {
    type Target = crate::FieldReader<bool, FFL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Lost Capture Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `LCV` reader - Lost Capture Value"]
pub struct LCV_R(crate::FieldReader<bool, LCV_A>);
impl LCV_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCV_R(crate::FieldReader::new(bits))
    }
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
        **self == LCV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LCV_A::VALUE2
    }
}
impl core::ops::Deref for LCV_R {
    type Target = crate::FieldReader<bool, LCV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Extended Read Back 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecrd0](index.html) module"]
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
