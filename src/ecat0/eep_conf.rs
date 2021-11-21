#[doc = "Register `EEP_CONF` reader"]
pub struct R(crate::R<EEP_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEP_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEP_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEP_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "EEPROM control is offered to PDI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TO_PDI_A {
    #[doc = "0: No"]
    VALUE1 = 0,
    #[doc = "1: Yes (PDI has EEPROM control)"]
    VALUE2 = 1,
}
impl From<TO_PDI_A> for bool {
    #[inline(always)]
    fn from(variant: TO_PDI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TO_PDI` reader - EEPROM control is offered to PDI"]
pub struct TO_PDI_R(crate::FieldReader<bool, TO_PDI_A>);
impl TO_PDI_R {
    pub(crate) fn new(bits: bool) -> Self {
        TO_PDI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TO_PDI_A {
        match self.bits {
            false => TO_PDI_A::VALUE1,
            true => TO_PDI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TO_PDI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TO_PDI_A::VALUE2
    }
}
impl core::ops::Deref for TO_PDI_R {
    type Target = crate::FieldReader<bool, TO_PDI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Force ECAT access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCE_A {
    #[doc = "0: Do not change Bit 501.0"]
    VALUE1 = 0,
    #[doc = "1: Reset Bit 501.0 to 0"]
    VALUE2 = 1,
}
impl From<FORCE_A> for bool {
    #[inline(always)]
    fn from(variant: FORCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCE` reader - Force ECAT access"]
pub struct FORCE_R(crate::FieldReader<bool, FORCE_A>);
impl FORCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCE_A {
        match self.bits {
            false => FORCE_A::VALUE1,
            true => FORCE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == FORCE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FORCE_A::VALUE2
    }
}
impl core::ops::Deref for FORCE_R {
    type Target = crate::FieldReader<bool, FORCE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - EEPROM control is offered to PDI"]
    #[inline(always)]
    pub fn to_pdi(&self) -> TO_PDI_R {
        TO_PDI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Force ECAT access"]
    #[inline(always)]
    pub fn force(&self) -> FORCE_R {
        FORCE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "EEPROM Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eep_conf](index.html) module"]
pub struct EEP_CONF_SPEC;
impl crate::RegisterSpec for EEP_CONF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [eep_conf::R](R) reader structure"]
impl crate::Readable for EEP_CONF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EEP_CONF to value 0"]
impl crate::Resettable for EEP_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
