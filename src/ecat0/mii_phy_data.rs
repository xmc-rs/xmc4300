#[doc = "Register `MII_PHY_DATA` reader"]
pub type R = crate::R<MiiPhyDataSpec>;
#[doc = "Register `MII_PHY_DATA` writer"]
pub type W = crate::W<MiiPhyDataSpec>;
#[doc = "Field `PHY_RW_DATA` reader - PHY Read/Write Data"]
pub type PhyRwDataR = crate::FieldReader<u16>;
#[doc = "Field `PHY_RW_DATA` writer - PHY Read/Write Data"]
pub type PhyRwDataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - PHY Read/Write Data"]
    #[inline(always)]
    pub fn phy_rw_data(&self) -> PhyRwDataR {
        PhyRwDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - PHY Read/Write Data"]
    #[inline(always)]
    #[must_use]
    pub fn phy_rw_data(&mut self) -> PhyRwDataW<MiiPhyDataSpec> {
        PhyRwDataW::new(self, 0)
    }
}
#[doc = "PHY Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mii_phy_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mii_phy_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiiPhyDataSpec;
impl crate::RegisterSpec for MiiPhyDataSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mii_phy_data::R`](R) reader structure"]
impl crate::Readable for MiiPhyDataSpec {}
#[doc = "`write(|w| ..)` method takes [`mii_phy_data::W`](W) writer structure"]
impl crate::Writable for MiiPhyDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets MII_PHY_DATA to value 0"]
impl crate::Resettable for MiiPhyDataSpec {
    const RESET_VALUE: u16 = 0;
}
