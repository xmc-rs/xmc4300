#[doc = "Reader of register IDMANUF"]
pub type R = crate::R<u32, super::IDMANUF>;
#[doc = "Reader of field `DEPT`"]
pub type DEPT_R = crate::R<u8, u8>;
#[doc = "Reader of field `MANUF`"]
pub type MANUF_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:4 - Department Identification Number"]
    #[inline(always)]
    pub fn dept(&self) -> DEPT_R {
        DEPT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:15 - Manufacturer Identification Number"]
    #[inline(always)]
    pub fn manuf(&self) -> MANUF_R {
        MANUF_R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
}
