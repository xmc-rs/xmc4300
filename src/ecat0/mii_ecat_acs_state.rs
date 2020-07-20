#[doc = "Reader of register MII_ECAT_ACS_STATE"]
pub type R = crate::R<u8, super::MII_ECAT_ACS_STATE>;
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
#[doc = "Reader of field `EN_ACS_MII_BY_PDI`"]
pub type EN_ACS_MII_BY_PDI_R = crate::R<bool, EN_ACS_MII_BY_PDI_A>;
impl EN_ACS_MII_BY_PDI_R {
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
        *self == EN_ACS_MII_BY_PDI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EN_ACS_MII_BY_PDI_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Access to MII management"]
    #[inline(always)]
    pub fn en_acs_mii_by_pdi(&self) -> EN_ACS_MII_BY_PDI_R {
        EN_ACS_MII_BY_PDI_R::new((self.bits & 0x01) != 0)
    }
}
