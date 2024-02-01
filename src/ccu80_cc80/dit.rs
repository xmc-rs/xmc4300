#[doc = "Register `DIT` reader"]
pub type R = crate::R<DIT_SPEC>;
#[doc = "Field `DCV` reader - Dither compare Value"]
pub type DCV_R = crate::FieldReader;
#[doc = "Field `DCNT` reader - Dither counter actual value"]
pub type DCNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Dither compare Value"]
    #[inline(always)]
    pub fn dcv(&self) -> DCV_R {
        DCV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Dither counter actual value"]
    #[inline(always)]
    pub fn dcnt(&self) -> DCNT_R {
        DCNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "Dither Config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dit::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIT_SPEC;
impl crate::RegisterSpec for DIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dit::R`](R) reader structure"]
impl crate::Readable for DIT_SPEC {}
#[doc = "`reset()` method sets DIT to value 0"]
impl crate::Resettable for DIT_SPEC {
    const RESET_VALUE: u32 = 0;
}
