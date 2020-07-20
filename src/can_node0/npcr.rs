#[doc = "Reader of register NPCR"]
pub type R = crate::R<u32, super::NPCR>;
#[doc = "Writer for register NPCR"]
pub type W = crate::W<u32, super::NPCR>;
#[doc = "Register NPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::NPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXSEL`"]
pub type RXSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXSEL`"]
pub struct RXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Loop-Back Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBM_A {
    #[doc = "0: Loop-Back Mode is disabled."]
    VALUE1 = 0,
    #[doc = "1: Loop-Back Mode is enabled. This node is connected to an internal (virtual) loop-back CAN bus. All CAN nodes which are in Loop-Back Mode are connected to this virtual CAN bus so that they can communicate with each other internally. The external transmit line is forced recessive in Loop-Back Mode."]
    VALUE2 = 1,
}
impl From<LBM_A> for bool {
    #[inline(always)]
    fn from(variant: LBM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LBM`"]
pub type LBM_R = crate::R<bool, LBM_A>;
impl LBM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBM_A {
        match self.bits {
            false => LBM_A::VALUE1,
            true => LBM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LBM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LBM_A::VALUE2
    }
}
#[doc = "Write proxy for field `LBM`"]
pub struct LBM_W<'a> {
    w: &'a mut W,
}
impl<'a> LBM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Loop-Back Mode is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LBM_A::VALUE1)
    }
    #[doc = "Loop-Back Mode is enabled. This node is connected to an internal (virtual) loop-back CAN bus. All CAN nodes which are in Loop-Back Mode are connected to this virtual CAN bus so that they can communicate with each other internally. The external transmit line is forced recessive in Loop-Back Mode."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LBM_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Receive Select"]
    #[inline(always)]
    pub fn rxsel(&self) -> RXSEL_R {
        RXSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - Loop-Back Mode"]
    #[inline(always)]
    pub fn lbm(&self) -> LBM_R {
        LBM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive Select"]
    #[inline(always)]
    pub fn rxsel(&mut self) -> RXSEL_W {
        RXSEL_W { w: self }
    }
    #[doc = "Bit 8 - Loop-Back Mode"]
    #[inline(always)]
    pub fn lbm(&mut self) -> LBM_W {
        LBM_W { w: self }
    }
}
