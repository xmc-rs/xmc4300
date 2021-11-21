#[doc = "Register `CLRSR` writer"]
pub struct W(crate::W<CLRSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLRSR_SPEC>;
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
impl From<crate::W<CLRSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLRSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPSE` writer - Periodic Seconds Interrupt Clear"]
pub struct RPSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RPSE_W<'a> {
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
#[doc = "Field `RPMI` writer - Periodic Minutes Interrupt Clear"]
pub struct RPMI_W<'a> {
    w: &'a mut W,
}
impl<'a> RPMI_W<'a> {
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
#[doc = "Field `RPHO` writer - Periodic Hours Interrupt Clear"]
pub struct RPHO_W<'a> {
    w: &'a mut W,
}
impl<'a> RPHO_W<'a> {
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
#[doc = "Field `RPDA` writer - Periodic Days Interrupt Clear"]
pub struct RPDA_W<'a> {
    w: &'a mut W,
}
impl<'a> RPDA_W<'a> {
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
#[doc = "Field `RPMO` writer - Periodic Months Interrupt Clear"]
pub struct RPMO_W<'a> {
    w: &'a mut W,
}
impl<'a> RPMO_W<'a> {
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
#[doc = "Field `RPYE` writer - Periodic Years Interrupt Clear"]
pub struct RPYE_W<'a> {
    w: &'a mut W,
}
impl<'a> RPYE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `RAI` writer - Alarm Interrupt Clear"]
pub struct RAI_W<'a> {
    w: &'a mut W,
}
impl<'a> RAI_W<'a> {
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
    #[doc = "Bit 0 - Periodic Seconds Interrupt Clear"]
    #[inline(always)]
    pub fn rpse(&mut self) -> RPSE_W {
        RPSE_W { w: self }
    }
    #[doc = "Bit 1 - Periodic Minutes Interrupt Clear"]
    #[inline(always)]
    pub fn rpmi(&mut self) -> RPMI_W {
        RPMI_W { w: self }
    }
    #[doc = "Bit 2 - Periodic Hours Interrupt Clear"]
    #[inline(always)]
    pub fn rpho(&mut self) -> RPHO_W {
        RPHO_W { w: self }
    }
    #[doc = "Bit 3 - Periodic Days Interrupt Clear"]
    #[inline(always)]
    pub fn rpda(&mut self) -> RPDA_W {
        RPDA_W { w: self }
    }
    #[doc = "Bit 5 - Periodic Months Interrupt Clear"]
    #[inline(always)]
    pub fn rpmo(&mut self) -> RPMO_W {
        RPMO_W { w: self }
    }
    #[doc = "Bit 6 - Periodic Years Interrupt Clear"]
    #[inline(always)]
    pub fn rpye(&mut self) -> RPYE_W {
        RPYE_W { w: self }
    }
    #[doc = "Bit 8 - Alarm Interrupt Clear"]
    #[inline(always)]
    pub fn rai(&mut self) -> RAI_W {
        RAI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Clear Service Request Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clrsr](index.html) module"]
pub struct CLRSR_SPEC;
impl crate::RegisterSpec for CLRSR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clrsr::W](W) writer structure"]
impl crate::Writable for CLRSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLRSR to value 0"]
impl crate::Resettable for CLRSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
