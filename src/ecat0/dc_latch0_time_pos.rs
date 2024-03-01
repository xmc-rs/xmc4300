#[doc = "Register `DC_LATCH0_TIME_POS[%s]` reader"]
pub type R = crate::R<DcLatch0TimePosSpec>;
#[doc = "Field `DC_LATCH0_TIME_POS` reader - Captures System time at the positive edge of the Latch0 signal"]
pub type DcLatch0TimePosR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Captures System time at the positive edge of the Latch0 signal"]
    #[inline(always)]
    pub fn dc_latch0_time_pos(&self) -> DcLatch0TimePosR {
        DcLatch0TimePosR::new(self.bits)
    }
}
#[doc = "Register captures System time at the positive edge of the Latch0 signal\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_latch0_time_pos::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcLatch0TimePosSpec;
impl crate::RegisterSpec for DcLatch0TimePosSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_latch0_time_pos::R`](R) reader structure"]
impl crate::Readable for DcLatch0TimePosSpec {}
#[doc = "`reset()` method sets DC_LATCH0_TIME_POS[%s]
to value 0"]
impl crate::Resettable for DcLatch0TimePosSpec {
    const RESET_VALUE: u32 = 0;
}
