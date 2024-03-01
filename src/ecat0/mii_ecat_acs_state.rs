#[doc = "Register `MII_ECAT_ACS_STATE` reader"]
pub type R = crate::R<MiiEcatAcsStateSpec>;
#[doc = "Access to MII management\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnAcsMiiByPdi {
    #[doc = "0: ECAT enables PDI takeover of MII management control"]
    Value1 = 0,
    #[doc = "1: ECAT claims exclusive access to MII management"]
    Value2 = 1,
}
impl From<EnAcsMiiByPdi> for bool {
    #[inline(always)]
    fn from(variant: EnAcsMiiByPdi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN_ACS_MII_BY_PDI` reader - Access to MII management"]
pub type EnAcsMiiByPdiR = crate::BitReader<EnAcsMiiByPdi>;
impl EnAcsMiiByPdiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnAcsMiiByPdi {
        match self.bits {
            false => EnAcsMiiByPdi::Value1,
            true => EnAcsMiiByPdi::Value2,
        }
    }
    #[doc = "ECAT enables PDI takeover of MII management control"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EnAcsMiiByPdi::Value1
    }
    #[doc = "ECAT claims exclusive access to MII management"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EnAcsMiiByPdi::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Access to MII management"]
    #[inline(always)]
    pub fn en_acs_mii_by_pdi(&self) -> EnAcsMiiByPdiR {
        EnAcsMiiByPdiR::new((self.bits & 1) != 0)
    }
}
#[doc = "MII ECAT ACS STATE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mii_ecat_acs_state::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiiEcatAcsStateSpec;
impl crate::RegisterSpec for MiiEcatAcsStateSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mii_ecat_acs_state::R`](R) reader structure"]
impl crate::Readable for MiiEcatAcsStateSpec {}
#[doc = "`reset()` method sets MII_ECAT_ACS_STATE to value 0"]
impl crate::Resettable for MiiEcatAcsStateSpec {
    const RESET_VALUE: u8 = 0;
}
