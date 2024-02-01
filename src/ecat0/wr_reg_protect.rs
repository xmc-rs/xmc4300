#[doc = "Register `WR_REG_PROTECT` reader"]
pub type R = crate::R<WR_REG_PROTECT_SPEC>;
#[doc = "Field `WR_REG_P` reader - Write register protection"]
pub type WR_REG_P_R = crate::BitReader<WR_REG_P_A>;
#[doc = "Write register protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl WR_REG_P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WR_REG_P_A {
        match self.bits {
            false => WR_REG_P_A::VALUE1,
            true => WR_REG_P_A::VALUE2,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WR_REG_P_A::VALUE1
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WR_REG_P_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Write register protection"]
    #[inline(always)]
    pub fn wr_reg_p(&self) -> WR_REG_P_R {
        WR_REG_P_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Write Register Protection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_reg_protect::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WR_REG_PROTECT_SPEC;
impl crate::RegisterSpec for WR_REG_PROTECT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wr_reg_protect::R`](R) reader structure"]
impl crate::Readable for WR_REG_PROTECT_SPEC {}
#[doc = "`reset()` method sets WR_REG_PROTECT to value 0"]
impl crate::Resettable for WR_REG_PROTECT_SPEC {
    const RESET_VALUE: u8 = 0;
}
