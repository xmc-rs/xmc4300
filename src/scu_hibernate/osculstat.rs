#[doc = "Register `OSCULSTAT` reader"]
pub struct R(crate::R<OSCULSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCULSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCULSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCULSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `X1D` reader - XTAL1 Data Value"]
pub type X1D_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - XTAL1 Data Value"]
    #[inline(always)]
    pub fn x1d(&self) -> X1D_R {
        X1D_R::new((self.bits & 1) != 0)
    }
}
#[doc = "OSC_ULP Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osculstat](index.html) module"]
pub struct OSCULSTAT_SPEC;
impl crate::RegisterSpec for OSCULSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osculstat::R](R) reader structure"]
impl crate::Readable for OSCULSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OSCULSTAT to value 0"]
impl crate::Resettable for OSCULSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
