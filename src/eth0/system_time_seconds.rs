#[doc = "Reader of register SYSTEM_TIME_SECONDS"]
pub type R = crate::R<u32, super::SYSTEM_TIME_SECONDS>;
#[doc = "Reader of field `TSS`"]
pub type TSS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Second"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
