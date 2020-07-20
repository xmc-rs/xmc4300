#[doc = "Reader of register GRXSTSR_DEVICEMODE"]
pub type R = crate::R<u32, super::GRXSTSR_DEVICEMODE>;
#[doc = "Reader of field `EPNum`"]
pub type EPNUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `BCnt`"]
pub type BCNT_R = crate::R<u16, u16>;
#[doc = "Data PID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DPID_A {
    #[doc = "0: DATA0"]
    VALUE1 = 0,
    #[doc = "2: DATA1"]
    VALUE2 = 2,
    #[doc = "1: DATA2"]
    VALUE3 = 1,
    #[doc = "3: MDATA"]
    VALUE4 = 3,
}
impl From<DPID_A> for u8 {
    #[inline(always)]
    fn from(variant: DPID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DPID`"]
pub type DPID_R = crate::R<u8, DPID_A>;
impl DPID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPID_A {
        match self.bits {
            0 => DPID_A::VALUE1,
            2 => DPID_A::VALUE2,
            1 => DPID_A::VALUE3,
            3 => DPID_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DPID_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DPID_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DPID_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DPID_A::VALUE4
    }
}
#[doc = "Packet Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PKTSTS_A {
    #[doc = "1: Global OUT NAK (triggers an interrupt)"]
    VALUE1 = 1,
    #[doc = "2: OUT data packet received"]
    VALUE2 = 2,
    #[doc = "3: OUT transfer completed (triggers an interrupt)"]
    VALUE3 = 3,
    #[doc = "4: SETUP transaction completed (triggers an interrupt)"]
    VALUE4 = 4,
    #[doc = "6: SETUP data packet received"]
    VALUE5 = 6,
}
impl From<PKTSTS_A> for u8 {
    #[inline(always)]
    fn from(variant: PKTSTS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PktSts`"]
pub type PKTSTS_R = crate::R<u8, PKTSTS_A>;
impl PKTSTS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PKTSTS_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(PKTSTS_A::VALUE1),
            2 => Val(PKTSTS_A::VALUE2),
            3 => Val(PKTSTS_A::VALUE3),
            4 => Val(PKTSTS_A::VALUE4),
            6 => Val(PKTSTS_A::VALUE5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PKTSTS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PKTSTS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PKTSTS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PKTSTS_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PKTSTS_A::VALUE5
    }
}
#[doc = "Reader of field `FN`"]
pub type FN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Endpoint Number"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14 - Byte Count"]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bits 17:20 - Packet Status"]
    #[inline(always)]
    pub fn pkt_sts(&self) -> PKTSTS_R {
        PKTSTS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:24 - Frame Number"]
    #[inline(always)]
    pub fn fn_(&self) -> FN_R {
        FN_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
}
