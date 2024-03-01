#[doc = "Register `CPUCLKCR` reader"]
pub type R = crate::R<CpuclkcrSpec>;
#[doc = "Register `CPUCLKCR` writer"]
pub type W = crate::W<CpuclkcrSpec>;
#[doc = "CPU Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpudiv {
    #[doc = "0: fCPU = fSYS"]
    Const0 = 0,
    #[doc = "1: fCPU = fSYS / 2"]
    Const1 = 1,
}
impl From<Cpudiv> for bool {
    #[inline(always)]
    fn from(variant: Cpudiv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPUDIV` reader - CPU Clock Divider Enable"]
pub type CpudivR = crate::BitReader<Cpudiv>;
impl CpudivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpudiv {
        match self.bits {
            false => Cpudiv::Const0,
            true => Cpudiv::Const1,
        }
    }
    #[doc = "fCPU = fSYS"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Cpudiv::Const0
    }
    #[doc = "fCPU = fSYS / 2"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Cpudiv::Const1
    }
}
#[doc = "Field `CPUDIV` writer - CPU Clock Divider Enable"]
pub type CpudivW<'a, REG> = crate::BitWriter<'a, REG, Cpudiv>;
impl<'a, REG> CpudivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fCPU = fSYS"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpudiv::Const0)
    }
    #[doc = "fCPU = fSYS / 2"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpudiv::Const1)
    }
}
impl R {
    #[doc = "Bit 0 - CPU Clock Divider Enable"]
    #[inline(always)]
    pub fn cpudiv(&self) -> CpudivR {
        CpudivR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU Clock Divider Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpudiv(&mut self) -> CpudivW<CpuclkcrSpec> {
        CpudivW::new(self, 0)
    }
}
#[doc = "CPU Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuclkcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuclkcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuclkcrSpec;
impl crate::RegisterSpec for CpuclkcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuclkcr::R`](R) reader structure"]
impl crate::Readable for CpuclkcrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpuclkcr::W`](W) writer structure"]
impl crate::Writable for CpuclkcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUCLKCR to value 0"]
impl crate::Resettable for CpuclkcrSpec {
    const RESET_VALUE: u32 = 0;
}
