#[doc = "Register `OSCULSTAT` reader"]
pub type R = crate::R<OSCULSTAT_SPEC>;
#[doc = "Field `X1D` reader - XTAL1 Data Value"]
pub type X1D_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - XTAL1 Data Value"]
    #[inline(always)]
    pub fn x1d(&self) -> X1D_R {
        X1D_R::new((self.bits & 1) != 0)
    }
}
#[doc = "OSC_ULP Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osculstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCULSTAT_SPEC;
impl crate::RegisterSpec for OSCULSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osculstat::R`](R) reader structure"]
impl crate::Readable for OSCULSTAT_SPEC {}
#[doc = "`reset()` method sets OSCULSTAT to value 0"]
impl crate::Resettable for OSCULSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
