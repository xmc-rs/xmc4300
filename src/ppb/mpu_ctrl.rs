#[doc = "Register `MPU_CTRL` reader"]
pub struct R(crate::R<MPU_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPU_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPU_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPU_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPU_CTRL` writer"]
pub struct W(crate::W<MPU_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPU_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MPU_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPU_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable MPU"]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "Enable MPU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "0: MPU disabled"]
    VALUE1 = 0,
    #[doc = "1: MPU enabled."]
    VALUE2 = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::VALUE1,
            true => ENABLE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENABLE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENABLE_A::VALUE2
    }
}
#[doc = "Field `ENABLE` writer - Enable MPU"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_CTRL_SPEC, ENABLE_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "MPU disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENABLE_A::VALUE1)
    }
    #[doc = "MPU enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENABLE_A::VALUE2)
    }
}
#[doc = "Field `HFNMIENA` reader - Enable the operation of MPU during hard fault, NMI, and FAULTMASK handlers"]
pub type HFNMIENA_R = crate::BitReader<HFNMIENA_A>;
#[doc = "Enable the operation of MPU during hard fault, NMI, and FAULTMASK handlers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HFNMIENA_A {
    #[doc = "0: MPU is disabled during hard fault, NMI, and FAULTMASK handlers, regardless of the value of the ENABLE bit"]
    VALUE1 = 0,
    #[doc = "1: the MPU is enabled during hard fault, NMI, and FAULTMASK handlers."]
    VALUE2 = 1,
}
impl From<HFNMIENA_A> for bool {
    #[inline(always)]
    fn from(variant: HFNMIENA_A) -> Self {
        variant as u8 != 0
    }
}
impl HFNMIENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFNMIENA_A {
        match self.bits {
            false => HFNMIENA_A::VALUE1,
            true => HFNMIENA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HFNMIENA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HFNMIENA_A::VALUE2
    }
}
#[doc = "Field `HFNMIENA` writer - Enable the operation of MPU during hard fault, NMI, and FAULTMASK handlers"]
pub type HFNMIENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_CTRL_SPEC, HFNMIENA_A, O>;
impl<'a, const O: u8> HFNMIENA_W<'a, O> {
    #[doc = "MPU is disabled during hard fault, NMI, and FAULTMASK handlers, regardless of the value of the ENABLE bit"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HFNMIENA_A::VALUE1)
    }
    #[doc = "the MPU is enabled during hard fault, NMI, and FAULTMASK handlers."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HFNMIENA_A::VALUE2)
    }
}
#[doc = "Field `PRIVDEFENA` reader - Enables privileged software access to the default memory map"]
pub type PRIVDEFENA_R = crate::BitReader<PRIVDEFENA_A>;
#[doc = "Enables privileged software access to the default memory map\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIVDEFENA_A {
    #[doc = "0: If the MPU is enabled, disables use of the default memory map. Any memory access to a location not covered by any enabled region causes a fault."]
    VALUE1 = 0,
    #[doc = "1: If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses."]
    VALUE2 = 1,
}
impl From<PRIVDEFENA_A> for bool {
    #[inline(always)]
    fn from(variant: PRIVDEFENA_A) -> Self {
        variant as u8 != 0
    }
}
impl PRIVDEFENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRIVDEFENA_A {
        match self.bits {
            false => PRIVDEFENA_A::VALUE1,
            true => PRIVDEFENA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRIVDEFENA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRIVDEFENA_A::VALUE2
    }
}
#[doc = "Field `PRIVDEFENA` writer - Enables privileged software access to the default memory map"]
pub type PRIVDEFENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_CTRL_SPEC, PRIVDEFENA_A, O>;
impl<'a, const O: u8> PRIVDEFENA_W<'a, O> {
    #[doc = "If the MPU is enabled, disables use of the default memory map. Any memory access to a location not covered by any enabled region causes a fault."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRIVDEFENA_A::VALUE1)
    }
    #[doc = "If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRIVDEFENA_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Enable MPU"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the operation of MPU during hard fault, NMI, and FAULTMASK handlers"]
    #[inline(always)]
    pub fn hfnmiena(&self) -> HFNMIENA_R {
        HFNMIENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables privileged software access to the default memory map"]
    #[inline(always)]
    pub fn privdefena(&self) -> PRIVDEFENA_R {
        PRIVDEFENA_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable MPU"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Enable the operation of MPU during hard fault, NMI, and FAULTMASK handlers"]
    #[inline(always)]
    #[must_use]
    pub fn hfnmiena(&mut self) -> HFNMIENA_W<1> {
        HFNMIENA_W::new(self)
    }
    #[doc = "Bit 2 - Enables privileged software access to the default memory map"]
    #[inline(always)]
    #[must_use]
    pub fn privdefena(&mut self) -> PRIVDEFENA_W<2> {
        PRIVDEFENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_ctrl](index.html) module"]
pub struct MPU_CTRL_SPEC;
impl crate::RegisterSpec for MPU_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpu_ctrl::R](R) reader structure"]
impl crate::Readable for MPU_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpu_ctrl::W](W) writer structure"]
impl crate::Writable for MPU_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPU_CTRL to value 0"]
impl crate::Resettable for MPU_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
