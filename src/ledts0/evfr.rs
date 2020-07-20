#[doc = "Reader of register EVFR"]
pub type R = crate::R<u32, super::EVFR>;
#[doc = "Writer for register EVFR"]
pub type W = crate::W<u32, super::EVFR>;
#[doc = "Register EVFR `reset()`'s with value 0"]
impl crate::ResetValue for super::EVFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSF`"]
pub type TSF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFF`"]
pub type TFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TPF`"]
pub type TPF_R = crate::R<bool, bool>;
#[doc = "TS-Counter Overflow Indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSCTROVF_A {
    #[doc = "0: No overflow has occurred."]
    VALUE1 = 0,
    #[doc = "1: The TS-counter has overflowed at least once."]
    VALUE2 = 1,
}
impl From<TSCTROVF_A> for bool {
    #[inline(always)]
    fn from(variant: TSCTROVF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TSCTROVF`"]
pub type TSCTROVF_R = crate::R<bool, TSCTROVF_A>;
impl TSCTROVF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSCTROVF_A {
        match self.bits {
            false => TSCTROVF_A::VALUE1,
            true => TSCTROVF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSCTROVF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSCTROVF_A::VALUE2
    }
}
#[doc = "Clear Time Slice Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSF_AW {
    #[doc = "0: No action."]
    VALUE1 = 0,
    #[doc = "1: Bit TSF is cleared."]
    VALUE2 = 1,
}
impl From<CTSF_AW> for bool {
    #[inline(always)]
    fn from(variant: CTSF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CTSF`"]
pub struct CTSF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CTSF_AW::VALUE1)
    }
    #[doc = "Bit TSF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CTSF_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Clear (Extended) Time Frame Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTFF_AW {
    #[doc = "0: No action."]
    VALUE1 = 0,
    #[doc = "1: Bit TFF is cleared."]
    VALUE2 = 1,
}
impl From<CTFF_AW> for bool {
    #[inline(always)]
    fn from(variant: CTFF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CTFF`"]
pub struct CTFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTFF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CTFF_AW::VALUE1)
    }
    #[doc = "Bit TFF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CTFF_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Clear Autoscan Time Period Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTPF_AW {
    #[doc = "0: No action."]
    VALUE1 = 0,
    #[doc = "1: Bit TPF is cleared."]
    VALUE2 = 1,
}
impl From<CTPF_AW> for bool {
    #[inline(always)]
    fn from(variant: CTPF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CTPF`"]
pub struct CTPF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTPF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTPF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CTPF_AW::VALUE1)
    }
    #[doc = "Bit TPF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CTPF_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Time Slice Interrupt Flag"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - (Extended) Time Frame Interrupt Flag"]
    #[inline(always)]
    pub fn tff(&self) -> TFF_R {
        TFF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Autoscan Time Period Interrupt Flag"]
    #[inline(always)]
    pub fn tpf(&self) -> TPF_R {
        TPF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TS-Counter Overflow Indication"]
    #[inline(always)]
    pub fn tsctrovf(&self) -> TSCTROVF_R {
        TSCTROVF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Clear Time Slice Interrupt Flag"]
    #[inline(always)]
    pub fn ctsf(&mut self) -> CTSF_W {
        CTSF_W { w: self }
    }
    #[doc = "Bit 17 - Clear (Extended) Time Frame Interrupt Flag"]
    #[inline(always)]
    pub fn ctff(&mut self) -> CTFF_W {
        CTFF_W { w: self }
    }
    #[doc = "Bit 18 - Clear Autoscan Time Period Interrupt Flag"]
    #[inline(always)]
    pub fn ctpf(&mut self) -> CTPF_W {
        CTPF_W { w: self }
    }
}
