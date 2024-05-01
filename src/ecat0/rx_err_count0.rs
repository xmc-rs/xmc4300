#[doc = "Register `RX_ERR_COUNT0` reader"]
pub type R = crate::R<RxErrCount0Spec>;
#[doc = "Field `INVALID_FRAME` reader - Invalid frame counter of Port y"]
pub type InvalidFrameR = crate::FieldReader;
#[doc = "Field `RX_ERROR` reader - RX Error counter of Port y"]
pub type RxErrorR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Invalid frame counter of Port y"]
    #[inline(always)]
    pub fn invalid_frame(&self) -> InvalidFrameR {
        InvalidFrameR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RX Error counter of Port y"]
    #[inline(always)]
    pub fn rx_error(&self) -> RxErrorR {
        RxErrorR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "RX Error Counter Port 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_err_count0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxErrCount0Spec;
impl crate::RegisterSpec for RxErrCount0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rx_err_count0::R`](R) reader structure"]
impl crate::Readable for RxErrCount0Spec {}
#[doc = "`reset()` method sets RX_ERR_COUNT0 to value 0"]
impl crate::Resettable for RxErrCount0Spec {
    const RESET_VALUE: u16 = 0;
}
