#[doc = "Register `MII_PHY_REG_ADR` reader"]
pub struct R(crate::R<MII_PHY_REG_ADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MII_PHY_REG_ADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MII_PHY_REG_ADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MII_PHY_REG_ADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MII_PHY_REG_ADR` writer"]
pub struct W(crate::W<MII_PHY_REG_ADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MII_PHY_REG_ADR_SPEC>;
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
impl From<crate::W<MII_PHY_REG_ADR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MII_PHY_REG_ADR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHY_REG_ADDR` reader - Address of PHY Register that shall beread/written"]
pub struct PHY_REG_ADDR_R(crate::FieldReader<u8, u8>);
impl PHY_REG_ADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PHY_REG_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHY_REG_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHY_REG_ADDR` writer - Address of PHY Register that shall beread/written"]
pub struct PHY_REG_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_REG_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u8 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Address of PHY Register that shall beread/written"]
    #[inline(always)]
    pub fn phy_reg_addr(&self) -> PHY_REG_ADDR_R {
        PHY_REG_ADDR_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Address of PHY Register that shall beread/written"]
    #[inline(always)]
    pub fn phy_reg_addr(&mut self) -> PHY_REG_ADDR_W {
        PHY_REG_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Register Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mii_phy_reg_adr](index.html) module"]
pub struct MII_PHY_REG_ADR_SPEC;
impl crate::RegisterSpec for MII_PHY_REG_ADR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mii_phy_reg_adr::R](R) reader structure"]
impl crate::Readable for MII_PHY_REG_ADR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mii_phy_reg_adr::W](W) writer structure"]
impl crate::Writable for MII_PHY_REG_ADR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MII_PHY_REG_ADR to value 0"]
impl crate::Resettable for MII_PHY_REG_ADR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
