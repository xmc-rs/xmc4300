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
pub type IN_EP_MSK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `InEpMsk` writer - IN EP Interrupt Mask Bits"]
pub type IN_EP_MSK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAINTMSK_SPEC, u16, u16, 16, O>;
#[doc = "Field `OutEpMsk` reader - OUT EP Interrupt Mask Bits"]
pub type OUT_EP_MSK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OutEpMsk` writer - OUT EP Interrupt Mask Bits"]
pub type OUT_EP_MSK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAINTMSK_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - IN EP Interrupt Mask Bits"]
    #[inline(always)]
    pub fn in_ep_msk(&self) -> IN_EP_MSK_R {
        IN_EP_MSK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT EP Interrupt Mask Bits"]
    #[inline(always)]
    pub fn out_ep_msk(&self) -> OUT_EP_MSK_R {
        OUT_EP_MSK_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN EP Interrupt Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn in_ep_msk(&mut self) -> IN_EP_MSK_W<0> {
        IN_EP_MSK_W::new(self)
    }
    #[doc = "Bits 16:31 - OUT EP Interrupt Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn out_ep_msk(&mut self) -> OUT_EP_MSK_W<16> {
        OUT_EP_MSK_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAINTMSK to value 0"]
impl crate::Resettable for DAINTMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
