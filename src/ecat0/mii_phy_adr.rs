#[doc = "Register `MII_PHY_ADR` reader"]
pub type R = crate::R<MII_PHY_ADR_SPEC>;
#[doc = "Register `MII_PHY_ADR` writer"]
pub type W = crate::W<MII_PHY_ADR_SPEC>;
#[doc = "Field `PHY_ADDR` reader - PHY Address"]
pub type PHY_ADDR_R = crate::FieldReader;
#[doc = "Field `PHY_ADDR` writer - PHY Address"]
pub type PHY_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
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
#[doc = "Field `PHY_CADDR` reader - Show configured PHY address of port 0-3 in registerECAT0_MII_CONT_STAT\\[7:3\\]. Select port x with bits \\[4:0\\]
of this register (valid values are 0-3)"]
pub type PHY_CADDR_R = crate::BitReader<PHY_CADDR_A>;
impl PHY_CADDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PHY_CADDR_A {
        match self.bits {
            false => PHY_CADDR_A::VALUE1,
            true => PHY_CADDR_A::VALUE2,
        }
    }
    #[doc = "Show address of port 0 (offset)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PHY_CADDR_A::VALUE1
    }
    #[doc = "Show individual address of port x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PHY_CADDR_A::VALUE2
    }
}
#[doc = "Field `PHY_CADDR` writer - Show configured PHY address of port 0-3 in registerECAT0_MII_CONT_STAT\\[7:3\\]. Select port x with bits \\[4:0\\]
of this register (valid values are 0-3)"]
pub type PHY_CADDR_W<'a, REG> = crate::BitWriter<'a, REG, PHY_CADDR_A>;
impl<'a, REG> PHY_CADDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Show address of port 0 (offset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PHY_CADDR_A::VALUE1)
    }
    #[doc = "Show individual address of port x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
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
    pub fn phy_addr(&mut self) -> PHY_ADDR_W<MII_PHY_ADR_SPEC> {
        PHY_ADDR_W::new(self, 0)
    }
    #[doc = "Bit 7 - Show configured PHY address of port 0-3 in registerECAT0_MII_CONT_STAT\\[7:3\\]. Select port x with bits \\[4:0\\]
of this register (valid values are 0-3)"]
    #[inline(always)]
    #[must_use]
    pub fn phy_caddr(&mut self) -> PHY_CADDR_W<MII_PHY_ADR_SPEC> {
        PHY_CADDR_W::new(self, 7)
    }
}
#[doc = "PHY Address\n\nYou can [`read`](crate::Reg::read) this register and get [`mii_phy_adr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mii_phy_adr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MII_PHY_ADR_SPEC;
impl crate::RegisterSpec for MII_PHY_ADR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mii_phy_adr::R`](R) reader structure"]
impl crate::Readable for MII_PHY_ADR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mii_phy_adr::W`](W) writer structure"]
impl crate::Writable for MII_PHY_ADR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MII_PHY_ADR to value 0"]
impl crate::Resettable for MII_PHY_ADR_SPEC {
    const RESET_VALUE: u8 = 0;
}
