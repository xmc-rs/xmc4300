#[doc = "Reader of register MII_PHY_REG_ADR"]
pub type R = crate::R<u8, super::MII_PHY_REG_ADR>;
#[doc = "Writer for register MII_PHY_REG_ADR"]
pub type W = crate::W<u8, super::MII_PHY_REG_ADR>;
#[doc = "Register MII_PHY_REG_ADR `reset()`'s with value 0"]
impl crate::ResetValue for super::MII_PHY_REG_ADR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PHY_REG_ADDR`"]
pub type PHY_REG_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PHY_REG_ADDR`"]
pub struct PHY_REG_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_REG_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u8) & 0x1f);
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
}
