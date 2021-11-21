#[doc = "Register `TCCLR` writer"]
pub struct W(crate::W<TCCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCLR_SPEC>;
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
impl From<crate::W<TCCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRBC` writer - Timer Run Bit Clear"]
pub struct TRBC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRBC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `TCC` writer - Timer Clear"]
pub struct TCC_W<'a> {
    w: &'a mut W,
}
impl<'a> TCC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `DITC` writer - Dither Counter Clear"]
pub struct DITC_W<'a> {
    w: &'a mut W,
}
impl<'a> DITC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Timer Run Bit Clear"]
    #[inline(always)]
    pub fn trbc(&mut self) -> TRBC_W {
        TRBC_W { w: self }
    }
    #[doc = "Bit 1 - Timer Clear"]
    #[inline(always)]
    pub fn tcc(&mut self) -> TCC_W {
        TCC_W { w: self }
    }
    #[doc = "Bit 2 - Dither Counter Clear"]
    #[inline(always)]
    pub fn ditc(&mut self) -> DITC_W {
        DITC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slice Timer Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcclr](index.html) module"]
pub struct TCCLR_SPEC;
impl crate::RegisterSpec for TCCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tcclr::W](W) writer structure"]
impl crate::Writable for TCCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCCLR to value 0"]
impl crate::Resettable for TCCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
