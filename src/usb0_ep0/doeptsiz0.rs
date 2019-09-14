#[doc = "Reader of register DOEPTSIZ0"]
pub type R = crate::R<u32, super::DOEPTSIZ0>;
#[doc = "Writer for register DOEPTSIZ0"]
pub type W = crate::W<u32, super::DOEPTSIZ0>;
#[doc = "Register DOEPTSIZ0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DOEPTSIZ0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XferSize`"]
pub type XFERSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XferSize`"]
pub struct XFERSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> XFERSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `PktCnt`"]
pub type PKTCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PktCnt`"]
pub struct PKTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PKTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "SETUP Packet Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUPCNT_A {
    #[doc = "1: 1 packet"]
    VALUE1,
    #[doc = "2: 2 packets"]
    VALUE2,
    #[doc = "3: 3 packets"]
    VALUE3,
}
impl From<SUPCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: SUPCNT_A) -> Self {
        match variant {
            SUPCNT_A::VALUE1 => 1,
            SUPCNT_A::VALUE2 => 2,
            SUPCNT_A::VALUE3 => 3,
        }
    }
}
#[doc = "Reader of field `SUPCnt`"]
pub type SUPCNT_R = crate::R<u8, SUPCNT_A>;
impl SUPCNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SUPCNT_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SUPCNT_A::VALUE1),
            2 => Val(SUPCNT_A::VALUE2),
            3 => Val(SUPCNT_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SUPCNT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SUPCNT_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SUPCNT_A::VALUE3
    }
}
#[doc = "Write proxy for field `SUPCnt`"]
pub struct SUPCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SUPCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUPCNT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 packet"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SUPCNT_A::VALUE1)
    }
    #[doc = "2 packets"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SUPCNT_A::VALUE2)
    }
    #[doc = "3 packets"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SUPCNT_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    pub fn xfer_size(&self) -> XFERSIZE_R {
        XFERSIZE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline(always)]
    pub fn pkt_cnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bits 29:30 - SETUP Packet Count"]
    #[inline(always)]
    pub fn supcnt(&self) -> SUPCNT_R {
        SUPCNT_R::new(((self.bits >> 29) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    pub fn xfer_size(&mut self) -> XFERSIZE_W {
        XFERSIZE_W { w: self }
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline(always)]
    pub fn pkt_cnt(&mut self) -> PKTCNT_W {
        PKTCNT_W { w: self }
    }
    #[doc = "Bits 29:30 - SETUP Packet Count"]
    #[inline(always)]
    pub fn supcnt(&mut self) -> SUPCNT_W {
        SUPCNT_W { w: self }
    }
}
