#[doc = "Reader of register EVRVADCSTAT"]
pub type R = crate::R<u32, super::EVRVADCSTAT>;
#[doc = "Reader of field `VADC13V`"]
pub type VADC13V_R = crate::R<u8, u8>;
#[doc = "Reader of field `VADC33V`"]
pub type VADC33V_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - VADC 1.3 V Conversion Result"]
    #[inline(always)]
    pub fn vadc13v(&self) -> VADC13V_R {
        VADC13V_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - VADC 3.3 V Conversion Result"]
    #[inline(always)]
    pub fn vadc33v(&self) -> VADC33V_R {
        VADC33V_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
