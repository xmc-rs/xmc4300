#[doc = "Reader of register REVISION"]
pub type R = crate::R<u8, super::REVISION>;
#[doc = "Reader of field `Revision`"]
pub type REVISION_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Revision of EtherCAT controller"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new((self.bits & 0xff) as u8)
    }
}
