#[doc = "Reader of register PCGCCTL"]
pub type R = crate::R<u32, super::PCGCCTL>;
#[doc = "Writer for register PCGCCTL"]
pub type W = crate::W<u32, super::PCGCCTL>;
#[doc = "Register PCGCCTL `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::PCGCCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "Reader of field `StopPclk`"]
pub type STOPPCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `StopPclk`"]
pub struct STOPPCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPPCLK_W<'a> {
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
#[doc = "Reader of field `GateHclk`"]
pub type GATEHCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GateHclk`"]
pub struct GATEHCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> GATEHCLK_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Stop Pclk"]
    #[inline(always)]
    pub fn stop_pclk(&self) -> STOPPCLK_R {
        STOPPCLK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Gate Hclk"]
    #[inline(always)]
    pub fn gate_hclk(&self) -> GATEHCLK_R {
        GATEHCLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop Pclk"]
    #[inline(always)]
    pub fn stop_pclk(&mut self) -> STOPPCLK_W {
        STOPPCLK_W { w: self }
    }
    #[doc = "Bit 1 - Gate Hclk"]
    #[inline(always)]
    pub fn gate_hclk(&mut self) -> GATEHCLK_W {
        GATEHCLK_W { w: self }
    }
}
