#[doc = "Register `MSKSR` reader"]
pub struct R(crate::R<MSKSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSKSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSKSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSKSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSKSR` writer"]
pub struct W(crate::W<MSKSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSKSR_SPEC>;
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
impl From<crate::W<MSKSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSKSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPSE` reader - Periodic Seconds Interrupt Mask"]
pub struct MPSE_R(crate::FieldReader<bool, bool>);
impl MPSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPSE` writer - Periodic Seconds Interrupt Mask"]
pub struct MPSE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPSE_W<'a> {
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
#[doc = "Field `MPMI` reader - Periodic Minutes Interrupt Mask"]
pub struct MPMI_R(crate::FieldReader<bool, bool>);
impl MPMI_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPMI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPMI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPMI` writer - Periodic Minutes Interrupt Mask"]
pub struct MPMI_W<'a> {
    w: &'a mut W,
}
impl<'a> MPMI_W<'a> {
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
#[doc = "Field `MPHO` reader - Periodic Hours Interrupt Mask"]
pub struct MPHO_R(crate::FieldReader<bool, bool>);
impl MPHO_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPHO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPHO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPHO` writer - Periodic Hours Interrupt Mask"]
pub struct MPHO_W<'a> {
    w: &'a mut W,
}
impl<'a> MPHO_W<'a> {
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
#[doc = "Field `MPDA` reader - Periodic Days Interrupt Mask"]
pub struct MPDA_R(crate::FieldReader<bool, bool>);
impl MPDA_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPDA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPDA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPDA` writer - Periodic Days Interrupt Mask"]
pub struct MPDA_W<'a> {
    w: &'a mut W,
}
impl<'a> MPDA_W<'a> {
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
#[doc = "Field `MPMO` reader - Periodic Months Interrupt Mask"]
pub struct MPMO_R(crate::FieldReader<bool, bool>);
impl MPMO_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPMO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPMO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPMO` writer - Periodic Months Interrupt Mask"]
pub struct MPMO_W<'a> {
    w: &'a mut W,
}
impl<'a> MPMO_W<'a> {
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
#[doc = "Field `MPYE` reader - Periodic Years Interrupt Mask"]
pub struct MPYE_R(crate::FieldReader<bool, bool>);
impl MPYE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPYE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPYE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPYE` writer - Periodic Years Interrupt Mask"]
pub struct MPYE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPYE_W<'a> {
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
#[doc = "Field `MAI` reader - Alarm Interrupt Mask"]
pub struct MAI_R(crate::FieldReader<bool, bool>);
impl MAI_R {
    pub(crate) fn new(bits: bool) -> Self {
        MAI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAI` writer - Alarm Interrupt Mask"]
pub struct MAI_W<'a> {
    w: &'a mut W,
}
impl<'a> MAI_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Periodic Seconds Interrupt Mask"]
    #[inline(always)]
    pub fn mpse(&self) -> MPSE_R {
        MPSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Periodic Minutes Interrupt Mask"]
    #[inline(always)]
    pub fn mpmi(&self) -> MPMI_R {
        MPMI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Periodic Hours Interrupt Mask"]
    #[inline(always)]
    pub fn mpho(&self) -> MPHO_R {
        MPHO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Periodic Days Interrupt Mask"]
    #[inline(always)]
    pub fn mpda(&self) -> MPDA_R {
        MPDA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Periodic Months Interrupt Mask"]
    #[inline(always)]
    pub fn mpmo(&self) -> MPMO_R {
        MPMO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Periodic Years Interrupt Mask"]
    #[inline(always)]
    pub fn mpye(&self) -> MPYE_R {
        MPYE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Alarm Interrupt Mask"]
    #[inline(always)]
    pub fn mai(&self) -> MAI_R {
        MAI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Periodic Seconds Interrupt Mask"]
    #[inline(always)]
    pub fn mpse(&mut self) -> MPSE_W {
        MPSE_W { w: self }
    }
    #[doc = "Bit 1 - Periodic Minutes Interrupt Mask"]
    #[inline(always)]
    pub fn mpmi(&mut self) -> MPMI_W {
        MPMI_W { w: self }
    }
    #[doc = "Bit 2 - Periodic Hours Interrupt Mask"]
    #[inline(always)]
    pub fn mpho(&mut self) -> MPHO_W {
        MPHO_W { w: self }
    }
    #[doc = "Bit 3 - Periodic Days Interrupt Mask"]
    #[inline(always)]
    pub fn mpda(&mut self) -> MPDA_W {
        MPDA_W { w: self }
    }
    #[doc = "Bit 5 - Periodic Months Interrupt Mask"]
    #[inline(always)]
    pub fn mpmo(&mut self) -> MPMO_W {
        MPMO_W { w: self }
    }
    #[doc = "Bit 6 - Periodic Years Interrupt Mask"]
    #[inline(always)]
    pub fn mpye(&mut self) -> MPYE_W {
        MPYE_W { w: self }
    }
    #[doc = "Bit 8 - Alarm Interrupt Mask"]
    #[inline(always)]
    pub fn mai(&mut self) -> MAI_W {
        MAI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Service Request Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msksr](index.html) module"]
pub struct MSKSR_SPEC;
impl crate::RegisterSpec for MSKSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msksr::R](R) reader structure"]
impl crate::Readable for MSKSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msksr::W](W) writer structure"]
impl crate::Writable for MSKSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MSKSR to value 0"]
impl crate::Resettable for MSKSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
