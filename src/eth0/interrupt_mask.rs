#[doc = "Reader of register INTERRUPT_MASK"]
pub type R = crate::R<u32, super::INTERRUPT_MASK>;
#[doc = "Writer for register INTERRUPT_MASK"]
pub type W = crate::W<u32, super::INTERRUPT_MASK>;
#[doc = "Register INTERRUPT_MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::INTERRUPT_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PMTIM`"]
pub type PMTIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMTIM`"]
pub struct PMTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PMTIM_W<'a> {
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
#[doc = "Reader of field `TSIM`"]
pub type TSIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSIM`"]
pub struct TSIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIM_W<'a> {
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
impl R {
    #[doc = "Bit 3 - PMT Interrupt Mask"]
    #[inline(always)]
    pub fn pmtim(&self) -> PMTIM_R {
        PMTIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Mask"]
    #[inline(always)]
    pub fn tsim(&self) -> TSIM_R {
        TSIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PMT Interrupt Mask"]
    #[inline(always)]
    pub fn pmtim(&mut self) -> PMTIM_W {
        PMTIM_W { w: self }
    }
    #[doc = "Bit 9 - Timestamp Interrupt Mask"]
    #[inline(always)]
    pub fn tsim(&mut self) -> TSIM_W {
        TSIM_W { w: self }
    }
}
