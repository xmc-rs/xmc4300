#[doc = "Register `EEP_CONF` reader"]
pub type R = crate::R<EepConfSpec>;
#[doc = "EEPROM control is offered to PDI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToPdi {
    #[doc = "0: No"]
    Value1 = 0,
    #[doc = "1: Yes (PDI has EEPROM control)"]
    Value2 = 1,
}
impl From<ToPdi> for bool {
    #[inline(always)]
    fn from(variant: ToPdi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TO_PDI` reader - EEPROM control is offered to PDI"]
pub type ToPdiR = crate::BitReader<ToPdi>;
impl ToPdiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ToPdi {
        match self.bits {
            false => ToPdi::Value1,
            true => ToPdi::Value2,
        }
    }
    #[doc = "No"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ToPdi::Value1
    }
    #[doc = "Yes (PDI has EEPROM control)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ToPdi::Value2
    }
}
#[doc = "Force ECAT access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Force {
    #[doc = "0: Do not change Bit 501.0"]
    Value1 = 0,
    #[doc = "1: Reset Bit 501.0 to 0"]
    Value2 = 1,
}
impl From<Force> for bool {
    #[inline(always)]
    fn from(variant: Force) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCE` reader - Force ECAT access"]
pub type ForceR = crate::BitReader<Force>;
impl ForceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Force {
        match self.bits {
            false => Force::Value1,
            true => Force::Value2,
        }
    }
    #[doc = "Do not change Bit 501.0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Force::Value1
    }
    #[doc = "Reset Bit 501.0 to 0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Force::Value2
    }
}
impl R {
    #[doc = "Bit 0 - EEPROM control is offered to PDI"]
    #[inline(always)]
    pub fn to_pdi(&self) -> ToPdiR {
        ToPdiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force ECAT access"]
    #[inline(always)]
    pub fn force(&self) -> ForceR {
        ForceR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "EEPROM Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eep_conf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EepConfSpec;
impl crate::RegisterSpec for EepConfSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eep_conf::R`](R) reader structure"]
impl crate::Readable for EepConfSpec {}
#[doc = "`reset()` method sets EEP_CONF to value 0"]
impl crate::Resettable for EepConfSpec {
    const RESET_VALUE: u8 = 0;
}
