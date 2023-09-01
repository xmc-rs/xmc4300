#[doc = "Register `FPDSCR` reader"]
pub type R = crate::R<FPDSCR_SPEC>;
#[doc = "Register `FPDSCR` writer"]
pub type W = crate::W<FPDSCR_SPEC>;
#[doc = "Field `RMode` reader - Default value for FPSCR.RMode"]
pub type RMODE_R = crate::FieldReader;
#[doc = "Field `RMode` writer - Default value for FPSCR.RMode"]
pub type RMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `FZ` reader - Default value for FPSCR.FZ"]
pub type FZ_R = crate::BitReader;
#[doc = "Field `FZ` writer - Default value for FPSCR.FZ"]
pub type FZ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DN` reader - Default value for FPSCR.DN"]
pub type DN_R = crate::BitReader;
#[doc = "Field `DN` writer - Default value for FPSCR.DN"]
pub type DN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AHP` reader - Default value for FPSCR.AHP"]
pub type AHP_R = crate::BitReader;
#[doc = "Field `AHP` writer - Default value for FPSCR.AHP"]
pub type AHP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 22:23 - Default value for FPSCR.RMode"]
    #[inline(always)]
    pub fn rmode(&self) -> RMODE_R {
        RMODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Default value for FPSCR.FZ"]
    #[inline(always)]
    pub fn fz(&self) -> FZ_R {
        FZ_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Default value for FPSCR.DN"]
    #[inline(always)]
    pub fn dn(&self) -> DN_R {
        DN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Default value for FPSCR.AHP"]
    #[inline(always)]
    pub fn ahp(&self) -> AHP_R {
        AHP_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 22:23 - Default value for FPSCR.RMode"]
    #[inline(always)]
    #[must_use]
    pub fn rmode(&mut self) -> RMODE_W<FPDSCR_SPEC, 22> {
        RMODE_W::new(self)
    }
    #[doc = "Bit 24 - Default value for FPSCR.FZ"]
    #[inline(always)]
    #[must_use]
    pub fn fz(&mut self) -> FZ_W<FPDSCR_SPEC, 24> {
        FZ_W::new(self)
    }
    #[doc = "Bit 25 - Default value for FPSCR.DN"]
    #[inline(always)]
    #[must_use]
    pub fn dn(&mut self) -> DN_W<FPDSCR_SPEC, 25> {
        DN_W::new(self)
    }
    #[doc = "Bit 26 - Default value for FPSCR.AHP"]
    #[inline(always)]
    #[must_use]
    pub fn ahp(&mut self) -> AHP_W<FPDSCR_SPEC, 26> {
        AHP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Floating-point Default Status Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpdscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpdscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPDSCR_SPEC;
impl crate::RegisterSpec for FPDSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpdscr::R`](R) reader structure"]
impl crate::Readable for FPDSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fpdscr::W`](W) writer structure"]
impl crate::Writable for FPDSCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPDSCR to value 0"]
impl crate::Resettable for FPDSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
