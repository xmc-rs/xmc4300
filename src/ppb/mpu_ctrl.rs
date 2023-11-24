#[doc = "Register `MPU_CTRL` reader"]
pub type R = crate::R<MPU_CTRL_SPEC>;
#[doc = "Register `MPU_CTRL` writer"]
pub type W = crate::W<MPU_CTRL_SPEC>;
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
    pub const fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::VALUE1,
            true => ENABLE_A::VALUE2,
        }
    }
    #[doc = "MPU disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENABLE_A::VALUE1
    }
    #[doc = "MPU enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENABLE_A::VALUE2
    }
}
#[doc = "Field `ENABLE` writer - Enable MPU"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, ENABLE_A>;
impl<'a, REG> ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MPU disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_A::VALUE1)
    }
    #[doc = "MPU enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> HFNMIENA_A {
        match self.bits {
            false => HFNMIENA_A::VALUE1,
            true => HFNMIENA_A::VALUE2,
        }
    }
    #[doc = "MPU is disabled during hard fault, NMI, and FAULTMASK handlers, regardless of the value of the ENABLE bit"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HFNMIENA_A::VALUE1
    }
    #[doc = "the MPU is enabled during hard fault, NMI, and FAULTMASK handlers."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HFNMIENA_A::VALUE2
    }
}
#[doc = "Field `HFNMIENA` writer - Enable the operation of MPU during hard fault, NMI, and FAULTMASK handlers"]
pub type HFNMIENA_W<'a, REG> = crate::BitWriter<'a, REG, HFNMIENA_A>;
impl<'a, REG> HFNMIENA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MPU is disabled during hard fault, NMI, and FAULTMASK handlers, regardless of the value of the ENABLE bit"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HFNMIENA_A::VALUE1)
    }
    #[doc = "the MPU is enabled during hard fault, NMI, and FAULTMASK handlers."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> PRIVDEFENA_A {
        match self.bits {
            false => PRIVDEFENA_A::VALUE1,
            true => PRIVDEFENA_A::VALUE2,
        }
    }
    #[doc = "If the MPU is enabled, disables use of the default memory map. Any memory access to a location not covered by any enabled region causes a fault."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRIVDEFENA_A::VALUE1
    }
    #[doc = "If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRIVDEFENA_A::VALUE2
    }
}
#[doc = "Field `PRIVDEFENA` writer - Enables privileged software access to the default memory map"]
pub type PRIVDEFENA_W<'a, REG> = crate::BitWriter<'a, REG, PRIVDEFENA_A>;
impl<'a, REG> PRIVDEFENA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If the MPU is enabled, disables use of the default memory map. Any memory access to a location not covered by any enabled region causes a fault."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIVDEFENA_A::VALUE1)
    }
    #[doc = "If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
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
    pub fn enable(&mut self) -> ENABLE_W<MPU_CTRL_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable the operation of MPU during hard fault, NMI, and FAULTMASK handlers"]
    #[inline(always)]
    #[must_use]
    pub fn hfnmiena(&mut self) -> HFNMIENA_W<MPU_CTRL_SPEC> {
        HFNMIENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enables privileged software access to the default memory map"]
    #[inline(always)]
    #[must_use]
    pub fn privdefena(&mut self) -> PRIVDEFENA_W<MPU_CTRL_SPEC> {
        PRIVDEFENA_W::new(self, 2)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MPU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPU_CTRL_SPEC;
impl crate::RegisterSpec for MPU_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_ctrl::R`](R) reader structure"]
impl crate::Readable for MPU_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mpu_ctrl::W`](W) writer structure"]
impl crate::Writable for MPU_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPU_CTRL to value 0"]
impl crate::Resettable for MPU_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
