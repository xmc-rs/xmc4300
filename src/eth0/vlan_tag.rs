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
pub type VL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VL` writer - VLAN Tag Identifier for Receive Frames"]
pub type VL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VLAN_TAG_SPEC, u16, u16, 16, O>;
#[doc = "Field `ETV` reader - Enable 12-Bit VLAN Tag Comparison"]
pub type ETV_R = crate::BitReader<bool>;
#[doc = "Field `ETV` writer - Enable 12-Bit VLAN Tag Comparison"]
pub type ETV_W<'a, const O: u8> = crate::BitWriter<'a, u32, VLAN_TAG_SPEC, bool, O>;
#[doc = "Field `VTIM` reader - VLAN Tag Inverse Match Enable"]
pub type VTIM_R = crate::BitReader<bool>;
#[doc = "Field `VTIM` writer - VLAN Tag Inverse Match Enable"]
pub type VTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, VLAN_TAG_SPEC, bool, O>;
#[doc = "Field `ESVL` reader - Enable S-VLAN"]
pub type ESVL_R = crate::BitReader<bool>;
#[doc = "Field `ESVL` writer - Enable S-VLAN"]
pub type ESVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, VLAN_TAG_SPEC, bool, O>;
#[doc = "Field `VTHM` reader - VLAN Tag Hash Table Match Enable"]
pub type VTHM_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Frames"]
    #[inline(always)]
    pub fn vl(&self) -> VL_R {
        VL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable"]
    #[inline(always)]
    pub fn vtim(&self) -> VTIM_R {
        VTIM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable S-VLAN"]
    #[inline(always)]
    pub fn esvl(&self) -> ESVL_R {
        ESVL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - VLAN Tag Hash Table Match Enable"]
    #[inline(always)]
    pub fn vthm(&self) -> VTHM_R {
        VTHM_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Frames"]
    #[inline(always)]
    #[must_use]
    pub fn vl(&mut self) -> VL_W<0> {
        VL_W::new(self)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    #[must_use]
    pub fn etv(&mut self) -> ETV_W<16> {
        ETV_W::new(self)
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vtim(&mut self) -> VTIM_W<17> {
        VTIM_W::new(self)
    }
    #[doc = "Bit 18 - Enable S-VLAN"]
    #[inline(always)]
    #[must_use]
    pub fn esvl(&mut self) -> ESVL_W<18> {
        ESVL_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VLAN_TAG to value 0"]
impl crate::Resettable for VLAN_TAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
