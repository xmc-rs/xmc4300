#[doc = "Reader of register GINTSTS_HOSTMODE"]
pub type R = crate::R<u32, super::GINTSTS_HOSTMODE>;
#[doc = "Writer for register GINTSTS_HOSTMODE"]
pub type W = crate::W<u32, super::GINTSTS_HOSTMODE>;
#[doc = "Register GINTSTS_HOSTMODE `reset()`'s with value 0x1400_0020"]
impl crate::ResetValue for super::GINTSTS_HOSTMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1400_0020
    }
}
#[doc = "Current Mode of Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CURMOD_A {
    #[doc = "0: Device mode"]
    VALUE1 = 0,
    #[doc = "1: Host mode"]
    VALUE2 = 1,
}
impl From<CURMOD_A> for bool {
    #[inline(always)]
    fn from(variant: CURMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CurMod`"]
pub type CURMOD_R = crate::R<bool, CURMOD_A>;
impl CURMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CURMOD_A {
        match self.bits {
            false => CURMOD_A::VALUE1,
            true => CURMOD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CURMOD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CURMOD_A::VALUE2
    }
}
#[doc = "Reader of field `ModeMis`"]
pub type MODEMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ModeMis`"]
pub struct MODEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MODEMIS_W<'a> {
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
#[doc = "Reader of field `OTGInt`"]
pub type OTGINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `Sof`"]
pub type SOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Sof`"]
pub struct SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_W<'a> {
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
#[doc = "Reader of field `RxFLvl`"]
pub type RXFLVL_R = crate::R<bool, bool>;
#[doc = "Reader of field `incomplP`"]
pub type INCOMPLP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `incomplP`"]
pub struct INCOMPLP_W<'a> {
    w: &'a mut W,
}
impl<'a> INCOMPLP_W<'a> {
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
#[doc = "Reader of field `PrtInt`"]
pub type PRTINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `HChInt`"]
pub type HCHINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `PTxFEmp`"]
pub type PTXFEMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `ConIDStsChng`"]
pub type CONIDSTSCHNG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ConIDStsChng`"]
pub struct CONIDSTSCHNG_W<'a> {
    w: &'a mut W,
}
impl<'a> CONIDSTSCHNG_W<'a> {
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
#[doc = "Reader of field `DisconnInt`"]
pub type DISCONNINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DisconnInt`"]
pub struct DISCONNINT_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCONNINT_W<'a> {
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
#[doc = "Reader of field `SessReqInt`"]
pub type SESSREQINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SessReqInt`"]
pub struct SESSREQINT_W<'a> {
    w: &'a mut W,
}
impl<'a> SESSREQINT_W<'a> {
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
#[doc = "Reader of field `WkUpInt`"]
pub type WKUPINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WkUpInt`"]
pub struct WKUPINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPINT_W<'a> {
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
    #[doc = "Bit 0 - Current Mode of Operation"]
    #[inline(always)]
    pub fn cur_mod(&self) -> CURMOD_R {
        CURMOD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mode Mismatch Interrupt"]
    #[inline(always)]
    pub fn mode_mis(&self) -> MODEMIS_R {
        MODEMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OTG Interrupt"]
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start of Frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RxFIFO Non-Empty"]
    #[inline(always)]
    pub fn rx_flvl(&self) -> RXFLVL_R {
        RXFLVL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer"]
    #[inline(always)]
    pub fn incompl_p(&self) -> INCOMPLP_R {
        INCOMPLP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Host Port Interrupt"]
    #[inline(always)]
    pub fn prt_int(&self) -> PRTINT_R {
        PRTINT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Host Channels Interrupt"]
    #[inline(always)]
    pub fn hch_int(&self) -> HCHINT_R {
        HCHINT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO Empty"]
    #[inline(always)]
    pub fn ptx_femp(&self) -> PTXFEMP_R {
        PTXFEMP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Connector ID Status Change"]
    #[inline(always)]
    pub fn con_idsts_chng(&self) -> CONIDSTSCHNG_R {
        CONIDSTSCHNG_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt"]
    #[inline(always)]
    pub fn disconn_int(&self) -> DISCONNINT_R {
        DISCONNINT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt"]
    #[inline(always)]
    pub fn sess_req_int(&self) -> SESSREQINT_R {
        SESSREQINT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt"]
    #[inline(always)]
    pub fn wk_up_int(&self) -> WKUPINT_R {
        WKUPINT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode Mismatch Interrupt"]
    #[inline(always)]
    pub fn mode_mis(&mut self) -> MODEMIS_W {
        MODEMIS_W { w: self }
    }
    #[doc = "Bit 3 - Start of Frame"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W {
        SOF_W { w: self }
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer"]
    #[inline(always)]
    pub fn incompl_p(&mut self) -> INCOMPLP_W {
        INCOMPLP_W { w: self }
    }
    #[doc = "Bit 28 - Connector ID Status Change"]
    #[inline(always)]
    pub fn con_idsts_chng(&mut self) -> CONIDSTSCHNG_W {
        CONIDSTSCHNG_W { w: self }
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt"]
    #[inline(always)]
    pub fn disconn_int(&mut self) -> DISCONNINT_W {
        DISCONNINT_W { w: self }
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt"]
    #[inline(always)]
    pub fn sess_req_int(&mut self) -> SESSREQINT_W {
        SESSREQINT_W { w: self }
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt"]
    #[inline(always)]
    pub fn wk_up_int(&mut self) -> WKUPINT_W {
        WKUPINT_W { w: self }
    }
}
