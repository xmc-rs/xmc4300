#[doc = "Register `DAC01DATA` reader"]
pub struct R(crate::R<DAC01DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC01DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC01DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC01DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC01DATA` writer"]
pub struct W(crate::W<DAC01DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC01DATA_SPEC>;
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
impl From<crate::W<DAC01DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC01DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA0` reader - DAC0 Data Bits"]
pub struct DATA0_R(crate::FieldReader<u16, u16>);
impl DATA0_R {
    pub(crate) fn new(bits: u16) -> Self {
        DATA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA0` writer - DAC0 Data Bits"]
pub struct DATA0_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `DATA1` reader - DAC1 Data Bits"]
pub struct DATA1_R(crate::FieldReader<u16, u16>);
impl DATA1_R {
    pub(crate) fn new(bits: u16) -> Self {
        DATA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA1` writer - DAC1 Data Bits"]
pub struct DATA1_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - DAC0 Data Bits"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - DAC1 Data Bits"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC0 Data Bits"]
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W {
        DATA0_W { w: self }
    }
    #[doc = "Bits 16:27 - DAC1 Data Bits"]
    #[inline(always)]
    pub fn data1(&mut self) -> DATA1_W {
        DATA1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC01 Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac01data](index.html) module"]
pub struct DAC01DATA_SPEC;
impl crate::RegisterSpec for DAC01DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac01data::R](R) reader structure"]
impl crate::Readable for DAC01DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac01data::W](W) writer structure"]
impl crate::Writable for DAC01DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC01DATA to value 0"]
impl crate::Resettable for DAC01DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
