#[doc = "Register `DSR` reader"]
pub struct R(crate::R<DSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSR` writer"]
pub struct W(crate::W<DSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSR_SPEC>;
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
impl From<crate::W<DSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSI` reader - Destination scatter interval"]
pub type DSI_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DSI` writer - Destination scatter interval"]
pub type DSI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DSR_SPEC, u32, u32, 20, O>;
#[doc = "Field `DSC` reader - Destination scatter count"]
pub type DSC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DSC` writer - Destination scatter count"]
pub type DSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DSR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:19 - Destination scatter interval"]
    #[inline(always)]
    pub fn dsi(&self) -> DSI_R {
        DSI_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - Destination scatter count"]
    #[inline(always)]
    pub fn dsc(&self) -> DSC_R {
        DSC_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - Destination scatter interval"]
    #[inline(always)]
    #[must_use]
    pub fn dsi(&mut self) -> DSI_W<0> {
        DSI_W::new(self)
    }
    #[doc = "Bits 20:31 - Destination scatter count"]
    #[inline(always)]
    #[must_use]
    pub fn dsc(&mut self) -> DSC_W<20> {
        DSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Destination Scatter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsr](index.html) module"]
pub struct DSR_SPEC;
impl crate::RegisterSpec for DSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsr::R](R) reader structure"]
impl crate::Readable for DSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsr::W](W) writer structure"]
impl crate::Writable for DSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSR to value 0"]
impl crate::Resettable for DSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
