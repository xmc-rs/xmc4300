#[doc = "Reader of register HCDMAB"]
pub type R = crate::R<u32, super::HCDMAB>;
#[doc = "Reader of field `Buffer_Address`"]
pub type BUFFER_ADDRESS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Address"]
    #[inline(always)]
    pub fn buffer_address(&self) -> BUFFER_ADDRESS_R {
        BUFFER_ADDRESS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
