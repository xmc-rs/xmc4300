#[doc = "Register `CCUCLKCR` reader"]
pub type R = crate::R<CCUCLKCR_SPEC>;
#[doc = "Register `CCUCLKCR` writer"]
pub type W = crate::W<CCUCLKCR_SPEC>;
#[doc = "Field `CCUDIV` reader - CCU Clock Divider Enable"]
pub type CCUDIV_R = crate::BitReader<CCUDIV_A>;
#[doc = "CCU Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCUDIV_A {
    #[doc = "0: fCCU = fSYS"]
    CONST_0 = 0,
    #[doc = "1: fCCU = fSYS / 2"]
    CONST_1 = 1,
}
impl From<CCUDIV_A> for bool {
    #[inline(always)]
    fn from(variant: CCUDIV_A) -> Self {
        variant as u8 != 0
    }
}
impl CCUDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCUDIV_A {
        match self.bits {
            false => CCUDIV_A::CONST_0,
            true => CCUDIV_A::CONST_1,
        }
    }
    #[doc = "fCCU = fSYS"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == CCUDIV_A::CONST_0
    }
    #[doc = "fCCU = fSYS / 2"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == CCUDIV_A::CONST_1
    }
}
#[doc = "Field `CCUDIV` writer - CCU Clock Divider Enable"]
pub type CCUDIV_W<'a, REG> = crate::BitWriter<'a, REG, CCUDIV_A>;
impl<'a, REG> CCUDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fCCU = fSYS"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(CCUDIV_A::CONST_0)
    }
    #[doc = "fCCU = fSYS / 2"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(CCUDIV_A::CONST_1)
    }
}
impl R {
    #[doc = "Bit 0 - CCU Clock Divider Enable"]
    #[inline(always)]
    pub fn ccudiv(&self) -> CCUDIV_R {
        CCUDIV_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCU Clock Divider Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccudiv(&mut self) -> CCUDIV_W<CCUCLKCR_SPEC> {
        CCUDIV_W::new(self, 0)
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
#[doc = "CCU Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccuclkcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccuclkcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCUCLKCR_SPEC;
impl crate::RegisterSpec for CCUCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccuclkcr::R`](R) reader structure"]
impl crate::Readable for CCUCLKCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccuclkcr::W`](W) writer structure"]
impl crate::Writable for CCUCLKCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCUCLKCR to value 0"]
impl crate::Resettable for CCUCLKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
