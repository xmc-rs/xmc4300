#[doc = "Register `FMMU_LEN` reader"]
pub type R = crate::R<FmmuLenSpec>;
#[doc = "Field `OFFSET` reader - Offset from the first logical FMMU Byte to the last FMMU Byte + 1"]
pub type OffsetR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Offset from the first logical FMMU Byte to the last FMMU Byte + 1"]
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new(self.bits)
    }
}
#[doc = "Length FMMU 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_len::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmmuLenSpec;
impl crate::RegisterSpec for FmmuLenSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fmmu_len::R`](R) reader structure"]
impl crate::Readable for FmmuLenSpec {}
#[doc = "`reset()` method sets FMMU_LEN to value 0"]
impl crate::Resettable for FmmuLenSpec {
    const RESET_VALUE: u16 = 0;
}
