#[doc = "Register `EVRSTAT` reader"]
pub type R = crate::R<EvrstatSpec>;
#[doc = "Regulator Overvoltage for 1.3 V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ov13 {
    #[doc = "0: No overvoltage condition"]
    Const0 = 0,
    #[doc = "1: Regulator is in overvoltage"]
    Const1 = 1,
}
impl From<Ov13> for bool {
    #[inline(always)]
    fn from(variant: Ov13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OV13` reader - Regulator Overvoltage for 1.3 V"]
pub type Ov13R = crate::BitReader<Ov13>;
impl Ov13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ov13 {
        match self.bits {
            false => Ov13::Const0,
            true => Ov13::Const1,
        }
    }
    #[doc = "No overvoltage condition"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ov13::Const0
    }
    #[doc = "Regulator is in overvoltage"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ov13::Const1
    }
}
impl R {
    #[doc = "Bit 1 - Regulator Overvoltage for 1.3 V"]
    #[inline(always)]
    pub fn ov13(&self) -> Ov13R {
        Ov13R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "EVR Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evrstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvrstatSpec;
impl crate::RegisterSpec for EvrstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evrstat::R`](R) reader structure"]
impl crate::Readable for EvrstatSpec {}
#[doc = "`reset()` method sets EVRSTAT to value 0"]
impl crate::Resettable for EvrstatSpec {
    const RESET_VALUE: u32 = 0;
}
