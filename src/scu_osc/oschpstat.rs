#[doc = "Register `OSCHPSTAT` reader"]
pub type R = crate::R<OSCHPSTAT_SPEC>;
#[doc = "Field `X1D` reader - XTAL1 Data Value"]
pub type X1D_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - XTAL1 Data Value"]
    #[inline(always)]
    pub fn x1d(&self) -> X1D_R {
        X1D_R::new((self.bits & 1) != 0)
    }
}
#[doc = "OSC_HP Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`oschpstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCHPSTAT_SPEC;
impl crate::RegisterSpec for OSCHPSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oschpstat::R`](R) reader structure"]
impl crate::Readable for OSCHPSTAT_SPEC {}
#[doc = "`reset()` method sets OSCHPSTAT to value 0"]
impl crate::Resettable for OSCHPSTAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
