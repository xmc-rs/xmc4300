#[doc = "Register `GRXSTSP_DEVICEMODE` reader"]
pub struct R(crate::R<GRXSTSP_DEVICEMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRXSTSP_DEVICEMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRXSTSP_DEVICEMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRXSTSP_DEVICEMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EPNum` reader - Endpoint Number"]
pub struct EPNUM_R(crate::FieldReader<u8, u8>);
impl EPNUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        EPNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCnt` reader - Byte Count"]
pub struct BCNT_R(crate::FieldReader<u16, u16>);
impl BCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        BCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Field `DPID` reader - Data PID"]
pub struct DPID_R(crate::FieldReader<u8, DPID_A>);
impl DPID_R {
    pub(crate) fn new(bits: u8) -> Self {
        DPID_R(crate::FieldReader::new(bits))
    }
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
        **self == DPID_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DPID_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == DPID_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == DPID_A::VALUE4
    }
}
impl core::ops::Deref for DPID_R {
    type Target = crate::FieldReader<u8, DPID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `PktSts` reader - Packet Status"]
pub struct PKTSTS_R(crate::FieldReader<u8, PKTSTS_A>);
impl PKTSTS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PKTSTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PKTSTS_A> {
        match self.bits {
            1 => Some(PKTSTS_A::VALUE1),
            2 => Some(PKTSTS_A::VALUE2),
            3 => Some(PKTSTS_A::VALUE3),
            4 => Some(PKTSTS_A::VALUE4),
            6 => Some(PKTSTS_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PKTSTS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PKTSTS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PKTSTS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == PKTSTS_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == PKTSTS_A::VALUE5
    }
}
impl core::ops::Deref for PKTSTS_R {
    type Target = crate::FieldReader<u8, PKTSTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FN` reader - Frame Number"]
pub struct FN_R(crate::FieldReader<u8, u8>);
impl FN_R {
    pub(crate) fn new(bits: u8) -> Self {
        FN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Receive Status Read and Pop Register \\[DEVICEMODE\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grxstsp_devicemode](index.html) module"]
pub struct GRXSTSP_DEVICEMODE_SPEC;
impl crate::RegisterSpec for GRXSTSP_DEVICEMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [grxstsp_devicemode::R](R) reader structure"]
impl crate::Readable for GRXSTSP_DEVICEMODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GRXSTSP_DEVICEMODE to value 0"]
impl crate::Resettable for GRXSTSP_DEVICEMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
