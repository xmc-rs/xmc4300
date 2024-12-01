#[doc = "Register `OSCSICTRL` reader"]
pub type R = crate::R<OSCSICTRL_SPEC>;
#[doc = "Register `OSCSICTRL` writer"]
pub type W = crate::W<OSCSICTRL_SPEC>;
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
#[doc = "Field `PWD` reader - Turn OFF the fOSI Clock Source"]
pub type PWD_R = crate::BitReader<PWD_A>;
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
    pub fn pwd(&mut self) -> PWD_W<OSCSICTRL_SPEC> {
        PWD_W::new(self, 0)
    }
}
#[doc = "fOSI Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`oscsictrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oscsictrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCSICTRL_SPEC;
impl crate::RegisterSpec for OSCSICTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oscsictrl::R`](R) reader structure"]
impl crate::Readable for OSCSICTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oscsictrl::W`](W) writer structure"]
impl crate::Writable for OSCSICTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSCSICTRL to value 0x01"]
impl crate::Resettable for OSCSICTRL_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
