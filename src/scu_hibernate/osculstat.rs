#[doc = "Reader of register OSCULSTAT"]
pub type R = crate::R<u32, super::OSCULSTAT>;
#[doc = "Reader of field `X1D`"]
pub type X1D_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - XTAL1 Data Value"]
    #[inline(always)]
    pub fn x1d(&self) -> X1D_R {
        X1D_R::new((self.bits & 0x01) != 0)
    }
}
