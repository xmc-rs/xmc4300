#[doc = "Register `MII_PHY_ADR` reader"]
pub type R = crate::R<MiiPhyAdrSpec>;
#[doc = "Register `MII_PHY_ADR` writer"]
pub type W = crate::W<MiiPhyAdrSpec>;
#[doc = "Field `PHY_ADDR` reader - PHY Address"]
pub type PhyAddrR = crate::FieldReader;
#[doc = "Field `PHY_ADDR` writer - PHY Address"]
pub type PhyAddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Show configured PHY address of port 0-3 in registerECAT0_MII_CONT_STAT\\[7:3\\]. Select port x with bits \\[4:0\\]
of this register (valid values are 0-3)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhyCaddr {
    #[doc = "0: Show address of port 0 (offset)"]
    Value1 = 0,
    #[doc = "1: Show individual address of port x"]
    Value2 = 1,
}
impl From<PhyCaddr> for bool {
    #[inline(always)]
    fn from(variant: PhyCaddr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHY_CADDR` reader - Show configured PHY address of port 0-3 in registerECAT0_MII_CONT_STAT\\[7:3\\]. Select port x with bits \\[4:0\\]
of this register (valid values are 0-3)"]
pub type PhyCaddrR = crate::BitReader<PhyCaddr>;
impl PhyCaddrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PhyCaddr {
        match self.bits {
            false => PhyCaddr::Value1,
            true => PhyCaddr::Value2,
        }
    }
    #[doc = "Show address of port 0 (offset)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PhyCaddr::Value1
    }
    #[doc = "Show individual address of port x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PhyCaddr::Value2
    }
}
#[doc = "Field `PHY_CADDR` writer - Show configured PHY address of port 0-3 in registerECAT0_MII_CONT_STAT\\[7:3\\]. Select port x with bits \\[4:0\\]
of this register (valid values are 0-3)"]
pub type PhyCaddrW<'a, REG> = crate::BitWriter<'a, REG, PhyCaddr>;
impl<'a, REG> PhyCaddrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Show address of port 0 (offset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PhyCaddr::Value1)
    }
    #[doc = "Show individual address of port x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PhyCaddr::Value2)
    }
}
impl R {
    #[doc = "Bits 0:4 - PHY Address"]
    #[inline(always)]
    pub fn phy_addr(&self) -> PhyAddrR {
        PhyAddrR::new(self.bits & 0x1f)
    }
    #[doc = "Bit 7 - Show configured PHY address of port 0-3 in registerECAT0_MII_CONT_STAT\\[7:3\\]. Select port x with bits \\[4:0\\]
of this register (valid values are 0-3)"]
    #[inline(always)]
    pub fn phy_caddr(&self) -> PhyCaddrR {
        PhyCaddrR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - PHY Address"]
    #[inline(always)]
    #[must_use]
    pub fn phy_addr(&mut self) -> PhyAddrW<MiiPhyAdrSpec> {
        PhyAddrW::new(self, 0)
    }
    #[doc = "Bit 7 - Show configured PHY address of port 0-3 in registerECAT0_MII_CONT_STAT\\[7:3\\]. Select port x with bits \\[4:0\\]
of this register (valid values are 0-3)"]
    #[inline(always)]
    #[must_use]
    pub fn phy_caddr(&mut self) -> PhyCaddrW<MiiPhyAdrSpec> {
        PhyCaddrW::new(self, 7)
    }
}
#[doc = "PHY Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mii_phy_adr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mii_phy_adr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiiPhyAdrSpec;
impl crate::RegisterSpec for MiiPhyAdrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mii_phy_adr::R`](R) reader structure"]
impl crate::Readable for MiiPhyAdrSpec {}
#[doc = "`write(|w| ..)` method takes [`mii_phy_adr::W`](W) writer structure"]
impl crate::Writable for MiiPhyAdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MII_PHY_ADR to value 0"]
impl crate::Resettable for MiiPhyAdrSpec {
    const RESET_VALUE: u8 = 0;
}
