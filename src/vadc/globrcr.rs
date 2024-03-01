#[doc = "Register `GLOBRCR` reader"]
pub type R = crate::R<GlobrcrSpec>;
#[doc = "Register `GLOBRCR` writer"]
pub type W = crate::W<GlobrcrSpec>;
#[doc = "Data Reduction Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Drctr {
    #[doc = "0: Data reduction disabled"]
    Value1 = 0,
}
impl From<Drctr> for u8 {
    #[inline(always)]
    fn from(variant: Drctr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Drctr {
    type Ux = u8;
}
#[doc = "Field `DRCTR` reader - Data Reduction Control"]
pub type DrctrR = crate::FieldReader<Drctr>;
impl DrctrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Drctr> {
        match self.bits {
            0 => Some(Drctr::Value1),
            _ => None,
        }
    }
    #[doc = "Data reduction disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Drctr::Value1
    }
}
#[doc = "Field `DRCTR` writer - Data Reduction Control"]
pub type DrctrW<'a, REG> = crate::FieldWriter<'a, REG, 4, Drctr>;
impl<'a, REG> DrctrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data reduction disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Drctr::Value1)
    }
}
#[doc = "Wait-for-Read Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wfr {
    #[doc = "0: Overwrite mode"]
    Value1 = 0,
    #[doc = "1: Wait-for-read mode enabled for this register"]
    Value2 = 1,
}
impl From<Wfr> for bool {
    #[inline(always)]
    fn from(variant: Wfr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WFR` reader - Wait-for-Read Mode Enable"]
pub type WfrR = crate::BitReader<Wfr>;
impl WfrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wfr {
        match self.bits {
            false => Wfr::Value1,
            true => Wfr::Value2,
        }
    }
    #[doc = "Overwrite mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wfr::Value1
    }
    #[doc = "Wait-for-read mode enabled for this register"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wfr::Value2
    }
}
#[doc = "Field `WFR` writer - Wait-for-Read Mode Enable"]
pub type WfrW<'a, REG> = crate::BitWriter<'a, REG, Wfr>;
impl<'a, REG> WfrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overwrite mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wfr::Value1)
    }
    #[doc = "Wait-for-read mode enabled for this register"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wfr::Value2)
    }
}
#[doc = "Service Request Generation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srgen {
    #[doc = "0: No service request"]
    Value1 = 0,
    #[doc = "1: Service request after a result event"]
    Value2 = 1,
}
impl From<Srgen> for bool {
    #[inline(always)]
    fn from(variant: Srgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRGEN` reader - Service Request Generation Enable"]
pub type SrgenR = crate::BitReader<Srgen>;
impl SrgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srgen {
        match self.bits {
            false => Srgen::Value1,
            true => Srgen::Value2,
        }
    }
    #[doc = "No service request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Srgen::Value1
    }
    #[doc = "Service request after a result event"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Srgen::Value2
    }
}
#[doc = "Field `SRGEN` writer - Service Request Generation Enable"]
pub type SrgenW<'a, REG> = crate::BitWriter<'a, REG, Srgen>;
impl<'a, REG> SrgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No service request"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Srgen::Value1)
    }
    #[doc = "Service request after a result event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Srgen::Value2)
    }
}
impl R {
    #[doc = "Bits 16:19 - Data Reduction Control"]
    #[inline(always)]
    pub fn drctr(&self) -> DrctrR {
        DrctrR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Wait-for-Read Mode Enable"]
    #[inline(always)]
    pub fn wfr(&self) -> WfrR {
        WfrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 31 - Service Request Generation Enable"]
    #[inline(always)]
    pub fn srgen(&self) -> SrgenR {
        SrgenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:19 - Data Reduction Control"]
    #[inline(always)]
    #[must_use]
    pub fn drctr(&mut self) -> DrctrW<GlobrcrSpec> {
        DrctrW::new(self, 16)
    }
    #[doc = "Bit 24 - Wait-for-Read Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wfr(&mut self) -> WfrW<GlobrcrSpec> {
        WfrW::new(self, 24)
    }
    #[doc = "Bit 31 - Service Request Generation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn srgen(&mut self) -> SrgenW<GlobrcrSpec> {
        SrgenW::new(self, 31)
    }
}
#[doc = "Global Result Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globrcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globrcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlobrcrSpec;
impl crate::RegisterSpec for GlobrcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`globrcr::R`](R) reader structure"]
impl crate::Readable for GlobrcrSpec {}
#[doc = "`write(|w| ..)` method takes [`globrcr::W`](W) writer structure"]
impl crate::Writable for GlobrcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLOBRCR to value 0"]
impl crate::Resettable for GlobrcrSpec {
    const RESET_VALUE: u32 = 0;
}
