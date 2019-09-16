#[doc = "Reader of register GRXSTSR_HOSTMODE"]
pub type R = crate::R<u32, super::GRXSTSR_HOSTMODE>;
#[doc = "Reader of field `ChNum`"]
pub type CHNUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `BCnt`"]
pub type BCNT_R = crate::R<u16, u16>;
#[doc = "Data PID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPID_A {
    #[doc = "0: DATA0"]
    VALUE1,
    #[doc = "2: DATA1"]
    VALUE2,
    #[doc = "1: DATA2"]
    VALUE3,
    #[doc = "3: MDATA"]
    VALUE4,
}
impl From<DPID_A> for u8 {
    #[inline(always)]
    fn from(variant: DPID_A) -> Self {
        match variant {
            DPID_A::VALUE1 => 0,
            DPID_A::VALUE2 => 2,
            DPID_A::VALUE3 => 1,
            DPID_A::VALUE4 => 3,
        }
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
pub enum PKTSTS_A {
    #[doc = "2: IN data packet received"]
    VALUE1,
    #[doc = "3: IN transfer completed (triggers an interrupt)"]
    VALUE2,
    #[doc = "5: Data toggle error (triggers an interrupt)"]
    VALUE3,
    #[doc = "7: Channel halted (triggers an interrupt)"]
    VALUE4,
}
impl From<PKTSTS_A> for u8 {
    #[inline(always)]
    fn from(variant: PKTSTS_A) -> Self {
        match variant {
            PKTSTS_A::VALUE1 => 2,
            PKTSTS_A::VALUE2 => 3,
            PKTSTS_A::VALUE3 => 5,
            PKTSTS_A::VALUE4 => 7,
        }
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
            2 => Val(PKTSTS_A::VALUE1),
            3 => Val(PKTSTS_A::VALUE2),
            5 => Val(PKTSTS_A::VALUE3),
            7 => Val(PKTSTS_A::VALUE4),
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
}
impl R {
    #[doc = "Bits 0:3 - Channel Number"]
    #[inline(always)]
    pub fn ch_num(&self) -> CHNUM_R {
        CHNUM_R::new((self.bits & 0x0f) as u8)
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
}
