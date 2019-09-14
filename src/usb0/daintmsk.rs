#[doc = "Reader of register DAINTMSK"]
pub type R = crate::R<u32, super::DAINTMSK>;
#[doc = "Writer for register DAINTMSK"]
pub type W = crate::W<u32, super::DAINTMSK>;
#[doc = "Register DAINTMSK `reset()`'s with value 0"]
impl crate::ResetValue for super::DAINTMSK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `InEpMsk`"]
pub type INEPMSK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `InEpMsk`"]
pub struct INEPMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPMSK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `OutEpMsk`"]
pub type OUTEPMSK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OutEpMsk`"]
pub struct OUTEPMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTEPMSK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - IN EP Interrupt Mask Bits"]
    #[inline(always)]
    pub fn in_ep_msk(&self) -> INEPMSK_R {
        INEPMSK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT EP Interrupt Mask Bits"]
    #[inline(always)]
    pub fn out_ep_msk(&self) -> OUTEPMSK_R {
        OUTEPMSK_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN EP Interrupt Mask Bits"]
    #[inline(always)]
    pub fn in_ep_msk(&mut self) -> INEPMSK_W {
        INEPMSK_W { w: self }
    }
    #[doc = "Bits 16:31 - OUT EP Interrupt Mask Bits"]
    #[inline(always)]
    pub fn out_ep_msk(&mut self) -> OUTEPMSK_W {
        OUTEPMSK_W { w: self }
    }
}
