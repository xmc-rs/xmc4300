#[doc = "Reader of register MMC_TRANSMIT_INTERRUPT_MASK"]
pub type R = crate::R<u32, super::MMC_TRANSMIT_INTERRUPT_MASK>;
#[doc = "Writer for register MMC_TRANSMIT_INTERRUPT_MASK"]
pub type W = crate::W<u32, super::MMC_TRANSMIT_INTERRUPT_MASK>;
#[doc = "Register MMC_TRANSMIT_INTERRUPT_MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::MMC_TRANSMIT_INTERRUPT_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXGBOCTIM`"]
pub type TXGBOCTIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXGBOCTIM`"]
pub struct TXGBOCTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXGBOCTIM_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `TXGBFRMIM`"]
pub type TXGBFRMIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXGBFRMIM`"]
pub struct TXGBFRMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXGBFRMIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TXBCGFIM`"]
pub type TXBCGFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXBCGFIM`"]
pub struct TXBCGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBCGFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TXMCGFIM`"]
pub type TXMCGFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXMCGFIM`"]
pub struct TXMCGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMCGFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TX64OCTGBFIM`"]
pub type TX64OCTGBFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX64OCTGBFIM`"]
pub struct TX64OCTGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX64OCTGBFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TX65T127OCTGBFIM`"]
pub type TX65T127OCTGBFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX65T127OCTGBFIM`"]
pub struct TX65T127OCTGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX65T127OCTGBFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `TX128T255OCTGBFIM`"]
pub type TX128T255OCTGBFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX128T255OCTGBFIM`"]
pub struct TX128T255OCTGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX128T255OCTGBFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `TX256T511OCTGBFIM`"]
pub type TX256T511OCTGBFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX256T511OCTGBFIM`"]
pub struct TX256T511OCTGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX256T511OCTGBFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `TX512T1023OCTGBFIM`"]
pub type TX512T1023OCTGBFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX512T1023OCTGBFIM`"]
pub struct TX512T1023OCTGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX512T1023OCTGBFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TX1024TMAXOCTGBFIM`"]
pub type TX1024TMAXOCTGBFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX1024TMAXOCTGBFIM`"]
pub struct TX1024TMAXOCTGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX1024TMAXOCTGBFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TXUCGBFIM`"]
pub type TXUCGBFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXUCGBFIM`"]
pub struct TXUCGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUCGBFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TXMCGBFIM`"]
pub type TXMCGBFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXMCGBFIM`"]
pub struct TXMCGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMCGBFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `TXBCGBFIM`"]
pub type TXBCGBFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXBCGBFIM`"]
pub struct TXBCGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBCGBFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TXUFLOWERFIM`"]
pub type TXUFLOWERFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXUFLOWERFIM`"]
pub struct TXUFLOWERFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUFLOWERFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `TXSCOLGFIM`"]
pub type TXSCOLGFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXSCOLGFIM`"]
pub struct TXSCOLGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSCOLGFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `TXMCOLGFIM`"]
pub type TXMCOLGFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXMCOLGFIM`"]
pub struct TXMCOLGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMCOLGFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `TXDEFFIM`"]
pub type TXDEFFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXDEFFIM`"]
pub struct TXDEFFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDEFFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `TXLATCOLFIM`"]
pub type TXLATCOLFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXLATCOLFIM`"]
pub struct TXLATCOLFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXLATCOLFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `TXEXCOLFIM`"]
pub type TXEXCOLFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXEXCOLFIM`"]
pub struct TXEXCOLFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEXCOLFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `TXCARERFIM`"]
pub type TXCARERFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXCARERFIM`"]
pub struct TXCARERFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCARERFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `TXGOCTIM`"]
pub type TXGOCTIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXGOCTIM`"]
pub struct TXGOCTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXGOCTIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `TXGFRMIM`"]
pub type TXGFRMIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXGFRMIM`"]
pub struct TXGFRMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXGFRMIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `TXEXDEFFIM`"]
pub type TXEXDEFFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXEXDEFFIM`"]
pub struct TXEXDEFFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEXDEFFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `TXPAUSFIM`"]
pub type TXPAUSFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXPAUSFIM`"]
pub struct TXPAUSFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPAUSFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `TXVLANGFIM`"]
pub type TXVLANGFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXVLANGFIM`"]
pub struct TXVLANGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXVLANGFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `TXOSIZEGFIM`"]
pub type TXOSIZEGFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXOSIZEGFIM`"]
pub struct TXOSIZEGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOSIZEGFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - MMC Transmit Good Bad Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgboctim(&self) -> TXGBOCTIM_R {
        TXGBOCTIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MMC Transmit Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgbfrmim(&self) -> TXGBFRMIM_R {
        TXGBFRMIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MMC Transmit Broadcast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txbcgfim(&self) -> TXBCGFIM_R {
        TXBCGFIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MMC Transmit Multicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcgfim(&self) -> TXMCGFIM_R {
        TXMCGFIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx64octgbfim(&self) -> TX64OCTGBFIM_R {
        TX64OCTGBFIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx65t127octgbfim(&self) -> TX65T127OCTGBFIM_R {
        TX65T127OCTGBFIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx128t255octgbfim(&self) -> TX128T255OCTGBFIM_R {
        TX128T255OCTGBFIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx256t511octgbfim(&self) -> TX256T511OCTGBFIM_R {
        TX256T511OCTGBFIM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx512t1023octgbfim(&self) -> TX512T1023OCTGBFIM_R {
        TX512T1023OCTGBFIM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx1024tmaxoctgbfim(&self) -> TX1024TMAXOCTGBFIM_R {
        TX1024TMAXOCTGBFIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MMC Transmit Unicast Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txucgbfim(&self) -> TXUCGBFIM_R {
        TXUCGBFIM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MMC Transmit Multicast Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcgbfim(&self) -> TXMCGBFIM_R {
        TXMCGBFIM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txbcgbfim(&self) -> TXBCGBFIM_R {
        TXBCGBFIM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MMC Transmit Underflow Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txuflowerfim(&self) -> TXUFLOWERFIM_R {
        TXUFLOWERFIM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txscolgfim(&self) -> TXSCOLGFIM_R {
        TXSCOLGFIM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcolgfim(&self) -> TXMCOLGFIM_R {
        TXMCOLGFIM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - MMC Transmit Deferred Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txdeffim(&self) -> TXDEFFIM_R {
        TXDEFFIM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MMC Transmit Late Collision Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txlatcolfim(&self) -> TXLATCOLFIM_R {
        TXLATCOLFIM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - MMC Transmit Excessive Collision Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txexcolfim(&self) -> TXEXCOLFIM_R {
        TXEXCOLFIM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - MMC Transmit Carrier Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txcarerfim(&self) -> TXCARERFIM_R {
        TXCARERFIM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - MMC Transmit Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgoctim(&self) -> TXGOCTIM_R {
        TXGOCTIM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - MMC Transmit Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgfrmim(&self) -> TXGFRMIM_R {
        TXGFRMIM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - MMC Transmit Excessive Deferral Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txexdeffim(&self) -> TXEXDEFFIM_R {
        TXEXDEFFIM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - MMC Transmit Pause Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txpausfim(&self) -> TXPAUSFIM_R {
        TXPAUSFIM_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - MMC Transmit VLAN Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txvlangfim(&self) -> TXVLANGFIM_R {
        TXVLANGFIM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - MMC Transmit Oversize Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txosizegfim(&self) -> TXOSIZEGFIM_R {
        TXOSIZEGFIM_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MMC Transmit Good Bad Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgboctim(&mut self) -> TXGBOCTIM_W {
        TXGBOCTIM_W { w: self }
    }
    #[doc = "Bit 1 - MMC Transmit Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgbfrmim(&mut self) -> TXGBFRMIM_W {
        TXGBFRMIM_W { w: self }
    }
    #[doc = "Bit 2 - MMC Transmit Broadcast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txbcgfim(&mut self) -> TXBCGFIM_W {
        TXBCGFIM_W { w: self }
    }
    #[doc = "Bit 3 - MMC Transmit Multicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcgfim(&mut self) -> TXMCGFIM_W {
        TXMCGFIM_W { w: self }
    }
    #[doc = "Bit 4 - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx64octgbfim(&mut self) -> TX64OCTGBFIM_W {
        TX64OCTGBFIM_W { w: self }
    }
    #[doc = "Bit 5 - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx65t127octgbfim(&mut self) -> TX65T127OCTGBFIM_W {
        TX65T127OCTGBFIM_W { w: self }
    }
    #[doc = "Bit 6 - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx128t255octgbfim(&mut self) -> TX128T255OCTGBFIM_W {
        TX128T255OCTGBFIM_W { w: self }
    }
    #[doc = "Bit 7 - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx256t511octgbfim(&mut self) -> TX256T511OCTGBFIM_W {
        TX256T511OCTGBFIM_W { w: self }
    }
    #[doc = "Bit 8 - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx512t1023octgbfim(&mut self) -> TX512T1023OCTGBFIM_W {
        TX512T1023OCTGBFIM_W { w: self }
    }
    #[doc = "Bit 9 - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx1024tmaxoctgbfim(&mut self) -> TX1024TMAXOCTGBFIM_W {
        TX1024TMAXOCTGBFIM_W { w: self }
    }
    #[doc = "Bit 10 - MMC Transmit Unicast Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txucgbfim(&mut self) -> TXUCGBFIM_W {
        TXUCGBFIM_W { w: self }
    }
    #[doc = "Bit 11 - MMC Transmit Multicast Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcgbfim(&mut self) -> TXMCGBFIM_W {
        TXMCGBFIM_W { w: self }
    }
    #[doc = "Bit 12 - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txbcgbfim(&mut self) -> TXBCGBFIM_W {
        TXBCGBFIM_W { w: self }
    }
    #[doc = "Bit 13 - MMC Transmit Underflow Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txuflowerfim(&mut self) -> TXUFLOWERFIM_W {
        TXUFLOWERFIM_W { w: self }
    }
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txscolgfim(&mut self) -> TXSCOLGFIM_W {
        TXSCOLGFIM_W { w: self }
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcolgfim(&mut self) -> TXMCOLGFIM_W {
        TXMCOLGFIM_W { w: self }
    }
    #[doc = "Bit 16 - MMC Transmit Deferred Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txdeffim(&mut self) -> TXDEFFIM_W {
        TXDEFFIM_W { w: self }
    }
    #[doc = "Bit 17 - MMC Transmit Late Collision Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txlatcolfim(&mut self) -> TXLATCOLFIM_W {
        TXLATCOLFIM_W { w: self }
    }
    #[doc = "Bit 18 - MMC Transmit Excessive Collision Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txexcolfim(&mut self) -> TXEXCOLFIM_W {
        TXEXCOLFIM_W { w: self }
    }
    #[doc = "Bit 19 - MMC Transmit Carrier Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txcarerfim(&mut self) -> TXCARERFIM_W {
        TXCARERFIM_W { w: self }
    }
    #[doc = "Bit 20 - MMC Transmit Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgoctim(&mut self) -> TXGOCTIM_W {
        TXGOCTIM_W { w: self }
    }
    #[doc = "Bit 21 - MMC Transmit Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgfrmim(&mut self) -> TXGFRMIM_W {
        TXGFRMIM_W { w: self }
    }
    #[doc = "Bit 22 - MMC Transmit Excessive Deferral Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txexdeffim(&mut self) -> TXEXDEFFIM_W {
        TXEXDEFFIM_W { w: self }
    }
    #[doc = "Bit 23 - MMC Transmit Pause Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txpausfim(&mut self) -> TXPAUSFIM_W {
        TXPAUSFIM_W { w: self }
    }
    #[doc = "Bit 24 - MMC Transmit VLAN Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txvlangfim(&mut self) -> TXVLANGFIM_W {
        TXVLANGFIM_W { w: self }
    }
    #[doc = "Bit 25 - MMC Transmit Oversize Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txosizegfim(&mut self) -> TXOSIZEGFIM_W {
        TXOSIZEGFIM_W { w: self }
    }
}
