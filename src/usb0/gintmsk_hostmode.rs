#[doc = "Reader of register GINTMSK_HOSTMODE"]
pub type R = crate::R<u32, super::GINTMSK_HOSTMODE>;
#[doc = "Writer for register GINTMSK_HOSTMODE"]
pub type W = crate::W<u32, super::GINTMSK_HOSTMODE>;
#[doc = "Register GINTMSK_HOSTMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::GINTMSK_HOSTMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ModeMisMsk`"]
pub type MODEMISMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ModeMisMsk`"]
pub struct MODEMISMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> MODEMISMSK_W<'a> {
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
#[doc = "Reader of field `OTGIntMsk`"]
pub type OTGINTMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTGIntMsk`"]
pub struct OTGINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGINTMSK_W<'a> {
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
#[doc = "Reader of field `SofMsk`"]
pub type SOFMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SofMsk`"]
pub struct SOFMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFMSK_W<'a> {
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
#[doc = "Reader of field `RxFLvlMsk`"]
pub type RXFLVLMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RxFLvlMsk`"]
pub struct RXFLVLMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFLVLMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `incomplPMsk`"]
pub type INCOMPLPMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `incomplPMsk`"]
pub struct INCOMPLPMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> INCOMPLPMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `PrtIntMsk`"]
pub type PRTINTMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PrtIntMsk`"]
pub struct PRTINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTINTMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `HChIntMsk`"]
pub type HCHINTMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HChIntMsk`"]
pub struct HCHINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> HCHINTMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `PTxFEmpMsk`"]
pub type PTXFEMPMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTxFEmpMsk`"]
pub struct PTXFEMPMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFEMPMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `ConIDStsChngMsk`"]
pub type CONIDSTSCHNGMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ConIDStsChngMsk`"]
pub struct CONIDSTSCHNGMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> CONIDSTSCHNGMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `DisconnIntMsk`"]
pub type DISCONNINTMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DisconnIntMsk`"]
pub struct DISCONNINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCONNINTMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `SessReqIntMsk`"]
pub type SESSREQINTMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SessReqIntMsk`"]
pub struct SESSREQINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> SESSREQINTMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `WkUpIntMsk`"]
pub type WKUPINTMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WkUpIntMsk`"]
pub struct WKUPINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPINTMSK_W<'a> {
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
    #[doc = "Bit 1 - Mode Mismatch Interrupt Mask"]
    #[inline(always)]
    pub fn mode_mis_msk(&self) -> MODEMISMSK_R {
        MODEMISMSK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OTG Interrupt Mask"]
    #[inline(always)]
    pub fn otgint_msk(&self) -> OTGINTMSK_R {
        OTGINTMSK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start of Frame Mask"]
    #[inline(always)]
    pub fn sof_msk(&self) -> SOFMSK_R {
        SOFMSK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask"]
    #[inline(always)]
    pub fn rx_flvl_msk(&self) -> RXFLVLMSK_R {
        RXFLVLMSK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer Mask"]
    #[inline(always)]
    pub fn incompl_pmsk(&self) -> INCOMPLPMSK_R {
        INCOMPLPMSK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Host Port Interrupt Mask"]
    #[inline(always)]
    pub fn prt_int_msk(&self) -> PRTINTMSK_R {
        PRTINTMSK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Host Channels Interrupt Mask"]
    #[inline(always)]
    pub fn hch_int_msk(&self) -> HCHINTMSK_R {
        HCHINTMSK_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO Empty Mask"]
    #[inline(always)]
    pub fn ptx_femp_msk(&self) -> PTXFEMPMSK_R {
        PTXFEMPMSK_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Connector ID Status Change Mask"]
    #[inline(always)]
    pub fn con_idsts_chng_msk(&self) -> CONIDSTSCHNGMSK_R {
        CONIDSTSCHNGMSK_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt Mask"]
    #[inline(always)]
    pub fn disconn_int_msk(&self) -> DISCONNINTMSK_R {
        DISCONNINTMSK_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt Mask"]
    #[inline(always)]
    pub fn sess_req_int_msk(&self) -> SESSREQINTMSK_R {
        SESSREQINTMSK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask"]
    #[inline(always)]
    pub fn wk_up_int_msk(&self) -> WKUPINTMSK_R {
        WKUPINTMSK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode Mismatch Interrupt Mask"]
    #[inline(always)]
    pub fn mode_mis_msk(&mut self) -> MODEMISMSK_W {
        MODEMISMSK_W { w: self }
    }
    #[doc = "Bit 2 - OTG Interrupt Mask"]
    #[inline(always)]
    pub fn otgint_msk(&mut self) -> OTGINTMSK_W {
        OTGINTMSK_W { w: self }
    }
    #[doc = "Bit 3 - Start of Frame Mask"]
    #[inline(always)]
    pub fn sof_msk(&mut self) -> SOFMSK_W {
        SOFMSK_W { w: self }
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask"]
    #[inline(always)]
    pub fn rx_flvl_msk(&mut self) -> RXFLVLMSK_W {
        RXFLVLMSK_W { w: self }
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer Mask"]
    #[inline(always)]
    pub fn incompl_pmsk(&mut self) -> INCOMPLPMSK_W {
        INCOMPLPMSK_W { w: self }
    }
    #[doc = "Bit 24 - Host Port Interrupt Mask"]
    #[inline(always)]
    pub fn prt_int_msk(&mut self) -> PRTINTMSK_W {
        PRTINTMSK_W { w: self }
    }
    #[doc = "Bit 25 - Host Channels Interrupt Mask"]
    #[inline(always)]
    pub fn hch_int_msk(&mut self) -> HCHINTMSK_W {
        HCHINTMSK_W { w: self }
    }
    #[doc = "Bit 26 - Periodic TxFIFO Empty Mask"]
    #[inline(always)]
    pub fn ptx_femp_msk(&mut self) -> PTXFEMPMSK_W {
        PTXFEMPMSK_W { w: self }
    }
    #[doc = "Bit 28 - Connector ID Status Change Mask"]
    #[inline(always)]
    pub fn con_idsts_chng_msk(&mut self) -> CONIDSTSCHNGMSK_W {
        CONIDSTSCHNGMSK_W { w: self }
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt Mask"]
    #[inline(always)]
    pub fn disconn_int_msk(&mut self) -> DISCONNINTMSK_W {
        DISCONNINTMSK_W { w: self }
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt Mask"]
    #[inline(always)]
    pub fn sess_req_int_msk(&mut self) -> SESSREQINTMSK_W {
        SESSREQINTMSK_W { w: self }
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask"]
    #[inline(always)]
    pub fn wk_up_int_msk(&mut self) -> WKUPINTMSK_W {
        WKUPINTMSK_W { w: self }
    }
}
