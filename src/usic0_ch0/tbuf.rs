#[doc = "Register `TBUF[%s]` reader"]
pub struct R(crate::R<TBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBUF[%s]` writer"]
pub struct W(crate::W<TBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBUF_SPEC>;
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
impl From<crate::W<TBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDATA` reader - Transmit Data"]
pub struct TDATA_R(crate::FieldReader<u16, u16>);
impl TDATA_R {
    pub(crate) fn new(bits: u16) -> Self {
        TDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDATA` writer - Transmit Data"]
pub struct TDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    pub fn tdata(&self) -> TDATA_R {
        TDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    pub fn tdata(&mut self) -> TDATA_W {
        TDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbuf](index.html) module"]
pub struct TBUF_SPEC;
impl crate::RegisterSpec for TBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbuf::R](R) reader structure"]
impl crate::Readable for TBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbuf::W](W) writer structure"]
impl crate::Writable for TBUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBUF[%s]
to value 0"]
impl crate::Resettable for TBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
