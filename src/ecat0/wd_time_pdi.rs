#[doc = "Register `WD_TIME_PDI` reader"]
pub struct R(crate::R<WD_TIME_PDI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WD_TIME_PDI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WD_TIME_PDI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WD_TIME_PDI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WD_TIME_PDI` writer"]
pub struct W(crate::W<WD_TIME_PDI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WD_TIME_PDI_SPEC>;
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
impl From<crate::W<WD_TIME_PDI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WD_TIME_PDI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WD_TIME_PDI` reader - Watchdog Time PDI"]
pub struct WD_TIME_PDI_R(crate::FieldReader<u16, u16>);
impl WD_TIME_PDI_R {
    pub(crate) fn new(bits: u16) -> Self {
        WD_TIME_PDI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WD_TIME_PDI_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WD_TIME_PDI` writer - Watchdog Time PDI"]
pub struct WD_TIME_PDI_W<'a> {
    w: &'a mut W,
}
impl<'a> WD_TIME_PDI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Watchdog Time PDI"]
    #[inline(always)]
    pub fn wd_time_pdi(&self) -> WD_TIME_PDI_R {
        WD_TIME_PDI_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Watchdog Time PDI"]
    #[inline(always)]
    pub fn wd_time_pdi(&mut self) -> WD_TIME_PDI_W {
        WD_TIME_PDI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Time PDI\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wd_time_pdi](index.html) module"]
pub struct WD_TIME_PDI_SPEC;
impl crate::RegisterSpec for WD_TIME_PDI_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wd_time_pdi::R](R) reader structure"]
impl crate::Readable for WD_TIME_PDI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wd_time_pdi::W](W) writer structure"]
impl crate::Writable for WD_TIME_PDI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WD_TIME_PDI to value 0x03e8"]
impl crate::Resettable for WD_TIME_PDI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03e8
    }
}
