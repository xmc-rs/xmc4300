#[doc = "Register `INTERRUPT_MASK` reader"]
pub struct R(crate::R<INTERRUPT_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPT_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPT_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPT_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERRUPT_MASK` writer"]
pub struct W(crate::W<INTERRUPT_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERRUPT_MASK_SPEC>;
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
impl From<crate::W<INTERRUPT_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERRUPT_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMTIM` reader - PMT Interrupt Mask"]
pub struct PMTIM_R(crate::FieldReader<bool, bool>);
impl PMTIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMTIM` writer - PMT Interrupt Mask"]
pub struct PMTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PMTIM_W<'a> {
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
#[doc = "Field `TSIM` reader - Timestamp Interrupt Mask"]
pub struct TSIM_R(crate::FieldReader<bool, bool>);
impl TSIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSIM` writer - Timestamp Interrupt Mask"]
pub struct TSIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIM_W<'a> {
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
impl R {
    #[doc = "Bit 3 - PMT Interrupt Mask"]
    #[inline(always)]
    pub fn pmtim(&self) -> PMTIM_R {
        PMTIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Mask"]
    #[inline(always)]
    pub fn tsim(&self) -> TSIM_R {
        TSIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PMT Interrupt Mask"]
    #[inline(always)]
    pub fn pmtim(&mut self) -> PMTIM_W {
        PMTIM_W { w: self }
    }
    #[doc = "Bit 9 - Timestamp Interrupt Mask"]
    #[inline(always)]
    pub fn tsim(&mut self) -> TSIM_W {
        TSIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_mask](index.html) module"]
pub struct INTERRUPT_MASK_SPEC;
impl crate::RegisterSpec for INTERRUPT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interrupt_mask::R](R) reader structure"]
impl crate::Readable for INTERRUPT_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interrupt_mask::W](W) writer structure"]
impl crate::Writable for INTERRUPT_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTERRUPT_MASK to value 0"]
impl crate::Resettable for INTERRUPT_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
