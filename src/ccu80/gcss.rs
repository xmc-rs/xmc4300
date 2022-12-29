#[doc = "Register `GCSS` writer"]
pub struct W(crate::W<GCSS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCSS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GCSS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCSS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `S0SE` writer - Slice 0 shadow transfer set enable"]
pub type S0SE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSS_SPEC, bool, O>;
#[doc = "Field `S0DSE` writer - Slice 0 Dither shadow transfer set enable"]
pub type S0DSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSS_SPEC, bool, O>;
#[doc = "Field `S0PSE` writer - Slice 0 Prescaler shadow transfer set enable"]
pub type S0PSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSS_SPEC, bool, O>;
#[doc = "Field `S1SE` writer - Slice 1 shadow transfer set enable"]
pub type S1SE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSS_SPEC, bool, O>;
#[doc = "Field `S1DSE` writer - Slice 1 Dither shadow transfer set enable"]
pub type S1DSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSS_SPEC, bool, O>;
#[doc = "Field `S1PSE` writer - Slice 1 Prescaler shadow transfer set enable"]
pub type S1PSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSS_SPEC, bool, O>;
#[doc = "Field `S2SE` writer - Slice 2 shadow transfer set enable"]
pub type S2SE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSS_SPEC, bool, O>;
#[doc = "Field `S2DSE` writer - Slice 2 Dither shadow transfer set enable"]
pub type S2DSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSS_SPEC, bool, O>;
#[doc = "Field `S2PSE` writer - Slice 2 Prescaler shadow transfer set enable"]
pub type S2PSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSS_SPEC, bool, O>;
#[doc = "Field `S3SE` writer - Slice 3 shadow transfer set enable"]
pub type S3SE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSS_SPEC, bool, O>;
#[doc = "Field `S3DSE` writer - Slice 3 Dither shadow transfer set enable"]
pub type S3DSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSS_SPEC, bool, O>;
#[doc = "Field `S3PSE` writer - Slice 3 Prescaler shadow transfer set enable"]
pub type S3PSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSS_SPEC, bool, O>;
#[doc = "Field `S0ST1S` writer - Slice 0 status bit 1 set"]
pub type S0ST1S_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSS_SPEC, bool, O>;
#[doc = "Field `S1ST1S` writer - Slice 1 status bit 1 set"]
pub type S1ST1S_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSS_SPEC, bool, O>;
#[doc = "Field `S2ST1S` writer - Slice 2 status bit 1 set"]
pub type S2ST1S_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSS_SPEC, bool, O>;
#[doc = "Field `S3ST1S` writer - Slice 3 status bit 1 set"]
pub type S3ST1S_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSS_SPEC, bool, O>;
#[doc = "Field `S0ST2S` writer - Slice 0 status bit 2 set"]
pub type S0ST2S_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSS_SPEC, bool, O>;
#[doc = "Field `S1ST2S` writer - Slice 1 status bit 2 set"]
pub type S1ST2S_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSS_SPEC, bool, O>;
#[doc = "Field `S2ST2S` writer - Slice 2 status bit 2 set"]
pub type S2ST2S_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSS_SPEC, bool, O>;
#[doc = "Field `S3ST2S` writer - Slice 3 status bit 2 set"]
pub type S3ST2S_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSS_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Slice 0 shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s0se(&mut self) -> S0SE_W<0> {
        S0SE_W::new(self)
    }
    #[doc = "Bit 1 - Slice 0 Dither shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s0dse(&mut self) -> S0DSE_W<1> {
        S0DSE_W::new(self)
    }
    #[doc = "Bit 2 - Slice 0 Prescaler shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s0pse(&mut self) -> S0PSE_W<2> {
        S0PSE_W::new(self)
    }
    #[doc = "Bit 4 - Slice 1 shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s1se(&mut self) -> S1SE_W<4> {
        S1SE_W::new(self)
    }
    #[doc = "Bit 5 - Slice 1 Dither shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s1dse(&mut self) -> S1DSE_W<5> {
        S1DSE_W::new(self)
    }
    #[doc = "Bit 6 - Slice 1 Prescaler shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s1pse(&mut self) -> S1PSE_W<6> {
        S1PSE_W::new(self)
    }
    #[doc = "Bit 8 - Slice 2 shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s2se(&mut self) -> S2SE_W<8> {
        S2SE_W::new(self)
    }
    #[doc = "Bit 9 - Slice 2 Dither shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s2dse(&mut self) -> S2DSE_W<9> {
        S2DSE_W::new(self)
    }
    #[doc = "Bit 10 - Slice 2 Prescaler shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s2pse(&mut self) -> S2PSE_W<10> {
        S2PSE_W::new(self)
    }
    #[doc = "Bit 12 - Slice 3 shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s3se(&mut self) -> S3SE_W<12> {
        S3SE_W::new(self)
    }
    #[doc = "Bit 13 - Slice 3 Dither shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s3dse(&mut self) -> S3DSE_W<13> {
        S3DSE_W::new(self)
    }
    #[doc = "Bit 14 - Slice 3 Prescaler shadow transfer set enable"]
    #[inline(always)]
    #[must_use]
    pub fn s3pse(&mut self) -> S3PSE_W<14> {
        S3PSE_W::new(self)
    }
    #[doc = "Bit 16 - Slice 0 status bit 1 set"]
    #[inline(always)]
    #[must_use]
    pub fn s0st1s(&mut self) -> S0ST1S_W<16> {
        S0ST1S_W::new(self)
    }
    #[doc = "Bit 17 - Slice 1 status bit 1 set"]
    #[inline(always)]
    #[must_use]
    pub fn s1st1s(&mut self) -> S1ST1S_W<17> {
        S1ST1S_W::new(self)
    }
    #[doc = "Bit 18 - Slice 2 status bit 1 set"]
    #[inline(always)]
    #[must_use]
    pub fn s2st1s(&mut self) -> S2ST1S_W<18> {
        S2ST1S_W::new(self)
    }
    #[doc = "Bit 19 - Slice 3 status bit 1 set"]
    #[inline(always)]
    #[must_use]
    pub fn s3st1s(&mut self) -> S3ST1S_W<19> {
        S3ST1S_W::new(self)
    }
    #[doc = "Bit 20 - Slice 0 status bit 2 set"]
    #[inline(always)]
    #[must_use]
    pub fn s0st2s(&mut self) -> S0ST2S_W<20> {
        S0ST2S_W::new(self)
    }
    #[doc = "Bit 21 - Slice 1 status bit 2 set"]
    #[inline(always)]
    #[must_use]
    pub fn s1st2s(&mut self) -> S1ST2S_W<21> {
        S1ST2S_W::new(self)
    }
    #[doc = "Bit 22 - Slice 2 status bit 2 set"]
    #[inline(always)]
    #[must_use]
    pub fn s2st2s(&mut self) -> S2ST2S_W<22> {
        S2ST2S_W::new(self)
    }
    #[doc = "Bit 23 - Slice 3 status bit 2 set"]
    #[inline(always)]
    #[must_use]
    pub fn s3st2s(&mut self) -> S3ST2S_W<23> {
        S3ST2S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Channel Set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcss](index.html) module"]
pub struct GCSS_SPEC;
impl crate::RegisterSpec for GCSS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gcss::W](W) writer structure"]
impl crate::Writable for GCSS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCSS to value 0"]
impl crate::Resettable for GCSS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
