#[doc = "Reader of register HCTSIZ_BUFFERMODE"]
pub type R = crate::R<u32, super::HCTSIZ_BUFFERMODE>;
#[doc = "Writer for register HCTSIZ_BUFFERMODE"]
pub type W = crate::W<u32, super::HCTSIZ_BUFFERMODE>;
#[doc = "Register HCTSIZ_BUFFERMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::HCTSIZ_BUFFERMODE {
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
#[doc = "PID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PID_A {
    #[doc = "0: DATA0"]
    VALUE1 = 0,
    #[doc = "1: DATA2"]
    VALUE2 = 1,
    #[doc = "2: DATA1"]
    VALUE3 = 2,
    #[doc = "3: MDATA (non-control)/SETUP (control)"]
    VALUE4 = 3,
}
impl From<PID_A> for u8 {
    #[inline(always)]
    fn from(variant: PID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `Pid`"]
pub type PID_R = crate::R<u8, PID_A>;
impl PID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID_A {
        match self.bits {
            0 => PID_A::VALUE1,
            1 => PID_A::VALUE2,
            2 => PID_A::VALUE3,
            3 => PID_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PID_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PID_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PID_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PID_A::VALUE4
    }
}
#[doc = "Write proxy for field `Pid`"]
pub struct PID_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "DATA0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PID_A::VALUE1)
    }
    #[doc = "DATA2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PID_A::VALUE2)
    }
    #[doc = "DATA1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PID_A::VALUE3)
    }
    #[doc = "MDATA (non-control)/SETUP (control)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PID_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 29:30 - PID"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(((self.bits >> 29) & 0x03) as u8)
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
    #[doc = "Bits 29:30 - PID"]
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W {
        PID_W { w: self }
    }
}
