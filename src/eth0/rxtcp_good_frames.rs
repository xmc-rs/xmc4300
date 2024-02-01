#[doc = "Register `RXTCP_GOOD_FRAMES` reader"]
pub type R = crate::R<RXTCP_GOOD_FRAMES_SPEC>;
#[doc = "Field `RXTCPGDFRM` reader - This field indicates the number of good IP datagrams with a good TCP payload."]
pub type RXTCPGDFRM_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good IP datagrams with a good TCP payload."]
    #[inline(always)]
    pub fn rxtcpgdfrm(&self) -> RXTCPGDFRM_R {
        RXTCPGDFRM_R::new(self.bits)
    }
}
#[doc = "RxTCP Good Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtcp_good_frames::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXTCP_GOOD_FRAMES_SPEC;
impl crate::RegisterSpec for RXTCP_GOOD_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxtcp_good_frames::R`](R) reader structure"]
impl crate::Readable for RXTCP_GOOD_FRAMES_SPEC {}
#[doc = "`reset()` method sets RXTCP_GOOD_FRAMES to value 0"]
impl crate::Resettable for RXTCP_GOOD_FRAMES_SPEC {
    const RESET_VALUE: u32 = 0;
}
