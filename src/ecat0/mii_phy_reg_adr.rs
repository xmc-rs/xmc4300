#[doc = "Register `MII_PHY_REG_ADR` reader"]
pub type R = crate::R<MiiPhyRegAdrSpec>;
#[doc = "Register `MII_PHY_REG_ADR` writer"]
pub type W = crate::W<MiiPhyRegAdrSpec>;
#[doc = "Field `PHY_REG_ADDR` reader - Address of PHY Register that shall beread/written"]
pub type PhyRegAddrR = crate::FieldReader;
#[doc = "Field `PHY_REG_ADDR` writer - Address of PHY Register that shall beread/written"]
pub type PhyRegAddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Address of PHY Register that shall beread/written"]
    #[inline(always)]
    pub fn phy_reg_addr(&self) -> PhyRegAddrR {
        PhyRegAddrR::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Address of PHY Register that shall beread/written"]
    #[inline(always)]
    #[must_use]
    pub fn phy_reg_addr(&mut self) -> PhyRegAddrW<MiiPhyRegAdrSpec> {
        PhyRegAddrW::new(self, 0)
    }
}
#[doc = "PHY Register Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mii_phy_reg_adr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mii_phy_reg_adr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiiPhyRegAdrSpec;
impl crate::RegisterSpec for MiiPhyRegAdrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mii_phy_reg_adr::R`](R) reader structure"]
impl crate::Readable for MiiPhyRegAdrSpec {}
#[doc = "`write(|w| ..)` method takes [`mii_phy_reg_adr::W`](W) writer structure"]
impl crate::Writable for MiiPhyRegAdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MII_PHY_REG_ADR to value 0"]
impl crate::Resettable for MiiPhyRegAdrSpec {
    const RESET_VALUE: u8 = 0;
}
