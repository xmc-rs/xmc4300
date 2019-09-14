#[doc = "Reader of register CLC"]
pub type R = crate::R<u32, super::CLC>;
#[doc = "Writer for register CLC"]
pub type W = crate::W<u32, super::CLC>;
#[doc = "Register CLC `reset()`'s with value 0x03"]
impl crate::ResetValue for super::CLC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Reader of field `DISR`"]
pub type DISR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISR`"]
pub struct DISR_W<'a> {
    w: &'a mut W,
}
impl<'a> DISR_W<'a> {
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
#[doc = "Reader of field `DISS`"]
pub type DISS_R = crate::R<bool, bool>;
#[doc = "Reader of field `EDIS`"]
pub type EDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDIS`"]
pub struct EDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EDIS_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Module Disable Request Bit"]
    #[inline(always)]
    pub fn disr(&self) -> DISR_R {
        DISR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Module Disable Status Bit"]
    #[inline(always)]
    pub fn diss(&self) -> DISS_R {
        DISS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sleep Mode Enable Control"]
    #[inline(always)]
    pub fn edis(&self) -> EDIS_R {
        EDIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Module Disable Request Bit"]
    #[inline(always)]
    pub fn disr(&mut self) -> DISR_W {
        DISR_W { w: self }
    }
    #[doc = "Bit 3 - Sleep Mode Enable Control"]
    #[inline(always)]
    pub fn edis(&mut self) -> EDIS_W {
        EDIS_W { w: self }
    }
}
