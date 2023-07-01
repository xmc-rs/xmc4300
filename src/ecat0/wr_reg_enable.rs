#[doc = "Register `WR_REG_ENABLE` reader"]
pub struct R(crate::R<WR_REG_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WR_REG_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WR_REG_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WR_REG_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WR_REG_EN` reader - Write register protection enabled"]
pub type WR_REG_EN_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Write register protection enabled"]
    #[inline(always)]
    pub fn wr_reg_en(&self) -> WR_REG_EN_R {
        WR_REG_EN_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Write Register Enable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_reg_enable](index.html) module"]
pub struct WR_REG_ENABLE_SPEC;
impl crate::RegisterSpec for WR_REG_ENABLE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [wr_reg_enable::R](R) reader structure"]
impl crate::Readable for WR_REG_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WR_REG_ENABLE to value 0"]
impl crate::Resettable for WR_REG_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
