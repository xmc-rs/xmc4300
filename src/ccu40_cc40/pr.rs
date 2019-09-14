#[doc = "Reader of register PR"]
pub type R = crate::R<u32, super::PR>;
#[doc = "Reader of field `PR`"]
pub type PR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Period Register"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 0xffff) as u16)
    }
}
