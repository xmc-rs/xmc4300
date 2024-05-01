#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "PARITY ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Parerr {
    #[doc = "0: No Error"]
    Value1 = 0,
    #[doc = "1: Parity Error in User or Process RAM"]
    Value2 = 1,
}
impl From<Parerr> for bool {
    #[inline(always)]
    fn from(variant: Parerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARERR` reader - PARITY ERROR"]
pub type ParerrR = crate::BitReader<Parerr>;
impl ParerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Parerr {
        match self.bits {
            false => Parerr::Value1,
            true => Parerr::Value2,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Parerr::Value1
    }
    #[doc = "Parity Error in User or Process RAM"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Parerr::Value2
    }
}
impl R {
    #[doc = "Bit 0 - PARITY ERROR"]
    #[inline(always)]
    pub fn parerr(&self) -> ParerrR {
        ParerrR::new((self.bits & 1) != 0)
    }
}
#[doc = "ECAT0 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
