#[doc = "Register `WR_REG_PROTECT` reader"]
pub type R = crate::R<WrRegProtectSpec>;
#[doc = "Write register protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WrRegP {
    #[doc = "0: Protection disabled"]
    Value1 = 0,
    #[doc = "1: Protection enabled"]
    Value2 = 1,
}
impl From<WrRegP> for bool {
    #[inline(always)]
    fn from(variant: WrRegP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WR_REG_P` reader - Write register protection"]
pub type WrRegPR = crate::BitReader<WrRegP>;
impl WrRegPR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WrRegP {
        match self.bits {
            false => WrRegP::Value1,
            true => WrRegP::Value2,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WrRegP::Value1
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WrRegP::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Write register protection"]
    #[inline(always)]
    pub fn wr_reg_p(&self) -> WrRegPR {
        WrRegPR::new((self.bits & 1) != 0)
    }
}
#[doc = "Write Register Protection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_reg_protect::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrRegProtectSpec;
impl crate::RegisterSpec for WrRegProtectSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wr_reg_protect::R`](R) reader structure"]
impl crate::Readable for WrRegProtectSpec {}
#[doc = "`reset()` method sets WR_REG_PROTECT to value 0"]
impl crate::Resettable for WrRegProtectSpec {
    const RESET_VALUE: u8 = 0;
}
