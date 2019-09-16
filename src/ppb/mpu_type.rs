#[doc = "Reader of register MPU_TYPE"]
pub type R = crate::R<u32, super::MPU_TYPE>;
#[doc = "Reader of field `SEPARATE`"]
pub type SEPARATE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DREGION`"]
pub type DREGION_R = crate::R<u8, u8>;
#[doc = "Reader of field `IREGION`"]
pub type IREGION_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Support for unified or separate instruction and date memory maps"]
    #[inline(always)]
    pub fn separate(&self) -> SEPARATE_R {
        SEPARATE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Number of supported MPU data regions"]
    #[inline(always)]
    pub fn dregion(&self) -> DREGION_R {
        DREGION_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of supported MPU instruction regions"]
    #[inline(always)]
    pub fn iregion(&self) -> IREGION_R {
        IREGION_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
