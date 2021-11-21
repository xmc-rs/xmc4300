#[doc = "Register `MAC_ADDRESS1_LOW` reader"]
pub struct R(crate::R<MAC_ADDRESS1_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_ADDRESS1_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_ADDRESS1_LOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_ADDRESS1_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_ADDRESS1_LOW` writer"]
pub struct W(crate::W<MAC_ADDRESS1_LOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_ADDRESS1_LOW_SPEC>;
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
impl From<crate::W<MAC_ADDRESS1_LOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_ADDRESS1_LOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRLO` reader - MAC Address1 \\[31:0\\]"]
pub struct ADDRLO_R(crate::FieldReader<u32, u32>);
impl ADDRLO_R {
    pub(crate) fn new(bits: u32) -> Self {
        ADDRLO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRLO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRLO` writer - MAC Address1 \\[31:0\\]"]
pub struct ADDRLO_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRLO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - MAC Address1 \\[31:0\\]"]
    #[inline(always)]
    pub fn addrlo(&self) -> ADDRLO_R {
        ADDRLO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC Address1 \\[31:0\\]"]
    #[inline(always)]
    pub fn addrlo(&mut self) -> ADDRLO_W {
        ADDRLO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Address1 Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_address1_low](index.html) module"]
pub struct MAC_ADDRESS1_LOW_SPEC;
impl crate::RegisterSpec for MAC_ADDRESS1_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_address1_low::R](R) reader structure"]
impl crate::Readable for MAC_ADDRESS1_LOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_address1_low::W](W) writer structure"]
impl crate::Writable for MAC_ADDRESS1_LOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_ADDRESS1_LOW to value 0xffff_ffff"]
impl crate::Resettable for MAC_ADDRESS1_LOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
