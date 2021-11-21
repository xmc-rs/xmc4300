#[doc = "Register `CRC` reader"]
pub struct R(crate::R<CRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC` writer"]
pub struct W(crate::W<CRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_SPEC>;
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
impl From<crate::W<CRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC` reader - CRC Register"]
pub struct CRC_R(crate::FieldReader<u32, u32>);
impl CRC_R {
    pub(crate) fn new(bits: u32) -> Self {
        CRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC` writer - CRC Register"]
pub struct CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRC Register"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Register"]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W {
        CRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc](index.html) module"]
pub struct CRC_SPEC;
impl crate::RegisterSpec for CRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc::R](R) reader structure"]
impl crate::Readable for CRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc::W](W) writer structure"]
impl crate::Writable for CRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC to value 0"]
impl crate::Resettable for CRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
