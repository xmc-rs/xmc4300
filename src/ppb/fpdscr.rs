#[doc = "Register `FPDSCR` reader"]
pub type R = crate::R<FpdscrSpec>;
#[doc = "Register `FPDSCR` writer"]
pub type W = crate::W<FpdscrSpec>;
#[doc = "Field `RMode` reader - Default value for FPSCR.RMode"]
pub type RmodeR = crate::FieldReader;
#[doc = "Field `RMode` writer - Default value for FPSCR.RMode"]
pub type RmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FZ` reader - Default value for FPSCR.FZ"]
pub type FzR = crate::BitReader;
#[doc = "Field `FZ` writer - Default value for FPSCR.FZ"]
pub type FzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DN` reader - Default value for FPSCR.DN"]
pub type DnR = crate::BitReader;
#[doc = "Field `DN` writer - Default value for FPSCR.DN"]
pub type DnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHP` reader - Default value for FPSCR.AHP"]
pub type AhpR = crate::BitReader;
#[doc = "Field `AHP` writer - Default value for FPSCR.AHP"]
pub type AhpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 22:23 - Default value for FPSCR.RMode"]
    #[inline(always)]
    pub fn rmode(&self) -> RmodeR {
        RmodeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Default value for FPSCR.FZ"]
    #[inline(always)]
    pub fn fz(&self) -> FzR {
        FzR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Default value for FPSCR.DN"]
    #[inline(always)]
    pub fn dn(&self) -> DnR {
        DnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Default value for FPSCR.AHP"]
    #[inline(always)]
    pub fn ahp(&self) -> AhpR {
        AhpR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 22:23 - Default value for FPSCR.RMode"]
    #[inline(always)]
    #[must_use]
    pub fn rmode(&mut self) -> RmodeW<FpdscrSpec> {
        RmodeW::new(self, 22)
    }
    #[doc = "Bit 24 - Default value for FPSCR.FZ"]
    #[inline(always)]
    #[must_use]
    pub fn fz(&mut self) -> FzW<FpdscrSpec> {
        FzW::new(self, 24)
    }
    #[doc = "Bit 25 - Default value for FPSCR.DN"]
    #[inline(always)]
    #[must_use]
    pub fn dn(&mut self) -> DnW<FpdscrSpec> {
        DnW::new(self, 25)
    }
    #[doc = "Bit 26 - Default value for FPSCR.AHP"]
    #[inline(always)]
    #[must_use]
    pub fn ahp(&mut self) -> AhpW<FpdscrSpec> {
        AhpW::new(self, 26)
    }
}
#[doc = "Floating-point Default Status Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpdscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpdscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpdscrSpec;
impl crate::RegisterSpec for FpdscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpdscr::R`](R) reader structure"]
impl crate::Readable for FpdscrSpec {}
#[doc = "`write(|w| ..)` method takes [`fpdscr::W`](W) writer structure"]
impl crate::Writable for FpdscrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPDSCR to value 0"]
impl crate::Resettable for FpdscrSpec {
    const RESET_VALUE: u32 = 0;
}
