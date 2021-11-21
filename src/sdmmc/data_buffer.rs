#[doc = "Register `DATA_BUFFER` reader"]
pub struct R(crate::R<DATA_BUFFER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_BUFFER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_BUFFER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_BUFFER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA_BUFFER` writer"]
pub struct W(crate::W<DATA_BUFFER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_BUFFER_SPEC>;
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
impl From<crate::W<DATA_BUFFER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_BUFFER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_BUFFER` reader - Data Buffer"]
pub struct DATA_BUFFER_R(crate::FieldReader<u32, u32>);
impl DATA_BUFFER_R {
    pub(crate) fn new(bits: u32) -> Self {
        DATA_BUFFER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_BUFFER_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_BUFFER` writer - Data Buffer"]
pub struct DATA_BUFFER_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BUFFER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data Buffer"]
    #[inline(always)]
    pub fn data_buffer(&self) -> DATA_BUFFER_R {
        DATA_BUFFER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Buffer"]
    #[inline(always)]
    pub fn data_buffer(&mut self) -> DATA_BUFFER_W {
        DATA_BUFFER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_buffer](index.html) module"]
pub struct DATA_BUFFER_SPEC;
impl crate::RegisterSpec for DATA_BUFFER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_buffer::R](R) reader structure"]
impl crate::Readable for DATA_BUFFER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_buffer::W](W) writer structure"]
impl crate::Writable for DATA_BUFFER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA_BUFFER to value 0"]
impl crate::Resettable for DATA_BUFFER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
