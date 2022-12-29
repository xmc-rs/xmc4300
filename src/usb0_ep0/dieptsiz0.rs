#[doc = "Register `DIEPTSIZ0` reader"]
pub struct R(crate::R<DIEPTSIZ0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPTSIZ0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPTSIZ0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPTSIZ0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPTSIZ0` writer"]
pub struct W(crate::W<DIEPTSIZ0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPTSIZ0_SPEC>;
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
impl From<crate::W<DIEPTSIZ0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPTSIZ0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XferSize` reader - Transfer Size"]
pub type XFER_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XferSize` writer - Transfer Size"]
pub type XFER_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEPTSIZ0_SPEC, u8, u8, 7, O>;
#[doc = "Field `PktCnt` reader - Packet Count"]
pub type PKT_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PktCnt` writer - Packet Count"]
pub type PKT_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEPTSIZ0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    pub fn xfer_size(&self) -> XFER_SIZE_R {
        XFER_SIZE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline(always)]
    pub fn pkt_cnt(&self) -> PKT_CNT_R {
        PKT_CNT_R::new(((self.bits >> 19) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_size(&mut self) -> XFER_SIZE_W<0> {
        XFER_SIZE_W::new(self)
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_cnt(&mut self) -> PKT_CNT_W<19> {
        PKT_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device IN Endpoint Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptsiz0](index.html) module"]
pub struct DIEPTSIZ0_SPEC;
impl crate::RegisterSpec for DIEPTSIZ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dieptsiz0::R](R) reader structure"]
impl crate::Readable for DIEPTSIZ0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dieptsiz0::W](W) writer structure"]
impl crate::Writable for DIEPTSIZ0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPTSIZ0 to value 0"]
impl crate::Resettable for DIEPTSIZ0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
