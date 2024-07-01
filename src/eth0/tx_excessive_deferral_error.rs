#[doc = "Register `TX_EXCESSIVE_DEFERRAL_ERROR` reader"]
pub type R = crate::R<TX_EXCESSIVE_DEFERRAL_ERROR_SPEC>;
#[doc = "Field `TXEXSDEF` reader - This field indicates the number of frames aborted because of excessive deferral error, that is, frames deferred for more than two max-sized frame times."]
pub type TXEXSDEF_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames aborted because of excessive deferral error, that is, frames deferred for more than two max-sized frame times."]
    #[inline(always)]
    pub fn txexsdef(&self) -> TXEXSDEF_R {
        TXEXSDEF_R::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Excessive Deferral Error Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_excessive_deferral_error::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_EXCESSIVE_DEFERRAL_ERROR_SPEC;
impl crate::RegisterSpec for TX_EXCESSIVE_DEFERRAL_ERROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_excessive_deferral_error::R`](R) reader structure"]
impl crate::Readable for TX_EXCESSIVE_DEFERRAL_ERROR_SPEC {}
#[doc = "`reset()` method sets TX_EXCESSIVE_DEFERRAL_ERROR to value 0"]
impl crate::Resettable for TX_EXCESSIVE_DEFERRAL_ERROR_SPEC {
    const RESET_VALUE: u32 = 0;
}
