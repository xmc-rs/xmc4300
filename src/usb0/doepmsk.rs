#[doc = "Reader of register DOEPMSK"]
pub type R = crate::R<u32, super::DOEPMSK>;
#[doc = "Writer for register DOEPMSK"]
pub type W = crate::W<u32, super::DOEPMSK>;
#[doc = "Register DOEPMSK `reset()`'s with value 0"]
impl crate::ResetValue for super::DOEPMSK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XferComplMsk`"]
pub type XFERCOMPLMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XferComplMsk`"]
pub struct XFERCOMPLMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> XFERCOMPLMSK_W<'a> {
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
#[doc = "Reader of field `EPDisbldMsk`"]
pub type EPDISBLDMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPDisbldMsk`"]
pub struct EPDISBLDMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDISBLDMSK_W<'a> {
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
#[doc = "Reader of field `AHBErrMsk`"]
pub type AHBERRMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHBErrMsk`"]
pub struct AHBERRMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBERRMSK_W<'a> {
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
#[doc = "Reader of field `SetUPMsk`"]
pub type SETUPMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SetUPMsk`"]
pub struct SETUPMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUPMSK_W<'a> {
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
#[doc = "Reader of field `OUTTknEPdisMsk`"]
pub type OUTTKNEPDISMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTTknEPdisMsk`"]
pub struct OUTTKNEPDISMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTKNEPDISMSK_W<'a> {
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
#[doc = "Reader of field `Back2BackSETup`"]
pub type BACK2BACKSETUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Back2BackSETup`"]
pub struct BACK2BACKSETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> BACK2BACKSETUP_W<'a> {
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
#[doc = "Reader of field `OutPktErrMsk`"]
pub type OUTPKTERRMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OutPktErrMsk`"]
pub struct OUTPKTERRMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPKTERRMSK_W<'a> {
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
#[doc = "Reader of field `BnaOutIntrMsk`"]
pub type BNAOUTINTRMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BnaOutIntrMsk`"]
pub struct BNAOUTINTRMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> BNAOUTINTRMSK_W<'a> {
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
#[doc = "Reader of field `BbleErrMsk`"]
pub type BBLEERRMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BbleErrMsk`"]
pub struct BBLEERRMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> BBLEERRMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `NAKMsk`"]
pub type NAKMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NAKMsk`"]
pub struct NAKMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKMSK_W<'a> {
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
#[doc = "Reader of field `NYETMsk`"]
pub type NYETMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NYETMsk`"]
pub struct NYETMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> NYETMSK_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Transfer Completed Interrupt Mask"]
    #[inline(always)]
    pub fn xfer_compl_msk(&self) -> XFERCOMPLMSK_R {
        XFERCOMPLMSK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask"]
    #[inline(always)]
    pub fn epdisbld_msk(&self) -> EPDISBLDMSK_R {
        EPDISBLDMSK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr_msk(&self) -> AHBERRMSK_R {
        AHBERRMSK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SETUP Phase Done Mask"]
    #[inline(always)]
    pub fn set_upmsk(&self) -> SETUPMSK_R {
        SETUPMSK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OUT Token Received when Endpoint Disabled Mask"]
    #[inline(always)]
    pub fn outtkn_epdis_msk(&self) -> OUTTKNEPDISMSK_R {
        OUTTKNEPDISMSK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received Mask"]
    #[inline(always)]
    pub fn back2back_setup(&self) -> BACK2BACKSETUP_R {
        BACK2BACKSETUP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - OUT Packet Error Mask"]
    #[inline(always)]
    pub fn out_pkt_err_msk(&self) -> OUTPKTERRMSK_R {
        OUTPKTERRMSK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BNA interrupt Mask"]
    #[inline(always)]
    pub fn bna_out_intr_msk(&self) -> BNAOUTINTRMSK_R {
        BNAOUTINTRMSK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Babble Interrupt Mask"]
    #[inline(always)]
    pub fn bble_err_msk(&self) -> BBLEERRMSK_R {
        BBLEERRMSK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - NAK Interrupt Mask"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - NYET Interrupt Mask"]
    #[inline(always)]
    pub fn nyetmsk(&self) -> NYETMSK_R {
        NYETMSK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt Mask"]
    #[inline(always)]
    pub fn xfer_compl_msk(&mut self) -> XFERCOMPLMSK_W {
        XFERCOMPLMSK_W { w: self }
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask"]
    #[inline(always)]
    pub fn epdisbld_msk(&mut self) -> EPDISBLDMSK_W {
        EPDISBLDMSK_W { w: self }
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr_msk(&mut self) -> AHBERRMSK_W {
        AHBERRMSK_W { w: self }
    }
    #[doc = "Bit 3 - SETUP Phase Done Mask"]
    #[inline(always)]
    pub fn set_upmsk(&mut self) -> SETUPMSK_W {
        SETUPMSK_W { w: self }
    }
    #[doc = "Bit 4 - OUT Token Received when Endpoint Disabled Mask"]
    #[inline(always)]
    pub fn outtkn_epdis_msk(&mut self) -> OUTTKNEPDISMSK_W {
        OUTTKNEPDISMSK_W { w: self }
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received Mask"]
    #[inline(always)]
    pub fn back2back_setup(&mut self) -> BACK2BACKSETUP_W {
        BACK2BACKSETUP_W { w: self }
    }
    #[doc = "Bit 8 - OUT Packet Error Mask"]
    #[inline(always)]
    pub fn out_pkt_err_msk(&mut self) -> OUTPKTERRMSK_W {
        OUTPKTERRMSK_W { w: self }
    }
    #[doc = "Bit 9 - BNA interrupt Mask"]
    #[inline(always)]
    pub fn bna_out_intr_msk(&mut self) -> BNAOUTINTRMSK_W {
        BNAOUTINTRMSK_W { w: self }
    }
    #[doc = "Bit 12 - Babble Interrupt Mask"]
    #[inline(always)]
    pub fn bble_err_msk(&mut self) -> BBLEERRMSK_W {
        BBLEERRMSK_W { w: self }
    }
    #[doc = "Bit 13 - NAK Interrupt Mask"]
    #[inline(always)]
    pub fn nakmsk(&mut self) -> NAKMSK_W {
        NAKMSK_W { w: self }
    }
    #[doc = "Bit 14 - NYET Interrupt Mask"]
    #[inline(always)]
    pub fn nyetmsk(&mut self) -> NYETMSK_W {
        NYETMSK_W { w: self }
    }
}
