#[doc = "Register `ESC_WR_ENABLE` reader"]
pub type R = crate::R<EscWrEnableSpec>;
#[doc = "Field `ESC_WR_PROT` reader - Write protection enabled"]
pub type EscWrProtR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Write protection enabled"]
    #[inline(always)]
    pub fn esc_wr_prot(&self) -> EscWrProtR {
        EscWrProtR::new((self.bits & 1) != 0)
    }
}
#[doc = "ESC Write Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esc_wr_enable::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EscWrEnableSpec;
impl crate::RegisterSpec for EscWrEnableSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`esc_wr_enable::R`](R) reader structure"]
impl crate::Readable for EscWrEnableSpec {}
#[doc = "`reset()` method sets ESC_WR_ENABLE to value 0"]
impl crate::Resettable for EscWrEnableSpec {
    const RESET_VALUE: u8 = 0;
}
