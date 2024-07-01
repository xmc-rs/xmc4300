#[doc = "Register `ECRD0` reader"]
pub type R = crate::R<ECRD0_SPEC>;
#[doc = "Field `CAPV` reader - Timer Capture Value"]
pub type CAPV_R = crate::FieldReader<u16>;
#[doc = "Field `FPCV` reader - Prescaler Capture value"]
pub type FPCV_R = crate::FieldReader;
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
impl crate::FieldSpec for SPTR_A {
    type Ux = u8;
}
impl crate::IsEnum for SPTR_A {}
#[doc = "Field `SPTR` reader - Slice pointer"]
pub type SPTR_R = crate::FieldReader<SPTR_A>;
impl SPTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPTR_A {
        match self.bits {
            0 => SPTR_A::VALUE1,
            1 => SPTR_A::VALUE2,
            2 => SPTR_A::VALUE3,
            3 => SPTR_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "CC80"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SPTR_A::VALUE1
    }
    #[doc = "CC81"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SPTR_A::VALUE2
    }
    #[doc = "CC82"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SPTR_A::VALUE3
    }
    #[doc = "CC83"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SPTR_A::VALUE4
    }
}
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
impl crate::FieldSpec for VPTR_A {
    type Ux = u8;
}
impl crate::IsEnum for VPTR_A {}
#[doc = "Field `VPTR` reader - Capture register pointer"]
pub type VPTR_R = crate::FieldReader<VPTR_A>;
impl VPTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VPTR_A {
        match self.bits {
            0 => VPTR_A::VALUE1,
            1 => VPTR_A::VALUE2,
            2 => VPTR_A::VALUE3,
            3 => VPTR_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Capture register 0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VPTR_A::VALUE1
    }
    #[doc = "Capture register 1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VPTR_A::VALUE2
    }
    #[doc = "Capture register 2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == VPTR_A::VALUE3
    }
    #[doc = "Capture register 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == VPTR_A::VALUE4
    }
}
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
#[doc = "Field `FFL` reader - Full Flag"]
pub type FFL_R = crate::BitReader<FFL_A>;
impl FFL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FFL_A {
        match self.bits {
            false => FFL_A::VALUE1,
            true => FFL_A::VALUE2,
        }
    }
    #[doc = "No new value was captured into this register"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FFL_A::VALUE1
    }
    #[doc = "A new value has been captured into this register"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FFL_A::VALUE2
    }
}
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
#[doc = "Field `LCV` reader - Lost Capture Value"]
pub type LCV_R = crate::BitReader<LCV_A>;
impl LCV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCV_A {
        match self.bits {
            false => LCV_A::VALUE1,
            true => LCV_A::VALUE2,
        }
    }
    #[doc = "No capture was lost"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LCV_A::VALUE1
    }
    #[doc = "A capture was lost"]
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
#[doc = "Extended Read Back 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ecrd0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct ECRD0_SPEC;
impl crate::RegisterSpec for ECRD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecrd0::R`](R) reader structure"]
impl crate::Readable for ECRD0_SPEC {}
#[doc = "`reset()` method sets ECRD0 to value 0"]
impl crate::Resettable for ECRD0_SPEC {
    const RESET_VALUE: u32 = 0;
}
