#[doc = "Register `ACTLR` reader"]
pub type R = crate::R<ACTLR_SPEC>;
#[doc = "Register `ACTLR` writer"]
pub type W = crate::W<ACTLR_SPEC>;
#[doc = "Field `DISMCYCINT` reader - Disable load/store multiple"]
pub type DISMCYCINT_R = crate::BitReader;
#[doc = "Field `DISMCYCINT` writer - Disable load/store multiple"]
pub type DISMCYCINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISDEFWBUF` reader - Disable write buffer"]
pub type DISDEFWBUF_R = crate::BitReader;
#[doc = "Field `DISDEFWBUF` writer - Disable write buffer"]
pub type DISDEFWBUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISFOLD` reader - Disable IT folding"]
pub type DISFOLD_R = crate::BitReader;
#[doc = "Field `DISFOLD` writer - Disable IT folding"]
pub type DISFOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISFPCA` reader - Disable FPCA update"]
pub type DISFPCA_R = crate::BitReader;
#[doc = "Field `DISFPCA` writer - Disable FPCA update"]
pub type DISFPCA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISOOFP` reader - Disable out of order FP execution"]
pub type DISOOFP_R = crate::BitReader;
#[doc = "Field `DISOOFP` writer - Disable out of order FP execution"]
pub type DISOOFP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Disable load/store multiple"]
    #[inline(always)]
    pub fn dismcycint(&self) -> DISMCYCINT_R {
        DISMCYCINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable write buffer"]
    #[inline(always)]
    pub fn disdefwbuf(&self) -> DISDEFWBUF_R {
        DISDEFWBUF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable IT folding"]
    #[inline(always)]
    pub fn disfold(&self) -> DISFOLD_R {
        DISFOLD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable FPCA update"]
    #[inline(always)]
    pub fn disfpca(&self) -> DISFPCA_R {
        DISFPCA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable out of order FP execution"]
    #[inline(always)]
    pub fn disoofp(&self) -> DISOOFP_R {
        DISOOFP_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable load/store multiple"]
    #[inline(always)]
    #[must_use]
    pub fn dismcycint(&mut self) -> DISMCYCINT_W<ACTLR_SPEC> {
        DISMCYCINT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Disable write buffer"]
    #[inline(always)]
    #[must_use]
    pub fn disdefwbuf(&mut self) -> DISDEFWBUF_W<ACTLR_SPEC> {
        DISDEFWBUF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Disable IT folding"]
    #[inline(always)]
    #[must_use]
    pub fn disfold(&mut self) -> DISFOLD_W<ACTLR_SPEC> {
        DISFOLD_W::new(self, 2)
    }
    #[doc = "Bit 8 - Disable FPCA update"]
    #[inline(always)]
    #[must_use]
    pub fn disfpca(&mut self) -> DISFPCA_W<ACTLR_SPEC> {
        DISFPCA_W::new(self, 8)
    }
    #[doc = "Bit 9 - Disable out of order FP execution"]
    #[inline(always)]
    #[must_use]
    pub fn disoofp(&mut self) -> DISOOFP_W<ACTLR_SPEC> {
        DISOOFP_W::new(self, 9)
    }
}
#[doc = "Auxiliary Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`actlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`actlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACTLR_SPEC;
impl crate::RegisterSpec for ACTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`actlr::R`](R) reader structure"]
impl crate::Readable for ACTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`actlr::W`](W) writer structure"]
impl crate::Writable for ACTLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACTLR to value 0"]
impl crate::Resettable for ACTLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
