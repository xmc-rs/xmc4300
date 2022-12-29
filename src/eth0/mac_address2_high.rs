#[doc = "Register `MAC_ADDRESS2_HIGH` reader"]
pub struct R(crate::R<MAC_ADDRESS2_HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_ADDRESS2_HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_ADDRESS2_HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_ADDRESS2_HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_ADDRESS2_HIGH` writer"]
pub struct W(crate::W<MAC_ADDRESS2_HIGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_ADDRESS2_HIGH_SPEC>;
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
impl From<crate::W<MAC_ADDRESS2_HIGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_ADDRESS2_HIGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRHI` reader - MAC Address2 \\[47:32\\]"]
pub type ADDRHI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDRHI` writer - MAC Address2 \\[47:32\\]"]
pub type ADDRHI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_ADDRESS2_HIGH_SPEC, u16, u16, 16, O>;
#[doc = "Field `MBC` reader - Mask Byte Control"]
pub type MBC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MBC` writer - Mask Byte Control"]
pub type MBC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_ADDRESS2_HIGH_SPEC, u8, u8, 6, O>;
#[doc = "Field `SA` reader - Source Address"]
pub type SA_R = crate::BitReader<bool>;
#[doc = "Field `SA` writer - Source Address"]
pub type SA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_ADDRESS2_HIGH_SPEC, bool, O>;
#[doc = "Field `AE` reader - Address Enable"]
pub type AE_R = crate::BitReader<bool>;
#[doc = "Field `AE` writer - Address Enable"]
pub type AE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_ADDRESS2_HIGH_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - MAC Address2 \\[47:32\\]"]
    #[inline(always)]
    pub fn addrhi(&self) -> ADDRHI_R {
        ADDRHI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - Mask Byte Control"]
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Source Address"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Address Enable"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Address2 \\[47:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn addrhi(&mut self) -> ADDRHI_W<0> {
        ADDRHI_W::new(self)
    }
    #[doc = "Bits 24:29 - Mask Byte Control"]
    #[inline(always)]
    #[must_use]
    pub fn mbc(&mut self) -> MBC_W<24> {
        MBC_W::new(self)
    }
    #[doc = "Bit 30 - Source Address"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<30> {
        SA_W::new(self)
    }
    #[doc = "Bit 31 - Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae(&mut self) -> AE_W<31> {
        AE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Address2 High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_address2_high](index.html) module"]
pub struct MAC_ADDRESS2_HIGH_SPEC;
impl crate::RegisterSpec for MAC_ADDRESS2_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_address2_high::R](R) reader structure"]
impl crate::Readable for MAC_ADDRESS2_HIGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_address2_high::W](W) writer structure"]
impl crate::Writable for MAC_ADDRESS2_HIGH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_ADDRESS2_HIGH to value 0xffff"]
impl crate::Resettable for MAC_ADDRESS2_HIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
