#[doc = "Register `DC_RCV_TIME_PORT1` reader"]
pub type R = crate::R<DcRcvTimePort1Spec>;
#[doc = "Field `LOCAL_TIME_P1` reader - Local time of the beginning of a frame"]
pub type LocalTimeP1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Local time of the beginning of a frame"]
    #[inline(always)]
    pub fn local_time_p1(&self) -> LocalTimeP1R {
        LocalTimeP1R::new(self.bits)
    }
}
#[doc = "Receive Time Port 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_rcv_time_port1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcRcvTimePort1Spec;
impl crate::RegisterSpec for DcRcvTimePort1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_rcv_time_port1::R`](R) reader structure"]
impl crate::Readable for DcRcvTimePort1Spec {}
#[doc = "`reset()` method sets DC_RCV_TIME_PORT1 to value 0"]
impl crate::Resettable for DcRcvTimePort1Spec {
    const RESET_VALUE: u32 = 0;
}
