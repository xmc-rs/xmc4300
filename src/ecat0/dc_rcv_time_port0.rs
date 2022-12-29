#[doc = "Register `DC_RCV_TIME_PORT0` reader"]
pub struct R(crate::R<DC_RCV_TIME_PORT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_RCV_TIME_PORT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_RCV_TIME_PORT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_RCV_TIME_PORT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOCAL_TIME_P0` reader - Write by EtherCAT master"]
pub type LOCAL_TIME_P0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write by EtherCAT master"]
    #[inline(always)]
    pub fn local_time_p0(&self) -> LOCAL_TIME_P0_R {
        LOCAL_TIME_P0_R::new(self.bits)
    }
}
#[doc = "Receive Time Port 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_rcv_time_port0](index.html) module"]
pub struct DC_RCV_TIME_PORT0_SPEC;
impl crate::RegisterSpec for DC_RCV_TIME_PORT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dc_rcv_time_port0::R](R) reader structure"]
impl crate::Readable for DC_RCV_TIME_PORT0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DC_RCV_TIME_PORT0 to value 0"]
impl crate::Resettable for DC_RCV_TIME_PORT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
