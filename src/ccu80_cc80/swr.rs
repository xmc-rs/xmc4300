#[doc = "Register `SWR` writer"]
pub struct W(crate::W<SWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWR_SPEC>;
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
impl From<crate::W<SWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPM` writer - Period match while counting up clear"]
pub struct RPM_W<'a> {
    w: &'a mut W,
}
impl<'a> RPM_W<'a> {
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
#[doc = "Field `ROM` writer - One match while counting down clear"]
pub struct ROM_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_W<'a> {
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
#[doc = "Field `RCM1U` writer - Channel 1 Compare match while counting up clear"]
pub struct RCM1U_W<'a> {
    w: &'a mut W,
}
impl<'a> RCM1U_W<'a> {
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
#[doc = "Field `RCM1D` writer - Channel 1 Compare match while counting down clear"]
pub struct RCM1D_W<'a> {
    w: &'a mut W,
}
impl<'a> RCM1D_W<'a> {
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
#[doc = "Field `RCM2U` writer - Channel 2 Compare match while counting up clear"]
pub struct RCM2U_W<'a> {
    w: &'a mut W,
}
impl<'a> RCM2U_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `RCM2D` writer - Channel 2 Compare match while counting down clear"]
pub struct RCM2D_W<'a> {
    w: &'a mut W,
}
impl<'a> RCM2D_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RE0A` writer - Event 0 detection clear"]
pub struct RE0A_W<'a> {
    w: &'a mut W,
}
impl<'a> RE0A_W<'a> {
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
#[doc = "Field `RE1A` writer - Event 1 detection clear"]
pub struct RE1A_W<'a> {
    w: &'a mut W,
}
impl<'a> RE1A_W<'a> {
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
#[doc = "Field `RE2A` writer - Event 2 detection clear"]
pub struct RE2A_W<'a> {
    w: &'a mut W,
}
impl<'a> RE2A_W<'a> {
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
#[doc = "Field `RTRPF` writer - Trap Flag status clear"]
pub struct RTRPF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTRPF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Period match while counting up clear"]
    #[inline(always)]
    pub fn rpm(&mut self) -> RPM_W {
        RPM_W { w: self }
    }
    #[doc = "Bit 1 - One match while counting down clear"]
    #[inline(always)]
    pub fn rom(&mut self) -> ROM_W {
        ROM_W { w: self }
    }
    #[doc = "Bit 2 - Channel 1 Compare match while counting up clear"]
    #[inline(always)]
    pub fn rcm1u(&mut self) -> RCM1U_W {
        RCM1U_W { w: self }
    }
    #[doc = "Bit 3 - Channel 1 Compare match while counting down clear"]
    #[inline(always)]
    pub fn rcm1d(&mut self) -> RCM1D_W {
        RCM1D_W { w: self }
    }
    #[doc = "Bit 4 - Channel 2 Compare match while counting up clear"]
    #[inline(always)]
    pub fn rcm2u(&mut self) -> RCM2U_W {
        RCM2U_W { w: self }
    }
    #[doc = "Bit 5 - Channel 2 Compare match while counting down clear"]
    #[inline(always)]
    pub fn rcm2d(&mut self) -> RCM2D_W {
        RCM2D_W { w: self }
    }
    #[doc = "Bit 8 - Event 0 detection clear"]
    #[inline(always)]
    pub fn re0a(&mut self) -> RE0A_W {
        RE0A_W { w: self }
    }
    #[doc = "Bit 9 - Event 1 detection clear"]
    #[inline(always)]
    pub fn re1a(&mut self) -> RE1A_W {
        RE1A_W { w: self }
    }
    #[doc = "Bit 10 - Event 2 detection clear"]
    #[inline(always)]
    pub fn re2a(&mut self) -> RE2A_W {
        RE2A_W { w: self }
    }
    #[doc = "Bit 11 - Trap Flag status clear"]
    #[inline(always)]
    pub fn rtrpf(&mut self) -> RTRPF_W {
        RTRPF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swr](index.html) module"]
pub struct SWR_SPEC;
impl crate::RegisterSpec for SWR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [swr::W](W) writer structure"]
impl crate::Writable for SWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWR to value 0"]
impl crate::Resettable for SWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
