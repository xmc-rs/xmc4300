#[doc = "Register `SDMMCDEL` reader"]
pub type R = crate::R<SDMMCDEL_SPEC>;
#[doc = "Register `SDMMCDEL` writer"]
pub type W = crate::W<SDMMCDEL_SPEC>;
#[doc = "Field `TAPEN` reader - Enable delay on the CMD/DAT out lines"]
pub type TAPEN_R = crate::BitReader<TAPEN_A>;
#[doc = "Enable delay on the CMD/DAT out lines\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAPEN_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<TAPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TAPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TAPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAPEN_A {
        match self.bits {
            false => TAPEN_A::VALUE1,
            true => TAPEN_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TAPEN_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TAPEN_A::VALUE2
    }
}
#[doc = "Field `TAPEN` writer - Enable delay on the CMD/DAT out lines"]
pub type TAPEN_W<'a, REG> = crate::BitWriter<'a, REG, TAPEN_A>;
impl<'a, REG> TAPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TAPEN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TAPEN_A::VALUE2)
    }
}
#[doc = "Field `TAPDEL` reader - Number of Delay Elements Select"]
pub type TAPDEL_R = crate::FieldReader;
#[doc = "Field `TAPDEL` writer - Number of Delay Elements Select"]
pub type TAPDEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Enable delay on the CMD/DAT out lines"]
    #[inline(always)]
    pub fn tapen(&self) -> TAPEN_R {
        TAPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Number of Delay Elements Select"]
    #[inline(always)]
    pub fn tapdel(&self) -> TAPDEL_R {
        TAPDEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable delay on the CMD/DAT out lines"]
    #[inline(always)]
    #[must_use]
    pub fn tapen(&mut self) -> TAPEN_W<SDMMCDEL_SPEC> {
        TAPEN_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Number of Delay Elements Select"]
    #[inline(always)]
    #[must_use]
    pub fn tapdel(&mut self) -> TAPDEL_W<SDMMCDEL_SPEC> {
        TAPDEL_W::new(self, 4)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SD-MMC Delay Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmcdel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmcdel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMCDEL_SPEC;
impl crate::RegisterSpec for SDMMCDEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmcdel::R`](R) reader structure"]
impl crate::Readable for SDMMCDEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdmmcdel::W`](W) writer structure"]
impl crate::Writable for SDMMCDEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMCDEL to value 0"]
impl crate::Resettable for SDMMCDEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
