#[doc = "Register `RECEIVE_TIME_PU[%s]` reader"]
pub type R = crate::R<ReceiveTimePuSpec>;
#[doc = "Field `RECEIVE_TIME_PU` reader - Local time of the beginning of a frame"]
pub type ReceiveTimePuR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Local time of the beginning of a frame"]
    #[inline(always)]
    pub fn receive_time_pu(&self) -> ReceiveTimePuR {
        ReceiveTimePuR::new(self.bits)
    }
}
#[doc = "Local time of the beginning of a frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_time_pu::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReceiveTimePuSpec;
impl crate::RegisterSpec for ReceiveTimePuSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`receive_time_pu::R`](R) reader structure"]
impl crate::Readable for ReceiveTimePuSpec {}
#[doc = "`reset()` method sets RECEIVE_TIME_PU[%s]
to value 0"]
impl crate::Resettable for ReceiveTimePuSpec {
    const RESET_VALUE: u32 = 0;
}
