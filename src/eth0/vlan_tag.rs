#[doc = "Register `VLAN_TAG` reader"]
pub struct R(crate::R<VLAN_TAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VLAN_TAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VLAN_TAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VLAN_TAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VLAN_TAG` writer"]
pub struct W(crate::W<VLAN_TAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VLAN_TAG_SPEC>;
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
impl From<crate::W<VLAN_TAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VLAN_TAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VL` reader - VLAN Tag Identifier for Receive Frames"]
pub struct VL_R(crate::FieldReader<u16, u16>);
impl VL_R {
    pub(crate) fn new(bits: u16) -> Self {
        VL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VL` writer - VLAN Tag Identifier for Receive Frames"]
pub struct VL_W<'a> {
    w: &'a mut W,
}
impl<'a> VL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `ETV` reader - Enable 12-Bit VLAN Tag Comparison"]
pub struct ETV_R(crate::FieldReader<bool, bool>);
impl ETV_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETV` writer - Enable 12-Bit VLAN Tag Comparison"]
pub struct ETV_W<'a> {
    w: &'a mut W,
}
impl<'a> ETV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `VTIM` reader - VLAN Tag Inverse Match Enable"]
pub struct VTIM_R(crate::FieldReader<bool, bool>);
impl VTIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTIM` writer - VLAN Tag Inverse Match Enable"]
pub struct VTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> VTIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `ESVL` reader - Enable S-VLAN"]
pub struct ESVL_R(crate::FieldReader<bool, bool>);
impl ESVL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ESVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESVL` writer - Enable S-VLAN"]
pub struct ESVL_W<'a> {
    w: &'a mut W,
}
impl<'a> ESVL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `VTHM` reader - VLAN Tag Hash Table Match Enable"]
pub struct VTHM_R(crate::FieldReader<bool, bool>);
impl VTHM_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTHM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTHM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Frames"]
    #[inline(always)]
    pub fn vl(&self) -> VL_R {
        VL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable"]
    #[inline(always)]
    pub fn vtim(&self) -> VTIM_R {
        VTIM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable S-VLAN"]
    #[inline(always)]
    pub fn esvl(&self) -> ESVL_R {
        ESVL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - VLAN Tag Hash Table Match Enable"]
    #[inline(always)]
    pub fn vthm(&self) -> VTHM_R {
        VTHM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Frames"]
    #[inline(always)]
    pub fn vl(&mut self) -> VL_W {
        VL_W { w: self }
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    pub fn etv(&mut self) -> ETV_W {
        ETV_W { w: self }
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable"]
    #[inline(always)]
    pub fn vtim(&mut self) -> VTIM_W {
        VTIM_W { w: self }
    }
    #[doc = "Bit 18 - Enable S-VLAN"]
    #[inline(always)]
    pub fn esvl(&mut self) -> ESVL_W {
        ESVL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VLAN Tag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vlan_tag](index.html) module"]
pub struct VLAN_TAG_SPEC;
impl crate::RegisterSpec for VLAN_TAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vlan_tag::R](R) reader structure"]
impl crate::Readable for VLAN_TAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vlan_tag::W](W) writer structure"]
impl crate::Writable for VLAN_TAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VLAN_TAG to value 0"]
impl crate::Resettable for VLAN_TAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
