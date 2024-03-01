#[doc = "Register `GRXSTSP_HOSTMODE` reader"]
pub type R = crate::R<GrxstspHostmodeSpec>;
#[doc = "Field `ChNum` reader - Channel Number"]
pub type ChNumR = crate::FieldReader;
#[doc = "Field `BCnt` reader - Byte Count"]
pub type BcntR = crate::FieldReader<u16>;
#[doc = "Data PID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dpid {
    #[doc = "0: DATA0"]
    Value1 = 0,
    #[doc = "2: DATA1"]
    Value2 = 2,
    #[doc = "1: DATA2"]
    Value3 = 1,
    #[doc = "3: MDATA"]
    Value4 = 3,
}
impl From<Dpid> for u8 {
    #[inline(always)]
    fn from(variant: Dpid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dpid {
    type Ux = u8;
}
#[doc = "Field `DPID` reader - Data PID"]
pub type DpidR = crate::FieldReader<Dpid>;
impl DpidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dpid {
        match self.bits {
            0 => Dpid::Value1,
            2 => Dpid::Value2,
            1 => Dpid::Value3,
            3 => Dpid::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "DATA0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dpid::Value1
    }
    #[doc = "DATA1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dpid::Value2
    }
    #[doc = "DATA2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Dpid::Value3
    }
    #[doc = "MDATA"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Dpid::Value4
    }
}
#[doc = "Packet Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PktSts {
    #[doc = "2: IN data packet received"]
    Value1 = 2,
    #[doc = "3: IN transfer completed (triggers an interrupt)"]
    Value2 = 3,
    #[doc = "5: Data toggle error (triggers an interrupt)"]
    Value3 = 5,
    #[doc = "7: Channel halted (triggers an interrupt)"]
    Value4 = 7,
}
impl From<PktSts> for u8 {
    #[inline(always)]
    fn from(variant: PktSts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PktSts {
    type Ux = u8;
}
#[doc = "Field `PktSts` reader - Packet Status"]
pub type PktStsR = crate::FieldReader<PktSts>;
impl PktStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PktSts> {
        match self.bits {
            2 => Some(PktSts::Value1),
            3 => Some(PktSts::Value2),
            5 => Some(PktSts::Value3),
            7 => Some(PktSts::Value4),
            _ => None,
        }
    }
    #[doc = "IN data packet received"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PktSts::Value1
    }
    #[doc = "IN transfer completed (triggers an interrupt)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PktSts::Value2
    }
    #[doc = "Data toggle error (triggers an interrupt)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PktSts::Value3
    }
    #[doc = "Channel halted (triggers an interrupt)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PktSts::Value4
    }
}
impl R {
    #[doc = "Bits 0:3 - Channel Number"]
    #[inline(always)]
    pub fn ch_num(&self) -> ChNumR {
        ChNumR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14 - Byte Count"]
    #[inline(always)]
    pub fn bcnt(&self) -> BcntR {
        BcntR::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DpidR {
        DpidR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:20 - Packet Status"]
    #[inline(always)]
    pub fn pkt_sts(&self) -> PktStsR {
        PktStsR::new(((self.bits >> 17) & 0x0f) as u8)
    }
}
#[doc = "Receive Status Read and Pop Register \\[HOSTMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsp_hostmode::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrxstspHostmodeSpec;
impl crate::RegisterSpec for GrxstspHostmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grxstsp_hostmode::R`](R) reader structure"]
impl crate::Readable for GrxstspHostmodeSpec {}
#[doc = "`reset()` method sets GRXSTSP_HOSTMODE to value 0"]
impl crate::Resettable for GrxstspHostmodeSpec {
    const RESET_VALUE: u32 = 0;
}
