#[doc = "Register `MII_PDI_ACS_STATE` reader"]
pub struct R(crate::R<MII_PDI_ACS_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MII_PDI_ACS_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MII_PDI_ACS_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MII_PDI_ACS_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MII_PDI_ACS_STATE` writer"]
pub struct W(crate::W<MII_PDI_ACS_STATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MII_PDI_ACS_STATE_SPEC>;
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
impl From<crate::W<MII_PDI_ACS_STATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MII_PDI_ACS_STATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Access to MII management\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `ACS_MII_BY_PDI` reader - Access to MII management"]
pub struct ACS_MII_BY_PDI_R(crate::FieldReader<bool, ACS_MII_BY_PDI_A>);
impl ACS_MII_BY_PDI_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACS_MII_BY_PDI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACS_MII_BY_PDI_A {
        match self.bits {
            false => ACS_MII_BY_PDI_A::VALUE1,
            true => ACS_MII_BY_PDI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ACS_MII_BY_PDI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ACS_MII_BY_PDI_A::VALUE2
    }
}
impl core::ops::Deref for ACS_MII_BY_PDI_R {
    type Target = crate::FieldReader<bool, ACS_MII_BY_PDI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACS_MII_BY_PDI` writer - Access to MII management"]
pub struct ACS_MII_BY_PDI_W<'a> {
    w: &'a mut W,
}
impl<'a> ACS_MII_BY_PDI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACS_MII_BY_PDI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ECAT has access to MII managment"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ACS_MII_BY_PDI_A::VALUE1)
    }
    #[doc = "PDI has access to MII managment"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ACS_MII_BY_PDI_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Force PDI Access State by ECAT master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `FORCE_PDI_ACS_S` reader - Force PDI Access State by ECAT master"]
pub struct FORCE_PDI_ACS_S_R(crate::FieldReader<bool, FORCE_PDI_ACS_S_A>);
impl FORCE_PDI_ACS_S_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_PDI_ACS_S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCE_PDI_ACS_S_A {
        match self.bits {
            false => FORCE_PDI_ACS_S_A::VALUE1,
            true => FORCE_PDI_ACS_S_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == FORCE_PDI_ACS_S_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FORCE_PDI_ACS_S_A::VALUE2
    }
}
impl core::ops::Deref for FORCE_PDI_ACS_S_R {
    type Target = crate::FieldReader<bool, FORCE_PDI_ACS_S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Access to MII management"]
    #[inline(always)]
    pub fn acs_mii_by_pdi(&self) -> ACS_MII_BY_PDI_R {
        ACS_MII_BY_PDI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Force PDI Access State by ECAT master"]
    #[inline(always)]
    pub fn force_pdi_acs_s(&self) -> FORCE_PDI_ACS_S_R {
        FORCE_PDI_ACS_S_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access to MII management"]
    #[inline(always)]
    pub fn acs_mii_by_pdi(&mut self) -> ACS_MII_BY_PDI_W {
        ACS_MII_BY_PDI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MII PDI ACS STATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mii_pdi_acs_state](index.html) module"]
pub struct MII_PDI_ACS_STATE_SPEC;
impl crate::RegisterSpec for MII_PDI_ACS_STATE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mii_pdi_acs_state::R](R) reader structure"]
impl crate::Readable for MII_PDI_ACS_STATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mii_pdi_acs_state::W](W) writer structure"]
impl crate::Writable for MII_PDI_ACS_STATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MII_PDI_ACS_STATE to value 0"]
impl crate::Resettable for MII_PDI_ACS_STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
