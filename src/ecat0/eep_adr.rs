#[doc = "Register `EEP_ADR` reader"]
pub struct R(crate::R<EEP_ADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEP_ADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEP_ADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEP_ADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEP_ADR` writer"]
pub struct W(crate::W<EEP_ADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEP_ADR_SPEC>;
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
impl From<crate::W<EEP_ADR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEP_ADR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "EEPROM Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum EEPROM_ADDR_A {
    #[doc = "0: First word (= 16 bits)"]
    VALUE1 = 0,
    #[doc = "1: Second word"]
    VALUE2 = 1,
}
impl From<EEPROM_ADDR_A> for u32 {
    #[inline(always)]
    fn from(variant: EEPROM_ADDR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EEPROM_ADDR` reader - EEPROM Address"]
pub struct EEPROM_ADDR_R(crate::FieldReader<u32, EEPROM_ADDR_A>);
impl EEPROM_ADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        EEPROM_ADDR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EEPROM_ADDR_A> {
        match self.bits {
            0 => Some(EEPROM_ADDR_A::VALUE1),
            1 => Some(EEPROM_ADDR_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EEPROM_ADDR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EEPROM_ADDR_A::VALUE2
    }
}
impl core::ops::Deref for EEPROM_ADDR_R {
    type Target = crate::FieldReader<u32, EEPROM_ADDR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEPROM_ADDR` writer - EEPROM Address"]
pub struct EEPROM_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> EEPROM_ADDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEPROM_ADDR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "First word (= 16 bits)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EEPROM_ADDR_A::VALUE1)
    }
    #[doc = "Second word"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EEPROM_ADDR_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - EEPROM Address"]
    #[inline(always)]
    pub fn eeprom_addr(&self) -> EEPROM_ADDR_R {
        EEPROM_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - EEPROM Address"]
    #[inline(always)]
    pub fn eeprom_addr(&mut self) -> EEPROM_ADDR_W {
        EEPROM_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eep_adr](index.html) module"]
pub struct EEP_ADR_SPEC;
impl crate::RegisterSpec for EEP_ADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eep_adr::R](R) reader structure"]
impl crate::Readable for EEP_ADR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eep_adr::W](W) writer structure"]
impl crate::Writable for EEP_ADR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEP_ADR to value 0"]
impl crate::Resettable for EEP_ADR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
