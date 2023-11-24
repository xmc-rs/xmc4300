#[doc = "Register `OSCSICTRL` reader"]
pub type R = crate::R<OSCSICTRL_SPEC>;
#[doc = "Register `OSCSICTRL` writer"]
pub type W = crate::W<OSCSICTRL_SPEC>;
#[doc = "Field `PWD` reader - Turn OFF the fOSI Clock Source"]
pub type PWD_R = crate::BitReader<PWD_A>;
#[doc = "Turn OFF the fOSI Clock Source\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWD_A {
    #[doc = "0: Enabled"]
    CONST_0 = 0,
    #[doc = "1: Disabled"]
    CONST_1 = 1,
}
impl From<PWD_A> for bool {
    #[inline(always)]
    fn from(variant: PWD_A) -> Self {
        variant as u8 != 0
    }
}
impl PWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWD_A {
        match self.bits {
            false => PWD_A::CONST_0,
            true => PWD_A::CONST_1,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PWD_A::CONST_0
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PWD_A::CONST_1
    }
}
#[doc = "Field `PWD` writer - Turn OFF the fOSI Clock Source"]
pub type PWD_W<'a, REG> = crate::BitWriter<'a, REG, PWD_A>;
impl<'a, REG> PWD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PWD_A::CONST_0)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PWD_A::CONST_1)
    }
}
impl R {
    #[doc = "Bit 0 - Turn OFF the fOSI Clock Source"]
    #[inline(always)]
    pub fn pwd(&self) -> PWD_R {
        PWD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Turn OFF the fOSI Clock Source"]
    #[inline(always)]
    #[must_use]
    pub fn pwd(&mut self) -> PWD_W<OSCSICTRL_SPEC> {
        PWD_W::new(self, 0)
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
#[doc = "fOSI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oscsictrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscsictrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCSICTRL_SPEC;
impl crate::RegisterSpec for OSCSICTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oscsictrl::R`](R) reader structure"]
impl crate::Readable for OSCSICTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oscsictrl::W`](W) writer structure"]
impl crate::Writable for OSCSICTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCSICTRL to value 0x01"]
impl crate::Resettable for OSCSICTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
