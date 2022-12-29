#[doc = "Register `GNPTXSTS` reader"]
pub struct R(crate::R<GNPTXSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GNPTXSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GNPTXSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GNPTXSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NPTxFSpcAvail` reader - Non-periodic TxFIFO Space Avail"]
pub type NPTX_FSPC_AVAIL_R = crate::FieldReader<u16, NPTX_FSPC_AVAIL_A>;
#[doc = "Non-periodic TxFIFO Space Avail\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum NPTX_FSPC_AVAIL_A {
    #[doc = "0: Non-periodic TxFIFO is full"]
    VALUE1 = 0,
    #[doc = "1: 1 word available"]
    VALUE2 = 1,
    #[doc = "2: 2 words available"]
    VALUE3 = 2,
}
impl From<NPTX_FSPC_AVAIL_A> for u16 {
    #[inline(always)]
    fn from(variant: NPTX_FSPC_AVAIL_A) -> Self {
        variant as _
    }
}
impl NPTX_FSPC_AVAIL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NPTX_FSPC_AVAIL_A> {
        match self.bits {
            0 => Some(NPTX_FSPC_AVAIL_A::VALUE1),
            1 => Some(NPTX_FSPC_AVAIL_A::VALUE2),
            2 => Some(NPTX_FSPC_AVAIL_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NPTX_FSPC_AVAIL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NPTX_FSPC_AVAIL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == NPTX_FSPC_AVAIL_A::VALUE3
    }
}
#[doc = "Field `NPTxQSpcAvail` reader - Non-periodic Transmit Request Queue Space Available"]
pub type NPTX_QSPC_AVAIL_R = crate::FieldReader<u8, NPTX_QSPC_AVAIL_A>;
#[doc = "Non-periodic Transmit Request Queue Space Available\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NPTX_QSPC_AVAIL_A {
    #[doc = "0: Non-periodic Transmit Request Queue is full"]
    VALUE1 = 0,
    #[doc = "1: 1 location available"]
    VALUE2 = 1,
    #[doc = "2: 2 locations available"]
    VALUE3 = 2,
}
impl From<NPTX_QSPC_AVAIL_A> for u8 {
    #[inline(always)]
    fn from(variant: NPTX_QSPC_AVAIL_A) -> Self {
        variant as _
    }
}
impl NPTX_QSPC_AVAIL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NPTX_QSPC_AVAIL_A> {
        match self.bits {
            0 => Some(NPTX_QSPC_AVAIL_A::VALUE1),
            1 => Some(NPTX_QSPC_AVAIL_A::VALUE2),
            2 => Some(NPTX_QSPC_AVAIL_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NPTX_QSPC_AVAIL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NPTX_QSPC_AVAIL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == NPTX_QSPC_AVAIL_A::VALUE3
    }
}
#[doc = "Field `NPTxQTop` reader - Top of the Non-periodic Transmit Request Queue"]
pub type NPTX_QTOP_R = crate::FieldReader<u8, NPTX_QTOP_A>;
#[doc = "Top of the Non-periodic Transmit Request Queue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NPTX_QTOP_A {
    #[doc = "0: IN/OUT token"]
    VALUE1 = 0,
    #[doc = "1: Zero-length transmit packet (device IN/host OUT)"]
    VALUE2 = 1,
    #[doc = "3: Channel halt command"]
    VALUE4 = 3,
}
impl From<NPTX_QTOP_A> for u8 {
    #[inline(always)]
    fn from(variant: NPTX_QTOP_A) -> Self {
        variant as _
    }
}
impl NPTX_QTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NPTX_QTOP_A> {
        match self.bits {
            0 => Some(NPTX_QTOP_A::VALUE1),
            1 => Some(NPTX_QTOP_A::VALUE2),
            3 => Some(NPTX_QTOP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NPTX_QTOP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NPTX_QTOP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == NPTX_QTOP_A::VALUE4
    }
}
impl R {
    #[doc = "Bits 0:15 - Non-periodic TxFIFO Space Avail"]
    #[inline(always)]
    pub fn nptx_fspc_avail(&self) -> NPTX_FSPC_AVAIL_R {
        NPTX_FSPC_AVAIL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Non-periodic Transmit Request Queue Space Available"]
    #[inline(always)]
    pub fn nptx_qspc_avail(&self) -> NPTX_QSPC_AVAIL_R {
        NPTX_QSPC_AVAIL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Top of the Non-periodic Transmit Request Queue"]
    #[inline(always)]
    pub fn nptx_qtop(&self) -> NPTX_QTOP_R {
        NPTX_QTOP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "Non-Periodic Transmit FIFO/Queue Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gnptxsts](index.html) module"]
pub struct GNPTXSTS_SPEC;
impl crate::RegisterSpec for GNPTXSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gnptxsts::R](R) reader structure"]
impl crate::Readable for GNPTXSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GNPTXSTS to value 0x0008_0010"]
impl crate::Resettable for GNPTXSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0010;
}
