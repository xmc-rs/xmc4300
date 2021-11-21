#[doc = "Register `SWS` writer"]
pub struct W(crate::W<SWS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWS_SPEC>;
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
impl From<crate::W<SWS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPM` writer - Period match while counting up set"]
pub struct SPM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPM_W<'a> {
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
#[doc = "Field `SOM` writer - One match while counting down set"]
pub struct SOM_W<'a> {
    w: &'a mut W,
}
impl<'a> SOM_W<'a> {
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
#[doc = "Field `SCM1U` writer - Channel 1 Compare match while counting up set"]
pub struct SCM1U_W<'a> {
    w: &'a mut W,
}
impl<'a> SCM1U_W<'a> {
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
#[doc = "Field `SCM1D` writer - Channel 1 Compare match while counting down set"]
pub struct SCM1D_W<'a> {
    w: &'a mut W,
}
impl<'a> SCM1D_W<'a> {
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
#[doc = "Field `SCM2U` writer - Compare match while counting up set"]
pub struct SCM2U_W<'a> {
    w: &'a mut W,
}
impl<'a> SCM2U_W<'a> {
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
#[doc = "Field `SCM2D` writer - Compare match while counting down set"]
pub struct SCM2D_W<'a> {
    w: &'a mut W,
}
impl<'a> SCM2D_W<'a> {
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
#[doc = "Field `SE0A` writer - Event 0 detection set"]
pub struct SE0A_W<'a> {
    w: &'a mut W,
}
impl<'a> SE0A_W<'a> {
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
#[doc = "Field `SE1A` writer - Event 1 detection set"]
pub struct SE1A_W<'a> {
    w: &'a mut W,
}
impl<'a> SE1A_W<'a> {
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
#[doc = "Field `SE2A` writer - Event 2 detection set"]
pub struct SE2A_W<'a> {
    w: &'a mut W,
}
impl<'a> SE2A_W<'a> {
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
#[doc = "Field `STRPF` writer - Trap Flag status set"]
pub struct STRPF_W<'a> {
    w: &'a mut W,
}
impl<'a> STRPF_W<'a> {
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
    #[doc = "Bit 0 - Period match while counting up set"]
    #[inline(always)]
    pub fn spm(&mut self) -> SPM_W {
        SPM_W { w: self }
    }
    #[doc = "Bit 1 - One match while counting down set"]
    #[inline(always)]
    pub fn som(&mut self) -> SOM_W {
        SOM_W { w: self }
    }
    #[doc = "Bit 2 - Channel 1 Compare match while counting up set"]
    #[inline(always)]
    pub fn scm1u(&mut self) -> SCM1U_W {
        SCM1U_W { w: self }
    }
    #[doc = "Bit 3 - Channel 1 Compare match while counting down set"]
    #[inline(always)]
    pub fn scm1d(&mut self) -> SCM1D_W {
        SCM1D_W { w: self }
    }
    #[doc = "Bit 4 - Compare match while counting up set"]
    #[inline(always)]
    pub fn scm2u(&mut self) -> SCM2U_W {
        SCM2U_W { w: self }
    }
    #[doc = "Bit 5 - Compare match while counting down set"]
    #[inline(always)]
    pub fn scm2d(&mut self) -> SCM2D_W {
        SCM2D_W { w: self }
    }
    #[doc = "Bit 8 - Event 0 detection set"]
    #[inline(always)]
    pub fn se0a(&mut self) -> SE0A_W {
        SE0A_W { w: self }
    }
    #[doc = "Bit 9 - Event 1 detection set"]
    #[inline(always)]
    pub fn se1a(&mut self) -> SE1A_W {
        SE1A_W { w: self }
    }
    #[doc = "Bit 10 - Event 2 detection set"]
    #[inline(always)]
    pub fn se2a(&mut self) -> SE2A_W {
        SE2A_W { w: self }
    }
    #[doc = "Bit 11 - Trap Flag status set"]
    #[inline(always)]
    pub fn strpf(&mut self) -> STRPF_W {
        STRPF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status Set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sws](index.html) module"]
pub struct SWS_SPEC;
impl crate::RegisterSpec for SWS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sws::W](W) writer structure"]
impl crate::Writable for SWS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWS to value 0"]
impl crate::Resettable for SWS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
