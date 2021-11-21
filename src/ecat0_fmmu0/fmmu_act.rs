#[doc = "Register `FMMU_ACT` reader"]
pub struct R(crate::R<FMMU_ACT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMMU_ACT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMMU_ACT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMMU_ACT_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "Field `ACT` reader - FMMU Activation"]
pub struct ACT_R(crate::FieldReader<bool, ACT_A>);
impl ACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACT_R(crate::FieldReader::new(bits))
    }
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
        **self == ACT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ACT_A::VALUE2
    }
}
impl core::ops::Deref for ACT_R {
    type Target = crate::FieldReader<bool, ACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - FMMU Activation"]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Activate FMMU 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmmu_act](index.html) module"]
pub struct FMMU_ACT_SPEC;
impl crate::RegisterSpec for FMMU_ACT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fmmu_act::R](R) reader structure"]
impl crate::Readable for FMMU_ACT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMMU_ACT to value 0"]
impl crate::Resettable for FMMU_ACT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
