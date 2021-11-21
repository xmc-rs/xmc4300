#[doc = "Register `WR_REG_PROTECT` reader"]
pub struct R(crate::R<WR_REG_PROTECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WR_REG_PROTECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WR_REG_PROTECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WR_REG_PROTECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Write register protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WR_REG_P_A {
    #[doc = "0: Protection disabled"]
    VALUE1 = 0,
    #[doc = "1: Protection enabled"]
    VALUE2 = 1,
}
impl From<WR_REG_P_A> for bool {
    #[inline(always)]
    fn from(variant: WR_REG_P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WR_REG_P` reader - Write register protection"]
pub struct WR_REG_P_R(crate::FieldReader<bool, WR_REG_P_A>);
impl WR_REG_P_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_REG_P_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WR_REG_P_A {
        match self.bits {
            false => WR_REG_P_A::VALUE1,
            true => WR_REG_P_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == WR_REG_P_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == WR_REG_P_A::VALUE2
    }
}
impl core::ops::Deref for WR_REG_P_R {
    type Target = crate::FieldReader<bool, WR_REG_P_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Write register protection"]
    #[inline(always)]
    pub fn wr_reg_p(&self) -> WR_REG_P_R {
        WR_REG_P_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Write Register Protection\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_reg_protect](index.html) module"]
pub struct WR_REG_PROTECT_SPEC;
impl crate::RegisterSpec for WR_REG_PROTECT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [wr_reg_protect::R](R) reader structure"]
impl crate::Readable for WR_REG_PROTECT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WR_REG_PROTECT to value 0"]
impl crate::Resettable for WR_REG_PROTECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
