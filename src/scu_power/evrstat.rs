#[doc = "Register `EVRSTAT` reader"]
pub type R = crate::R<EVRSTAT_SPEC>;
#[doc = "Regulator Overvoltage for 1.3 V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OV13_A {
    #[doc = "0: No overvoltage condition"]
    CONST_0 = 0,
    #[doc = "1: Regulator is in overvoltage"]
    CONST_1 = 1,
}
impl From<OV13_A> for bool {
    #[inline(always)]
    fn from(variant: OV13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OV13` reader - Regulator Overvoltage for 1.3 V"]
pub type OV13_R = crate::BitReader<OV13_A>;
impl OV13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OV13_A {
        match self.bits {
            false => OV13_A::CONST_0,
            true => OV13_A::CONST_1,
        }
    }
    #[doc = "No overvoltage condition"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == OV13_A::CONST_0
    }
    #[doc = "Regulator is in overvoltage"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == OV13_A::CONST_1
    }
}
impl R {
    #[doc = "Bit 1 - Regulator Overvoltage for 1.3 V"]
    #[inline(always)]
    pub fn ov13(&self) -> OV13_R {
        OV13_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "EVR Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evrstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVRSTAT_SPEC;
impl crate::RegisterSpec for EVRSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evrstat::R`](R) reader structure"]
impl crate::Readable for EVRSTAT_SPEC {}
#[doc = "`reset()` method sets EVRSTAT to value 0"]
impl crate::Resettable for EVRSTAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
