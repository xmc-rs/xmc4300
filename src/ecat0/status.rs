#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `PARERR` reader - PARITY ERROR"]
pub type PARERR_R = crate::BitReader<PARERR_A>;
#[doc = "PARITY ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PARERR_A {
    #[doc = "0: No Error"]
    VALUE1 = 0,
    #[doc = "1: Parity Error in User or Process RAM"]
    VALUE2 = 1,
}
impl From<PARERR_A> for bool {
    #[inline(always)]
    fn from(variant: PARERR_A) -> Self {
        variant as u8 != 0
    }
}
impl PARERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARERR_A {
        match self.bits {
            false => PARERR_A::VALUE1,
            true => PARERR_A::VALUE2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PARERR_A::VALUE1
    }
    #[doc = "Parity Error in User or Process RAM"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PARERR_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - PARITY ERROR"]
    #[inline(always)]
    pub fn parerr(&self) -> PARERR_R {
        PARERR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "ECAT0 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
