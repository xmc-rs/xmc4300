#[doc = "Register `GCSC` writer"]
pub type W = crate::W<GCSC_SPEC>;
#[doc = "Field `S0SC` writer - Slice 0 shadow transfer clear"]
pub type S0SC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S0DSC` writer - Slice 0 Dither shadow transfer clear"]
pub type S0DSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S0PSC` writer - Slice 0 Prescaler shadow transfer clear"]
pub type S0PSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1SC` writer - Slice 1 shadow transfer clear"]
pub type S1SC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1DSC` writer - Slice 1 Dither shadow transfer clear"]
pub type S1DSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1PSC` writer - Slice 1 Prescaler shadow transfer clear"]
pub type S1PSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S2SC` writer - Slice 2 shadow transfer clear"]
pub type S2SC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S2DSC` writer - Slice 2 Dither shadow transfer clear"]
pub type S2DSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S2PSC` writer - Slice 2 Prescaler shadow transfer clear"]
pub type S2PSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S3SC` writer - Slice 3 shadow transfer clear"]
pub type S3SC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S3DSC` writer - Slice 3 Dither shadow transfer clear"]
pub type S3DSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S3PSC` writer - Slice 3 Prescaler shadow transfer clear"]
pub type S3PSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S0STC` writer - Slice 0 status bit clear"]
pub type S0STC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1STC` writer - Slice 1 status bit clear"]
pub type S1STC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S2STC` writer - Slice 2 status bit clear"]
pub type S2STC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S3STC` writer - Slice 3 status bit clear"]
pub type S3STC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Slice 0 shadow transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn s0sc(&mut self) -> S0SC_W<GCSC_SPEC> {
        S0SC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Slice 0 Dither shadow transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn s0dsc(&mut self) -> S0DSC_W<GCSC_SPEC> {
        S0DSC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Slice 0 Prescaler shadow transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn s0psc(&mut self) -> S0PSC_W<GCSC_SPEC> {
        S0PSC_W::new(self, 2)
    }
    #[doc = "Bit 4 - Slice 1 shadow transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn s1sc(&mut self) -> S1SC_W<GCSC_SPEC> {
        S1SC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Slice 1 Dither shadow transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn s1dsc(&mut self) -> S1DSC_W<GCSC_SPEC> {
        S1DSC_W::new(self, 5)
    }
    #[doc = "Bit 6 - Slice 1 Prescaler shadow transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn s1psc(&mut self) -> S1PSC_W<GCSC_SPEC> {
        S1PSC_W::new(self, 6)
    }
    #[doc = "Bit 8 - Slice 2 shadow transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn s2sc(&mut self) -> S2SC_W<GCSC_SPEC> {
        S2SC_W::new(self, 8)
    }
    #[doc = "Bit 9 - Slice 2 Dither shadow transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn s2dsc(&mut self) -> S2DSC_W<GCSC_SPEC> {
        S2DSC_W::new(self, 9)
    }
    #[doc = "Bit 10 - Slice 2 Prescaler shadow transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn s2psc(&mut self) -> S2PSC_W<GCSC_SPEC> {
        S2PSC_W::new(self, 10)
    }
    #[doc = "Bit 12 - Slice 3 shadow transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn s3sc(&mut self) -> S3SC_W<GCSC_SPEC> {
        S3SC_W::new(self, 12)
    }
    #[doc = "Bit 13 - Slice 3 Dither shadow transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn s3dsc(&mut self) -> S3DSC_W<GCSC_SPEC> {
        S3DSC_W::new(self, 13)
    }
    #[doc = "Bit 14 - Slice 3 Prescaler shadow transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn s3psc(&mut self) -> S3PSC_W<GCSC_SPEC> {
        S3PSC_W::new(self, 14)
    }
    #[doc = "Bit 16 - Slice 0 status bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn s0stc(&mut self) -> S0STC_W<GCSC_SPEC> {
        S0STC_W::new(self, 16)
    }
    #[doc = "Bit 17 - Slice 1 status bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn s1stc(&mut self) -> S1STC_W<GCSC_SPEC> {
        S1STC_W::new(self, 17)
    }
    #[doc = "Bit 18 - Slice 2 status bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn s2stc(&mut self) -> S2STC_W<GCSC_SPEC> {
        S2STC_W::new(self, 18)
    }
    #[doc = "Bit 19 - Slice 3 status bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn s3stc(&mut self) -> S3STC_W<GCSC_SPEC> {
        S3STC_W::new(self, 19)
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
#[doc = "Global Channel Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcsc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCSC_SPEC;
impl crate::RegisterSpec for GCSC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gcsc::W`](W) writer structure"]
impl crate::Writable for GCSC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCSC to value 0"]
impl crate::Resettable for GCSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
