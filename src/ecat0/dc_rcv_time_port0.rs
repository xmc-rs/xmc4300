#[doc = "Register `DC_RCV_TIME_PORT0` reader"]
pub type R = crate::R<DC_RCV_TIME_PORT0_SPEC>;
#[doc = "Field `LOCAL_TIME_P0` reader - Write by EtherCAT master"]
pub type LOCAL_TIME_P0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Write by EtherCAT master"]
    #[inline(always)]
    pub fn local_time_p0(&self) -> LOCAL_TIME_P0_R {
        LOCAL_TIME_P0_R::new(self.bits)
    }
}
#[doc = "Receive Time Port 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_rcv_time_port0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC_RCV_TIME_PORT0_SPEC;
impl crate::RegisterSpec for DC_RCV_TIME_PORT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_rcv_time_port0::R`](R) reader structure"]
impl crate::Readable for DC_RCV_TIME_PORT0_SPEC {}
#[doc = "`reset()` method sets DC_RCV_TIME_PORT0 to value 0"]
impl crate::Resettable for DC_RCV_TIME_PORT0_SPEC {
    const RESET_VALUE: u32 = 0;
}
