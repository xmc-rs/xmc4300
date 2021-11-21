#[doc = "Register `MII_ECAT_ACS_STATE` reader"]
pub struct R(crate::R<MII_ECAT_ACS_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MII_ECAT_ACS_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MII_ECAT_ACS_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MII_ECAT_ACS_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Access to MII management\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_ACS_MII_BY_PDI_A {
    #[doc = "0: ECAT enables PDI takeover of MII management control"]
    VALUE1 = 0,
    #[doc = "1: ECAT claims exclusive access to MII management"]
    VALUE2 = 1,
}
impl From<EN_ACS_MII_BY_PDI_A> for bool {
    #[inline(always)]
    fn from(variant: EN_ACS_MII_BY_PDI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN_ACS_MII_BY_PDI` reader - Access to MII management"]
pub struct EN_ACS_MII_BY_PDI_R(crate::FieldReader<bool, EN_ACS_MII_BY_PDI_A>);
impl EN_ACS_MII_BY_PDI_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_ACS_MII_BY_PDI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_ACS_MII_BY_PDI_A {
        match self.bits {
            false => EN_ACS_MII_BY_PDI_A::VALUE1,
            true => EN_ACS_MII_BY_PDI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EN_ACS_MII_BY_PDI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EN_ACS_MII_BY_PDI_A::VALUE2
    }
}
impl core::ops::Deref for EN_ACS_MII_BY_PDI_R {
    type Target = crate::FieldReader<bool, EN_ACS_MII_BY_PDI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Access to MII management"]
    #[inline(always)]
    pub fn en_acs_mii_by_pdi(&self) -> EN_ACS_MII_BY_PDI_R {
        EN_ACS_MII_BY_PDI_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "MII ECAT ACS STATE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mii_ecat_acs_state](index.html) module"]
pub struct MII_ECAT_ACS_STATE_SPEC;
impl crate::RegisterSpec for MII_ECAT_ACS_STATE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mii_ecat_acs_state::R](R) reader structure"]
impl crate::Readable for MII_ECAT_ACS_STATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MII_ECAT_ACS_STATE to value 0"]
impl crate::Resettable for MII_ECAT_ACS_STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
