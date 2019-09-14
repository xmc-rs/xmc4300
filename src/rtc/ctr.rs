#[doc = "Reader of register CTR"]
pub type R = crate::R<u32, super::CTR>;
#[doc = "Writer for register CTR"]
pub type W = crate::W<u32, super::CTR>;
#[doc = "Register CTR `reset()`'s with value 0x7fff_0000"]
impl crate::ResetValue for super::CTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7fff_0000
    }
}
#[doc = "Reader of field `ENB`"]
pub type ENB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENB`"]
pub struct ENB_W<'a> {
    w: &'a mut W,
}
impl<'a> ENB_W<'a> {
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
#[doc = "Reader of field `TAE`"]
pub type TAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAE`"]
pub struct TAE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAE_W<'a> {
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
#[doc = "Reader of field `ESEC`"]
pub type ESEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ESEC`"]
pub struct ESEC_W<'a> {
    w: &'a mut W,
}
impl<'a> ESEC_W<'a> {
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
#[doc = "Reader of field `EMIC`"]
pub type EMIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMIC`"]
pub struct EMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> EMIC_W<'a> {
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
#[doc = "Reader of field `EHOC`"]
pub type EHOC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EHOC`"]
pub struct EHOC_W<'a> {
    w: &'a mut W,
}
impl<'a> EHOC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `EDAC`"]
pub type EDAC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDAC`"]
pub struct EDAC_W<'a> {
    w: &'a mut W,
}
impl<'a> EDAC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `EMOC`"]
pub type EMOC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMOC`"]
pub struct EMOC_W<'a> {
    w: &'a mut W,
}
impl<'a> EMOC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `EYEC`"]
pub type EYEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EYEC`"]
pub struct EYEC_W<'a> {
    w: &'a mut W,
}
impl<'a> EYEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `DIV`"]
pub type DIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DIV`"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RTC Module Enable"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer Alarm Enable for Hibernation Wake-up"]
    #[inline(always)]
    pub fn tae(&self) -> TAE_R {
        TAE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable Seconds Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn esec(&self) -> ESEC_R {
        ESEC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable Minutes Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn emic(&self) -> EMIC_R {
        EMIC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable Hours Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn ehoc(&self) -> EHOC_R {
        EHOC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable Days Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn edac(&self) -> EDAC_R {
        EDAC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable Months Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn emoc(&self) -> EMOC_R {
        EMOC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable Years Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn eyec(&self) -> EYEC_R {
        EYEC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - RTC Clock Divider Value"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Module Enable"]
    #[inline(always)]
    pub fn enb(&mut self) -> ENB_W {
        ENB_W { w: self }
    }
    #[doc = "Bit 2 - Timer Alarm Enable for Hibernation Wake-up"]
    #[inline(always)]
    pub fn tae(&mut self) -> TAE_W {
        TAE_W { w: self }
    }
    #[doc = "Bit 8 - Enable Seconds Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn esec(&mut self) -> ESEC_W {
        ESEC_W { w: self }
    }
    #[doc = "Bit 9 - Enable Minutes Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn emic(&mut self) -> EMIC_W {
        EMIC_W { w: self }
    }
    #[doc = "Bit 10 - Enable Hours Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn ehoc(&mut self) -> EHOC_W {
        EHOC_W { w: self }
    }
    #[doc = "Bit 11 - Enable Days Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn edac(&mut self) -> EDAC_W {
        EDAC_W { w: self }
    }
    #[doc = "Bit 13 - Enable Months Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn emoc(&mut self) -> EMOC_W {
        EMOC_W { w: self }
    }
    #[doc = "Bit 14 - Enable Years Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn eyec(&mut self) -> EYEC_W {
        EYEC_W { w: self }
    }
    #[doc = "Bits 16:31 - RTC Clock Divider Value"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
}
