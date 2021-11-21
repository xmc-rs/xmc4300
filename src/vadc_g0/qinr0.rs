#[doc = "Register `QINR0` writer"]
pub struct W(crate::W<QINR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QINR0_SPEC>;
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
impl From<crate::W<QINR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QINR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REQCHNR` writer - Request Channel Number"]
pub struct REQCHNR_W<'a> {
    w: &'a mut W,
}
impl<'a> REQCHNR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Refill\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_AW {
    #[doc = "0: No refill: this queue entry is converted once and then invalidated"]
    VALUE1 = 0,
    #[doc = "1: Automatic refill: this queue entry is automatically reloaded into QINRx when the related conversion is started"]
    VALUE2 = 1,
}
impl From<RF_AW> for bool {
    #[inline(always)]
    fn from(variant: RF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF` writer - Refill"]
pub struct RF_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No refill: this queue entry is converted once and then invalidated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RF_AW::VALUE1)
    }
    #[doc = "Automatic refill: this queue entry is automatically reloaded into QINRx when the related conversion is started"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RF_AW::VALUE2)
    }
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
#[doc = "Enable Source Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENSI_AW {
    #[doc = "0: No request source interrupt"]
    VALUE1 = 0,
    #[doc = "1: A request source event interrupt is generated upon a request source event (related conversion is finished)"]
    VALUE2 = 1,
}
impl From<ENSI_AW> for bool {
    #[inline(always)]
    fn from(variant: ENSI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENSI` writer - Enable Source Interrupt"]
pub struct ENSI_W<'a> {
    w: &'a mut W,
}
impl<'a> ENSI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENSI_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No request source interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENSI_AW::VALUE1)
    }
    #[doc = "A request source event interrupt is generated upon a request source event (related conversion is finished)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENSI_AW::VALUE2)
    }
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
#[doc = "External Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTR_AW {
    #[doc = "0: A valid queue entry immediately leads to a conversion request."]
    VALUE1 = 0,
    #[doc = "1: A valid queue entry waits for a trigger event to occur before issuing a conversion request."]
    VALUE2 = 1,
}
impl From<EXTR_AW> for bool {
    #[inline(always)]
    fn from(variant: EXTR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTR` writer - External Trigger"]
pub struct EXTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A valid queue entry immediately leads to a conversion request."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXTR_AW::VALUE1)
    }
    #[doc = "A valid queue entry waits for a trigger event to occur before issuing a conversion request."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXTR_AW::VALUE2)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:4 - Request Channel Number"]
    #[inline(always)]
    pub fn reqchnr(&mut self) -> REQCHNR_W {
        REQCHNR_W { w: self }
    }
    #[doc = "Bit 5 - Refill"]
    #[inline(always)]
    pub fn rf(&mut self) -> RF_W {
        RF_W { w: self }
    }
    #[doc = "Bit 6 - Enable Source Interrupt"]
    #[inline(always)]
    pub fn ensi(&mut self) -> ENSI_W {
        ENSI_W { w: self }
    }
    #[doc = "Bit 7 - External Trigger"]
    #[inline(always)]
    pub fn extr(&mut self) -> EXTR_W {
        EXTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Queue 0 Input Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qinr0](index.html) module"]
pub struct QINR0_SPEC;
impl crate::RegisterSpec for QINR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [qinr0::W](W) writer structure"]
impl crate::Writable for QINR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QINR0 to value 0"]
impl crate::Resettable for QINR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
