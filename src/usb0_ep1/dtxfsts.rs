#[doc = "Register `DTXFSTS` reader"]
pub type R = crate::R<DtxfstsSpec>;
#[doc = "IN Endpoint TxFIFO Space Avail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum IneptxFspcAvail {
    #[doc = "0: Endpoint TxFIFO is full"]
    Value1 = 0,
    #[doc = "1: 1 word available"]
    Value2 = 1,
    #[doc = "2: 2 words available"]
    Value3 = 2,
}
impl From<IneptxFspcAvail> for u16 {
    #[inline(always)]
    fn from(variant: IneptxFspcAvail) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IneptxFspcAvail {
    type Ux = u16;
}
#[doc = "Field `INEPTxFSpcAvail` reader - IN Endpoint TxFIFO Space Avail"]
pub type IneptxFspcAvailR = crate::FieldReader<IneptxFspcAvail>;
impl IneptxFspcAvailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IneptxFspcAvail> {
        match self.bits {
            0 => Some(IneptxFspcAvail::Value1),
            1 => Some(IneptxFspcAvail::Value2),
            2 => Some(IneptxFspcAvail::Value3),
            _ => None,
        }
    }
    #[doc = "Endpoint TxFIFO is full"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IneptxFspcAvail::Value1
    }
    #[doc = "1 word available"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IneptxFspcAvail::Value2
    }
    #[doc = "2 words available"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == IneptxFspcAvail::Value3
    }
}
impl R {
    #[doc = "Bits 0:15 - IN Endpoint TxFIFO Space Avail"]
    #[inline(always)]
    pub fn ineptx_fspc_avail(&self) -> IneptxFspcAvailR {
        IneptxFspcAvailR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Device IN Endpoint Transmit FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtxfstsSpec;
impl crate::RegisterSpec for DtxfstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts::R`](R) reader structure"]
impl crate::Readable for DtxfstsSpec {}
#[doc = "`reset()` method sets DTXFSTS to value 0"]
impl crate::Resettable for DtxfstsSpec {
    const RESET_VALUE: u32 = 0;
}
