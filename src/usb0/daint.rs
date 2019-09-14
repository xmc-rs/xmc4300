#[doc = "Reader of register DAINT"]
pub type R = crate::R<u32, super::DAINT>;
#[doc = "Reader of field `InEpInt`"]
pub type INEPINT_R = crate::R<u16, u16>;
#[doc = "Reader of field `OutEPInt`"]
pub type OUTEPINT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN Endpoint Interrupt Bits"]
    #[inline(always)]
    pub fn in_ep_int(&self) -> INEPINT_R {
        INEPINT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT Endpoint Interrupt Bits"]
    #[inline(always)]
    pub fn out_epint(&self) -> OUTEPINT_R {
        OUTEPINT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
