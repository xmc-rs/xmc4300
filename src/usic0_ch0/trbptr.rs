#[doc = "Register `TRBPTR` reader"]
pub type R = crate::R<TRBPTR_SPEC>;
#[doc = "Field `TDIPTR` reader - Transmitter Data Input Pointer"]
pub type TDIPTR_R = crate::FieldReader;
#[doc = "Field `TDOPTR` reader - Transmitter Data Output Pointer"]
pub type TDOPTR_R = crate::FieldReader;
#[doc = "Field `RDIPTR` reader - Receiver Data Input Pointer"]
pub type RDIPTR_R = crate::FieldReader;
#[doc = "Field `RDOPTR` reader - Receiver Data Output Pointer"]
pub type RDOPTR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Transmitter Data Input Pointer"]
    #[inline(always)]
    pub fn tdiptr(&self) -> TDIPTR_R {
        TDIPTR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Transmitter Data Output Pointer"]
    #[inline(always)]
    pub fn tdoptr(&self) -> TDOPTR_R {
        TDOPTR_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Receiver Data Input Pointer"]
    #[inline(always)]
    pub fn rdiptr(&self) -> RDIPTR_R {
        RDIPTR_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Receiver Data Output Pointer"]
    #[inline(always)]
    pub fn rdoptr(&self) -> RDOPTR_R {
        RDOPTR_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
#[doc = "Transmit/Receive Buffer Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trbptr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRBPTR_SPEC;
impl crate::RegisterSpec for TRBPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trbptr::R`](R) reader structure"]
impl crate::Readable for TRBPTR_SPEC {}
#[doc = "`reset()` method sets TRBPTR to value 0"]
impl crate::Resettable for TRBPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
