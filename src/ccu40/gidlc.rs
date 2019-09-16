#[doc = "Writer for register GIDLC"]
pub type W = crate::W<u32, super::GIDLC>;
#[doc = "Register GIDLC `reset()`'s with value 0"]
impl crate::ResetValue for super::GIDLC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CS0I`"]
pub struct CS0I_W<'a> {
    w: &'a mut W,
}
impl<'a> CS0I_W<'a> {
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
#[doc = "Write proxy for field `CS1I`"]
pub struct CS1I_W<'a> {
    w: &'a mut W,
}
impl<'a> CS1I_W<'a> {
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
#[doc = "Write proxy for field `CS2I`"]
pub struct CS2I_W<'a> {
    w: &'a mut W,
}
impl<'a> CS2I_W<'a> {
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
#[doc = "Write proxy for field `CS3I`"]
pub struct CS3I_W<'a> {
    w: &'a mut W,
}
impl<'a> CS3I_W<'a> {
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
#[doc = "Write proxy for field `SPRB`"]
pub struct SPRB_W<'a> {
    w: &'a mut W,
}
impl<'a> SPRB_W<'a> {
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
impl W {
    #[doc = "Bit 0 - CC40 IDLE mode clear"]
    #[inline(always)]
    pub fn cs0i(&mut self) -> CS0I_W {
        CS0I_W { w: self }
    }
    #[doc = "Bit 1 - CC41 IDLE mode clear"]
    #[inline(always)]
    pub fn cs1i(&mut self) -> CS1I_W {
        CS1I_W { w: self }
    }
    #[doc = "Bit 2 - CC42 IDLE mode clear"]
    #[inline(always)]
    pub fn cs2i(&mut self) -> CS2I_W {
        CS2I_W { w: self }
    }
    #[doc = "Bit 3 - CC43 IDLE mode clear"]
    #[inline(always)]
    pub fn cs3i(&mut self) -> CS3I_W {
        CS3I_W { w: self }
    }
    #[doc = "Bit 8 - Prescaler Run Bit Set"]
    #[inline(always)]
    pub fn sprb(&mut self) -> SPRB_W {
        SPRB_W { w: self }
    }
}
