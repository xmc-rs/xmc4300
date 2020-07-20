#[doc = "Writer for register SEFCLR"]
pub type W = crate::W<u32, super::SEFCLR>;
#[doc = "Register SEFCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::SEFCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clear Source Event 0/1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEV0_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the source event flag in GxSEFLAG"]
    VALUE2 = 1,
}
impl From<SEV0_AW> for bool {
    #[inline(always)]
    fn from(variant: SEV0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SEV0`"]
pub struct SEV0_W<'a> {
    w: &'a mut W,
}
impl<'a> SEV0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEV0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEV0_AW::VALUE1)
    }
    #[doc = "Clear the source event flag in GxSEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEV0_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Clear Source Event 0/1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEV1_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the source event flag in GxSEFLAG"]
    VALUE2 = 1,
}
impl From<SEV1_AW> for bool {
    #[inline(always)]
    fn from(variant: SEV1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SEV1`"]
pub struct SEV1_W<'a> {
    w: &'a mut W,
}
impl<'a> SEV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEV1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEV1_AW::VALUE1)
    }
    #[doc = "Clear the source event flag in GxSEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEV1_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Clear Source Event 0/1"]
    #[inline(always)]
    pub fn sev0(&mut self) -> SEV0_W {
        SEV0_W { w: self }
    }
    #[doc = "Bit 1 - Clear Source Event 0/1"]
    #[inline(always)]
    pub fn sev1(&mut self) -> SEV1_W {
        SEV1_W { w: self }
    }
}
