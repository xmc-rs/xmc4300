#[doc = "Register `EEP_DATA[%s]` reader"]
pub struct R(crate::R<EEP_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEP_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEP_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEP_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEP_DATA[%s]` writer"]
pub struct W(crate::W<EEP_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEP_DATA_SPEC>;
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
impl From<crate::W<EEP_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEP_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEP_DATA` reader - EEPROM Data"]
pub type EEP_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `EEP_DATA` writer - EEPROM Data"]
pub type EEP_DATA_W<'a, const O: u8> = crate::FieldWriter<'a, EEP_DATA_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - EEPROM Data"]
    #[inline(always)]
    pub fn eep_data(&self) -> EEP_DATA_R {
        EEP_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - EEPROM Data"]
    #[inline(always)]
    #[must_use]
    pub fn eep_data(&mut self) -> EEP_DATA_W<0> {
        EEP_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Read/Write data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eep_data](index.html) module"]
pub struct EEP_DATA_SPEC;
impl crate::RegisterSpec for EEP_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eep_data::R](R) reader structure"]
impl crate::Readable for EEP_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eep_data::W](W) writer structure"]
impl crate::Writable for EEP_DATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EEP_DATA[%s]
to value 0"]
impl crate::Resettable for EEP_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
