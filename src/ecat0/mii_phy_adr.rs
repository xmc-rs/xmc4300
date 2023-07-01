#[doc = "Register `MII_PHY_ADR` reader"]
pub struct R(crate::R<MII_PHY_ADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MII_PHY_ADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MII_PHY_ADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MII_PHY_ADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MII_PHY_ADR` writer"]
pub struct W(crate::W<MII_PHY_ADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MII_PHY_ADR_SPEC>;
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
impl From<crate::W<MII_PHY_ADR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MII_PHY_ADR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHY_ADDR` reader - PHY Address"]
pub type PHY_ADDR_R = crate::FieldReader;
#[doc = "Field `PHY_ADDR` writer - PHY Address"]
pub type PHY_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, MII_PHY_ADR_SPEC, 5, O>;
#[doc = "Field `PHY_CADDR` reader - Show configured PHY address of port 0-3 in registerECAT0_MII_CONT_STAT\\[7:3\\]. Select port x with bits \\[4:0\\]
of this register (valid values are 0-3)"]
pub type PHY_CADDR_R = crate::BitReader<PHY_CADDR_A>;
#[doc = "Show configured PHY address of port 0-3 in registerECAT0_MII_CONT_STAT\\[7:3\\]. Select port x with bits \\[4:0\\]
of this register (valid values are 0-3)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PHY_CADDR_A {
    #[doc = "0: Show address of port 0 (offset)"]
    VALUE1 = 0,
    #[doc = "1: Show individual address of port x"]
    VALUE2 = 1,
}
impl From<PHY_CADDR_A> for bool {
    #[inline(always)]
    fn from(variant: PHY_CADDR_A) -> Self {
        variant as u8 != 0
    }
}
impl PHY_CADDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHY_CADDR_A {
        match self.bits {
            false => PHY_CADDR_A::VALUE1,
            true => PHY_CADDR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PHY_CADDR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PHY_CADDR_A::VALUE2
    }
}
#[doc = "Field `PHY_CADDR` writer - Show configured PHY address of port 0-3 in registerECAT0_MII_CONT_STAT\\[7:3\\]. Select port x with bits \\[4:0\\]
of this register (valid values are 0-3)"]
pub type PHY_CADDR_W<'a, const O: u8> = crate::BitWriter<'a, MII_PHY_ADR_SPEC, O, PHY_CADDR_A>;
impl<'a, const O: u8> PHY_CADDR_W<'a, O> {
    #[doc = "Show address of port 0 (offset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PHY_CADDR_A::VALUE1)
    }
    #[doc = "Show individual address of port x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PHY_CADDR_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:4 - PHY Address"]
    #[inline(always)]
    pub fn phy_addr(&self) -> PHY_ADDR_R {
        PHY_ADDR_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 7 - Show configured PHY address of port 0-3 in registerECAT0_MII_CONT_STAT\\[7:3\\]. Select port x with bits \\[4:0\\]
of this register (valid values are 0-3)"]
    #[inline(always)]
    pub fn phy_caddr(&self) -> PHY_CADDR_R {
        PHY_CADDR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - PHY Address"]
    #[inline(always)]
    #[must_use]
    pub fn phy_addr(&mut self) -> PHY_ADDR_W<0> {
        PHY_ADDR_W::new(self)
    }
    #[doc = "Bit 7 - Show configured PHY address of port 0-3 in registerECAT0_MII_CONT_STAT\\[7:3\\]. Select port x with bits \\[4:0\\]
of this register (valid values are 0-3)"]
    #[inline(always)]
    #[must_use]
    pub fn phy_caddr(&mut self) -> PHY_CADDR_W<7> {
        PHY_CADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mii_phy_adr](index.html) module"]
pub struct MII_PHY_ADR_SPEC;
impl crate::RegisterSpec for MII_PHY_ADR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mii_phy_adr::R](R) reader structure"]
impl crate::Readable for MII_PHY_ADR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mii_phy_adr::W](W) writer structure"]
impl crate::Writable for MII_PHY_ADR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MII_PHY_ADR to value 0"]
impl crate::Resettable for MII_PHY_ADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
