#[doc = "Register `PMT_CONTROL_STATUS` reader"]
pub struct R(crate::R<PMT_CONTROL_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMT_CONTROL_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMT_CONTROL_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMT_CONTROL_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMT_CONTROL_STATUS` writer"]
pub struct W(crate::W<PMT_CONTROL_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMT_CONTROL_STATUS_SPEC>;
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
impl From<crate::W<PMT_CONTROL_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMT_CONTROL_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWRDWN` reader - Power Down"]
pub struct PWRDWN_R(crate::FieldReader<bool, bool>);
impl PWRDWN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRDWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRDWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRDWN` writer - Power Down"]
pub struct PWRDWN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRDWN_W<'a> {
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
#[doc = "Field `MGKPKTEN` reader - Magic Packet Enable"]
pub struct MGKPKTEN_R(crate::FieldReader<bool, bool>);
impl MGKPKTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MGKPKTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MGKPKTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MGKPKTEN` writer - Magic Packet Enable"]
pub struct MGKPKTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MGKPKTEN_W<'a> {
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
#[doc = "Field `RWKPKTEN` reader - Wake-Up Frame Enable"]
pub struct RWKPKTEN_R(crate::FieldReader<bool, bool>);
impl RWKPKTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWKPKTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWKPKTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWKPKTEN` writer - Wake-Up Frame Enable"]
pub struct RWKPKTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RWKPKTEN_W<'a> {
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
#[doc = "Field `MGKPRCVD` reader - Magic Packet Received"]
pub struct MGKPRCVD_R(crate::FieldReader<bool, bool>);
impl MGKPRCVD_R {
    pub(crate) fn new(bits: bool) -> Self {
        MGKPRCVD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MGKPRCVD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWKPRCVD` reader - Wake-Up Frame Received"]
pub struct RWKPRCVD_R(crate::FieldReader<bool, bool>);
impl RWKPRCVD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWKPRCVD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWKPRCVD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLBLUCAST` reader - Global Unicast"]
pub struct GLBLUCAST_R(crate::FieldReader<bool, bool>);
impl GLBLUCAST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GLBLUCAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GLBLUCAST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLBLUCAST` writer - Global Unicast"]
pub struct GLBLUCAST_W<'a> {
    w: &'a mut W,
}
impl<'a> GLBLUCAST_W<'a> {
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
#[doc = "Field `RWKFILTRST` reader - Wake-Up Frame Filter Register Pointer Reset"]
pub struct RWKFILTRST_R(crate::FieldReader<bool, bool>);
impl RWKFILTRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWKFILTRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWKFILTRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWKFILTRST` writer - Wake-Up Frame Filter Register Pointer Reset"]
pub struct RWKFILTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RWKFILTRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Power Down"]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Magic Packet Enable"]
    #[inline(always)]
    pub fn mgkpkten(&self) -> MGKPKTEN_R {
        MGKPKTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wake-Up Frame Enable"]
    #[inline(always)]
    pub fn rwkpkten(&self) -> RWKPKTEN_R {
        RWKPKTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Magic Packet Received"]
    #[inline(always)]
    pub fn mgkprcvd(&self) -> MGKPRCVD_R {
        MGKPRCVD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wake-Up Frame Received"]
    #[inline(always)]
    pub fn rwkprcvd(&self) -> RWKPRCVD_R {
        RWKPRCVD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Global Unicast"]
    #[inline(always)]
    pub fn glblucast(&self) -> GLBLUCAST_R {
        GLBLUCAST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Wake-Up Frame Filter Register Pointer Reset"]
    #[inline(always)]
    pub fn rwkfiltrst(&self) -> RWKFILTRST_R {
        RWKFILTRST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Down"]
    #[inline(always)]
    pub fn pwrdwn(&mut self) -> PWRDWN_W {
        PWRDWN_W { w: self }
    }
    #[doc = "Bit 1 - Magic Packet Enable"]
    #[inline(always)]
    pub fn mgkpkten(&mut self) -> MGKPKTEN_W {
        MGKPKTEN_W { w: self }
    }
    #[doc = "Bit 2 - Wake-Up Frame Enable"]
    #[inline(always)]
    pub fn rwkpkten(&mut self) -> RWKPKTEN_W {
        RWKPKTEN_W { w: self }
    }
    #[doc = "Bit 9 - Global Unicast"]
    #[inline(always)]
    pub fn glblucast(&mut self) -> GLBLUCAST_W {
        GLBLUCAST_W { w: self }
    }
    #[doc = "Bit 31 - Wake-Up Frame Filter Register Pointer Reset"]
    #[inline(always)]
    pub fn rwkfiltrst(&mut self) -> RWKFILTRST_W {
        RWKFILTRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMT Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmt_control_status](index.html) module"]
pub struct PMT_CONTROL_STATUS_SPEC;
impl crate::RegisterSpec for PMT_CONTROL_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmt_control_status::R](R) reader structure"]
impl crate::Readable for PMT_CONTROL_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmt_control_status::W](W) writer structure"]
impl crate::Writable for PMT_CONTROL_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMT_CONTROL_STATUS to value 0"]
impl crate::Resettable for PMT_CONTROL_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
