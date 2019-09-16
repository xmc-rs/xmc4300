#[doc = "Reader of register PMT_CONTROL_STATUS"]
pub type R = crate::R<u32, super::PMT_CONTROL_STATUS>;
#[doc = "Writer for register PMT_CONTROL_STATUS"]
pub type W = crate::W<u32, super::PMT_CONTROL_STATUS>;
#[doc = "Register PMT_CONTROL_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::PMT_CONTROL_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWRDWN`"]
pub type PWRDWN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRDWN`"]
pub struct PWRDWN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRDWN_W<'a> {
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
#[doc = "Reader of field `MGKPKTEN`"]
pub type MGKPKTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MGKPKTEN`"]
pub struct MGKPKTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MGKPKTEN_W<'a> {
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
#[doc = "Reader of field `RWKPKTEN`"]
pub type RWKPKTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RWKPKTEN`"]
pub struct RWKPKTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RWKPKTEN_W<'a> {
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
#[doc = "Reader of field `MGKPRCVD`"]
pub type MGKPRCVD_R = crate::R<bool, bool>;
#[doc = "Reader of field `RWKPRCVD`"]
pub type RWKPRCVD_R = crate::R<bool, bool>;
#[doc = "Reader of field `GLBLUCAST`"]
pub type GLBLUCAST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GLBLUCAST`"]
pub struct GLBLUCAST_W<'a> {
    w: &'a mut W,
}
impl<'a> GLBLUCAST_W<'a> {
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
#[doc = "Reader of field `RWKFILTRST`"]
pub type RWKFILTRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RWKFILTRST`"]
pub struct RWKFILTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RWKFILTRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Power Down"]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Magic Packet Enable"]
    #[inline(always)]
    pub fn mgkpkten(&self) -> MGKPKTEN_R {
        MGKPKTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wake-Up Frame Enable"]
    #[inline(always)]
    pub fn rwkpkten(&self) -> RWKPKTEN_R {
        RWKPKTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Magic Packet Received"]
    #[inline(always)]
    pub fn mgkprcvd(&self) -> MGKPRCVD_R {
        MGKPRCVD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wake-Up Frame Received"]
    #[inline(always)]
    pub fn rwkprcvd(&self) -> RWKPRCVD_R {
        RWKPRCVD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Global Unicast"]
    #[inline(always)]
    pub fn glblucast(&self) -> GLBLUCAST_R {
        GLBLUCAST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Wake-Up Frame Filter Register Pointer Reset"]
    #[inline(always)]
    pub fn rwkfiltrst(&self) -> RWKFILTRST_R {
        RWKFILTRST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Down"]
    #[inline(always)]
    pub fn pwrdwn(&mut self) -> PWRDWN_W {
        PWRDWN_W { w: self }
    }
    #[doc = "Bit 1 - Magic Packet Enable"]
    #[inline(always)]
    pub fn mgkpkten(&mut self) -> MGKPKTEN_W {
        MGKPKTEN_W { w: self }
    }
    #[doc = "Bit 2 - Wake-Up Frame Enable"]
    #[inline(always)]
    pub fn rwkpkten(&mut self) -> RWKPKTEN_W {
        RWKPKTEN_W { w: self }
    }
    #[doc = "Bit 9 - Global Unicast"]
    #[inline(always)]
    pub fn glblucast(&mut self) -> GLBLUCAST_W {
        GLBLUCAST_W { w: self }
    }
    #[doc = "Bit 31 - Wake-Up Frame Filter Register Pointer Reset"]
    #[inline(always)]
    pub fn rwkfiltrst(&mut self) -> RWKFILTRST_W {
        RWKFILTRST_W { w: self }
    }
}
