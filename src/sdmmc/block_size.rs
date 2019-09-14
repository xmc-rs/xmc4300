#[doc = "Reader of register BLOCK_SIZE"]
pub type R = crate::R<u16, super::BLOCK_SIZE>;
#[doc = "Writer for register BLOCK_SIZE"]
pub type W = crate::W<u16, super::BLOCK_SIZE>;
#[doc = "Register BLOCK_SIZE `reset()`'s with value 0"]
impl crate::ResetValue for super::BLOCK_SIZE {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_BLOCK_SIZE_12`"]
pub type TX_BLOCK_SIZE_12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_BLOCK_SIZE_12`"]
pub struct TX_BLOCK_SIZE_12_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BLOCK_SIZE_12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `TX_BLOCK_SIZE`"]
pub type TX_BLOCK_SIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TX_BLOCK_SIZE`"]
pub struct TX_BLOCK_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BLOCK_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u16) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Transfer Block Size 12th bit."]
    #[inline(always)]
    pub fn tx_block_size_12(&self) -> TX_BLOCK_SIZE_12_R {
        TX_BLOCK_SIZE_12_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 0:11 - Transfer Block Size"]
    #[inline(always)]
    pub fn tx_block_size(&self) -> TX_BLOCK_SIZE_R {
        TX_BLOCK_SIZE_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 15 - Transfer Block Size 12th bit."]
    #[inline(always)]
    pub fn tx_block_size_12(&mut self) -> TX_BLOCK_SIZE_12_W {
        TX_BLOCK_SIZE_12_W { w: self }
    }
    #[doc = "Bits 0:11 - Transfer Block Size"]
    #[inline(always)]
    pub fn tx_block_size(&mut self) -> TX_BLOCK_SIZE_W {
        TX_BLOCK_SIZE_W { w: self }
    }
}
