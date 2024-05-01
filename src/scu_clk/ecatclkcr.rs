#[doc = "Register `ECATCLKCR` reader"]
pub type R = crate::R<ECATCLKCR_SPEC>;
#[doc = "Register `ECATCLKCR` writer"]
pub type W = crate::W<ECATCLKCR_SPEC>;
#[doc = "Field `ECADIV` reader - EtherCAT Clock Divider Value"]
pub type ECADIV_R = crate::FieldReader;
#[doc = "Field `ECADIV` writer - EtherCAT Clock Divider Value"]
pub type ECADIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "EtherCAT Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECATSEL_A {
    #[doc = "0: fPLLUSB clock"]
    CONST_0 = 0,
    #[doc = "1: fPLL clock"]
    CONST_1 = 1,
}
impl From<ECATSEL_A> for bool {
    #[inline(always)]
    fn from(variant: ECATSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECATSEL` reader - EtherCAT Clock Selection Value"]
pub type ECATSEL_R = crate::BitReader<ECATSEL_A>;
impl ECATSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECATSEL_A {
        match self.bits {
            false => ECATSEL_A::CONST_0,
            true => ECATSEL_A::CONST_1,
        }
    }
    #[doc = "fPLLUSB clock"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ECATSEL_A::CONST_0
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ECATSEL_A::CONST_1
    }
}
#[doc = "Field `ECATSEL` writer - EtherCAT Clock Selection Value"]
pub type ECATSEL_W<'a, REG> = crate::BitWriter<'a, REG, ECATSEL_A>;
impl<'a, REG> ECATSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fPLLUSB clock"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(ECATSEL_A::CONST_0)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(ECATSEL_A::CONST_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - EtherCAT Clock Divider Value"]
    #[inline(always)]
    pub fn ecadiv(&self) -> ECADIV_R {
        ECADIV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 16 - EtherCAT Clock Selection Value"]
    #[inline(always)]
    pub fn ecatsel(&self) -> ECATSEL_R {
        ECATSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - EtherCAT Clock Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn ecadiv(&mut self) -> ECADIV_W<ECATCLKCR_SPEC> {
        ECADIV_W::new(self, 0)
    }
    #[doc = "Bit 16 - EtherCAT Clock Selection Value"]
    #[inline(always)]
    #[must_use]
    pub fn ecatsel(&mut self) -> ECATSEL_W<ECATCLKCR_SPEC> {
        ECATSEL_W::new(self, 16)
    }
}
#[doc = "EtherCAT Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecatclkcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecatclkcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECATCLKCR_SPEC;
impl crate::RegisterSpec for ECATCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecatclkcr::R`](R) reader structure"]
impl crate::Readable for ECATCLKCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ecatclkcr::W`](W) writer structure"]
impl crate::Writable for ECATCLKCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECATCLKCR to value 0"]
impl crate::Resettable for ECATCLKCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
