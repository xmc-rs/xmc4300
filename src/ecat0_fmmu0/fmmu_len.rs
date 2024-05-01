#[doc = "Register `FMMU_LEN` reader"]
pub type R = crate::R<FMMU_LEN_SPEC>;
#[doc = "Field `OFFSET` reader - Offset from the first logical FMMU Byte to the last FMMU Byte + 1"]
pub type OFFSET_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Offset from the first logical FMMU Byte to the last FMMU Byte + 1"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(self.bits)
    }
}
#[doc = "Length FMMU 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_len::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMMU_LEN_SPEC;
impl crate::RegisterSpec for FMMU_LEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fmmu_len::R`](R) reader structure"]
impl crate::Readable for FMMU_LEN_SPEC {}
#[doc = "`reset()` method sets FMMU_LEN to value 0"]
impl crate::Resettable for FMMU_LEN_SPEC {
    const RESET_VALUE: u16 = 0;
}
