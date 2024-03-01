#[doc = "Register `ECATCLKCR` reader"]
pub type R = crate::R<EcatclkcrSpec>;
#[doc = "Register `ECATCLKCR` writer"]
pub type W = crate::W<EcatclkcrSpec>;
#[doc = "Field `ECADIV` reader - EtherCAT Clock Divider Value"]
pub type EcadivR = crate::FieldReader;
#[doc = "Field `ECADIV` writer - EtherCAT Clock Divider Value"]
pub type EcadivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "EtherCAT Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecatsel {
    #[doc = "0: fPLLUSB clock"]
    Const0 = 0,
    #[doc = "1: fPLL clock"]
    Const1 = 1,
}
impl From<Ecatsel> for bool {
    #[inline(always)]
    fn from(variant: Ecatsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECATSEL` reader - EtherCAT Clock Selection Value"]
pub type EcatselR = crate::BitReader<Ecatsel>;
impl EcatselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecatsel {
        match self.bits {
            false => Ecatsel::Const0,
            true => Ecatsel::Const1,
        }
    }
    #[doc = "fPLLUSB clock"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ecatsel::Const0
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ecatsel::Const1
    }
}
#[doc = "Field `ECATSEL` writer - EtherCAT Clock Selection Value"]
pub type EcatselW<'a, REG> = crate::BitWriter<'a, REG, Ecatsel>;
impl<'a, REG> EcatselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fPLLUSB clock"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ecatsel::Const0)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ecatsel::Const1)
    }
}
impl R {
    #[doc = "Bits 0:1 - EtherCAT Clock Divider Value"]
    #[inline(always)]
    pub fn ecadiv(&self) -> EcadivR {
        EcadivR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 16 - EtherCAT Clock Selection Value"]
    #[inline(always)]
    pub fn ecatsel(&self) -> EcatselR {
        EcatselR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - EtherCAT Clock Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn ecadiv(&mut self) -> EcadivW<EcatclkcrSpec> {
        EcadivW::new(self, 0)
    }
    #[doc = "Bit 16 - EtherCAT Clock Selection Value"]
    #[inline(always)]
    #[must_use]
    pub fn ecatsel(&mut self) -> EcatselW<EcatclkcrSpec> {
        EcatselW::new(self, 16)
    }
}
#[doc = "EtherCAT Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecatclkcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecatclkcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcatclkcrSpec;
impl crate::RegisterSpec for EcatclkcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecatclkcr::R`](R) reader structure"]
impl crate::Readable for EcatclkcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ecatclkcr::W`](W) writer structure"]
impl crate::Writable for EcatclkcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECATCLKCR to value 0"]
impl crate::Resettable for EcatclkcrSpec {
    const RESET_VALUE: u32 = 0;
}
