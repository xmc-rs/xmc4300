#[doc = "Reader of register MII_PHY_DATA"]
pub type R = crate::R<u16, super::MII_PHY_DATA>;
#[doc = "Writer for register MII_PHY_DATA"]
pub type W = crate::W<u16, super::MII_PHY_DATA>;
#[doc = "Register MII_PHY_DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::MII_PHY_DATA {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PHY_RW_DATA`"]
pub type PHY_RW_DATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PHY_RW_DATA`"]
pub struct PHY_RW_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_RW_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PHY Read/Write Data"]
    #[inline(always)]
    pub fn phy_rw_data(&self) -> PHY_RW_DATA_R {
        PHY_RW_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PHY Read/Write Data"]
    #[inline(always)]
    pub fn phy_rw_data(&mut self) -> PHY_RW_DATA_W {
        PHY_RW_DATA_W { w: self }
    }
}
