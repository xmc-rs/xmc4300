#[doc = "Reader of register DIEPEMPMSK"]
pub type R = crate::R<u32, super::DIEPEMPMSK>;
#[doc = "Writer for register DIEPEMPMSK"]
pub type W = crate::W<u32, super::DIEPEMPMSK>;
#[doc = "Register DIEPEMPMSK `reset()`'s with value 0"]
impl crate::ResetValue for super::DIEPEMPMSK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `InEpTxfEmpMsk`"]
pub type INEPTXFEMPMSK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `InEpTxfEmpMsk`"]
pub struct INEPTXFEMPMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPTXFEMPMSK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - IN EP Tx FIFO Empty Interrupt Mask Bits"]
    #[inline(always)]
    pub fn in_ep_txf_emp_msk(&self) -> INEPTXFEMPMSK_R {
        INEPTXFEMPMSK_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN EP Tx FIFO Empty Interrupt Mask Bits"]
    #[inline(always)]
    pub fn in_ep_txf_emp_msk(&mut self) -> INEPTXFEMPMSK_W {
        INEPTXFEMPMSK_W { w: self }
    }
}
