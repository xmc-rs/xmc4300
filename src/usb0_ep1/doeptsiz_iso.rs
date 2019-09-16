#[doc = "Reader of register DOEPTSIZ_ISO"]
pub type R = crate::R<u32, super::DOEPTSIZ_ISO>;
#[doc = "Writer for register DOEPTSIZ_ISO"]
pub type W = crate::W<u32, super::DOEPTSIZ_ISO>;
#[doc = "Register DOEPTSIZ_ISO `reset()`'s with value 0"]
impl crate::ResetValue for super::DOEPTSIZ_ISO {
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
#[doc = "Received Data PID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDPID_A {
    #[doc = "0: DATA0"]
    VALUE1,
    #[doc = "1: DATA2"]
    VALUE2,
    #[doc = "2: DATA1"]
    VALUE3,
    #[doc = "3: MDATA"]
    VALUE4,
}
impl From<RXDPID_A> for u8 {
    #[inline(always)]
    fn from(variant: RXDPID_A) -> Self {
        match variant {
            RXDPID_A::VALUE1 => 0,
            RXDPID_A::VALUE2 => 1,
            RXDPID_A::VALUE3 => 2,
            RXDPID_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `RxDPID`"]
pub type RXDPID_R = crate::R<u8, RXDPID_A>;
impl RXDPID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDPID_A {
        match self.bits {
            0 => RXDPID_A::VALUE1,
            1 => RXDPID_A::VALUE2,
            2 => RXDPID_A::VALUE3,
            3 => RXDPID_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXDPID_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RXDPID_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RXDPID_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RXDPID_A::VALUE4
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
    #[doc = "Bits 29:30 - Received Data PID"]
    #[inline(always)]
    pub fn rx_dpid(&self) -> RXDPID_R {
        RXDPID_R::new(((self.bits >> 29) & 0x03) as u8)
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
}
