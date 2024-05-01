#[doc = "Register `PERSTEN` reader"]
pub type R = crate::R<PerstenSpec>;
#[doc = "Register `PERSTEN` writer"]
pub type W = crate::W<PerstenSpec>;
#[doc = "System Reset Enable upon Parity Error Trap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rsen {
    #[doc = "0: Reset request disabled"]
    Const0 = 0,
    #[doc = "1: Reset request enabled"]
    Const1 = 1,
}
impl From<Rsen> for bool {
    #[inline(always)]
    fn from(variant: Rsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSEN` reader - System Reset Enable upon Parity Error Trap"]
pub type RsenR = crate::BitReader<Rsen>;
impl RsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rsen {
        match self.bits {
            false => Rsen::Const0,
            true => Rsen::Const1,
        }
    }
    #[doc = "Reset request disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Rsen::Const0
    }
    #[doc = "Reset request enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Rsen::Const1
    }
}
#[doc = "Field `RSEN` writer - System Reset Enable upon Parity Error Trap"]
pub type RsenW<'a, REG> = crate::BitWriter<'a, REG, Rsen>;
impl<'a, REG> RsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset request disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rsen::Const0)
    }
    #[doc = "Reset request enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rsen::Const1)
    }
}
impl R {
    #[doc = "Bit 0 - System Reset Enable upon Parity Error Trap"]
    #[inline(always)]
    pub fn rsen(&self) -> RsenR {
        RsenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System Reset Enable upon Parity Error Trap"]
    #[inline(always)]
    #[must_use]
    pub fn rsen(&mut self) -> RsenW<PerstenSpec> {
        RsenW::new(self, 0)
    }
}
#[doc = "Parity Error Reset Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`persten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`persten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerstenSpec;
impl crate::RegisterSpec for PerstenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`persten::R`](R) reader structure"]
impl crate::Readable for PerstenSpec {}
#[doc = "`write(|w| ..)` method takes [`persten::W`](W) writer structure"]
impl crate::Writable for PerstenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERSTEN to value 0"]
impl crate::Resettable for PerstenSpec {
    const RESET_VALUE: u32 = 0;
}
