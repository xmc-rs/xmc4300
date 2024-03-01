#[doc = "Register `GNPTXSTS` reader"]
pub type R = crate::R<GnptxstsSpec>;
#[doc = "Non-periodic TxFIFO Space Avail\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum NptxFspcAvail {
    #[doc = "0: Non-periodic TxFIFO is full"]
    Value1 = 0,
    #[doc = "1: 1 word available"]
    Value2 = 1,
    #[doc = "2: 2 words available"]
    Value3 = 2,
}
impl From<NptxFspcAvail> for u16 {
    #[inline(always)]
    fn from(variant: NptxFspcAvail) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NptxFspcAvail {
    type Ux = u16;
}
#[doc = "Field `NPTxFSpcAvail` reader - Non-periodic TxFIFO Space Avail"]
pub type NptxFspcAvailR = crate::FieldReader<NptxFspcAvail>;
impl NptxFspcAvailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NptxFspcAvail> {
        match self.bits {
            0 => Some(NptxFspcAvail::Value1),
            1 => Some(NptxFspcAvail::Value2),
            2 => Some(NptxFspcAvail::Value3),
            _ => None,
        }
    }
    #[doc = "Non-periodic TxFIFO is full"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NptxFspcAvail::Value1
    }
    #[doc = "1 word available"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NptxFspcAvail::Value2
    }
    #[doc = "2 words available"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == NptxFspcAvail::Value3
    }
}
#[doc = "Non-periodic Transmit Request Queue Space Available\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NptxQspcAvail {
    #[doc = "0: Non-periodic Transmit Request Queue is full"]
    Value1 = 0,
    #[doc = "1: 1 location available"]
    Value2 = 1,
    #[doc = "2: 2 locations available"]
    Value3 = 2,
}
impl From<NptxQspcAvail> for u8 {
    #[inline(always)]
    fn from(variant: NptxQspcAvail) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NptxQspcAvail {
    type Ux = u8;
}
#[doc = "Field `NPTxQSpcAvail` reader - Non-periodic Transmit Request Queue Space Available"]
pub type NptxQspcAvailR = crate::FieldReader<NptxQspcAvail>;
impl NptxQspcAvailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NptxQspcAvail> {
        match self.bits {
            0 => Some(NptxQspcAvail::Value1),
            1 => Some(NptxQspcAvail::Value2),
            2 => Some(NptxQspcAvail::Value3),
            _ => None,
        }
    }
    #[doc = "Non-periodic Transmit Request Queue is full"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NptxQspcAvail::Value1
    }
    #[doc = "1 location available"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NptxQspcAvail::Value2
    }
    #[doc = "2 locations available"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == NptxQspcAvail::Value3
    }
}
#[doc = "Top of the Non-periodic Transmit Request Queue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NptxQtop {
    #[doc = "0: IN/OUT token"]
    Value1 = 0,
    #[doc = "1: Zero-length transmit packet (device IN/host OUT)"]
    Value2 = 1,
    #[doc = "3: Channel halt command"]
    Value4 = 3,
}
impl From<NptxQtop> for u8 {
    #[inline(always)]
    fn from(variant: NptxQtop) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NptxQtop {
    type Ux = u8;
}
#[doc = "Field `NPTxQTop` reader - Top of the Non-periodic Transmit Request Queue"]
pub type NptxQtopR = crate::FieldReader<NptxQtop>;
impl NptxQtopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NptxQtop> {
        match self.bits {
            0 => Some(NptxQtop::Value1),
            1 => Some(NptxQtop::Value2),
            3 => Some(NptxQtop::Value4),
            _ => None,
        }
    }
    #[doc = "IN/OUT token"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NptxQtop::Value1
    }
    #[doc = "Zero-length transmit packet (device IN/host OUT)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NptxQtop::Value2
    }
    #[doc = "Channel halt command"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == NptxQtop::Value4
    }
}
impl R {
    #[doc = "Bits 0:15 - Non-periodic TxFIFO Space Avail"]
    #[inline(always)]
    pub fn nptx_fspc_avail(&self) -> NptxFspcAvailR {
        NptxFspcAvailR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Non-periodic Transmit Request Queue Space Available"]
    #[inline(always)]
    pub fn nptx_qspc_avail(&self) -> NptxQspcAvailR {
        NptxQspcAvailR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Top of the Non-periodic Transmit Request Queue"]
    #[inline(always)]
    pub fn nptx_qtop(&self) -> NptxQtopR {
        NptxQtopR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "Non-Periodic Transmit FIFO/Queue Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GnptxstsSpec;
impl crate::RegisterSpec for GnptxstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gnptxsts::R`](R) reader structure"]
impl crate::Readable for GnptxstsSpec {}
#[doc = "`reset()` method sets GNPTXSTS to value 0x0008_0010"]
impl crate::Resettable for GnptxstsSpec {
    const RESET_VALUE: u32 = 0x0008_0010;
}
