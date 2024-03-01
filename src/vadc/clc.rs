#[doc = "Register `CLC` reader"]
pub type R = crate::R<ClcSpec>;
#[doc = "Register `CLC` writer"]
pub type W = crate::W<ClcSpec>;
#[doc = "Module Disable Request Bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disr {
    #[doc = "0: On request: enable the module clock"]
    Value1 = 0,
    #[doc = "1: Off request: stop the module clock"]
    Value2 = 1,
}
impl From<Disr> for bool {
    #[inline(always)]
    fn from(variant: Disr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISR` reader - Module Disable Request Bit"]
pub type DisrR = crate::BitReader<Disr>;
impl DisrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Disr {
        match self.bits {
            false => Disr::Value1,
            true => Disr::Value2,
        }
    }
    #[doc = "On request: enable the module clock"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Disr::Value1
    }
    #[doc = "Off request: stop the module clock"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Disr::Value2
    }
}
#[doc = "Field `DISR` writer - Module Disable Request Bit"]
pub type DisrW<'a, REG> = crate::BitWriter<'a, REG, Disr>;
impl<'a, REG> DisrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "On request: enable the module clock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Disr::Value1)
    }
    #[doc = "Off request: stop the module clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Disr::Value2)
    }
}
#[doc = "Module Disable Status Bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Diss {
    #[doc = "0: Module clock is enabled"]
    Value1 = 0,
    #[doc = "1: Off: module is not clocked"]
    Value2 = 1,
}
impl From<Diss> for bool {
    #[inline(always)]
    fn from(variant: Diss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISS` reader - Module Disable Status Bit"]
pub type DissR = crate::BitReader<Diss>;
impl DissR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Diss {
        match self.bits {
            false => Diss::Value1,
            true => Diss::Value2,
        }
    }
    #[doc = "Module clock is enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Diss::Value1
    }
    #[doc = "Off: module is not clocked"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Diss::Value2
    }
}
#[doc = "Sleep Mode Enable Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edis {
    #[doc = "0: Sleep mode request is enabled and functional"]
    Value1 = 0,
    #[doc = "1: Module disregards the sleep mode control signal"]
    Value2 = 1,
}
impl From<Edis> for bool {
    #[inline(always)]
    fn from(variant: Edis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDIS` reader - Sleep Mode Enable Control"]
pub type EdisR = crate::BitReader<Edis>;
impl EdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edis {
        match self.bits {
            false => Edis::Value1,
            true => Edis::Value2,
        }
    }
    #[doc = "Sleep mode request is enabled and functional"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Edis::Value1
    }
    #[doc = "Module disregards the sleep mode control signal"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Edis::Value2
    }
}
#[doc = "Field `EDIS` writer - Sleep Mode Enable Control"]
pub type EdisW<'a, REG> = crate::BitWriter<'a, REG, Edis>;
impl<'a, REG> EdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sleep mode request is enabled and functional"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Edis::Value1)
    }
    #[doc = "Module disregards the sleep mode control signal"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Edis::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Module Disable Request Bit"]
    #[inline(always)]
    pub fn disr(&self) -> DisrR {
        DisrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Module Disable Status Bit"]
    #[inline(always)]
    pub fn diss(&self) -> DissR {
        DissR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Sleep Mode Enable Control"]
    #[inline(always)]
    pub fn edis(&self) -> EdisR {
        EdisR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Module Disable Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn disr(&mut self) -> DisrW<ClcSpec> {
        DisrW::new(self, 0)
    }
    #[doc = "Bit 3 - Sleep Mode Enable Control"]
    #[inline(always)]
    #[must_use]
    pub fn edis(&mut self) -> EdisW<ClcSpec> {
        EdisW::new(self, 3)
    }
}
#[doc = "Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClcSpec;
impl crate::RegisterSpec for ClcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clc::R`](R) reader structure"]
impl crate::Readable for ClcSpec {}
#[doc = "`write(|w| ..)` method takes [`clc::W`](W) writer structure"]
impl crate::Writable for ClcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLC to value 0x03"]
impl crate::Resettable for ClcSpec {
    const RESET_VALUE: u32 = 0x03;
}
