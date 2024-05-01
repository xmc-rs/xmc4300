#[doc = "Register `RXTCP_ERROR_FRAMES` reader"]
pub type R = crate::R<RxtcpErrorFramesSpec>;
#[doc = "Field `RXTCPERRFRM` reader - This field indicates the number of good IP datagrams whose TCP payload has a checksum error."]
pub type RxtcperrfrmR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IP datagrams whose TCP payload has a checksum error."]
    #[inline(always)]
    pub fn rxtcperrfrm(&self) -> RxtcperrfrmR {
        RxtcperrfrmR::new(self.bits)
    }
}
#[doc = "RxTCP Error Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtcp_error_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxtcpErrorFramesSpec;
impl crate::RegisterSpec for RxtcpErrorFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxtcp_error_frames::R`](R) reader structure"]
impl crate::Readable for RxtcpErrorFramesSpec {}
#[doc = "`reset()` method sets RXTCP_ERROR_FRAMES to value 0"]
impl crate::Resettable for RxtcpErrorFramesSpec {
    const RESET_VALUE: u32 = 0;
}
