#[doc = "Register `MII_PDI_ACS_STATE` reader"]
pub type R = crate::R<MiiPdiAcsStateSpec>;
#[doc = "Register `MII_PDI_ACS_STATE` writer"]
pub type W = crate::W<MiiPdiAcsStateSpec>;
#[doc = "Access to MII management\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AcsMiiByPdi {
    #[doc = "0: ECAT has access to MII managment"]
    Value1 = 0,
    #[doc = "1: PDI has access to MII managment"]
    Value2 = 1,
}
impl From<AcsMiiByPdi> for bool {
    #[inline(always)]
    fn from(variant: AcsMiiByPdi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACS_MII_BY_PDI` reader - Access to MII management"]
pub type AcsMiiByPdiR = crate::BitReader<AcsMiiByPdi>;
impl AcsMiiByPdiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AcsMiiByPdi {
        match self.bits {
            false => AcsMiiByPdi::Value1,
            true => AcsMiiByPdi::Value2,
        }
    }
    #[doc = "ECAT has access to MII managment"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AcsMiiByPdi::Value1
    }
    #[doc = "PDI has access to MII managment"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AcsMiiByPdi::Value2
    }
}
#[doc = "Field `ACS_MII_BY_PDI` writer - Access to MII management"]
pub type AcsMiiByPdiW<'a, REG> = crate::BitWriter<'a, REG, AcsMiiByPdi>;
impl<'a, REG> AcsMiiByPdiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ECAT has access to MII managment"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AcsMiiByPdi::Value1)
    }
    #[doc = "PDI has access to MII managment"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AcsMiiByPdi::Value2)
    }
}
#[doc = "Force PDI Access State by ECAT master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForcePdiAcsS {
    #[doc = "0: no change"]
    Value1 = 0,
    #[doc = "1: Reset Bit ACS_MII_BY_PDI"]
    Value2 = 1,
}
impl From<ForcePdiAcsS> for bool {
    #[inline(always)]
    fn from(variant: ForcePdiAcsS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCE_PDI_ACS_S` reader - Force PDI Access State by ECAT master"]
pub type ForcePdiAcsSR = crate::BitReader<ForcePdiAcsS>;
impl ForcePdiAcsSR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ForcePdiAcsS {
        match self.bits {
            false => ForcePdiAcsS::Value1,
            true => ForcePdiAcsS::Value2,
        }
    }
    #[doc = "no change"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ForcePdiAcsS::Value1
    }
    #[doc = "Reset Bit ACS_MII_BY_PDI"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ForcePdiAcsS::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Access to MII management"]
    #[inline(always)]
    pub fn acs_mii_by_pdi(&self) -> AcsMiiByPdiR {
        AcsMiiByPdiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force PDI Access State by ECAT master"]
    #[inline(always)]
    pub fn force_pdi_acs_s(&self) -> ForcePdiAcsSR {
        ForcePdiAcsSR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access to MII management"]
    #[inline(always)]
    #[must_use]
    pub fn acs_mii_by_pdi(&mut self) -> AcsMiiByPdiW<MiiPdiAcsStateSpec> {
        AcsMiiByPdiW::new(self, 0)
    }
}
#[doc = "MII PDI ACS STATE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mii_pdi_acs_state::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mii_pdi_acs_state::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiiPdiAcsStateSpec;
impl crate::RegisterSpec for MiiPdiAcsStateSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mii_pdi_acs_state::R`](R) reader structure"]
impl crate::Readable for MiiPdiAcsStateSpec {}
#[doc = "`write(|w| ..)` method takes [`mii_pdi_acs_state::W`](W) writer structure"]
impl crate::Writable for MiiPdiAcsStateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MII_PDI_ACS_STATE to value 0"]
impl crate::Resettable for MiiPdiAcsStateSpec {
    const RESET_VALUE: u8 = 0;
}
