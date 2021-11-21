#[doc = "Register `DTXFSTS0` reader"]
pub struct R(crate::R<DTXFSTS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTXFSTS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTXFSTS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTXFSTS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "IN Endpoint TxFIFO Space Avail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum INEPTXFSPCAVAIL_A {
    #[doc = "0: Endpoint TxFIFO is full"]
    VALUE1 = 0,
    #[doc = "1: 1 word available"]
    VALUE2 = 1,
    #[doc = "2: 2 words available"]
    VALUE3 = 2,
}
impl From<INEPTXFSPCAVAIL_A> for u16 {
    #[inline(always)]
    fn from(variant: INEPTXFSPCAVAIL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INEPTxFSpcAvail` reader - IN Endpoint TxFIFO Space Avail"]
pub struct INEPTXFSPCAVAIL_R(crate::FieldReader<u16, INEPTXFSPCAVAIL_A>);
impl INEPTXFSPCAVAIL_R {
    pub(crate) fn new(bits: u16) -> Self {
        INEPTXFSPCAVAIL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INEPTXFSPCAVAIL_A> {
        match self.bits {
            0 => Some(INEPTXFSPCAVAIL_A::VALUE1),
            1 => Some(INEPTXFSPCAVAIL_A::VALUE2),
            2 => Some(INEPTXFSPCAVAIL_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == INEPTXFSPCAVAIL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == INEPTXFSPCAVAIL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == INEPTXFSPCAVAIL_A::VALUE3
    }
}
impl core::ops::Deref for INEPTXFSPCAVAIL_R {
    type Target = crate::FieldReader<u16, INEPTXFSPCAVAIL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - IN Endpoint TxFIFO Space Avail"]
    #[inline(always)]
    pub fn ineptx_fspc_avail(&self) -> INEPTXFSPCAVAIL_R {
        INEPTXFSPCAVAIL_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Device IN Endpoint Transmit FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtxfsts0](index.html) module"]
pub struct DTXFSTS0_SPEC;
impl crate::RegisterSpec for DTXFSTS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtxfsts0::R](R) reader structure"]
impl crate::Readable for DTXFSTS0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DTXFSTS0 to value 0"]
impl crate::Resettable for DTXFSTS0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
