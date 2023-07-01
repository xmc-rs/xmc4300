#[doc = "Register `TCSET` writer"]
pub struct W(crate::W<TCSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCSET_SPEC>;
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
impl From<crate::W<TCSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRBS` writer - Timer Run Bit set"]
pub type TRBS_W<'a, const O: u8> = crate::BitWriter<'a, TCSET_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Timer Run Bit set"]
    #[inline(always)]
    #[must_use]
    pub fn trbs(&mut self) -> TRBS_W<0> {
        TRBS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slice Timer Run Set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcset](index.html) module"]
pub struct TCSET_SPEC;
impl crate::RegisterSpec for TCSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tcset::W](W) writer structure"]
impl crate::Writable for TCSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCSET to value 0"]
impl crate::Resettable for TCSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
