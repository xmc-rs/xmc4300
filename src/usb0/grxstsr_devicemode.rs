#[doc = "Register `GRXSTSR_DEVICEMODE` reader"]
pub type R = crate::R<GrxstsrDevicemodeSpec>;
#[doc = "Field `EPNum` reader - Endpoint Number"]
pub type EpnumR = crate::FieldReader;
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
impl crate::IsEnum for Dpid {}
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
    #[doc = "1: Global OUT NAK (triggers an interrupt)"]
    Value1 = 1,
    #[doc = "2: OUT data packet received"]
    Value2 = 2,
    #[doc = "3: OUT transfer completed (triggers an interrupt)"]
    Value3 = 3,
    #[doc = "4: SETUP transaction completed (triggers an interrupt)"]
    Value4 = 4,
    #[doc = "6: SETUP data packet received"]
    Value5 = 6,
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
impl crate::IsEnum for PktSts {}
#[doc = "Field `PktSts` reader - Packet Status"]
pub type PktStsR = crate::FieldReader<PktSts>;
impl PktStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PktSts> {
        match self.bits {
            1 => Some(PktSts::Value1),
            2 => Some(PktSts::Value2),
            3 => Some(PktSts::Value3),
            4 => Some(PktSts::Value4),
            6 => Some(PktSts::Value5),
            _ => None,
        }
    }
    #[doc = "Global OUT NAK (triggers an interrupt)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PktSts::Value1
    }
    #[doc = "OUT data packet received"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PktSts::Value2
    }
    #[doc = "OUT transfer completed (triggers an interrupt)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PktSts::Value3
    }
    #[doc = "SETUP transaction completed (triggers an interrupt)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PktSts::Value4
    }
    #[doc = "SETUP data packet received"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PktSts::Value5
    }
}
#[doc = "Field `FN` reader - Frame Number"]
pub type FnR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Endpoint Number"]
    #[inline(always)]
    pub fn epnum(&self) -> EpnumR {
        EpnumR::new((self.bits & 0x0f) as u8)
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
    #[doc = "Bits 21:24 - Frame Number"]
    #[inline(always)]
    pub fn fn_(&self) -> FnR {
        FnR::new(((self.bits >> 21) & 0x0f) as u8)
    }
}
#[doc = "Receive Status Debug Read Register \\[DEVICEMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsr_devicemode::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrxstsrDevicemodeSpec;
impl crate::RegisterSpec for GrxstsrDevicemodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grxstsr_devicemode::R`](R) reader structure"]
impl crate::Readable for GrxstsrDevicemodeSpec {}
#[doc = "`reset()` method sets GRXSTSR_DEVICEMODE to value 0"]
impl crate::Resettable for GrxstsrDevicemodeSpec {
    const RESET_VALUE: u32 = 0;
}
