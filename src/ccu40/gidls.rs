#[doc = "Register `GIDLS` writer"]
pub type W = crate::W<GidlsSpec>;
#[doc = "Field `SS0I` writer - CC40 IDLE mode set"]
pub type Ss0iW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SS1I` writer - CC41 IDLE mode set"]
pub type Ss1iW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SS2I` writer - CC42 IDLE mode set"]
pub type Ss2iW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SS3I` writer - CC43 IDLE mode set"]
pub type Ss3iW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPRB` writer - Prescaler Run Bit Clear"]
pub type CprbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSIC` writer - Prescaler clear"]
pub type PsicW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - CC40 IDLE mode set"]
    #[inline(always)]
    #[must_use]
    pub fn ss0i(&mut self) -> Ss0iW<GidlsSpec> {
        Ss0iW::new(self, 0)
    }
    #[doc = "Bit 1 - CC41 IDLE mode set"]
    #[inline(always)]
    #[must_use]
    pub fn ss1i(&mut self) -> Ss1iW<GidlsSpec> {
        Ss1iW::new(self, 1)
    }
    #[doc = "Bit 2 - CC42 IDLE mode set"]
    #[inline(always)]
    #[must_use]
    pub fn ss2i(&mut self) -> Ss2iW<GidlsSpec> {
        Ss2iW::new(self, 2)
    }
    #[doc = "Bit 3 - CC43 IDLE mode set"]
    #[inline(always)]
    #[must_use]
    pub fn ss3i(&mut self) -> Ss3iW<GidlsSpec> {
        Ss3iW::new(self, 3)
    }
    #[doc = "Bit 8 - Prescaler Run Bit Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cprb(&mut self) -> CprbW<GidlsSpec> {
        CprbW::new(self, 8)
    }
    #[doc = "Bit 9 - Prescaler clear"]
    #[inline(always)]
    #[must_use]
    pub fn psic(&mut self) -> PsicW<GidlsSpec> {
        PsicW::new(self, 9)
    }
}
#[doc = "Global Idle Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gidls::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GidlsSpec;
impl crate::RegisterSpec for GidlsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gidls::W`](W) writer structure"]
impl crate::Writable for GidlsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIDLS to value 0"]
impl crate::Resettable for GidlsSpec {
    const RESET_VALUE: u32 = 0;
}
