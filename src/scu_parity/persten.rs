#[doc = "Register `PERSTEN` reader"]
pub type R = crate::R<PERSTEN_SPEC>;
#[doc = "Register `PERSTEN` writer"]
pub type W = crate::W<PERSTEN_SPEC>;
#[doc = "System Reset Enable upon Parity Error Trap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSEN_A {
    #[doc = "0: Reset request disabled"]
    CONST_0 = 0,
    #[doc = "1: Reset request enabled"]
    CONST_1 = 1,
}
impl From<RSEN_A> for bool {
    #[inline(always)]
    fn from(variant: RSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSEN` reader - System Reset Enable upon Parity Error Trap"]
pub type RSEN_R = crate::BitReader<RSEN_A>;
impl RSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSEN_A {
        match self.bits {
            false => RSEN_A::CONST_0,
            true => RSEN_A::CONST_1,
        }
    }
    #[doc = "Reset request disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RSEN_A::CONST_0
    }
    #[doc = "Reset request enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RSEN_A::CONST_1
    }
}
#[doc = "Field `RSEN` writer - System Reset Enable upon Parity Error Trap"]
pub type RSEN_W<'a, REG> = crate::BitWriter<'a, REG, RSEN_A>;
impl<'a, REG> RSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset request disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(RSEN_A::CONST_0)
    }
    #[doc = "Reset request enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(RSEN_A::CONST_1)
    }
}
impl R {
    #[doc = "Bit 0 - System Reset Enable upon Parity Error Trap"]
    #[inline(always)]
    pub fn rsen(&self) -> RSEN_R {
        RSEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System Reset Enable upon Parity Error Trap"]
    #[inline(always)]
    pub fn rsen(&mut self) -> RSEN_W<PERSTEN_SPEC> {
        RSEN_W::new(self, 0)
    }
}
#[doc = "Parity Error Reset Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`persten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`persten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERSTEN_SPEC;
impl crate::RegisterSpec for PERSTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`persten::R`](R) reader structure"]
impl crate::Readable for PERSTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`persten::W`](W) writer structure"]
impl crate::Writable for PERSTEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERSTEN to value 0"]
impl crate::Resettable for PERSTEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
