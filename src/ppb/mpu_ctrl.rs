#[doc = "Register `MPU_CTRL` reader"]
pub type R = crate::R<MpuCtrlSpec>;
#[doc = "Register `MPU_CTRL` writer"]
pub type W = crate::W<MpuCtrlSpec>;
#[doc = "Enable MPU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: MPU disabled"]
    Value1 = 0,
    #[doc = "1: MPU enabled."]
    Value2 = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable MPU"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Value1,
            true => Enable::Value2,
        }
    }
    #[doc = "MPU disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Enable::Value1
    }
    #[doc = "MPU enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Enable::Value2
    }
}
#[doc = "Field `ENABLE` writer - Enable MPU"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MPU disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Value1)
    }
    #[doc = "MPU enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Value2)
    }
}
#[doc = "Enable the operation of MPU during hard fault, NMI, and FAULTMASK handlers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfnmiena {
    #[doc = "0: MPU is disabled during hard fault, NMI, and FAULTMASK handlers, regardless of the value of the ENABLE bit"]
    Value1 = 0,
    #[doc = "1: the MPU is enabled during hard fault, NMI, and FAULTMASK handlers."]
    Value2 = 1,
}
impl From<Hfnmiena> for bool {
    #[inline(always)]
    fn from(variant: Hfnmiena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFNMIENA` reader - Enable the operation of MPU during hard fault, NMI, and FAULTMASK handlers"]
pub type HfnmienaR = crate::BitReader<Hfnmiena>;
impl HfnmienaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfnmiena {
        match self.bits {
            false => Hfnmiena::Value1,
            true => Hfnmiena::Value2,
        }
    }
    #[doc = "MPU is disabled during hard fault, NMI, and FAULTMASK handlers, regardless of the value of the ENABLE bit"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hfnmiena::Value1
    }
    #[doc = "the MPU is enabled during hard fault, NMI, and FAULTMASK handlers."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hfnmiena::Value2
    }
}
#[doc = "Field `HFNMIENA` writer - Enable the operation of MPU during hard fault, NMI, and FAULTMASK handlers"]
pub type HfnmienaW<'a, REG> = crate::BitWriter<'a, REG, Hfnmiena>;
impl<'a, REG> HfnmienaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MPU is disabled during hard fault, NMI, and FAULTMASK handlers, regardless of the value of the ENABLE bit"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hfnmiena::Value1)
    }
    #[doc = "the MPU is enabled during hard fault, NMI, and FAULTMASK handlers."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hfnmiena::Value2)
    }
}
#[doc = "Enables privileged software access to the default memory map\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Privdefena {
    #[doc = "0: If the MPU is enabled, disables use of the default memory map. Any memory access to a location not covered by any enabled region causes a fault."]
    Value1 = 0,
    #[doc = "1: If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses."]
    Value2 = 1,
}
impl From<Privdefena> for bool {
    #[inline(always)]
    fn from(variant: Privdefena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIVDEFENA` reader - Enables privileged software access to the default memory map"]
pub type PrivdefenaR = crate::BitReader<Privdefena>;
impl PrivdefenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Privdefena {
        match self.bits {
            false => Privdefena::Value1,
            true => Privdefena::Value2,
        }
    }
    #[doc = "If the MPU is enabled, disables use of the default memory map. Any memory access to a location not covered by any enabled region causes a fault."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Privdefena::Value1
    }
    #[doc = "If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Privdefena::Value2
    }
}
#[doc = "Field `PRIVDEFENA` writer - Enables privileged software access to the default memory map"]
pub type PrivdefenaW<'a, REG> = crate::BitWriter<'a, REG, Privdefena>;
impl<'a, REG> PrivdefenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If the MPU is enabled, disables use of the default memory map. Any memory access to a location not covered by any enabled region causes a fault."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Privdefena::Value1)
    }
    #[doc = "If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Privdefena::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Enable MPU"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the operation of MPU during hard fault, NMI, and FAULTMASK handlers"]
    #[inline(always)]
    pub fn hfnmiena(&self) -> HfnmienaR {
        HfnmienaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables privileged software access to the default memory map"]
    #[inline(always)]
    pub fn privdefena(&self) -> PrivdefenaR {
        PrivdefenaR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable MPU"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<MpuCtrlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable the operation of MPU during hard fault, NMI, and FAULTMASK handlers"]
    #[inline(always)]
    #[must_use]
    pub fn hfnmiena(&mut self) -> HfnmienaW<MpuCtrlSpec> {
        HfnmienaW::new(self, 1)
    }
    #[doc = "Bit 2 - Enables privileged software access to the default memory map"]
    #[inline(always)]
    #[must_use]
    pub fn privdefena(&mut self) -> PrivdefenaW<MpuCtrlSpec> {
        PrivdefenaW::new(self, 2)
    }
}
#[doc = "MPU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MpuCtrlSpec;
impl crate::RegisterSpec for MpuCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_ctrl::R`](R) reader structure"]
impl crate::Readable for MpuCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mpu_ctrl::W`](W) writer structure"]
impl crate::Writable for MpuCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_CTRL to value 0"]
impl crate::Resettable for MpuCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
