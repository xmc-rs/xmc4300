#[doc = "Register `TRBPTR` reader"]
pub type R = crate::R<TrbptrSpec>;
#[doc = "Field `TDIPTR` reader - Transmitter Data Input Pointer"]
pub type TdiptrR = crate::FieldReader;
#[doc = "Field `TDOPTR` reader - Transmitter Data Output Pointer"]
pub type TdoptrR = crate::FieldReader;
#[doc = "Field `RDIPTR` reader - Receiver Data Input Pointer"]
pub type RdiptrR = crate::FieldReader;
#[doc = "Field `RDOPTR` reader - Receiver Data Output Pointer"]
pub type RdoptrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Transmitter Data Input Pointer"]
    #[inline(always)]
    pub fn tdiptr(&self) -> TdiptrR {
        TdiptrR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Transmitter Data Output Pointer"]
    #[inline(always)]
    pub fn tdoptr(&self) -> TdoptrR {
        TdoptrR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Receiver Data Input Pointer"]
    #[inline(always)]
    pub fn rdiptr(&self) -> RdiptrR {
        RdiptrR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Receiver Data Output Pointer"]
    #[inline(always)]
    pub fn rdoptr(&self) -> RdoptrR {
        RdoptrR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
#[doc = "Transmit/Receive Buffer Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trbptr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrbptrSpec;
impl crate::RegisterSpec for TrbptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trbptr::R`](R) reader structure"]
impl crate::Readable for TrbptrSpec {}
#[doc = "`reset()` method sets TRBPTR to value 0"]
impl crate::Resettable for TrbptrSpec {
    const RESET_VALUE: u32 = 0;
}
