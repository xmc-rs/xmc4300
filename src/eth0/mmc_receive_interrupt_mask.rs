#[doc = "Reader of register MMC_RECEIVE_INTERRUPT_MASK"]
pub type R = crate::R<u32, super::MMC_RECEIVE_INTERRUPT_MASK>;
#[doc = "Writer for register MMC_RECEIVE_INTERRUPT_MASK"]
pub type W = crate::W<u32, super::MMC_RECEIVE_INTERRUPT_MASK>;
#[doc = "Register MMC_RECEIVE_INTERRUPT_MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::MMC_RECEIVE_INTERRUPT_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXGBFRMIM`"]
pub type RXGBFRMIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXGBFRMIM`"]
pub struct RXGBFRMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXGBFRMIM_W<'a> {
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
#[doc = "Reader of field `RXGBOCTIM`"]
pub type RXGBOCTIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXGBOCTIM`"]
pub struct RXGBOCTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXGBOCTIM_W<'a> {
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
#[doc = "Reader of field `RXGOCTIM`"]
pub type RXGOCTIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXGOCTIM`"]
pub struct RXGOCTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXGOCTIM_W<'a> {
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
#[doc = "Reader of field `RXBCGFIM`"]
pub type RXBCGFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXBCGFIM`"]
pub struct RXBCGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBCGFIM_W<'a> {
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
#[doc = "Reader of field `RXMCGFIM`"]
pub type RXMCGFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXMCGFIM`"]
pub struct RXMCGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMCGFIM_W<'a> {
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
#[doc = "Reader of field `RXCRCERFIM`"]
pub type RXCRCERFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXCRCERFIM`"]
pub struct RXCRCERFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCRCERFIM_W<'a> {
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
#[doc = "Reader of field `RXALGNERFIM`"]
pub type RXALGNERFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXALGNERFIM`"]
pub struct RXALGNERFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXALGNERFIM_W<'a> {
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
#[doc = "Reader of field `RXRUNTFIM`"]
pub type RXRUNTFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXRUNTFIM`"]
pub struct RXRUNTFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRUNTFIM_W<'a> {
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
#[doc = "Reader of field `RXJABERFIM`"]
pub type RXJABERFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXJABERFIM`"]
pub struct RXJABERFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXJABERFIM_W<'a> {
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
#[doc = "Reader of field `RXUSIZEGFIM`"]
pub type RXUSIZEGFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXUSIZEGFIM`"]
pub struct RXUSIZEGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUSIZEGFIM_W<'a> {
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
#[doc = "Reader of field `RXOSIZEGFIM`"]
pub type RXOSIZEGFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXOSIZEGFIM`"]
pub struct RXOSIZEGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOSIZEGFIM_W<'a> {
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
#[doc = "Reader of field `RX64OCTGBFIM`"]
pub type RX64OCTGBFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX64OCTGBFIM`"]
pub struct RX64OCTGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX64OCTGBFIM_W<'a> {
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
#[doc = "Reader of field `RX65T127OCTGBFIM`"]
pub type RX65T127OCTGBFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX65T127OCTGBFIM`"]
pub struct RX65T127OCTGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX65T127OCTGBFIM_W<'a> {
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
#[doc = "Reader of field `RX128T255OCTGBFIM`"]
pub type RX128T255OCTGBFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX128T255OCTGBFIM`"]
pub struct RX128T255OCTGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX128T255OCTGBFIM_W<'a> {
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
#[doc = "Reader of field `RX256T511OCTGBFIM`"]
pub type RX256T511OCTGBFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX256T511OCTGBFIM`"]
pub struct RX256T511OCTGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX256T511OCTGBFIM_W<'a> {
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
#[doc = "Reader of field `RX512T1023OCTGBFIM`"]
pub type RX512T1023OCTGBFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX512T1023OCTGBFIM`"]
pub struct RX512T1023OCTGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX512T1023OCTGBFIM_W<'a> {
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
#[doc = "Reader of field `RX1024TMAXOCTGBFIM`"]
pub type RX1024TMAXOCTGBFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX1024TMAXOCTGBFIM`"]
pub struct RX1024TMAXOCTGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX1024TMAXOCTGBFIM_W<'a> {
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
#[doc = "Reader of field `RXUCGFIM`"]
pub type RXUCGFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXUCGFIM`"]
pub struct RXUCGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUCGFIM_W<'a> {
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
#[doc = "Reader of field `RXLENERFIM`"]
pub type RXLENERFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXLENERFIM`"]
pub struct RXLENERFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXLENERFIM_W<'a> {
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
#[doc = "Reader of field `RXORANGEFIM`"]
pub type RXORANGEFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXORANGEFIM`"]
pub struct RXORANGEFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXORANGEFIM_W<'a> {
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
#[doc = "Reader of field `RXPAUSFIM`"]
pub type RXPAUSFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXPAUSFIM`"]
pub struct RXPAUSFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPAUSFIM_W<'a> {
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
#[doc = "Reader of field `RXFOVFIM`"]
pub type RXFOVFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFOVFIM`"]
pub struct RXFOVFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFOVFIM_W<'a> {
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
#[doc = "Reader of field `RXVLANGBFIM`"]
pub type RXVLANGBFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXVLANGBFIM`"]
pub struct RXVLANGBFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXVLANGBFIM_W<'a> {
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
#[doc = "Reader of field `RXWDOGFIM`"]
pub type RXWDOGFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXWDOGFIM`"]
pub struct RXWDOGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXWDOGFIM_W<'a> {
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
#[doc = "Reader of field `RXRCVERRFIM`"]
pub type RXRCVERRFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXRCVERRFIM`"]
pub struct RXRCVERRFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRCVERRFIM_W<'a> {
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
#[doc = "Reader of field `RXCTRLFIM`"]
pub type RXCTRLFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXCTRLFIM`"]
pub struct RXCTRLFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCTRLFIM_W<'a> {
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
    #[doc = "Bit 0 - MMC Receive Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxgbfrmim(&self) -> RXGBFRMIM_R {
        RXGBFRMIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MMC Receive Good Bad Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxgboctim(&self) -> RXGBOCTIM_R {
        RXGBOCTIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MMC Receive Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxgoctim(&self) -> RXGOCTIM_R {
        RXGOCTIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MMC Receive Broadcast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxbcgfim(&self) -> RXBCGFIM_R {
        RXBCGFIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MMC Receive Multicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxmcgfim(&self) -> RXMCGFIM_R {
        RXMCGFIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MMC Receive CRC Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxcrcerfim(&self) -> RXCRCERFIM_R {
        RXCRCERFIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxalgnerfim(&self) -> RXALGNERFIM_R {
        RXALGNERFIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MMC Receive Runt Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxruntfim(&self) -> RXRUNTFIM_R {
        RXRUNTFIM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MMC Receive Jabber Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxjaberfim(&self) -> RXJABERFIM_R {
        RXJABERFIM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MMC Receive Undersize Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxusizegfim(&self) -> RXUSIZEGFIM_R {
        RXUSIZEGFIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MMC Receive Oversize Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxosizegfim(&self) -> RXOSIZEGFIM_R {
        RXOSIZEGFIM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx64octgbfim(&self) -> RX64OCTGBFIM_R {
        RX64OCTGBFIM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx65t127octgbfim(&self) -> RX65T127OCTGBFIM_R {
        RX65T127OCTGBFIM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx128t255octgbfim(&self) -> RX128T255OCTGBFIM_R {
        RX128T255OCTGBFIM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx256t511octgbfim(&self) -> RX256T511OCTGBFIM_R {
        RX256T511OCTGBFIM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx512t1023octgbfim(&self) -> RX512T1023OCTGBFIM_R {
        RX512T1023OCTGBFIM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx1024tmaxoctgbfim(&self) -> RX1024TMAXOCTGBFIM_R {
        RX1024TMAXOCTGBFIM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxucgfim(&self) -> RXUCGFIM_R {
        RXUCGFIM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - MMC Receive Length Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxlenerfim(&self) -> RXLENERFIM_R {
        RXLENERFIM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - MMC Receive Out Of Range Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxorangefim(&self) -> RXORANGEFIM_R {
        RXORANGEFIM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - MMC Receive Pause Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxpausfim(&self) -> RXPAUSFIM_R {
        RXPAUSFIM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - MMC Receive FIFO Overflow Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxfovfim(&self) -> RXFOVFIM_R {
        RXFOVFIM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - MMC Receive VLAN Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxvlangbfim(&self) -> RXVLANGBFIM_R {
        RXVLANGBFIM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - MMC Receive Watchdog Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxwdogfim(&self) -> RXWDOGFIM_R {
        RXWDOGFIM_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - MMC Receive Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxrcverrfim(&self) -> RXRCVERRFIM_R {
        RXRCVERRFIM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - MMC Receive Control Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxctrlfim(&self) -> RXCTRLFIM_R {
        RXCTRLFIM_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MMC Receive Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxgbfrmim(&mut self) -> RXGBFRMIM_W {
        RXGBFRMIM_W { w: self }
    }
    #[doc = "Bit 1 - MMC Receive Good Bad Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxgboctim(&mut self) -> RXGBOCTIM_W {
        RXGBOCTIM_W { w: self }
    }
    #[doc = "Bit 2 - MMC Receive Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxgoctim(&mut self) -> RXGOCTIM_W {
        RXGOCTIM_W { w: self }
    }
    #[doc = "Bit 3 - MMC Receive Broadcast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxbcgfim(&mut self) -> RXBCGFIM_W {
        RXBCGFIM_W { w: self }
    }
    #[doc = "Bit 4 - MMC Receive Multicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxmcgfim(&mut self) -> RXMCGFIM_W {
        RXMCGFIM_W { w: self }
    }
    #[doc = "Bit 5 - MMC Receive CRC Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxcrcerfim(&mut self) -> RXCRCERFIM_W {
        RXCRCERFIM_W { w: self }
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxalgnerfim(&mut self) -> RXALGNERFIM_W {
        RXALGNERFIM_W { w: self }
    }
    #[doc = "Bit 7 - MMC Receive Runt Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxruntfim(&mut self) -> RXRUNTFIM_W {
        RXRUNTFIM_W { w: self }
    }
    #[doc = "Bit 8 - MMC Receive Jabber Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxjaberfim(&mut self) -> RXJABERFIM_W {
        RXJABERFIM_W { w: self }
    }
    #[doc = "Bit 9 - MMC Receive Undersize Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxusizegfim(&mut self) -> RXUSIZEGFIM_W {
        RXUSIZEGFIM_W { w: self }
    }
    #[doc = "Bit 10 - MMC Receive Oversize Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxosizegfim(&mut self) -> RXOSIZEGFIM_W {
        RXOSIZEGFIM_W { w: self }
    }
    #[doc = "Bit 11 - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx64octgbfim(&mut self) -> RX64OCTGBFIM_W {
        RX64OCTGBFIM_W { w: self }
    }
    #[doc = "Bit 12 - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx65t127octgbfim(&mut self) -> RX65T127OCTGBFIM_W {
        RX65T127OCTGBFIM_W { w: self }
    }
    #[doc = "Bit 13 - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx128t255octgbfim(&mut self) -> RX128T255OCTGBFIM_W {
        RX128T255OCTGBFIM_W { w: self }
    }
    #[doc = "Bit 14 - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx256t511octgbfim(&mut self) -> RX256T511OCTGBFIM_W {
        RX256T511OCTGBFIM_W { w: self }
    }
    #[doc = "Bit 15 - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx512t1023octgbfim(&mut self) -> RX512T1023OCTGBFIM_W {
        RX512T1023OCTGBFIM_W { w: self }
    }
    #[doc = "Bit 16 - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx1024tmaxoctgbfim(&mut self) -> RX1024TMAXOCTGBFIM_W {
        RX1024TMAXOCTGBFIM_W { w: self }
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxucgfim(&mut self) -> RXUCGFIM_W {
        RXUCGFIM_W { w: self }
    }
    #[doc = "Bit 18 - MMC Receive Length Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxlenerfim(&mut self) -> RXLENERFIM_W {
        RXLENERFIM_W { w: self }
    }
    #[doc = "Bit 19 - MMC Receive Out Of Range Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxorangefim(&mut self) -> RXORANGEFIM_W {
        RXORANGEFIM_W { w: self }
    }
    #[doc = "Bit 20 - MMC Receive Pause Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxpausfim(&mut self) -> RXPAUSFIM_W {
        RXPAUSFIM_W { w: self }
    }
    #[doc = "Bit 21 - MMC Receive FIFO Overflow Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxfovfim(&mut self) -> RXFOVFIM_W {
        RXFOVFIM_W { w: self }
    }
    #[doc = "Bit 22 - MMC Receive VLAN Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxvlangbfim(&mut self) -> RXVLANGBFIM_W {
        RXVLANGBFIM_W { w: self }
    }
    #[doc = "Bit 23 - MMC Receive Watchdog Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxwdogfim(&mut self) -> RXWDOGFIM_W {
        RXWDOGFIM_W { w: self }
    }
    #[doc = "Bit 24 - MMC Receive Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxrcverrfim(&mut self) -> RXRCVERRFIM_W {
        RXRCVERRFIM_W { w: self }
    }
    #[doc = "Bit 25 - MMC Receive Control Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxctrlfim(&mut self) -> RXCTRLFIM_W {
        RXCTRLFIM_W { w: self }
    }
}
