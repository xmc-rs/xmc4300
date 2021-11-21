#[doc = "Register `MAC_ADDRESS0_HIGH` reader"]
pub struct R(crate::R<MAC_ADDRESS0_HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_ADDRESS0_HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_ADDRESS0_HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_ADDRESS0_HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_ADDRESS0_HIGH` writer"]
pub struct W(crate::W<MAC_ADDRESS0_HIGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_ADDRESS0_HIGH_SPEC>;
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
impl From<crate::W<MAC_ADDRESS0_HIGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_ADDRESS0_HIGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRHI` reader - MAC Address0 \\[47:32\\]"]
pub struct ADDRHI_R(crate::FieldReader<u16, u16>);
impl ADDRHI_R {
    pub(crate) fn new(bits: u16) -> Self {
        ADDRHI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRHI_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRHI` writer - MAC Address0 \\[47:32\\]"]
pub struct ADDRHI_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRHI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `AE` reader - Address Enable"]
pub struct AE_R(crate::FieldReader<bool, bool>);
impl AE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - MAC Address0 \\[47:32\\]"]
    #[inline(always)]
    pub fn addrhi(&self) -> ADDRHI_R {
        ADDRHI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Address Enable"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Address0 \\[47:32\\]"]
    #[inline(always)]
    pub fn addrhi(&mut self) -> ADDRHI_W {
        ADDRHI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Address0 High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_address0_high](index.html) module"]
pub struct MAC_ADDRESS0_HIGH_SPEC;
impl crate::RegisterSpec for MAC_ADDRESS0_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_address0_high::R](R) reader structure"]
impl crate::Readable for MAC_ADDRESS0_HIGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_address0_high::W](W) writer structure"]
impl crate::Writable for MAC_ADDRESS0_HIGH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_ADDRESS0_HIGH to value 0x8000_ffff"]
impl crate::Resettable for MAC_ADDRESS0_HIGH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_ffff
    }
}
