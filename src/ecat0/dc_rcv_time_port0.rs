#[doc = "Register `DC_RCV_TIME_PORT0` reader"]
pub type R = crate::R<DcRcvTimePort0Spec>;
#[doc = "Field `LOCAL_TIME_P0` reader - Write by EtherCAT master"]
pub type LocalTimeP0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Write by EtherCAT master"]
    #[inline(always)]
    pub fn local_time_p0(&self) -> LocalTimeP0R {
        LocalTimeP0R::new(self.bits)
    }
}
#[doc = "Receive Time Port 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_rcv_time_port0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcRcvTimePort0Spec;
impl crate::RegisterSpec for DcRcvTimePort0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_rcv_time_port0::R`](R) reader structure"]
impl crate::Readable for DcRcvTimePort0Spec {}
#[doc = "`reset()` method sets DC_RCV_TIME_PORT0 to value 0"]
impl crate::Resettable for DcRcvTimePort0Spec {
    const RESET_VALUE: u32 = 0;
}
