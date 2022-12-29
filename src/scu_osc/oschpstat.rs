#[doc = "Register `OSCHPSTAT` reader"]
pub struct R(crate::R<OSCHPSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCHPSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCHPSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCHPSTAT_SPEC>) -> Self {
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
#[doc = "OSC_HP Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oschpstat](index.html) module"]
pub struct OSCHPSTAT_SPEC;
impl crate::RegisterSpec for OSCHPSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oschpstat::R](R) reader structure"]
impl crate::Readable for OSCHPSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OSCHPSTAT to value 0"]
impl crate::Resettable for OSCHPSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
