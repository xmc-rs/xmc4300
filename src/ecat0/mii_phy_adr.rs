#[doc = "Reader of register MII_PHY_ADR"]
pub type R = crate::R<u8, super::MII_PHY_ADR>;
#[doc = "Writer for register MII_PHY_ADR"]
pub type W = crate::W<u8, super::MII_PHY_ADR>;
#[doc = "Register MII_PHY_ADR `reset()`'s with value 0"]
impl crate::ResetValue for super::MII_PHY_ADR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PHY_ADDR`"]
pub type PHY_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PHY_ADDR`"]
pub struct PHY_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u8) & 0x1f);
        self.w
    }
}
#[doc = "Show configured PHY address of port 0-3 in registerECAT0_MII_CONT_STAT\\[7:3\\]. Select port x with bits \\[4:0\\] of this register (valid values are 0-3)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHY_CADDR_A {
    #[doc = "0: Show address of port 0 (offset)"]
    VALUE1,
    #[doc = "1: Show individual address of port x"]
    VALUE2,
}
impl From<PHY_CADDR_A> for bool {
    #[inline(always)]
    fn from(variant: PHY_CADDR_A) -> Self {
        match variant {
            PHY_CADDR_A::VALUE1 => false,
            PHY_CADDR_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PHY_CADDR`"]
pub type PHY_CADDR_R = crate::R<bool, PHY_CADDR_A>;
impl PHY_CADDR_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `PHY_CADDR`"]
pub struct PHY_CADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_CADDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHY_CADDR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - PHY Address"]
    #[inline(always)]
    pub fn phy_addr(&self) -> PHY_ADDR_R {
        PHY_ADDR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Show configured PHY address of port 0-3 in registerECAT0_MII_CONT_STAT\\[7:3\\]. Select port x with bits \\[4:0\\] of this register (valid values are 0-3)"]
    #[inline(always)]
    pub fn phy_caddr(&self) -> PHY_CADDR_R {
        PHY_CADDR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - PHY Address"]
    #[inline(always)]
    pub fn phy_addr(&mut self) -> PHY_ADDR_W {
        PHY_ADDR_W { w: self }
    }
    #[doc = "Bit 7 - Show configured PHY address of port 0-3 in registerECAT0_MII_CONT_STAT\\[7:3\\]. Select port x with bits \\[4:0\\] of this register (valid values are 0-3)"]
    #[inline(always)]
    pub fn phy_caddr(&mut self) -> PHY_CADDR_W {
        PHY_CADDR_W { w: self }
    }
}
