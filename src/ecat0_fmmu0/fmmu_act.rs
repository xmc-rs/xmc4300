#[doc = "Register `FMMU_ACT` reader"]
pub type R = crate::R<FMMU_ACT_SPEC>;
#[doc = "Field `ACT` reader - FMMU Activation"]
pub type ACT_R = crate::BitReader<ACT_A>;
#[doc = "FMMU Activation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACT_A {
        match self.bits {
            false => ACT_A::VALUE1,
            true => ACT_A::VALUE2,
        }
    }
    #[doc = "FMMU deactivated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ACT_A::VALUE1
    }
    #[doc = "FMMU activated. FMMU checks logical addressed blocks to be mapped according to mapping configured"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ACT_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - FMMU Activation"]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Activate FMMU 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_act::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMMU_ACT_SPEC;
impl crate::RegisterSpec for FMMU_ACT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fmmu_act::R`](R) reader structure"]
impl crate::Readable for FMMU_ACT_SPEC {}
#[doc = "`reset()` method sets FMMU_ACT to value 0"]
impl crate::Resettable for FMMU_ACT_SPEC {
    const RESET_VALUE: u8 = 0;
}
