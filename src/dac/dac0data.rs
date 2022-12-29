#[doc = "Register `DAC0DATA` reader"]
pub struct R(crate::R<DAC0DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC0DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC0DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC0DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC0DATA` writer"]
pub struct W(crate::W<DAC0DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC0DATA_SPEC>;
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
impl From<crate::W<DAC0DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC0DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA0` reader - DAC0 Data Bits"]
pub type DATA0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATA0` writer - DAC0 Data Bits"]
pub type DATA0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC0DATA_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - DAC0 Data Bits"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC0 Data Bits"]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> DATA0_W<0> {
        DATA0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC0 Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac0data](index.html) module"]
pub struct DAC0DATA_SPEC;
impl crate::RegisterSpec for DAC0DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac0data::R](R) reader structure"]
impl crate::Readable for DAC0DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac0data::W](W) writer structure"]
impl crate::Writable for DAC0DATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC0DATA to value 0"]
impl crate::Resettable for DAC0DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
