#[doc = "Register `RX_ERR_COUNT1` reader"]
pub type R = crate::R<RxErrCount1Spec>;
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
#[doc = "RX Error Counter Port 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_err_count1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxErrCount1Spec;
impl crate::RegisterSpec for RxErrCount1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rx_err_count1::R`](R) reader structure"]
impl crate::Readable for RxErrCount1Spec {}
#[doc = "`reset()` method sets RX_ERR_COUNT1 to value 0"]
impl crate::Resettable for RxErrCount1Spec {
    const RESET_VALUE: u16 = 0;
}
