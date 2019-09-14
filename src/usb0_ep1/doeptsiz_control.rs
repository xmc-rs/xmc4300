#[doc = "Reader of register DOEPTSIZ_CONTROL"]
pub type R = crate::R<u32, super::DOEPTSIZ_CONTROL>;
#[doc = "Writer for register DOEPTSIZ_CONTROL"]
pub type W = crate::W<u32, super::DOEPTSIZ_CONTROL>;
#[doc = "Register DOEPTSIZ_CONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::DOEPTSIZ_CONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XferSize`"]
pub type XFERSIZE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `XferSize`"]
pub struct XFERSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> XFERSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0007_ffff) | ((value as u32) & 0x0007_ffff);
        self.w
    }
}
#[doc = "Reader of field `PktCnt`"]
pub type PKTCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PktCnt`"]
pub struct PKTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PKTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 19)) | (((value as u32) & 0x03ff) << 19);
        self.w
    }
}
#[doc = "Reader of field `SUPCnt`"]
pub type SUPCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SUPCnt`"]
pub struct SUPCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SUPCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfer_size(&self) -> XFERSIZE_R {
        XFERSIZE_R::new((self.bits & 0x0007_ffff) as u32)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pkt_cnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - SETUP Packet Count: 0b00=1 packet, 0b00=2 packets, 0b00=3 packets,"]
    #[inline(always)]
    pub fn supcnt(&self) -> SUPCNT_R {
        SUPCNT_R::new(((self.bits >> 29) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfer_size(&mut self) -> XFERSIZE_W {
        XFERSIZE_W { w: self }
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pkt_cnt(&mut self) -> PKTCNT_W {
        PKTCNT_W { w: self }
    }
    #[doc = "Bits 29:30 - SETUP Packet Count: 0b00=1 packet, 0b00=2 packets, 0b00=3 packets,"]
    #[inline(always)]
    pub fn supcnt(&mut self) -> SUPCNT_W {
        SUPCNT_W { w: self }
    }
}
