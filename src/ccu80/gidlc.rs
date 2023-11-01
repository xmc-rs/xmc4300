#[doc = "Register `GIDLC` writer"]
pub type W = crate::W<GIDLC_SPEC>;
#[doc = "Field `CS0I` writer - CC80 IDLE mode clear"]
pub type CS0I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CS1I` writer - CC81 IDLE mode clear"]
pub type CS1I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CS2I` writer - CC82 IDLE mode clear"]
pub type CS2I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CS3I` writer - CC83 IDLE mode clear"]
pub type CS3I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPRB` writer - Prescaler Run Bit Set"]
pub type SPRB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPCH` writer - Parity Checker run bit set"]
pub type SPCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - CC80 IDLE mode clear"]
    #[inline(always)]
    #[must_use]
    pub fn cs0i(&mut self) -> CS0I_W<GIDLC_SPEC, 0> {
        CS0I_W::new(self)
    }
    #[doc = "Bit 1 - CC81 IDLE mode clear"]
    #[inline(always)]
    #[must_use]
    pub fn cs1i(&mut self) -> CS1I_W<GIDLC_SPEC, 1> {
        CS1I_W::new(self)
    }
    #[doc = "Bit 2 - CC82 IDLE mode clear"]
    #[inline(always)]
    #[must_use]
    pub fn cs2i(&mut self) -> CS2I_W<GIDLC_SPEC, 2> {
        CS2I_W::new(self)
    }
    #[doc = "Bit 3 - CC83 IDLE mode clear"]
    #[inline(always)]
    #[must_use]
    pub fn cs3i(&mut self) -> CS3I_W<GIDLC_SPEC, 3> {
        CS3I_W::new(self)
    }
    #[doc = "Bit 8 - Prescaler Run Bit Set"]
    #[inline(always)]
    #[must_use]
    pub fn sprb(&mut self) -> SPRB_W<GIDLC_SPEC, 8> {
        SPRB_W::new(self)
    }
    #[doc = "Bit 10 - Parity Checker run bit set"]
    #[inline(always)]
    #[must_use]
    pub fn spch(&mut self) -> SPCH_W<GIDLC_SPEC, 10> {
        SPCH_W::new(self)
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
#[doc = "Global Idle Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gidlc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GIDLC_SPEC;
impl crate::RegisterSpec for GIDLC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gidlc::W`](W) writer structure"]
impl crate::Writable for GIDLC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GIDLC to value 0"]
impl crate::Resettable for GIDLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
