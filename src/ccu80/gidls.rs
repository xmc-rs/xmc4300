#[doc = "Register `GIDLS` writer"]
pub struct W(crate::W<GIDLS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GIDLS_SPEC>;
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
impl From<crate::W<GIDLS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GIDLS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SS0I` writer - CC80 IDLE mode set"]
pub struct SS0I_W<'a> {
    w: &'a mut W,
}
impl<'a> SS0I_W<'a> {
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
#[doc = "Field `SS1I` writer - CC81 IDLE mode set"]
pub struct SS1I_W<'a> {
    w: &'a mut W,
}
impl<'a> SS1I_W<'a> {
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
#[doc = "Field `SS2I` writer - CC82 IDLE mode set"]
pub struct SS2I_W<'a> {
    w: &'a mut W,
}
impl<'a> SS2I_W<'a> {
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
#[doc = "Field `SS3I` writer - CC83 IDLE mode set"]
pub struct SS3I_W<'a> {
    w: &'a mut W,
}
impl<'a> SS3I_W<'a> {
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
#[doc = "Field `CPRB` writer - Prescaler# Run Bit Clear"]
pub struct CPRB_W<'a> {
    w: &'a mut W,
}
impl<'a> CPRB_W<'a> {
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
#[doc = "Field `PSIC` writer - Prescaler clear"]
pub struct PSIC_W<'a> {
    w: &'a mut W,
}
impl<'a> PSIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CPCH` writer - Parity Checker Run bit clear"]
pub struct CPCH_W<'a> {
    w: &'a mut W,
}
impl<'a> CPCH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - CC80 IDLE mode set"]
    #[inline(always)]
    pub fn ss0i(&mut self) -> SS0I_W {
        SS0I_W { w: self }
    }
    #[doc = "Bit 1 - CC81 IDLE mode set"]
    #[inline(always)]
    pub fn ss1i(&mut self) -> SS1I_W {
        SS1I_W { w: self }
    }
    #[doc = "Bit 2 - CC82 IDLE mode set"]
    #[inline(always)]
    pub fn ss2i(&mut self) -> SS2I_W {
        SS2I_W { w: self }
    }
    #[doc = "Bit 3 - CC83 IDLE mode set"]
    #[inline(always)]
    pub fn ss3i(&mut self) -> SS3I_W {
        SS3I_W { w: self }
    }
    #[doc = "Bit 8 - Prescaler# Run Bit Clear"]
    #[inline(always)]
    pub fn cprb(&mut self) -> CPRB_W {
        CPRB_W { w: self }
    }
    #[doc = "Bit 9 - Prescaler clear"]
    #[inline(always)]
    pub fn psic(&mut self) -> PSIC_W {
        PSIC_W { w: self }
    }
    #[doc = "Bit 10 - Parity Checker Run bit clear"]
    #[inline(always)]
    pub fn cpch(&mut self) -> CPCH_W {
        CPCH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Idle Set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gidls](index.html) module"]
pub struct GIDLS_SPEC;
impl crate::RegisterSpec for GIDLS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gidls::W](W) writer structure"]
impl crate::Writable for GIDLS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GIDLS to value 0"]
impl crate::Resettable for GIDLS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
