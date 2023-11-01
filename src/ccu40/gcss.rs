#[doc = "Register `GCSS` writer"]
pub type W = crate::W<GCSS_SPEC>;
#[doc = "Field `S0SE` writer - Slice 0 shadow transfer set enable"]
pub type S0SE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `S0DSE` writer - Slice 0 Dither shadow transfer set enable"]
pub type S0DSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `S0PSE` writer - Slice 0 Prescaler shadow transfer set enable"]
pub type S0PSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `S1SE` writer - Slice 1 shadow transfer set enable"]
pub type S1SE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `S1DSE` writer - Slice 1 Dither shadow transfer set enable"]
pub type S1DSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `S1PSE` writer - Slice 1 Prescaler shadow transfer set enable"]
pub type S1PSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `S2SE` writer - Slice 2 shadow transfer set enable"]
pub type S2SE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `S2DSE` writer - Slice 2 Dither shadow transfer set enable"]
pub type S2DSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `S2PSE` writer - Slice 2 Prescaler shadow transfer set enable"]
pub type S2PSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `S3SE` writer - Slice 3 shadow transfer set enable"]
pub type S3SE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `S3DSE` writer - Slice 3 Dither shadow transfer set enable"]
pub type S3DSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `S3PSE` writer - Slice 3 Prescaler shadow transfer set enable"]
pub type S3PSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `S0STS` writer - Slice 0 status bit set"]
pub type S0STS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `S1STS` writer - Slice 1 status bit set"]
pub type S1STS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `S2STS` writer - Slice 2 status bit set"]
pub type S2STS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `S3STS` writer - Slice 3 status bit set"]
pub type S3STS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Slice 0 shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s0se(&mut self) -> S0SE_W<GCSS_SPEC, 0> {
        S0SE_W::new(self)
    }
    #[doc = "Bit 1 - Slice 0 Dither shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s0dse(&mut self) -> S0DSE_W<GCSS_SPEC, 1> {
        S0DSE_W::new(self)
    }
    #[doc = "Bit 2 - Slice 0 Prescaler shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s0pse(&mut self) -> S0PSE_W<GCSS_SPEC, 2> {
        S0PSE_W::new(self)
    }
    #[doc = "Bit 4 - Slice 1 shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s1se(&mut self) -> S1SE_W<GCSS_SPEC, 4> {
        S1SE_W::new(self)
    }
    #[doc = "Bit 5 - Slice 1 Dither shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s1dse(&mut self) -> S1DSE_W<GCSS_SPEC, 5> {
        S1DSE_W::new(self)
    }
    #[doc = "Bit 6 - Slice 1 Prescaler shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s1pse(&mut self) -> S1PSE_W<GCSS_SPEC, 6> {
        S1PSE_W::new(self)
    }
    #[doc = "Bit 8 - Slice 2 shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s2se(&mut self) -> S2SE_W<GCSS_SPEC, 8> {
        S2SE_W::new(self)
    }
    #[doc = "Bit 9 - Slice 2 Dither shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s2dse(&mut self) -> S2DSE_W<GCSS_SPEC, 9> {
        S2DSE_W::new(self)
    }
    #[doc = "Bit 10 - Slice 2 Prescaler shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s2pse(&mut self) -> S2PSE_W<GCSS_SPEC, 10> {
        S2PSE_W::new(self)
    }
    #[doc = "Bit 12 - Slice 3 shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s3se(&mut self) -> S3SE_W<GCSS_SPEC, 12> {
        S3SE_W::new(self)
    }
    #[doc = "Bit 13 - Slice 3 Dither shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s3dse(&mut self) -> S3DSE_W<GCSS_SPEC, 13> {
        S3DSE_W::new(self)
    }
    #[doc = "Bit 14 - Slice 3 Prescaler shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s3pse(&mut self) -> S3PSE_W<GCSS_SPEC, 14> {
        S3PSE_W::new(self)
    }
    #[doc = "Bit 16 - Slice 0 status bit set"]
    #[inline(always)]
    #[must_use]
    pub fn s0sts(&mut self) -> S0STS_W<GCSS_SPEC, 16> {
        S0STS_W::new(self)
    }
    #[doc = "Bit 17 - Slice 1 status bit set"]
    #[inline(always)]
    #[must_use]
    pub fn s1sts(&mut self) -> S1STS_W<GCSS_SPEC, 17> {
        S1STS_W::new(self)
    }
    #[doc = "Bit 18 - Slice 2 status bit set"]
    #[inline(always)]
    #[must_use]
    pub fn s2sts(&mut self) -> S2STS_W<GCSS_SPEC, 18> {
        S2STS_W::new(self)
    }
    #[doc = "Bit 19 - Slice 3 status bit set"]
    #[inline(always)]
    #[must_use]
    pub fn s3sts(&mut self) -> S3STS_W<GCSS_SPEC, 19> {
        S3STS_W::new(self)
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
#[doc = "Global Channel Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcss::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCSS_SPEC;
impl crate::RegisterSpec for GCSS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gcss::W`](W) writer structure"]
impl crate::Writable for GCSS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCSS to value 0"]
impl crate::Resettable for GCSS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
