#[doc = "Register `EEP_CONF` reader"]
pub type R = crate::R<EEP_CONF_SPEC>;
#[doc = "EEPROM control is offered to PDI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type TO_PDI_R = crate::BitReader<TO_PDI_A>;
impl TO_PDI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TO_PDI_A {
        match self.bits {
            false => TO_PDI_A::VALUE1,
            true => TO_PDI_A::VALUE2,
        }
    }
    #[doc = "No"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TO_PDI_A::VALUE1
    }
    #[doc = "Yes (PDI has EEPROM control)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TO_PDI_A::VALUE2
    }
}
#[doc = "Force ECAT access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type FORCE_R = crate::BitReader<FORCE_A>;
impl FORCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FORCE_A {
        match self.bits {
            false => FORCE_A::VALUE1,
            true => FORCE_A::VALUE2,
        }
    }
    #[doc = "Do not change Bit 501.0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FORCE_A::VALUE1
    }
    #[doc = "Reset Bit 501.0 to 0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FORCE_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - EEPROM control is offered to PDI"]
    #[inline(always)]
    pub fn to_pdi(&self) -> TO_PDI_R {
        TO_PDI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force ECAT access"]
    #[inline(always)]
    pub fn force(&self) -> FORCE_R {
        FORCE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "EEPROM Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`eep_conf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EEP_CONF_SPEC;
impl crate::RegisterSpec for EEP_CONF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eep_conf::R`](R) reader structure"]
impl crate::Readable for EEP_CONF_SPEC {}
#[doc = "`reset()` method sets EEP_CONF to value 0"]
impl crate::Resettable for EEP_CONF_SPEC {
    const RESET_VALUE: u8 = 0;
}
