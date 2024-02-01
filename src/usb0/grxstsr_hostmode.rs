#[doc = "Register `GRXSTSR_HOSTMODE` reader"]
pub type R = crate::R<GRXSTSR_HOSTMODE_SPEC>;
#[doc = "Field `ChNum` reader - Channel Number"]
pub type CH_NUM_R = crate::FieldReader;
#[doc = "Field `BCnt` reader - Byte Count"]
pub type BCNT_R = crate::FieldReader<u16>;
#[doc = "Field `DPID` reader - Data PID"]
pub type DPID_R = crate::FieldReader<DPID_A>;
#[doc = "Data PID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for DPID_A {
    type Ux = u8;
}
impl DPID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DPID_A {
        match self.bits {
            0 => DPID_A::VALUE1,
            2 => DPID_A::VALUE2,
            1 => DPID_A::VALUE3,
            3 => DPID_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "DATA0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DPID_A::VALUE1
    }
    #[doc = "DATA1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DPID_A::VALUE2
    }
    #[doc = "DATA2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DPID_A::VALUE3
    }
    #[doc = "MDATA"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DPID_A::VALUE4
    }
}
#[doc = "Field `PktSts` reader - Packet Status"]
pub type PKT_STS_R = crate::FieldReader<PKT_STS_A>;
#[doc = "Packet Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PKT_STS_A {
    #[doc = "2: IN data packet received"]
    VALUE1 = 2,
    #[doc = "3: IN transfer completed (triggers an interrupt)"]
    VALUE2 = 3,
    #[doc = "5: Data toggle error (triggers an interrupt)"]
    VALUE3 = 5,
    #[doc = "7: Channel halted (triggers an interrupt)"]
    VALUE4 = 7,
}
impl From<PKT_STS_A> for u8 {
    #[inline(always)]
    fn from(variant: PKT_STS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PKT_STS_A {
    type Ux = u8;
}
impl PKT_STS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PKT_STS_A> {
        match self.bits {
            2 => Some(PKT_STS_A::VALUE1),
            3 => Some(PKT_STS_A::VALUE2),
            5 => Some(PKT_STS_A::VALUE3),
            7 => Some(PKT_STS_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "IN data packet received"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PKT_STS_A::VALUE1
    }
    #[doc = "IN transfer completed (triggers an interrupt)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PKT_STS_A::VALUE2
    }
    #[doc = "Data toggle error (triggers an interrupt)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PKT_STS_A::VALUE3
    }
    #[doc = "Channel halted (triggers an interrupt)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PKT_STS_A::VALUE4
    }
}
impl R {
    #[doc = "Bits 0:3 - Channel Number"]
    #[inline(always)]
    pub fn ch_num(&self) -> CH_NUM_R {
        CH_NUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14 - Byte Count"]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:20 - Packet Status"]
    #[inline(always)]
    pub fn pkt_sts(&self) -> PKT_STS_R {
        PKT_STS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
}
#[doc = "Receive Status Debug Read Register \\[HOSTMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsr_hostmode::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GRXSTSR_HOSTMODE_SPEC;
impl crate::RegisterSpec for GRXSTSR_HOSTMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grxstsr_hostmode::R`](R) reader structure"]
impl crate::Readable for GRXSTSR_HOSTMODE_SPEC {}
#[doc = "`reset()` method sets GRXSTSR_HOSTMODE to value 0"]
impl crate::Resettable for GRXSTSR_HOSTMODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
