#[doc = "Register `PBCLKCR` reader"]
pub type R = crate::R<PbclkcrSpec>;
#[doc = "Register `PBCLKCR` writer"]
pub type W = crate::W<PbclkcrSpec>;
#[doc = "PB Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbdiv {
    #[doc = "0: fPERIPH = fCPU"]
    Const0 = 0,
    #[doc = "1: fPERIPH = fCPU / 2"]
    Const1 = 1,
}
impl From<Pbdiv> for bool {
    #[inline(always)]
    fn from(variant: Pbdiv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBDIV` reader - PB Clock Divider Enable"]
pub type PbdivR = crate::BitReader<Pbdiv>;
impl PbdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbdiv {
        match self.bits {
            false => Pbdiv::Const0,
            true => Pbdiv::Const1,
        }
    }
    #[doc = "fPERIPH = fCPU"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pbdiv::Const0
    }
    #[doc = "fPERIPH = fCPU / 2"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pbdiv::Const1
    }
}
#[doc = "Field `PBDIV` writer - PB Clock Divider Enable"]
pub type PbdivW<'a, REG> = crate::BitWriter<'a, REG, Pbdiv>;
impl<'a, REG> PbdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fPERIPH = fCPU"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pbdiv::Const0)
    }
    #[doc = "fPERIPH = fCPU / 2"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pbdiv::Const1)
    }
}
impl R {
    #[doc = "Bit 0 - PB Clock Divider Enable"]
    #[inline(always)]
    pub fn pbdiv(&self) -> PbdivR {
        PbdivR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PB Clock Divider Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pbdiv(&mut self) -> PbdivW<PbclkcrSpec> {
        PbdivW::new(self, 0)
    }
}
#[doc = "Peripheral Bus Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pbclkcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pbclkcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbclkcrSpec;
impl crate::RegisterSpec for PbclkcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbclkcr::R`](R) reader structure"]
impl crate::Readable for PbclkcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pbclkcr::W`](W) writer structure"]
impl crate::Writable for PbclkcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBCLKCR to value 0"]
impl crate::Resettable for PbclkcrSpec {
    const RESET_VALUE: u32 = 0;
}
