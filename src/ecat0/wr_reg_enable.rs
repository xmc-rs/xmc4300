#[doc = "Register `WR_REG_ENABLE` reader"]
pub type R = crate::R<WrRegEnableSpec>;
#[doc = "Field `WR_REG_EN` reader - Write register protection enabled"]
pub type WrRegEnR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Write register protection enabled"]
    #[inline(always)]
    pub fn wr_reg_en(&self) -> WrRegEnR {
        WrRegEnR::new((self.bits & 1) != 0)
    }
}
#[doc = "Write Register Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_reg_enable::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrRegEnableSpec;
impl crate::RegisterSpec for WrRegEnableSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wr_reg_enable::R`](R) reader structure"]
impl crate::Readable for WrRegEnableSpec {}
#[doc = "`reset()` method sets WR_REG_ENABLE to value 0"]
impl crate::Resettable for WrRegEnableSpec {
    const RESET_VALUE: u8 = 0;
}
