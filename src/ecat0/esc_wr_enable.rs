#[doc = "Register `ESC_WR_ENABLE` reader"]
pub type R = crate::R<ESC_WR_ENABLE_SPEC>;
#[doc = "Field `ESC_WR_PROT` reader - Write protection enabled"]
pub type ESC_WR_PROT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Write protection enabled"]
    #[inline(always)]
    pub fn esc_wr_prot(&self) -> ESC_WR_PROT_R {
        ESC_WR_PROT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "ESC Write Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esc_wr_enable::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ESC_WR_ENABLE_SPEC;
impl crate::RegisterSpec for ESC_WR_ENABLE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`esc_wr_enable::R`](R) reader structure"]
impl crate::Readable for ESC_WR_ENABLE_SPEC {}
#[doc = "`reset()` method sets ESC_WR_ENABLE to value 0"]
impl crate::Resettable for ESC_WR_ENABLE_SPEC {
    const RESET_VALUE: u8 = 0;
}
