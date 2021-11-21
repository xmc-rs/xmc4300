#[doc = "Register `TSVAL` reader"]
pub struct R(crate::R<TSVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSVAL` writer"]
pub struct W(crate::W<TSVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSVAL_SPEC>;
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
impl From<crate::W<TSVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSCTRVALR` reader - Shadow TS-Counter (Read)"]
pub struct TSCTRVALR_R(crate::FieldReader<u16, u16>);
impl TSCTRVALR_R {
    pub(crate) fn new(bits: u16) -> Self {
        TSCTRVALR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSCTRVALR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSCTRVAL` reader - TS-Counter Value"]
pub struct TSCTRVAL_R(crate::FieldReader<u16, u16>);
impl TSCTRVAL_R {
    pub(crate) fn new(bits: u16) -> Self {
        TSCTRVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSCTRVAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSCTRVAL` writer - TS-Counter Value"]
pub struct TSCTRVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCTRVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Shadow TS-Counter (Read)"]
    #[inline(always)]
    pub fn tsctrvalr(&self) -> TSCTRVALR_R {
        TSCTRVALR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - TS-Counter Value"]
    #[inline(always)]
    pub fn tsctrval(&self) -> TSCTRVAL_R {
        TSCTRVAL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - TS-Counter Value"]
    #[inline(always)]
    pub fn tsctrval(&mut self) -> TSCTRVAL_W {
        TSCTRVAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Touch-sense TS-Counter Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsval](index.html) module"]
pub struct TSVAL_SPEC;
impl crate::RegisterSpec for TSVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsval::R](R) reader structure"]
impl crate::Readable for TSVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsval::W](W) writer structure"]
impl crate::Writable for TSVAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSVAL to value 0"]
impl crate::Resettable for TSVAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
