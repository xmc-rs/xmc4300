#[doc = "Writer for register CLRSR"]
pub type W = crate::W<u32, super::CLRSR>;
#[doc = "Register CLRSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CLRSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RPSE`"]
pub struct RPSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RPSE_W<'a> {
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
#[doc = "Write proxy for field `RPMI`"]
pub struct RPMI_W<'a> {
    w: &'a mut W,
}
impl<'a> RPMI_W<'a> {
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
#[doc = "Write proxy for field `RPHO`"]
pub struct RPHO_W<'a> {
    w: &'a mut W,
}
impl<'a> RPHO_W<'a> {
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
#[doc = "Write proxy for field `RPDA`"]
pub struct RPDA_W<'a> {
    w: &'a mut W,
}
impl<'a> RPDA_W<'a> {
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
#[doc = "Write proxy for field `RPMO`"]
pub struct RPMO_W<'a> {
    w: &'a mut W,
}
impl<'a> RPMO_W<'a> {
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
#[doc = "Write proxy for field `RPYE`"]
pub struct RPYE_W<'a> {
    w: &'a mut W,
}
impl<'a> RPYE_W<'a> {
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
#[doc = "Write proxy for field `RAI`"]
pub struct RAI_W<'a> {
    w: &'a mut W,
}
impl<'a> RAI_W<'a> {
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
    #[doc = "Bit 0 - Periodic Seconds Interrupt Clear"]
    #[inline(always)]
    pub fn rpse(&mut self) -> RPSE_W {
        RPSE_W { w: self }
    }
    #[doc = "Bit 1 - Periodic Minutes Interrupt Clear"]
    #[inline(always)]
    pub fn rpmi(&mut self) -> RPMI_W {
        RPMI_W { w: self }
    }
    #[doc = "Bit 2 - Periodic Hours Interrupt Clear"]
    #[inline(always)]
    pub fn rpho(&mut self) -> RPHO_W {
        RPHO_W { w: self }
    }
    #[doc = "Bit 3 - Periodic Days Interrupt Clear"]
    #[inline(always)]
    pub fn rpda(&mut self) -> RPDA_W {
        RPDA_W { w: self }
    }
    #[doc = "Bit 5 - Periodic Months Interrupt Clear"]
    #[inline(always)]
    pub fn rpmo(&mut self) -> RPMO_W {
        RPMO_W { w: self }
    }
    #[doc = "Bit 6 - Periodic Years Interrupt Clear"]
    #[inline(always)]
    pub fn rpye(&mut self) -> RPYE_W {
        RPYE_W { w: self }
    }
    #[doc = "Bit 8 - Alarm Interrupt Clear"]
    #[inline(always)]
    pub fn rai(&mut self) -> RAI_W {
        RAI_W { w: self }
    }
}
