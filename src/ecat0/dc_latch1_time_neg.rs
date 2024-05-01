#[doc = "Register `DC_LATCH1_TIME_NEG[%s]` reader"]
pub type R = crate::R<DcLatch1TimeNegSpec>;
#[doc = "Field `DC_LATCH1_TIME_NEG` reader - Captures System time at the negative edge of the Latch1 signal"]
pub type DcLatch1TimeNegR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Captures System time at the negative edge of the Latch1 signal"]
    #[inline(always)]
    pub fn dc_latch1_time_neg(&self) -> DcLatch1TimeNegR {
        DcLatch1TimeNegR::new(self.bits)
    }
}
#[doc = "Register captures System time at the negative edge of the Latch1 signal\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_latch1_time_neg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcLatch1TimeNegSpec;
impl crate::RegisterSpec for DcLatch1TimeNegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_latch1_time_neg::R`](R) reader structure"]
impl crate::Readable for DcLatch1TimeNegSpec {}
#[doc = "`reset()` method sets DC_LATCH1_TIME_NEG[%s]
to value 0"]
impl crate::Resettable for DcLatch1TimeNegSpec {
    const RESET_VALUE: u32 = 0;
}
