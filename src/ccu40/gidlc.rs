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
#[doc = "Field `CS0I` writer - CC40 IDLE mode clear"]
pub struct CS0I_W<'a> {
    w: &'a mut W,
}
impl<'a> CS0I_W<'a> {
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
#[doc = "Field `CS1I` writer - CC41 IDLE mode clear"]
pub struct CS1I_W<'a> {
    w: &'a mut W,
}
impl<'a> CS1I_W<'a> {
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
#[doc = "Field `CS2I` writer - CC42 IDLE mode clear"]
pub struct CS2I_W<'a> {
    w: &'a mut W,
}
impl<'a> CS2I_W<'a> {
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
#[doc = "Field `CS3I` writer - CC43 IDLE mode clear"]
pub struct CS3I_W<'a> {
    w: &'a mut W,
}
impl<'a> CS3I_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `SPRB` writer - Prescaler Run Bit Set"]
pub struct SPRB_W<'a> {
    w: &'a mut W,
}
impl<'a> SPRB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - CC40 IDLE mode clear"]
    #[inline(always)]
    pub fn cs0i(&mut self) -> CS0I_W {
        CS0I_W { w: self }
    }
    #[doc = "Bit 1 - CC41 IDLE mode clear"]
    #[inline(always)]
    pub fn cs1i(&mut self) -> CS1I_W {
        CS1I_W { w: self }
    }
    #[doc = "Bit 2 - CC42 IDLE mode clear"]
    #[inline(always)]
    pub fn cs2i(&mut self) -> CS2I_W {
        CS2I_W { w: self }
    }
    #[doc = "Bit 3 - CC43 IDLE mode clear"]
    #[inline(always)]
    pub fn cs3i(&mut self) -> CS3I_W {
        CS3I_W { w: self }
    }
    #[doc = "Bit 8 - Prescaler Run Bit Set"]
    #[inline(always)]
    pub fn sprb(&mut self) -> SPRB_W {
        SPRB_W { w: self }
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
}
#[doc = "`reset()` method sets GIDLC to value 0"]
impl crate::Resettable for GIDLC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
