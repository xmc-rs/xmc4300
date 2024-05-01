#[doc = "Register `LNEN` reader"]
pub type R = crate::R<LnenSpec>;
#[doc = "Register `LNEN` writer"]
pub type W = crate::W<LnenSpec>;
#[doc = "Line 0 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ln0 {
    #[doc = "0: Disables the line"]
    Const0 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    Const1 = 1,
}
impl From<Ln0> for bool {
    #[inline(always)]
    fn from(variant: Ln0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LN0` reader - Line 0 Enable"]
pub type Ln0R = crate::BitReader<Ln0>;
impl Ln0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ln0 {
        match self.bits {
            false => Ln0::Const0,
            true => Ln0::Const1,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ln0::Const0
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ln0::Const1
    }
}
#[doc = "Field `LN0` writer - Line 0 Enable"]
pub type Ln0W<'a, REG> = crate::BitWriter<'a, REG, Ln0>;
impl<'a, REG> Ln0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ln0::Const0)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ln0::Const1)
    }
}
#[doc = "Line 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ln1 {
    #[doc = "0: Disables the line"]
    Const0 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    Const1 = 1,
}
impl From<Ln1> for bool {
    #[inline(always)]
    fn from(variant: Ln1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LN1` reader - Line 1 Enable"]
pub type Ln1R = crate::BitReader<Ln1>;
impl Ln1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ln1 {
        match self.bits {
            false => Ln1::Const0,
            true => Ln1::Const1,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ln1::Const0
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ln1::Const1
    }
}
#[doc = "Field `LN1` writer - Line 1 Enable"]
pub type Ln1W<'a, REG> = crate::BitWriter<'a, REG, Ln1>;
impl<'a, REG> Ln1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ln1::Const0)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ln1::Const1)
    }
}
#[doc = "Line 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ln2 {
    #[doc = "0: Disables the line"]
    Const0 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    Const1 = 1,
}
impl From<Ln2> for bool {
    #[inline(always)]
    fn from(variant: Ln2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LN2` reader - Line 2 Enable"]
pub type Ln2R = crate::BitReader<Ln2>;
impl Ln2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ln2 {
        match self.bits {
            false => Ln2::Const0,
            true => Ln2::Const1,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ln2::Const0
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ln2::Const1
    }
}
#[doc = "Field `LN2` writer - Line 2 Enable"]
pub type Ln2W<'a, REG> = crate::BitWriter<'a, REG, Ln2>;
impl<'a, REG> Ln2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ln2::Const0)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ln2::Const1)
    }
}
#[doc = "Line 3 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ln3 {
    #[doc = "0: Disables the line"]
    Const0 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    Const1 = 1,
}
impl From<Ln3> for bool {
    #[inline(always)]
    fn from(variant: Ln3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LN3` reader - Line 3 Enable"]
pub type Ln3R = crate::BitReader<Ln3>;
impl Ln3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ln3 {
        match self.bits {
            false => Ln3::Const0,
            true => Ln3::Const1,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ln3::Const0
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ln3::Const1
    }
}
#[doc = "Field `LN3` writer - Line 3 Enable"]
pub type Ln3W<'a, REG> = crate::BitWriter<'a, REG, Ln3>;
impl<'a, REG> Ln3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ln3::Const0)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ln3::Const1)
    }
}
#[doc = "Line 4 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ln4 {
    #[doc = "0: Disables the line"]
    Const0 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    Const1 = 1,
}
impl From<Ln4> for bool {
    #[inline(always)]
    fn from(variant: Ln4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LN4` reader - Line 4 Enable"]
pub type Ln4R = crate::BitReader<Ln4>;
impl Ln4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ln4 {
        match self.bits {
            false => Ln4::Const0,
            true => Ln4::Const1,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ln4::Const0
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ln4::Const1
    }
}
#[doc = "Field `LN4` writer - Line 4 Enable"]
pub type Ln4W<'a, REG> = crate::BitWriter<'a, REG, Ln4>;
impl<'a, REG> Ln4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ln4::Const0)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ln4::Const1)
    }
}
#[doc = "Line 5 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ln5 {
    #[doc = "0: Disables the line"]
    Const0 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    Const1 = 1,
}
impl From<Ln5> for bool {
    #[inline(always)]
    fn from(variant: Ln5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LN5` reader - Line 5 Enable"]
pub type Ln5R = crate::BitReader<Ln5>;
impl Ln5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ln5 {
        match self.bits {
            false => Ln5::Const0,
            true => Ln5::Const1,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ln5::Const0
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ln5::Const1
    }
}
#[doc = "Field `LN5` writer - Line 5 Enable"]
pub type Ln5W<'a, REG> = crate::BitWriter<'a, REG, Ln5>;
impl<'a, REG> Ln5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ln5::Const0)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ln5::Const1)
    }
}
#[doc = "Line 6 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ln6 {
    #[doc = "0: Disables the line"]
    Const0 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    Const1 = 1,
}
impl From<Ln6> for bool {
    #[inline(always)]
    fn from(variant: Ln6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LN6` reader - Line 6 Enable"]
pub type Ln6R = crate::BitReader<Ln6>;
impl Ln6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ln6 {
        match self.bits {
            false => Ln6::Const0,
            true => Ln6::Const1,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ln6::Const0
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ln6::Const1
    }
}
#[doc = "Field `LN6` writer - Line 6 Enable"]
pub type Ln6W<'a, REG> = crate::BitWriter<'a, REG, Ln6>;
impl<'a, REG> Ln6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ln6::Const0)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ln6::Const1)
    }
}
#[doc = "Line 7 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ln7 {
    #[doc = "0: Disables the line"]
    Const0 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    Const1 = 1,
}
impl From<Ln7> for bool {
    #[inline(always)]
    fn from(variant: Ln7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LN7` reader - Line 7 Enable"]
pub type Ln7R = crate::BitReader<Ln7>;
impl Ln7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ln7 {
        match self.bits {
            false => Ln7::Const0,
            true => Ln7::Const1,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ln7::Const0
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ln7::Const1
    }
}
#[doc = "Field `LN7` writer - Line 7 Enable"]
pub type Ln7W<'a, REG> = crate::BitWriter<'a, REG, Ln7>;
impl<'a, REG> Ln7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ln7::Const0)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ln7::Const1)
    }
}
impl R {
    #[doc = "Bit 0 - Line 0 Enable"]
    #[inline(always)]
    pub fn ln0(&self) -> Ln0R {
        Ln0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Line 1 Enable"]
    #[inline(always)]
    pub fn ln1(&self) -> Ln1R {
        Ln1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Line 2 Enable"]
    #[inline(always)]
    pub fn ln2(&self) -> Ln2R {
        Ln2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Line 3 Enable"]
    #[inline(always)]
    pub fn ln3(&self) -> Ln3R {
        Ln3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line 4 Enable"]
    #[inline(always)]
    pub fn ln4(&self) -> Ln4R {
        Ln4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Line 5 Enable"]
    #[inline(always)]
    pub fn ln5(&self) -> Ln5R {
        Ln5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Line 6 Enable"]
    #[inline(always)]
    pub fn ln6(&self) -> Ln6R {
        Ln6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Line 7 Enable"]
    #[inline(always)]
    pub fn ln7(&self) -> Ln7R {
        Ln7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Line 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln0(&mut self) -> Ln0W<LnenSpec> {
        Ln0W::new(self, 0)
    }
    #[doc = "Bit 1 - Line 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln1(&mut self) -> Ln1W<LnenSpec> {
        Ln1W::new(self, 1)
    }
    #[doc = "Bit 2 - Line 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln2(&mut self) -> Ln2W<LnenSpec> {
        Ln2W::new(self, 2)
    }
    #[doc = "Bit 3 - Line 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln3(&mut self) -> Ln3W<LnenSpec> {
        Ln3W::new(self, 3)
    }
    #[doc = "Bit 4 - Line 4 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln4(&mut self) -> Ln4W<LnenSpec> {
        Ln4W::new(self, 4)
    }
    #[doc = "Bit 5 - Line 5 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln5(&mut self) -> Ln5W<LnenSpec> {
        Ln5W::new(self, 5)
    }
    #[doc = "Bit 6 - Line 6 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln6(&mut self) -> Ln6W<LnenSpec> {
        Ln6W::new(self, 6)
    }
    #[doc = "Bit 7 - Line 7 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln7(&mut self) -> Ln7W<LnenSpec> {
        Ln7W::new(self, 7)
    }
}
#[doc = "Line Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lnen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lnen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LnenSpec;
impl crate::RegisterSpec for LnenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lnen::R`](R) reader structure"]
impl crate::Readable for LnenSpec {}
#[doc = "`write(|w| ..)` method takes [`lnen::W`](W) writer structure"]
impl crate::Writable for LnenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LNEN to value 0"]
impl crate::Resettable for LnenSpec {
    const RESET_VALUE: u32 = 0;
}
