#[doc = "Register `DC_CYC_CONT` reader"]
pub struct R(crate::R<DC_CYC_CONT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_CYC_CONT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_CYC_CONT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_CYC_CONT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "SYNC out unit control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_A {
    #[doc = "0: ECAT controlled"]
    VALUE1 = 0,
    #[doc = "1: PDI controlled"]
    VALUE2 = 1,
}
impl From<SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC` reader - SYNC out unit control"]
pub struct SYNC_R(crate::FieldReader<bool, SYNC_A>);
impl SYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_A {
        match self.bits {
            false => SYNC_A::VALUE1,
            true => SYNC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SYNC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SYNC_A::VALUE2
    }
}
impl core::ops::Deref for SYNC_R {
    type Target = crate::FieldReader<bool, SYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Latch In unit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LATCH_U0_A {
    #[doc = "0: ECAT controlled"]
    VALUE1 = 0,
    #[doc = "1: PDI controlled"]
    VALUE2 = 1,
}
impl From<LATCH_U0_A> for bool {
    #[inline(always)]
    fn from(variant: LATCH_U0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LATCH_U0` reader - Latch In unit 0"]
pub struct LATCH_U0_R(crate::FieldReader<bool, LATCH_U0_A>);
impl LATCH_U0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATCH_U0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LATCH_U0_A {
        match self.bits {
            false => LATCH_U0_A::VALUE1,
            true => LATCH_U0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == LATCH_U0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LATCH_U0_A::VALUE2
    }
}
impl core::ops::Deref for LATCH_U0_R {
    type Target = crate::FieldReader<bool, LATCH_U0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Latch In unit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LATCH_U1_A {
    #[doc = "0: ECAT controlled"]
    VALUE1 = 0,
    #[doc = "1: PDI controlled"]
    VALUE2 = 1,
}
impl From<LATCH_U1_A> for bool {
    #[inline(always)]
    fn from(variant: LATCH_U1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LATCH_U1` reader - Latch In unit 1"]
pub struct LATCH_U1_R(crate::FieldReader<bool, LATCH_U1_A>);
impl LATCH_U1_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATCH_U1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LATCH_U1_A {
        match self.bits {
            false => LATCH_U1_A::VALUE1,
            true => LATCH_U1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == LATCH_U1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LATCH_U1_A::VALUE2
    }
}
impl core::ops::Deref for LATCH_U1_R {
    type Target = crate::FieldReader<bool, LATCH_U1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - SYNC out unit control"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Latch In unit 0"]
    #[inline(always)]
    pub fn latch_u0(&self) -> LATCH_U0_R {
        LATCH_U0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Latch In unit 1"]
    #[inline(always)]
    pub fn latch_u1(&self) -> LATCH_U1_R {
        LATCH_U1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "Cyclic Unit Control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_cyc_cont](index.html) module"]
pub struct DC_CYC_CONT_SPEC;
impl crate::RegisterSpec for DC_CYC_CONT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dc_cyc_cont::R](R) reader structure"]
impl crate::Readable for DC_CYC_CONT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DC_CYC_CONT to value 0"]
impl crate::Resettable for DC_CYC_CONT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
