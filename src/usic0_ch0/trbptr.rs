#[doc = "Register `TRBPTR` reader"]
pub struct R(crate::R<TRBPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRBPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRBPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRBPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TDIPTR` reader - Transmitter Data Input Pointer"]
pub type TDIPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TDOPTR` reader - Transmitter Data Output Pointer"]
pub type TDOPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDIPTR` reader - Receiver Data Input Pointer"]
pub type RDIPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDOPTR` reader - Receiver Data Output Pointer"]
pub type RDOPTR_R = crate::FieldReader<u8, u8>;
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
#[doc = "Transmit/Receive Buffer Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trbptr](index.html) module"]
pub struct TRBPTR_SPEC;
impl crate::RegisterSpec for TRBPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trbptr::R](R) reader structure"]
impl crate::Readable for TRBPTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRBPTR to value 0"]
impl crate::Resettable for TRBPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
