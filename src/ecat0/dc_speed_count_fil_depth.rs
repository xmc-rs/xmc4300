#[doc = "Register `DC_SPEED_COUNT_FIL_DEPTH` reader"]
pub struct R(crate::R<DC_SPEED_COUNT_FIL_DEPTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_SPEED_COUNT_FIL_DEPTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_SPEED_COUNT_FIL_DEPTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_SPEED_COUNT_FIL_DEPTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DC_SPEED_COUNT_FIL_DEPTH` writer"]
pub struct W(crate::W<DC_SPEED_COUNT_FIL_DEPTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DC_SPEED_COUNT_FIL_DEPTH_SPEC>;
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
impl From<crate::W<DC_SPEED_COUNT_FIL_DEPTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DC_SPEED_COUNT_FIL_DEPTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILTER_DEPTH` reader - Filter depth for averaging the clock period deviation"]
pub struct FILTER_DEPTH_R(crate::FieldReader<u8, u8>);
impl FILTER_DEPTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        FILTER_DEPTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTER_DEPTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTER_DEPTH` writer - Filter depth for averaging the clock period deviation"]
pub struct FILTER_DEPTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_DEPTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Filter depth for averaging the clock period deviation"]
    #[inline(always)]
    pub fn filter_depth(&self) -> FILTER_DEPTH_R {
        FILTER_DEPTH_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Filter depth for averaging the clock period deviation"]
    #[inline(always)]
    pub fn filter_depth(&mut self) -> FILTER_DEPTH_W {
        FILTER_DEPTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Speed Counter Filter Depth\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_speed_count_fil_depth](index.html) module"]
pub struct DC_SPEED_COUNT_FIL_DEPTH_SPEC;
impl crate::RegisterSpec for DC_SPEED_COUNT_FIL_DEPTH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dc_speed_count_fil_depth::R](R) reader structure"]
impl crate::Readable for DC_SPEED_COUNT_FIL_DEPTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dc_speed_count_fil_depth::W](W) writer structure"]
impl crate::Writable for DC_SPEED_COUNT_FIL_DEPTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DC_SPEED_COUNT_FIL_DEPTH to value 0x0c"]
impl crate::Resettable for DC_SPEED_COUNT_FIL_DEPTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c
    }
}
