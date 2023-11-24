#[doc = "Register `MII_PHY_REG_ADR` reader"]
pub type R = crate::R<MII_PHY_REG_ADR_SPEC>;
#[doc = "Register `MII_PHY_REG_ADR` writer"]
pub type W = crate::W<MII_PHY_REG_ADR_SPEC>;
#[doc = "Field `PHY_REG_ADDR` reader - Address of PHY Register that shall beread/written"]
pub type PHY_REG_ADDR_R = crate::FieldReader;
#[doc = "Field `PHY_REG_ADDR` writer - Address of PHY Register that shall beread/written"]
pub type PHY_REG_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Address of PHY Register that shall beread/written"]
    #[inline(always)]
    pub fn phy_reg_addr(&self) -> PHY_REG_ADDR_R {
        PHY_REG_ADDR_R::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Address of PHY Register that shall beread/written"]
    #[inline(always)]
    #[must_use]
    pub fn phy_reg_addr(&mut self) -> PHY_REG_ADDR_W<MII_PHY_REG_ADR_SPEC> {
        PHY_REG_ADDR_W::new(self, 0)
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
#[doc = "PHY Register Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mii_phy_reg_adr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mii_phy_reg_adr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MII_PHY_REG_ADR_SPEC;
impl crate::RegisterSpec for MII_PHY_REG_ADR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mii_phy_reg_adr::R`](R) reader structure"]
impl crate::Readable for MII_PHY_REG_ADR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mii_phy_reg_adr::W`](W) writer structure"]
impl crate::Writable for MII_PHY_REG_ADR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MII_PHY_REG_ADR to value 0"]
impl crate::Resettable for MII_PHY_REG_ADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
