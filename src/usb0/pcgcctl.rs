#[doc = "Register `PCGCCTL` reader"]
pub struct R(crate::R<PCGCCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCGCCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCGCCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCGCCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCGCCTL` writer"]
pub struct W(crate::W<PCGCCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCGCCTL_SPEC>;
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
impl From<crate::W<PCGCCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCGCCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `StopPclk` reader - Stop Pclk"]
pub struct STOPPCLK_R(crate::FieldReader<bool, bool>);
impl STOPPCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        STOPPCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOPPCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `StopPclk` writer - Stop Pclk"]
pub struct STOPPCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPPCLK_W<'a> {
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
#[doc = "Field `GateHclk` reader - Gate Hclk"]
pub struct GATEHCLK_R(crate::FieldReader<bool, bool>);
impl GATEHCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        GATEHCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GATEHCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GateHclk` writer - Gate Hclk"]
pub struct GATEHCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> GATEHCLK_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Stop Pclk"]
    #[inline(always)]
    pub fn stop_pclk(&self) -> STOPPCLK_R {
        STOPPCLK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Gate Hclk"]
    #[inline(always)]
    pub fn gate_hclk(&self) -> GATEHCLK_R {
        GATEHCLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop Pclk"]
    #[inline(always)]
    pub fn stop_pclk(&mut self) -> STOPPCLK_W {
        STOPPCLK_W { w: self }
    }
    #[doc = "Bit 1 - Gate Hclk"]
    #[inline(always)]
    pub fn gate_hclk(&mut self) -> GATEHCLK_W {
        GATEHCLK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power and Clock Gating Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcgcctl](index.html) module"]
pub struct PCGCCTL_SPEC;
impl crate::RegisterSpec for PCGCCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcgcctl::R](R) reader structure"]
impl crate::Readable for PCGCCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcgcctl::W](W) writer structure"]
impl crate::Writable for PCGCCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCGCCTL to value 0x0100"]
impl crate::Resettable for PCGCCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
