#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TI`"]
pub type TI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TI`"]
pub struct TI_W<'a> {
    w: &'a mut W,
}
impl<'a> TI_W<'a> {
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
#[doc = "Reader of field `TPS`"]
pub type TPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TPS`"]
pub struct TPS_W<'a> {
    w: &'a mut W,
}
impl<'a> TPS_W<'a> {
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
#[doc = "Reader of field `TU`"]
pub type TU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TU`"]
pub struct TU_W<'a> {
    w: &'a mut W,
}
impl<'a> TU_W<'a> {
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
#[doc = "Reader of field `TJT`"]
pub type TJT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TJT`"]
pub struct TJT_W<'a> {
    w: &'a mut W,
}
impl<'a> TJT_W<'a> {
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
#[doc = "Reader of field `OVF`"]
pub type OVF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVF`"]
pub struct OVF_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_W<'a> {
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
#[doc = "Reader of field `UNF`"]
pub type UNF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNF`"]
pub struct UNF_W<'a> {
    w: &'a mut W,
}
impl<'a> UNF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RI`"]
pub type RI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RI`"]
pub struct RI_W<'a> {
    w: &'a mut W,
}
impl<'a> RI_W<'a> {
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
#[doc = "Reader of field `RU`"]
pub type RU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RU`"]
pub struct RU_W<'a> {
    w: &'a mut W,
}
impl<'a> RU_W<'a> {
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
#[doc = "Reader of field `RPS`"]
pub type RPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPS`"]
pub struct RPS_W<'a> {
    w: &'a mut W,
}
impl<'a> RPS_W<'a> {
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
#[doc = "Reader of field `RWT`"]
pub type RWT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RWT`"]
pub struct RWT_W<'a> {
    w: &'a mut W,
}
impl<'a> RWT_W<'a> {
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
#[doc = "Reader of field `ETI`"]
pub type ETI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETI`"]
pub struct ETI_W<'a> {
    w: &'a mut W,
}
impl<'a> ETI_W<'a> {
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
#[doc = "Reader of field `FBI`"]
pub type FBI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBI`"]
pub struct FBI_W<'a> {
    w: &'a mut W,
}
impl<'a> FBI_W<'a> {
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
#[doc = "Reader of field `ERI`"]
pub type ERI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERI`"]
pub struct ERI_W<'a> {
    w: &'a mut W,
}
impl<'a> ERI_W<'a> {
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
#[doc = "Reader of field `AIS`"]
pub type AIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AIS`"]
pub struct AIS_W<'a> {
    w: &'a mut W,
}
impl<'a> AIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `NIS`"]
pub type NIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NIS`"]
pub struct NIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NIS_W<'a> {
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
#[doc = "Reader of field `RS`"]
pub type RS_R = crate::R<u8, u8>;
#[doc = "Reader of field `TS`"]
pub type TS_R = crate::R<u8, u8>;
#[doc = "Reader of field `EB`"]
pub type EB_R = crate::R<u8, u8>;
#[doc = "Reader of field `EMI`"]
pub type EMI_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPI`"]
pub type EPI_R = crate::R<bool, bool>;
#[doc = "Reader of field `TTI`"]
pub type TTI_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    pub fn tu(&self) -> TU_R {
        TU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout"]
    #[inline(always)]
    pub fn tjt(&self) -> TJT_R {
        TJT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive Overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit Underflow"]
    #[inline(always)]
    pub fn unf(&self) -> UNF_R {
        UNF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    pub fn ru(&self) -> RU_R {
        RU_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    pub fn eti(&self) -> ETI_R {
        ETI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt"]
    #[inline(always)]
    pub fn fbi(&self) -> FBI_R {
        FBI_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Early Receive Interrupt"]
    #[inline(always)]
    pub fn eri(&self) -> ERI_R {
        ERI_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - Received Process State"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Transmit Process State"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 23:25 - Error Bits"]
    #[inline(always)]
    pub fn eb(&self) -> EB_R {
        EB_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bit 27 - ETH MMC Interrupt"]
    #[inline(always)]
    pub fn emi(&self) -> EMI_R {
        EMI_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - ETH PMT Interrupt"]
    #[inline(always)]
    pub fn epi(&self) -> EPI_R {
        EPI_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Timestamp Trigger Interrupt"]
    #[inline(always)]
    pub fn tti(&self) -> TTI_R {
        TTI_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn ti(&mut self) -> TI_W {
        TI_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    pub fn tps(&mut self) -> TPS_W {
        TPS_W { w: self }
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    pub fn tu(&mut self) -> TU_W {
        TU_W { w: self }
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout"]
    #[inline(always)]
    pub fn tjt(&mut self) -> TJT_W {
        TJT_W { w: self }
    }
    #[doc = "Bit 4 - Receive Overflow"]
    #[inline(always)]
    pub fn ovf(&mut self) -> OVF_W {
        OVF_W { w: self }
    }
    #[doc = "Bit 5 - Transmit Underflow"]
    #[inline(always)]
    pub fn unf(&mut self) -> UNF_W {
        UNF_W { w: self }
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    pub fn ri(&mut self) -> RI_W {
        RI_W { w: self }
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    pub fn ru(&mut self) -> RU_W {
        RU_W { w: self }
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    pub fn rps(&mut self) -> RPS_W {
        RPS_W { w: self }
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    pub fn rwt(&mut self) -> RWT_W {
        RWT_W { w: self }
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    pub fn eti(&mut self) -> ETI_W {
        ETI_W { w: self }
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt"]
    #[inline(always)]
    pub fn fbi(&mut self) -> FBI_W {
        FBI_W { w: self }
    }
    #[doc = "Bit 14 - Early Receive Interrupt"]
    #[inline(always)]
    pub fn eri(&mut self) -> ERI_W {
        ERI_W { w: self }
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn ais(&mut self) -> AIS_W {
        AIS_W { w: self }
    }
    #[doc = "Bit 16 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn nis(&mut self) -> NIS_W {
        NIS_W { w: self }
    }
}
