#[doc = "Register `GIDLC` writer"]
pub struct W(crate::W<GIDLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GIDLC_SPEC>;
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
impl From<crate::W<GIDLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GIDLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS0I` writer - CC80 IDLE mode clear"]
pub type CS0I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIDLC_SPEC, bool, O>;
#[doc = "Field `CS1I` writer - CC81 IDLE mode clear"]
pub type CS1I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIDLC_SPEC, bool, O>;
#[doc = "Field `CS2I` writer - CC82 IDLE mode clear"]
pub type CS2I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIDLC_SPEC, bool, O>;
#[doc = "Field `CS3I` writer - CC83 IDLE mode clear"]
pub type CS3I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIDLC_SPEC, bool, O>;
#[doc = "Field `SPRB` writer - Prescaler Run Bit Set"]
pub type SPRB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIDLC_SPEC, bool, O>;
#[doc = "Field `SPCH` writer - Parity Checker run bit set"]
pub type SPCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIDLC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - CC80 IDLE mode clear"]
    #[inline(always)]
    #[must_use]
    pub fn cs0i(&mut self) -> CS0I_W<0> {
        CS0I_W::new(self)
    }
    #[doc = "Bit 1 - CC81 IDLE mode clear"]
    #[inline(always)]
    #[must_use]
    pub fn cs1i(&mut self) -> CS1I_W<1> {
        CS1I_W::new(self)
    }
    #[doc = "Bit 2 - CC82 IDLE mode clear"]
    #[inline(always)]
    #[must_use]
    pub fn cs2i(&mut self) -> CS2I_W<2> {
        CS2I_W::new(self)
    }
    #[doc = "Bit 3 - CC83 IDLE mode clear"]
    #[inline(always)]
    #[must_use]
    pub fn cs3i(&mut self) -> CS3I_W<3> {
        CS3I_W::new(self)
    }
    #[doc = "Bit 8 - Prescaler Run Bit Set"]
    #[inline(always)]
    #[must_use]
    pub fn sprb(&mut self) -> SPRB_W<8> {
        SPRB_W::new(self)
    }
    #[doc = "Bit 10 - Parity Checker run bit set"]
    #[inline(always)]
    #[must_use]
    pub fn spch(&mut self) -> SPCH_W<10> {
        SPCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Idle Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gidlc](index.html) module"]
pub struct GIDLC_SPEC;
impl crate::RegisterSpec for GIDLC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gidlc::W](W) writer structure"]
impl crate::Writable for GIDLC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GIDLC to value 0"]
impl crate::Resettable for GIDLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
