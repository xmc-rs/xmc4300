#[doc = "Register `DAINTMSK` reader"]
pub struct R(crate::R<DAINTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAINTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAINTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAINTMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAINTMSK` writer"]
pub struct W(crate::W<DAINTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAINTMSK_SPEC>;
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
impl From<crate::W<DAINTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAINTMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `InEpMsk` reader - IN EP Interrupt Mask Bits"]
pub struct INEPMSK_R(crate::FieldReader<u16, u16>);
impl INEPMSK_R {
    pub(crate) fn new(bits: u16) -> Self {
        INEPMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEPMSK_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `InEpMsk` writer - IN EP Interrupt Mask Bits"]
pub struct INEPMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPMSK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `OutEpMsk` reader - OUT EP Interrupt Mask Bits"]
pub struct OUTEPMSK_R(crate::FieldReader<u16, u16>);
impl OUTEPMSK_R {
    pub(crate) fn new(bits: u16) -> Self {
        OUTEPMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTEPMSK_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OutEpMsk` writer - OUT EP Interrupt Mask Bits"]
pub struct OUTEPMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTEPMSK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - IN EP Interrupt Mask Bits"]
    #[inline(always)]
    pub fn in_ep_msk(&self) -> INEPMSK_R {
        INEPMSK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT EP Interrupt Mask Bits"]
    #[inline(always)]
    pub fn out_ep_msk(&self) -> OUTEPMSK_R {
        OUTEPMSK_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN EP Interrupt Mask Bits"]
    #[inline(always)]
    pub fn in_ep_msk(&mut self) -> INEPMSK_W {
        INEPMSK_W { w: self }
    }
    #[doc = "Bits 16:31 - OUT EP Interrupt Mask Bits"]
    #[inline(always)]
    pub fn out_ep_msk(&mut self) -> OUTEPMSK_W {
        OUTEPMSK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device All Endpoints Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daintmsk](index.html) module"]
pub struct DAINTMSK_SPEC;
impl crate::RegisterSpec for DAINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [daintmsk::R](R) reader structure"]
impl crate::Readable for DAINTMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [daintmsk::W](W) writer structure"]
impl crate::Writable for DAINTMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAINTMSK to value 0"]
impl crate::Resettable for DAINTMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
