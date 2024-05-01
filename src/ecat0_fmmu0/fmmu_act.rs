#[doc = "Register `FMMU_ACT` reader"]
pub type R = crate::R<FmmuActSpec>;
#[doc = "FMMU Activation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Act {
    #[doc = "0: FMMU deactivated."]
    Value1 = 0,
    #[doc = "1: FMMU activated. FMMU checks logical addressed blocks to be mapped according to mapping configured"]
    Value2 = 1,
}
impl From<Act> for bool {
    #[inline(always)]
    fn from(variant: Act) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACT` reader - FMMU Activation"]
pub type ActR = crate::BitReader<Act>;
impl ActR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Act {
        match self.bits {
            false => Act::Value1,
            true => Act::Value2,
        }
    }
    #[doc = "FMMU deactivated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Act::Value1
    }
    #[doc = "FMMU activated. FMMU checks logical addressed blocks to be mapped according to mapping configured"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Act::Value2
    }
}
impl R {
    #[doc = "Bit 0 - FMMU Activation"]
    #[inline(always)]
    pub fn act(&self) -> ActR {
        ActR::new((self.bits & 1) != 0)
    }
}
#[doc = "Activate FMMU 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_act::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmmuActSpec;
impl crate::RegisterSpec for FmmuActSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fmmu_act::R`](R) reader structure"]
impl crate::Readable for FmmuActSpec {}
#[doc = "`reset()` method sets FMMU_ACT to value 0"]
impl crate::Resettable for FmmuActSpec {
    const RESET_VALUE: u8 = 0;
}
