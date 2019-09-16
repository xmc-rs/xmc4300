#[doc = "Writer for register GIDLS"]
pub type W = crate::W<u32, super::GIDLS>;
#[doc = "Register GIDLS `reset()`'s with value 0"]
impl crate::ResetValue for super::GIDLS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SS0I`"]
pub struct SS0I_W<'a> {
    w: &'a mut W,
}
impl<'a> SS0I_W<'a> {
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
#[doc = "Write proxy for field `SS1I`"]
pub struct SS1I_W<'a> {
    w: &'a mut W,
}
impl<'a> SS1I_W<'a> {
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
#[doc = "Write proxy for field `SS2I`"]
pub struct SS2I_W<'a> {
    w: &'a mut W,
}
impl<'a> SS2I_W<'a> {
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
#[doc = "Write proxy for field `SS3I`"]
pub struct SS3I_W<'a> {
    w: &'a mut W,
}
impl<'a> SS3I_W<'a> {
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
#[doc = "Write proxy for field `CPRB`"]
pub struct CPRB_W<'a> {
    w: &'a mut W,
}
impl<'a> CPRB_W<'a> {
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
#[doc = "Write proxy for field `PSIC`"]
pub struct PSIC_W<'a> {
    w: &'a mut W,
}
impl<'a> PSIC_W<'a> {
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
impl W {
    #[doc = "Bit 0 - CC40 IDLE mode set"]
    #[inline(always)]
    pub fn ss0i(&mut self) -> SS0I_W {
        SS0I_W { w: self }
    }
    #[doc = "Bit 1 - CC41 IDLE mode set"]
    #[inline(always)]
    pub fn ss1i(&mut self) -> SS1I_W {
        SS1I_W { w: self }
    }
    #[doc = "Bit 2 - CC42 IDLE mode set"]
    #[inline(always)]
    pub fn ss2i(&mut self) -> SS2I_W {
        SS2I_W { w: self }
    }
    #[doc = "Bit 3 - CC43 IDLE mode set"]
    #[inline(always)]
    pub fn ss3i(&mut self) -> SS3I_W {
        SS3I_W { w: self }
    }
    #[doc = "Bit 8 - Prescaler Run Bit Clear"]
    #[inline(always)]
    pub fn cprb(&mut self) -> CPRB_W {
        CPRB_W { w: self }
    }
    #[doc = "Bit 9 - Prescaler clear"]
    #[inline(always)]
    pub fn psic(&mut self) -> PSIC_W {
        PSIC_W { w: self }
    }
}
