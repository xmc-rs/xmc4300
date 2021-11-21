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
#[doc = "Non-periodic TxFIFO Space Avail\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum NPTXFSPCAVAIL_A {
    #[doc = "0: Non-periodic TxFIFO is full"]
    VALUE1 = 0,
    #[doc = "1: 1 word available"]
    VALUE2 = 1,
    #[doc = "2: 2 words available"]
    VALUE3 = 2,
}
impl From<NPTXFSPCAVAIL_A> for u16 {
    #[inline(always)]
    fn from(variant: NPTXFSPCAVAIL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NPTxFSpcAvail` reader - Non-periodic TxFIFO Space Avail"]
pub struct NPTXFSPCAVAIL_R(crate::FieldReader<u16, NPTXFSPCAVAIL_A>);
impl NPTXFSPCAVAIL_R {
    pub(crate) fn new(bits: u16) -> Self {
        NPTXFSPCAVAIL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NPTXFSPCAVAIL_A> {
        match self.bits {
            0 => Some(NPTXFSPCAVAIL_A::VALUE1),
            1 => Some(NPTXFSPCAVAIL_A::VALUE2),
            2 => Some(NPTXFSPCAVAIL_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == NPTXFSPCAVAIL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == NPTXFSPCAVAIL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == NPTXFSPCAVAIL_A::VALUE3
    }
}
impl core::ops::Deref for NPTXFSPCAVAIL_R {
    type Target = crate::FieldReader<u16, NPTXFSPCAVAIL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Non-periodic Transmit Request Queue Space Available\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NPTXQSPCAVAIL_A {
    #[doc = "0: Non-periodic Transmit Request Queue is full"]
    VALUE1 = 0,
    #[doc = "1: 1 location available"]
    VALUE2 = 1,
    #[doc = "2: 2 locations available"]
    VALUE3 = 2,
}
impl From<NPTXQSPCAVAIL_A> for u8 {
    #[inline(always)]
    fn from(variant: NPTXQSPCAVAIL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NPTxQSpcAvail` reader - Non-periodic Transmit Request Queue Space Available"]
pub struct NPTXQSPCAVAIL_R(crate::FieldReader<u8, NPTXQSPCAVAIL_A>);
impl NPTXQSPCAVAIL_R {
    pub(crate) fn new(bits: u8) -> Self {
        NPTXQSPCAVAIL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NPTXQSPCAVAIL_A> {
        match self.bits {
            0 => Some(NPTXQSPCAVAIL_A::VALUE1),
            1 => Some(NPTXQSPCAVAIL_A::VALUE2),
            2 => Some(NPTXQSPCAVAIL_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == NPTXQSPCAVAIL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == NPTXQSPCAVAIL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == NPTXQSPCAVAIL_A::VALUE3
    }
}
impl core::ops::Deref for NPTXQSPCAVAIL_R {
    type Target = crate::FieldReader<u8, NPTXQSPCAVAIL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Top of the Non-periodic Transmit Request Queue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NPTXQTOP_A {
    #[doc = "0: IN/OUT token"]
    VALUE1 = 0,
    #[doc = "1: Zero-length transmit packet (device IN/host OUT)"]
    VALUE2 = 1,
    #[doc = "3: Channel halt command"]
    VALUE4 = 3,
}
impl From<NPTXQTOP_A> for u8 {
    #[inline(always)]
    fn from(variant: NPTXQTOP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NPTxQTop` reader - Top of the Non-periodic Transmit Request Queue"]
pub struct NPTXQTOP_R(crate::FieldReader<u8, NPTXQTOP_A>);
impl NPTXQTOP_R {
    pub(crate) fn new(bits: u8) -> Self {
        NPTXQTOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NPTXQTOP_A> {
        match self.bits {
            0 => Some(NPTXQTOP_A::VALUE1),
            1 => Some(NPTXQTOP_A::VALUE2),
            3 => Some(NPTXQTOP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == NPTXQTOP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == NPTXQTOP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == NPTXQTOP_A::VALUE4
    }
}
impl core::ops::Deref for NPTXQTOP_R {
    type Target = crate::FieldReader<u8, NPTXQTOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Non-periodic TxFIFO Space Avail"]
    #[inline(always)]
    pub fn nptx_fspc_avail(&self) -> NPTXFSPCAVAIL_R {
        NPTXFSPCAVAIL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Non-periodic Transmit Request Queue Space Available"]
    #[inline(always)]
    pub fn nptx_qspc_avail(&self) -> NPTXQSPCAVAIL_R {
        NPTXQSPCAVAIL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Top of the Non-periodic Transmit Request Queue"]
    #[inline(always)]
    pub fn nptx_qtop(&self) -> NPTXQTOP_R {
        NPTXQTOP_R::new(((self.bits >> 24) & 0x7f) as u8)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0008_0010
    }
}
