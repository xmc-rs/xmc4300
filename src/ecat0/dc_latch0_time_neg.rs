#[doc = "Register `DC_LATCH0_TIME_NEG[%s]` reader"]
pub type R = crate::R<DcLatch0TimeNegSpec>;
#[doc = "Field `DC_LATCH0_TIME_NEG` reader - Captures System time at the negative edge of the Latch0 signal"]
pub type DcLatch0TimeNegR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Captures System time at the negative edge of the Latch0 signal"]
    #[inline(always)]
    pub fn dc_latch0_time_neg(&self) -> DcLatch0TimeNegR {
        DcLatch0TimeNegR::new(self.bits)
    }
}
#[doc = "Register captures System time at the negative edge of the Latch0 signal\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_latch0_time_neg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcLatch0TimeNegSpec;
impl crate::RegisterSpec for DcLatch0TimeNegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_latch0_time_neg::R`](R) reader structure"]
impl crate::Readable for DcLatch0TimeNegSpec {}
#[doc = "`reset()` method sets DC_LATCH0_TIME_NEG[%s]
to value 0"]
impl crate::Resettable for DcLatch0TimeNegSpec {
    const RESET_VALUE: u32 = 0;
}
