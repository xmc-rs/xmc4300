#[doc = "Register `MII_PHY_DATA` reader"]
pub type R = crate::R<MII_PHY_DATA_SPEC>;
#[doc = "Register `MII_PHY_DATA` writer"]
pub type W = crate::W<MII_PHY_DATA_SPEC>;
#[doc = "Field `PHY_RW_DATA` reader - PHY Read/Write Data"]
pub type PHY_RW_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RW_DATA` writer - PHY Read/Write Data"]
pub type PHY_RW_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - PHY Read/Write Data"]
    #[inline(always)]
    pub fn phy_rw_data(&self) -> PHY_RW_DATA_R {
        PHY_RW_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - PHY Read/Write Data"]
    #[inline(always)]
    #[must_use]
    pub fn phy_rw_data(&mut self) -> PHY_RW_DATA_W<MII_PHY_DATA_SPEC> {
        PHY_RW_DATA_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PHY Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mii_phy_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mii_phy_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MII_PHY_DATA_SPEC;
impl crate::RegisterSpec for MII_PHY_DATA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mii_phy_data::R`](R) reader structure"]
impl crate::Readable for MII_PHY_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mii_phy_data::W`](W) writer structure"]
impl crate::Writable for MII_PHY_DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MII_PHY_DATA to value 0"]
impl crate::Resettable for MII_PHY_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
