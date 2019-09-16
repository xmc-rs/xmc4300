#[doc = "Reader of register MAC_CONFIGURATION"]
pub type R = crate::R<u32, super::MAC_CONFIGURATION>;
#[doc = "Writer for register MAC_CONFIGURATION"]
pub type W = crate::W<u32, super::MAC_CONFIGURATION>;
#[doc = "Register MAC_CONFIGURATION `reset()`'s with value 0x8000"]
impl crate::ResetValue for super::MAC_CONFIGURATION {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000
    }
}
#[doc = "Reader of field `PRELEN`"]
pub type PRELEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRELEN`"]
pub struct PRELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `RE`"]
pub type RE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RE`"]
pub struct RE_W<'a> {
    w: &'a mut W,
}
impl<'a> RE_W<'a> {
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
#[doc = "Reader of field `TE`"]
pub type TE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TE`"]
pub struct TE_W<'a> {
    w: &'a mut W,
}
impl<'a> TE_W<'a> {
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
#[doc = "Reader of field `DC`"]
pub type DC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DC`"]
pub struct DC_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_W<'a> {
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
#[doc = "Reader of field `BL`"]
pub type BL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BL`"]
pub struct BL_W<'a> {
    w: &'a mut W,
}
impl<'a> BL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `ACS`"]
pub type ACS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACS`"]
pub struct ACS_W<'a> {
    w: &'a mut W,
}
impl<'a> ACS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `DR`"]
pub type DR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DR`"]
pub struct DR_W<'a> {
    w: &'a mut W,
}
impl<'a> DR_W<'a> {
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
#[doc = "Reader of field `IPC`"]
pub type IPC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPC`"]
pub struct IPC_W<'a> {
    w: &'a mut W,
}
impl<'a> IPC_W<'a> {
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
#[doc = "Reader of field `DM`"]
pub type DM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DM`"]
pub struct DM_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_W<'a> {
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
#[doc = "Reader of field `LM`"]
pub type LM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LM`"]
pub struct LM_W<'a> {
    w: &'a mut W,
}
impl<'a> LM_W<'a> {
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
#[doc = "Reader of field `DO`"]
pub type DO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DO`"]
pub struct DO_W<'a> {
    w: &'a mut W,
}
impl<'a> DO_W<'a> {
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
#[doc = "Reader of field `FES`"]
pub type FES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FES`"]
pub struct FES_W<'a> {
    w: &'a mut W,
}
impl<'a> FES_W<'a> {
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
#[doc = "Reader of field `DCRS`"]
pub type DCRS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCRS`"]
pub struct DCRS_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `IFG`"]
pub type IFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IFG`"]
pub struct IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> IFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
#[doc = "Reader of field `JE`"]
pub type JE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JE`"]
pub struct JE_W<'a> {
    w: &'a mut W,
}
impl<'a> JE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `BE`"]
pub type BE_R = crate::R<bool, bool>;
#[doc = "Reader of field `JD`"]
pub type JD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JD`"]
pub struct JD_W<'a> {
    w: &'a mut W,
}
impl<'a> JD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `WD`"]
pub type WD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WD`"]
pub struct WD_W<'a> {
    w: &'a mut W,
}
impl<'a> WD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `TC`"]
pub type TC_R = crate::R<bool, bool>;
#[doc = "Reader of field `CST`"]
pub type CST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CST`"]
pub struct CST_W<'a> {
    w: &'a mut W,
}
impl<'a> CST_W<'a> {
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
#[doc = "Reader of field `TWOKPE`"]
pub type TWOKPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TWOKPE`"]
pub struct TWOKPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TWOKPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `SARC`"]
pub type SARC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - Preamble Length for Transmit Frames"]
    #[inline(always)]
    pub fn prelen(&self) -> PRELEN_R {
        PRELEN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Deferral Check"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Back-Off Limit"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Automatic Pad or CRC Stripping"]
    #[inline(always)]
    pub fn acs(&self) -> ACS_R {
        ACS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Disable Retry"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Checksum Offload"]
    #[inline(always)]
    pub fn ipc(&self) -> IPC_R {
        IPC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Duplex Mode"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Disable Receive Own"]
    #[inline(always)]
    pub fn do_(&self) -> DO_R {
        DO_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Speed"]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission"]
    #[inline(always)]
    pub fn dcrs(&self) -> DCRS_R {
        DCRS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - Inter-Frame Gap"]
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bit 20 - Jumbo Frame Enable"]
    #[inline(always)]
    pub fn je(&self) -> JE_R {
        JE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Frame Burst Enable"]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Jabber Disable"]
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Watchdog Disable"]
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Transmit Configuration in RMII"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CRC Stripping of Type Frames"]
    #[inline(always)]
    pub fn cst(&self) -> CST_R {
        CST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - IEEE 802.3as support for 2K packets Enable"]
    #[inline(always)]
    pub fn twokpe(&self) -> TWOKPE_R {
        TWOKPE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - Source Address Insertion or Replacement Control"]
    #[inline(always)]
    pub fn sarc(&self) -> SARC_R {
        SARC_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Preamble Length for Transmit Frames"]
    #[inline(always)]
    pub fn prelen(&mut self) -> PRELEN_W {
        PRELEN_W { w: self }
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W {
        RE_W { w: self }
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&mut self) -> TE_W {
        TE_W { w: self }
    }
    #[doc = "Bit 4 - Deferral Check"]
    #[inline(always)]
    pub fn dc(&mut self) -> DC_W {
        DC_W { w: self }
    }
    #[doc = "Bits 5:6 - Back-Off Limit"]
    #[inline(always)]
    pub fn bl(&mut self) -> BL_W {
        BL_W { w: self }
    }
    #[doc = "Bit 7 - Automatic Pad or CRC Stripping"]
    #[inline(always)]
    pub fn acs(&mut self) -> ACS_W {
        ACS_W { w: self }
    }
    #[doc = "Bit 9 - Disable Retry"]
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W {
        DR_W { w: self }
    }
    #[doc = "Bit 10 - Checksum Offload"]
    #[inline(always)]
    pub fn ipc(&mut self) -> IPC_W {
        IPC_W { w: self }
    }
    #[doc = "Bit 11 - Duplex Mode"]
    #[inline(always)]
    pub fn dm(&mut self) -> DM_W {
        DM_W { w: self }
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    pub fn lm(&mut self) -> LM_W {
        LM_W { w: self }
    }
    #[doc = "Bit 13 - Disable Receive Own"]
    #[inline(always)]
    pub fn do_(&mut self) -> DO_W {
        DO_W { w: self }
    }
    #[doc = "Bit 14 - Speed"]
    #[inline(always)]
    pub fn fes(&mut self) -> FES_W {
        FES_W { w: self }
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission"]
    #[inline(always)]
    pub fn dcrs(&mut self) -> DCRS_W {
        DCRS_W { w: self }
    }
    #[doc = "Bits 17:19 - Inter-Frame Gap"]
    #[inline(always)]
    pub fn ifg(&mut self) -> IFG_W {
        IFG_W { w: self }
    }
    #[doc = "Bit 20 - Jumbo Frame Enable"]
    #[inline(always)]
    pub fn je(&mut self) -> JE_W {
        JE_W { w: self }
    }
    #[doc = "Bit 22 - Jabber Disable"]
    #[inline(always)]
    pub fn jd(&mut self) -> JD_W {
        JD_W { w: self }
    }
    #[doc = "Bit 23 - Watchdog Disable"]
    #[inline(always)]
    pub fn wd(&mut self) -> WD_W {
        WD_W { w: self }
    }
    #[doc = "Bit 25 - CRC Stripping of Type Frames"]
    #[inline(always)]
    pub fn cst(&mut self) -> CST_W {
        CST_W { w: self }
    }
    #[doc = "Bit 27 - IEEE 802.3as support for 2K packets Enable"]
    #[inline(always)]
    pub fn twokpe(&mut self) -> TWOKPE_W {
        TWOKPE_W { w: self }
    }
}
