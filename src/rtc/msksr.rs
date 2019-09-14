#[doc = "Reader of register MSKSR"]
pub type R = crate::R<u32, super::MSKSR>;
#[doc = "Writer for register MSKSR"]
pub type W = crate::W<u32, super::MSKSR>;
#[doc = "Register MSKSR `reset()`'s with value 0"]
impl crate::ResetValue for super::MSKSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MPSE`"]
pub type MPSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPSE`"]
pub struct MPSE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPSE_W<'a> {
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
#[doc = "Reader of field `MPMI`"]
pub type MPMI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPMI`"]
pub struct MPMI_W<'a> {
    w: &'a mut W,
}
impl<'a> MPMI_W<'a> {
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
#[doc = "Reader of field `MPHO`"]
pub type MPHO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPHO`"]
pub struct MPHO_W<'a> {
    w: &'a mut W,
}
impl<'a> MPHO_W<'a> {
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
#[doc = "Reader of field `MPDA`"]
pub type MPDA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPDA`"]
pub struct MPDA_W<'a> {
    w: &'a mut W,
}
impl<'a> MPDA_W<'a> {
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
#[doc = "Reader of field `MPMO`"]
pub type MPMO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPMO`"]
pub struct MPMO_W<'a> {
    w: &'a mut W,
}
impl<'a> MPMO_W<'a> {
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
#[doc = "Reader of field `MPYE`"]
pub type MPYE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPYE`"]
pub struct MPYE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPYE_W<'a> {
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
#[doc = "Reader of field `MAI`"]
pub type MAI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAI`"]
pub struct MAI_W<'a> {
    w: &'a mut W,
}
impl<'a> MAI_W<'a> {
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
    #[doc = "Bit 0 - Periodic Seconds Interrupt Mask"]
    #[inline(always)]
    pub fn mpse(&self) -> MPSE_R {
        MPSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Periodic Minutes Interrupt Mask"]
    #[inline(always)]
    pub fn mpmi(&self) -> MPMI_R {
        MPMI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Periodic Hours Interrupt Mask"]
    #[inline(always)]
    pub fn mpho(&self) -> MPHO_R {
        MPHO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Periodic Days Interrupt Mask"]
    #[inline(always)]
    pub fn mpda(&self) -> MPDA_R {
        MPDA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Periodic Months Interrupt Mask"]
    #[inline(always)]
    pub fn mpmo(&self) -> MPMO_R {
        MPMO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Periodic Years Interrupt Mask"]
    #[inline(always)]
    pub fn mpye(&self) -> MPYE_R {
        MPYE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Alarm Interrupt Mask"]
    #[inline(always)]
    pub fn mai(&self) -> MAI_R {
        MAI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Periodic Seconds Interrupt Mask"]
    #[inline(always)]
    pub fn mpse(&mut self) -> MPSE_W {
        MPSE_W { w: self }
    }
    #[doc = "Bit 1 - Periodic Minutes Interrupt Mask"]
    #[inline(always)]
    pub fn mpmi(&mut self) -> MPMI_W {
        MPMI_W { w: self }
    }
    #[doc = "Bit 2 - Periodic Hours Interrupt Mask"]
    #[inline(always)]
    pub fn mpho(&mut self) -> MPHO_W {
        MPHO_W { w: self }
    }
    #[doc = "Bit 3 - Periodic Days Interrupt Mask"]
    #[inline(always)]
    pub fn mpda(&mut self) -> MPDA_W {
        MPDA_W { w: self }
    }
    #[doc = "Bit 5 - Periodic Months Interrupt Mask"]
    #[inline(always)]
    pub fn mpmo(&mut self) -> MPMO_W {
        MPMO_W { w: self }
    }
    #[doc = "Bit 6 - Periodic Years Interrupt Mask"]
    #[inline(always)]
    pub fn mpye(&mut self) -> MPYE_W {
        MPYE_W { w: self }
    }
    #[doc = "Bit 8 - Alarm Interrupt Mask"]
    #[inline(always)]
    pub fn mai(&mut self) -> MAI_W {
        MAI_W { w: self }
    }
}
