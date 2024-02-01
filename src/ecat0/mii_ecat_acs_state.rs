#[doc = "Register `MII_ECAT_ACS_STATE` reader"]
pub type R = crate::R<MII_ECAT_ACS_STATE_SPEC>;
#[doc = "Field `EN_ACS_MII_BY_PDI` reader - Access to MII management"]
pub type EN_ACS_MII_BY_PDI_R = crate::BitReader<EN_ACS_MII_BY_PDI_A>;
#[doc = "Access to MII management\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl EN_ACS_MII_BY_PDI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN_ACS_MII_BY_PDI_A {
        match self.bits {
            false => EN_ACS_MII_BY_PDI_A::VALUE1,
            true => EN_ACS_MII_BY_PDI_A::VALUE2,
        }
    }
    #[doc = "ECAT enables PDI takeover of MII management control"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EN_ACS_MII_BY_PDI_A::VALUE1
    }
    #[doc = "ECAT claims exclusive access to MII management"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EN_ACS_MII_BY_PDI_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Access to MII management"]
    #[inline(always)]
    pub fn en_acs_mii_by_pdi(&self) -> EN_ACS_MII_BY_PDI_R {
        EN_ACS_MII_BY_PDI_R::new((self.bits & 1) != 0)
    }
}
#[doc = "MII ECAT ACS STATE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mii_ecat_acs_state::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MII_ECAT_ACS_STATE_SPEC;
impl crate::RegisterSpec for MII_ECAT_ACS_STATE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mii_ecat_acs_state::R`](R) reader structure"]
impl crate::Readable for MII_ECAT_ACS_STATE_SPEC {}
#[doc = "`reset()` method sets MII_ECAT_ACS_STATE to value 0"]
impl crate::Resettable for MII_ECAT_ACS_STATE_SPEC {
    const RESET_VALUE: u8 = 0;
}
