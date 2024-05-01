#[doc = "Register `G1ORCEN` reader"]
pub type R = crate::R<G1ORCEN_SPEC>;
#[doc = "Register `G1ORCEN` writer"]
pub type W = crate::W<G1ORCEN_SPEC>;
#[doc = "Enable Out of Range Comparator, Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENORC6_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<ENORC6_A> for bool {
    #[inline(always)]
    fn from(variant: ENORC6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENORC6` reader - Enable Out of Range Comparator, Channel 6"]
pub type ENORC6_R = crate::BitReader<ENORC6_A>;
impl ENORC6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENORC6_A {
        match self.bits {
            false => ENORC6_A::CONST_0,
            true => ENORC6_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ENORC6_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ENORC6_A::CONST_1
    }
}
#[doc = "Field `ENORC6` writer - Enable Out of Range Comparator, Channel 6"]
pub type ENORC6_W<'a, REG> = crate::BitWriter<'a, REG, ENORC6_A>;
impl<'a, REG> ENORC6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(ENORC6_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(ENORC6_A::CONST_1)
    }
}
#[doc = "Enable Out of Range Comparator, Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENORC7_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<ENORC7_A> for bool {
    #[inline(always)]
    fn from(variant: ENORC7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENORC7` reader - Enable Out of Range Comparator, Channel 7"]
pub type ENORC7_R = crate::BitReader<ENORC7_A>;
impl ENORC7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENORC7_A {
        match self.bits {
            false => ENORC7_A::CONST_0,
            true => ENORC7_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ENORC7_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ENORC7_A::CONST_1
    }
}
#[doc = "Field `ENORC7` writer - Enable Out of Range Comparator, Channel 7"]
pub type ENORC7_W<'a, REG> = crate::BitWriter<'a, REG, ENORC7_A>;
impl<'a, REG> ENORC7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(ENORC7_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(ENORC7_A::CONST_1)
    }
}
impl R {
    #[doc = "Bit 6 - Enable Out of Range Comparator, Channel 6"]
    #[inline(always)]
    pub fn enorc6(&self) -> ENORC6_R {
        ENORC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Out of Range Comparator, Channel 7"]
    #[inline(always)]
    pub fn enorc7(&self) -> ENORC7_R {
        ENORC7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Enable Out of Range Comparator, Channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn enorc6(&mut self) -> ENORC6_W<G1ORCEN_SPEC> {
        ENORC6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Out of Range Comparator, Channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn enorc7(&mut self) -> ENORC7_W<G1ORCEN_SPEC> {
        ENORC7_W::new(self, 7)
    }
}
#[doc = "Out of Range Comparator Enable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g1orcen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`g1orcen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct G1ORCEN_SPEC;
impl crate::RegisterSpec for G1ORCEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`g1orcen::R`](R) reader structure"]
impl crate::Readable for G1ORCEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`g1orcen::W`](W) writer structure"]
impl crate::Writable for G1ORCEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets G1ORCEN to value 0"]
impl crate::Resettable for G1ORCEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
