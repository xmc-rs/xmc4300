#[doc = "Register `HCDMAB` reader"]
pub struct R(crate::R<HCDMAB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCDMAB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCDMAB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCDMAB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `Buffer_Address` reader - Buffer Address"]
pub type BUFFER_ADDRESS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Address"]
    #[inline(always)]
    pub fn buffer_address(&self) -> BUFFER_ADDRESS_R {
        BUFFER_ADDRESS_R::new(self.bits)
    }
}
#[doc = "Host Channel DMA Buffer Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdmab](index.html) module"]
pub struct HCDMAB_SPEC;
impl crate::RegisterSpec for HCDMAB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcdmab::R](R) reader structure"]
impl crate::Readable for HCDMAB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HCDMAB to value 0"]
impl crate::Resettable for HCDMAB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
