#[doc = "Reader of register CTR"]
pub type R = crate::R<u32, super::CTR>;
#[doc = "Writer for register CTR"]
pub type W = crate::W<u32, super::CTR>;
#[doc = "Register CTR `reset()`'s with value 0"]
impl crate::ResetValue for super::CTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FCM`"]
pub type FCM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FCM`"]
pub struct FCM_W<'a> {
    w: &'a mut W,
}
impl<'a> FCM_W<'a> {
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
#[doc = "Reader of field `FRM_CFG`"]
pub type FRM_CFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRM_CFG`"]
pub struct FRM_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> FRM_CFG_W<'a> {
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
#[doc = "Reader of field `FRM_CHECK`"]
pub type FRM_CHECK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRM_CHECK`"]
pub struct FRM_CHECK_W<'a> {
    w: &'a mut W,
}
impl<'a> FRM_CHECK_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Force CRC Mismatch"]
    #[inline(always)]
    pub fn fcm(&self) -> FCM_R {
        FCM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Force CFG Register Mismatch"]
    #[inline(always)]
    pub fn frm_cfg(&self) -> FRM_CFG_R {
        FRM_CFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Force Check Register Mismatch"]
    #[inline(always)]
    pub fn frm_check(&self) -> FRM_CHECK_R {
        FRM_CHECK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force CRC Mismatch"]
    #[inline(always)]
    pub fn fcm(&mut self) -> FCM_W {
        FCM_W { w: self }
    }
    #[doc = "Bit 1 - Force CFG Register Mismatch"]
    #[inline(always)]
    pub fn frm_cfg(&mut self) -> FRM_CFG_W {
        FRM_CFG_W { w: self }
    }
    #[doc = "Bit 2 - Force Check Register Mismatch"]
    #[inline(always)]
    pub fn frm_check(&mut self) -> FRM_CHECK_W {
        FRM_CHECK_W { w: self }
    }
}
