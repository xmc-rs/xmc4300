#[doc = "Register `GCSS` writer"]
pub type W = crate::W<GcssSpec>;
#[doc = "Field `S0SE` writer - Slice 0 shadow transfer set enable"]
pub type S0seW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S0DSE` writer - Slice 0 Dither shadow transfer set enable"]
pub type S0dseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S0PSE` writer - Slice 0 Prescaler shadow transfer set enable"]
pub type S0pseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1SE` writer - Slice 1 shadow transfer set enable"]
pub type S1seW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1DSE` writer - Slice 1 Dither shadow transfer set enable"]
pub type S1dseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1PSE` writer - Slice 1 Prescaler shadow transfer set enable"]
pub type S1pseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S2SE` writer - Slice 2 shadow transfer set enable"]
pub type S2seW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S2DSE` writer - Slice 2 Dither shadow transfer set enable"]
pub type S2dseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S2PSE` writer - Slice 2 Prescaler shadow transfer set enable"]
pub type S2pseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S3SE` writer - Slice 3 shadow transfer set enable"]
pub type S3seW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S3DSE` writer - Slice 3 Dither shadow transfer set enable"]
pub type S3dseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S3PSE` writer - Slice 3 Prescaler shadow transfer set enable"]
pub type S3pseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S0STS` writer - Slice 0 status bit set"]
pub type S0stsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1STS` writer - Slice 1 status bit set"]
pub type S1stsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S2STS` writer - Slice 2 status bit set"]
pub type S2stsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S3STS` writer - Slice 3 status bit set"]
pub type S3stsW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Slice 0 shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s0se(&mut self) -> S0seW<GcssSpec> {
        S0seW::new(self, 0)
    }
    #[doc = "Bit 1 - Slice 0 Dither shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s0dse(&mut self) -> S0dseW<GcssSpec> {
        S0dseW::new(self, 1)
    }
    #[doc = "Bit 2 - Slice 0 Prescaler shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s0pse(&mut self) -> S0pseW<GcssSpec> {
        S0pseW::new(self, 2)
    }
    #[doc = "Bit 4 - Slice 1 shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s1se(&mut self) -> S1seW<GcssSpec> {
        S1seW::new(self, 4)
    }
    #[doc = "Bit 5 - Slice 1 Dither shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s1dse(&mut self) -> S1dseW<GcssSpec> {
        S1dseW::new(self, 5)
    }
    #[doc = "Bit 6 - Slice 1 Prescaler shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s1pse(&mut self) -> S1pseW<GcssSpec> {
        S1pseW::new(self, 6)
    }
    #[doc = "Bit 8 - Slice 2 shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s2se(&mut self) -> S2seW<GcssSpec> {
        S2seW::new(self, 8)
    }
    #[doc = "Bit 9 - Slice 2 Dither shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s2dse(&mut self) -> S2dseW<GcssSpec> {
        S2dseW::new(self, 9)
    }
    #[doc = "Bit 10 - Slice 2 Prescaler shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s2pse(&mut self) -> S2pseW<GcssSpec> {
        S2pseW::new(self, 10)
    }
    #[doc = "Bit 12 - Slice 3 shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s3se(&mut self) -> S3seW<GcssSpec> {
        S3seW::new(self, 12)
    }
    #[doc = "Bit 13 - Slice 3 Dither shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s3dse(&mut self) -> S3dseW<GcssSpec> {
        S3dseW::new(self, 13)
    }
    #[doc = "Bit 14 - Slice 3 Prescaler shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s3pse(&mut self) -> S3pseW<GcssSpec> {
        S3pseW::new(self, 14)
    }
    #[doc = "Bit 16 - Slice 0 status bit set"]
    #[inline(always)]
    #[must_use]
    pub fn s0sts(&mut self) -> S0stsW<GcssSpec> {
        S0stsW::new(self, 16)
    }
    #[doc = "Bit 17 - Slice 1 status bit set"]
    #[inline(always)]
    #[must_use]
    pub fn s1sts(&mut self) -> S1stsW<GcssSpec> {
        S1stsW::new(self, 17)
    }
    #[doc = "Bit 18 - Slice 2 status bit set"]
    #[inline(always)]
    #[must_use]
    pub fn s2sts(&mut self) -> S2stsW<GcssSpec> {
        S2stsW::new(self, 18)
    }
    #[doc = "Bit 19 - Slice 3 status bit set"]
    #[inline(always)]
    #[must_use]
    pub fn s3sts(&mut self) -> S3stsW<GcssSpec> {
        S3stsW::new(self, 19)
    }
}
#[doc = "Global Channel Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcss::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcssSpec;
impl crate::RegisterSpec for GcssSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gcss::W`](W) writer structure"]
impl crate::Writable for GcssSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCSS to value 0"]
impl crate::Resettable for GcssSpec {
    const RESET_VALUE: u32 = 0;
}
