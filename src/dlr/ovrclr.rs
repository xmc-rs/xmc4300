#[doc = "Register `OVRCLR` writer"]
pub struct W(crate::W<OVRCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OVRCLR_SPEC>;
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
impl From<crate::W<OVRCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OVRCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LN0` writer - Line 0 Overrun Status Clear"]
pub type LN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OVRCLR_SPEC, bool, O>;
#[doc = "Field `LN1` writer - Line 1 Overrun Status Clear"]
pub type LN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OVRCLR_SPEC, bool, O>;
#[doc = "Field `LN2` writer - Line 2 Overrun Status Clear"]
pub type LN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, OVRCLR_SPEC, bool, O>;
#[doc = "Field `LN3` writer - Line 3 Overrun Status Clear"]
pub type LN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, OVRCLR_SPEC, bool, O>;
#[doc = "Field `LN4` writer - Line 4 Overrun Status Clear"]
pub type LN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, OVRCLR_SPEC, bool, O>;
#[doc = "Field `LN5` writer - Line 5 Overrun Status Clear"]
pub type LN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, OVRCLR_SPEC, bool, O>;
#[doc = "Field `LN6` writer - Line 6 Overrun Status Clear"]
pub type LN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, OVRCLR_SPEC, bool, O>;
#[doc = "Field `LN7` writer - Line 7 Overrun Status Clear"]
pub type LN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, OVRCLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Line 0 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln0(&mut self) -> LN0_W<0> {
        LN0_W::new(self)
    }
    #[doc = "Bit 1 - Line 1 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln1(&mut self) -> LN1_W<1> {
        LN1_W::new(self)
    }
    #[doc = "Bit 2 - Line 2 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln2(&mut self) -> LN2_W<2> {
        LN2_W::new(self)
    }
    #[doc = "Bit 3 - Line 3 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln3(&mut self) -> LN3_W<3> {
        LN3_W::new(self)
    }
    #[doc = "Bit 4 - Line 4 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln4(&mut self) -> LN4_W<4> {
        LN4_W::new(self)
    }
    #[doc = "Bit 5 - Line 5 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln5(&mut self) -> LN5_W<5> {
        LN5_W::new(self)
    }
    #[doc = "Bit 6 - Line 6 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln6(&mut self) -> LN6_W<6> {
        LN6_W::new(self)
    }
    #[doc = "Bit 7 - Line 7 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln7(&mut self) -> LN7_W<7> {
        LN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Overrun Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ovrclr](index.html) module"]
pub struct OVRCLR_SPEC;
impl crate::RegisterSpec for OVRCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ovrclr::W](W) writer structure"]
impl crate::Writable for OVRCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OVRCLR to value 0"]
impl crate::Resettable for OVRCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
