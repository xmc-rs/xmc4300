#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<STS_SPEC>;
#[doc = "Bufferable Write Access Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WERR_A {
    #[doc = "0: no write error occurred."]
    VALUE1 = 0,
    #[doc = "1: write error occurred, interrupt request is pending."]
    VALUE2 = 1,
}
impl From<WERR_A> for bool {
    #[inline(always)]
    fn from(variant: WERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WERR` reader - Bufferable Write Access Error"]
pub type WERR_R = crate::BitReader<WERR_A>;
impl WERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WERR_A {
        match self.bits {
            false => WERR_A::VALUE1,
            true => WERR_A::VALUE2,
        }
    }
    #[doc = "no write error occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WERR_A::VALUE1
    }
    #[doc = "write error occurred, interrupt request is pending."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WERR_A::VALUE2
    }
}
#[doc = "Field `WERR` writer - Bufferable Write Access Error"]
pub type WERR_W<'a, REG> = crate::BitWriter<'a, REG, WERR_A>;
impl<'a, REG> WERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no write error occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WERR_A::VALUE1)
    }
    #[doc = "write error occurred, interrupt request is pending."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WERR_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Bufferable Write Access Error"]
    #[inline(always)]
    pub fn werr(&self) -> WERR_R {
        WERR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bufferable Write Access Error"]
    #[inline(always)]
    #[must_use]
    pub fn werr(&mut self) -> WERR_W<STS_SPEC> {
        WERR_W::new(self, 0)
    }
}
#[doc = "Peripheral Bridge Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for STS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: u32 = 0;
}
