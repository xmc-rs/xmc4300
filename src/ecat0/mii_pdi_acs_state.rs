#[doc = "Register `MII_PDI_ACS_STATE` reader"]
pub type R = crate::R<MII_PDI_ACS_STATE_SPEC>;
#[doc = "Register `MII_PDI_ACS_STATE` writer"]
pub type W = crate::W<MII_PDI_ACS_STATE_SPEC>;
#[doc = "Field `ACS_MII_BY_PDI` reader - Access to MII management"]
pub type ACS_MII_BY_PDI_R = crate::BitReader<ACS_MII_BY_PDI_A>;
#[doc = "Access to MII management\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACS_MII_BY_PDI_A {
    #[doc = "0: ECAT has access to MII managment"]
    VALUE1 = 0,
    #[doc = "1: PDI has access to MII managment"]
    VALUE2 = 1,
}
impl From<ACS_MII_BY_PDI_A> for bool {
    #[inline(always)]
    fn from(variant: ACS_MII_BY_PDI_A) -> Self {
        variant as u8 != 0
    }
}
impl ACS_MII_BY_PDI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACS_MII_BY_PDI_A {
        match self.bits {
            false => ACS_MII_BY_PDI_A::VALUE1,
            true => ACS_MII_BY_PDI_A::VALUE2,
        }
    }
    #[doc = "ECAT has access to MII managment"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ACS_MII_BY_PDI_A::VALUE1
    }
    #[doc = "PDI has access to MII managment"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ACS_MII_BY_PDI_A::VALUE2
    }
}
#[doc = "Field `ACS_MII_BY_PDI` writer - Access to MII management"]
pub type ACS_MII_BY_PDI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ACS_MII_BY_PDI_A>;
impl<'a, REG, const O: u8> ACS_MII_BY_PDI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ECAT has access to MII managment"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ACS_MII_BY_PDI_A::VALUE1)
    }
    #[doc = "PDI has access to MII managment"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ACS_MII_BY_PDI_A::VALUE2)
    }
}
#[doc = "Field `FORCE_PDI_ACS_S` reader - Force PDI Access State by ECAT master"]
pub type FORCE_PDI_ACS_S_R = crate::BitReader<FORCE_PDI_ACS_S_A>;
#[doc = "Force PDI Access State by ECAT master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORCE_PDI_ACS_S_A {
    #[doc = "0: no change"]
    VALUE1 = 0,
    #[doc = "1: Reset Bit ACS_MII_BY_PDI"]
    VALUE2 = 1,
}
impl From<FORCE_PDI_ACS_S_A> for bool {
    #[inline(always)]
    fn from(variant: FORCE_PDI_ACS_S_A) -> Self {
        variant as u8 != 0
    }
}
impl FORCE_PDI_ACS_S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FORCE_PDI_ACS_S_A {
        match self.bits {
            false => FORCE_PDI_ACS_S_A::VALUE1,
            true => FORCE_PDI_ACS_S_A::VALUE2,
        }
    }
    #[doc = "no change"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FORCE_PDI_ACS_S_A::VALUE1
    }
    #[doc = "Reset Bit ACS_MII_BY_PDI"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FORCE_PDI_ACS_S_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Access to MII management"]
    #[inline(always)]
    pub fn acs_mii_by_pdi(&self) -> ACS_MII_BY_PDI_R {
        ACS_MII_BY_PDI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force PDI Access State by ECAT master"]
    #[inline(always)]
    pub fn force_pdi_acs_s(&self) -> FORCE_PDI_ACS_S_R {
        FORCE_PDI_ACS_S_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access to MII management"]
    #[inline(always)]
    #[must_use]
    pub fn acs_mii_by_pdi(&mut self) -> ACS_MII_BY_PDI_W<MII_PDI_ACS_STATE_SPEC, 0> {
        ACS_MII_BY_PDI_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MII PDI ACS STATE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mii_pdi_acs_state::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mii_pdi_acs_state::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MII_PDI_ACS_STATE_SPEC;
impl crate::RegisterSpec for MII_PDI_ACS_STATE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mii_pdi_acs_state::R`](R) reader structure"]
impl crate::Readable for MII_PDI_ACS_STATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mii_pdi_acs_state::W`](W) writer structure"]
impl crate::Writable for MII_PDI_ACS_STATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MII_PDI_ACS_STATE to value 0"]
impl crate::Resettable for MII_PDI_ACS_STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
