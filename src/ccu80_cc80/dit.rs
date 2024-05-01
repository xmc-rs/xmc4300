#[doc = "Register `DIT` reader"]
pub type R = crate::R<DitSpec>;
#[doc = "Field `DCV` reader - Dither compare Value"]
pub type DcvR = crate::FieldReader;
#[doc = "Field `DCNT` reader - Dither counter actual value"]
pub type DcntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Dither compare Value"]
    #[inline(always)]
    pub fn dcv(&self) -> DcvR {
        DcvR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Dither counter actual value"]
    #[inline(always)]
    pub fn dcnt(&self) -> DcntR {
        DcntR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "Dither Config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dit::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DitSpec;
impl crate::RegisterSpec for DitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dit::R`](R) reader structure"]
impl crate::Readable for DitSpec {}
#[doc = "`reset()` method sets DIT to value 0"]
impl crate::Resettable for DitSpec {
    const RESET_VALUE: u32 = 0;
}
