#[doc = "Reader of register DIT"]
pub type R = crate::R<u32, super::DIT>;
#[doc = "Reader of field `DCV`"]
pub type DCV_R = crate::R<u8, u8>;
#[doc = "Reader of field `DCNT`"]
pub type DCNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Dither compare Value"]
    #[inline(always)]
    pub fn dcv(&self) -> DCV_R {
        DCV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Dither counter actual value"]
    #[inline(always)]
    pub fn dcnt(&self) -> DCNT_R {
        DCNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
