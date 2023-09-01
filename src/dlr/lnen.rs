#[doc = "Register `LNEN` reader"]
pub type R = crate::R<LNEN_SPEC>;
#[doc = "Register `LNEN` writer"]
pub type W = crate::W<LNEN_SPEC>;
#[doc = "Field `LN0` reader - Line 0 Enable"]
pub type LN0_R = crate::BitReader<LN0_A>;
#[doc = "Line 0 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LN0_A {
    #[doc = "0: Disables the line"]
    CONST_0 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    CONST_1 = 1,
}
impl From<LN0_A> for bool {
    #[inline(always)]
    fn from(variant: LN0_A) -> Self {
        variant as u8 != 0
    }
}
impl LN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LN0_A {
        match self.bits {
            false => LN0_A::CONST_0,
            true => LN0_A::CONST_1,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == LN0_A::CONST_0
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == LN0_A::CONST_1
    }
}
#[doc = "Field `LN0` writer - Line 0 Enable"]
pub type LN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LN0_A>;
impl<'a, REG, const O: u8> LN0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(LN0_A::CONST_0)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(LN0_A::CONST_1)
    }
}
#[doc = "Field `LN1` reader - Line 1 Enable"]
pub type LN1_R = crate::BitReader<LN1_A>;
#[doc = "Line 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LN1_A {
    #[doc = "0: Disables the line"]
    CONST_0 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    CONST_1 = 1,
}
impl From<LN1_A> for bool {
    #[inline(always)]
    fn from(variant: LN1_A) -> Self {
        variant as u8 != 0
    }
}
impl LN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LN1_A {
        match self.bits {
            false => LN1_A::CONST_0,
            true => LN1_A::CONST_1,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == LN1_A::CONST_0
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == LN1_A::CONST_1
    }
}
#[doc = "Field `LN1` writer - Line 1 Enable"]
pub type LN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LN1_A>;
impl<'a, REG, const O: u8> LN1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(LN1_A::CONST_0)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(LN1_A::CONST_1)
    }
}
#[doc = "Field `LN2` reader - Line 2 Enable"]
pub type LN2_R = crate::BitReader<LN2_A>;
#[doc = "Line 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LN2_A {
    #[doc = "0: Disables the line"]
    CONST_0 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    CONST_1 = 1,
}
impl From<LN2_A> for bool {
    #[inline(always)]
    fn from(variant: LN2_A) -> Self {
        variant as u8 != 0
    }
}
impl LN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LN2_A {
        match self.bits {
            false => LN2_A::CONST_0,
            true => LN2_A::CONST_1,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == LN2_A::CONST_0
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == LN2_A::CONST_1
    }
}
#[doc = "Field `LN2` writer - Line 2 Enable"]
pub type LN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LN2_A>;
impl<'a, REG, const O: u8> LN2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(LN2_A::CONST_0)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(LN2_A::CONST_1)
    }
}
#[doc = "Field `LN3` reader - Line 3 Enable"]
pub type LN3_R = crate::BitReader<LN3_A>;
#[doc = "Line 3 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LN3_A {
    #[doc = "0: Disables the line"]
    CONST_0 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    CONST_1 = 1,
}
impl From<LN3_A> for bool {
    #[inline(always)]
    fn from(variant: LN3_A) -> Self {
        variant as u8 != 0
    }
}
impl LN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LN3_A {
        match self.bits {
            false => LN3_A::CONST_0,
            true => LN3_A::CONST_1,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == LN3_A::CONST_0
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == LN3_A::CONST_1
    }
}
#[doc = "Field `LN3` writer - Line 3 Enable"]
pub type LN3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LN3_A>;
impl<'a, REG, const O: u8> LN3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(LN3_A::CONST_0)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(LN3_A::CONST_1)
    }
}
#[doc = "Field `LN4` reader - Line 4 Enable"]
pub type LN4_R = crate::BitReader<LN4_A>;
#[doc = "Line 4 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LN4_A {
    #[doc = "0: Disables the line"]
    CONST_0 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    CONST_1 = 1,
}
impl From<LN4_A> for bool {
    #[inline(always)]
    fn from(variant: LN4_A) -> Self {
        variant as u8 != 0
    }
}
impl LN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LN4_A {
        match self.bits {
            false => LN4_A::CONST_0,
            true => LN4_A::CONST_1,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == LN4_A::CONST_0
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == LN4_A::CONST_1
    }
}
#[doc = "Field `LN4` writer - Line 4 Enable"]
pub type LN4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LN4_A>;
impl<'a, REG, const O: u8> LN4_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(LN4_A::CONST_0)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(LN4_A::CONST_1)
    }
}
#[doc = "Field `LN5` reader - Line 5 Enable"]
pub type LN5_R = crate::BitReader<LN5_A>;
#[doc = "Line 5 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LN5_A {
    #[doc = "0: Disables the line"]
    CONST_0 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    CONST_1 = 1,
}
impl From<LN5_A> for bool {
    #[inline(always)]
    fn from(variant: LN5_A) -> Self {
        variant as u8 != 0
    }
}
impl LN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LN5_A {
        match self.bits {
            false => LN5_A::CONST_0,
            true => LN5_A::CONST_1,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == LN5_A::CONST_0
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == LN5_A::CONST_1
    }
}
#[doc = "Field `LN5` writer - Line 5 Enable"]
pub type LN5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LN5_A>;
impl<'a, REG, const O: u8> LN5_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(LN5_A::CONST_0)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(LN5_A::CONST_1)
    }
}
#[doc = "Field `LN6` reader - Line 6 Enable"]
pub type LN6_R = crate::BitReader<LN6_A>;
#[doc = "Line 6 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LN6_A {
    #[doc = "0: Disables the line"]
    CONST_0 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    CONST_1 = 1,
}
impl From<LN6_A> for bool {
    #[inline(always)]
    fn from(variant: LN6_A) -> Self {
        variant as u8 != 0
    }
}
impl LN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LN6_A {
        match self.bits {
            false => LN6_A::CONST_0,
            true => LN6_A::CONST_1,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == LN6_A::CONST_0
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == LN6_A::CONST_1
    }
}
#[doc = "Field `LN6` writer - Line 6 Enable"]
pub type LN6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LN6_A>;
impl<'a, REG, const O: u8> LN6_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(LN6_A::CONST_0)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(LN6_A::CONST_1)
    }
}
#[doc = "Field `LN7` reader - Line 7 Enable"]
pub type LN7_R = crate::BitReader<LN7_A>;
#[doc = "Line 7 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LN7_A {
    #[doc = "0: Disables the line"]
    CONST_0 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    CONST_1 = 1,
}
impl From<LN7_A> for bool {
    #[inline(always)]
    fn from(variant: LN7_A) -> Self {
        variant as u8 != 0
    }
}
impl LN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LN7_A {
        match self.bits {
            false => LN7_A::CONST_0,
            true => LN7_A::CONST_1,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == LN7_A::CONST_0
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == LN7_A::CONST_1
    }
}
#[doc = "Field `LN7` writer - Line 7 Enable"]
pub type LN7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LN7_A>;
impl<'a, REG, const O: u8> LN7_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(LN7_A::CONST_0)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(LN7_A::CONST_1)
    }
}
impl R {
    #[doc = "Bit 0 - Line 0 Enable"]
    #[inline(always)]
    pub fn ln0(&self) -> LN0_R {
        LN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Line 1 Enable"]
    #[inline(always)]
    pub fn ln1(&self) -> LN1_R {
        LN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Line 2 Enable"]
    #[inline(always)]
    pub fn ln2(&self) -> LN2_R {
        LN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Line 3 Enable"]
    #[inline(always)]
    pub fn ln3(&self) -> LN3_R {
        LN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line 4 Enable"]
    #[inline(always)]
    pub fn ln4(&self) -> LN4_R {
        LN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Line 5 Enable"]
    #[inline(always)]
    pub fn ln5(&self) -> LN5_R {
        LN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Line 6 Enable"]
    #[inline(always)]
    pub fn ln6(&self) -> LN6_R {
        LN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Line 7 Enable"]
    #[inline(always)]
    pub fn ln7(&self) -> LN7_R {
        LN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Line 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln0(&mut self) -> LN0_W<LNEN_SPEC, 0> {
        LN0_W::new(self)
    }
    #[doc = "Bit 1 - Line 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln1(&mut self) -> LN1_W<LNEN_SPEC, 1> {
        LN1_W::new(self)
    }
    #[doc = "Bit 2 - Line 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln2(&mut self) -> LN2_W<LNEN_SPEC, 2> {
        LN2_W::new(self)
    }
    #[doc = "Bit 3 - Line 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln3(&mut self) -> LN3_W<LNEN_SPEC, 3> {
        LN3_W::new(self)
    }
    #[doc = "Bit 4 - Line 4 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln4(&mut self) -> LN4_W<LNEN_SPEC, 4> {
        LN4_W::new(self)
    }
    #[doc = "Bit 5 - Line 5 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln5(&mut self) -> LN5_W<LNEN_SPEC, 5> {
        LN5_W::new(self)
    }
    #[doc = "Bit 6 - Line 6 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln6(&mut self) -> LN6_W<LNEN_SPEC, 6> {
        LN6_W::new(self)
    }
    #[doc = "Bit 7 - Line 7 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln7(&mut self) -> LN7_W<LNEN_SPEC, 7> {
        LN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Line Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lnen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lnen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LNEN_SPEC;
impl crate::RegisterSpec for LNEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lnen::R`](R) reader structure"]
impl crate::Readable for LNEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lnen::W`](W) writer structure"]
impl crate::Writable for LNEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LNEN to value 0"]
impl crate::Resettable for LNEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
