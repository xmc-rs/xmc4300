#[doc = "Register `ESC_WR_PROTECT` reader"]
pub struct R(crate::R<ESC_WR_PROTECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESC_WR_PROTECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESC_WR_PROTECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESC_WR_PROTECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ESC_WR_PROT` reader - Write protect"]
pub type ESC_WR_PROT_R = crate::BitReader<ESC_WR_PROT_A>;
#[doc = "Write protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESC_WR_PROT_A {
    #[doc = "0: Protection disabled"]
    VALUE1 = 0,
    #[doc = "1: Protection enabled"]
    VALUE2 = 1,
}
impl From<ESC_WR_PROT_A> for bool {
    #[inline(always)]
    fn from(variant: ESC_WR_PROT_A) -> Self {
        variant as u8 != 0
    }
}
impl ESC_WR_PROT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESC_WR_PROT_A {
        match self.bits {
            false => ESC_WR_PROT_A::VALUE1,
            true => ESC_WR_PROT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ESC_WR_PROT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ESC_WR_PROT_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Write protect"]
    #[inline(always)]
    pub fn esc_wr_prot(&self) -> ESC_WR_PROT_R {
        ESC_WR_PROT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "ESC Write Protection\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esc_wr_protect](index.html) module"]
pub struct ESC_WR_PROTECT_SPEC;
impl crate::RegisterSpec for ESC_WR_PROTECT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [esc_wr_protect::R](R) reader structure"]
impl crate::Readable for ESC_WR_PROTECT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ESC_WR_PROTECT to value 0"]
impl crate::Resettable for ESC_WR_PROTECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
