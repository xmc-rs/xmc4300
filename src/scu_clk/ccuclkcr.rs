#[doc = "Register `CCUCLKCR` reader"]
pub type R = crate::R<CcuclkcrSpec>;
#[doc = "Register `CCUCLKCR` writer"]
pub type W = crate::W<CcuclkcrSpec>;
#[doc = "CCU Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccudiv {
    #[doc = "0: fCCU = fSYS"]
    Const0 = 0,
    #[doc = "1: fCCU = fSYS / 2"]
    Const1 = 1,
}
impl From<Ccudiv> for bool {
    #[inline(always)]
    fn from(variant: Ccudiv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUDIV` reader - CCU Clock Divider Enable"]
pub type CcudivR = crate::BitReader<Ccudiv>;
impl CcudivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccudiv {
        match self.bits {
            false => Ccudiv::Const0,
            true => Ccudiv::Const1,
        }
    }
    #[doc = "fCCU = fSYS"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ccudiv::Const0
    }
    #[doc = "fCCU = fSYS / 2"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ccudiv::Const1
    }
}
#[doc = "Field `CCUDIV` writer - CCU Clock Divider Enable"]
pub type CcudivW<'a, REG> = crate::BitWriter<'a, REG, Ccudiv>;
impl<'a, REG> CcudivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fCCU = fSYS"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccudiv::Const0)
    }
    #[doc = "fCCU = fSYS / 2"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccudiv::Const1)
    }
}
impl R {
    #[doc = "Bit 0 - CCU Clock Divider Enable"]
    #[inline(always)]
    pub fn ccudiv(&self) -> CcudivR {
        CcudivR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCU Clock Divider Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccudiv(&mut self) -> CcudivW<CcuclkcrSpec> {
        CcudivW::new(self, 0)
    }
}
#[doc = "CCU Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccuclkcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccuclkcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcuclkcrSpec;
impl crate::RegisterSpec for CcuclkcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccuclkcr::R`](R) reader structure"]
impl crate::Readable for CcuclkcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccuclkcr::W`](W) writer structure"]
impl crate::Writable for CcuclkcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCUCLKCR to value 0"]
impl crate::Resettable for CcuclkcrSpec {
    const RESET_VALUE: u32 = 0;
}
