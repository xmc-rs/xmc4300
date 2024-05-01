#[doc = "Register `ASSEL` reader"]
pub type R = crate::R<AsselSpec>;
#[doc = "Register `ASSEL` writer"]
pub type W = crate::W<AsselSpec>;
#[doc = "Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel0 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    Value2 = 1,
}
impl From<Chsel0> for bool {
    #[inline(always)]
    fn from(variant: Chsel0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL0` reader - Channel Selection"]
pub type Chsel0R = crate::BitReader<Chsel0>;
impl Chsel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel0 {
        match self.bits {
            false => Chsel0::Value1,
            true => Chsel0::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chsel0::Value1
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chsel0::Value2
    }
}
#[doc = "Field `CHSEL0` writer - Channel Selection"]
pub type Chsel0W<'a, REG> = crate::BitWriter<'a, REG, Chsel0>;
impl<'a, REG> Chsel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel0::Value1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel0::Value2)
    }
}
#[doc = "Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel1 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    Value2 = 1,
}
impl From<Chsel1> for bool {
    #[inline(always)]
    fn from(variant: Chsel1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL1` reader - Channel Selection"]
pub type Chsel1R = crate::BitReader<Chsel1>;
impl Chsel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel1 {
        match self.bits {
            false => Chsel1::Value1,
            true => Chsel1::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chsel1::Value1
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chsel1::Value2
    }
}
#[doc = "Field `CHSEL1` writer - Channel Selection"]
pub type Chsel1W<'a, REG> = crate::BitWriter<'a, REG, Chsel1>;
impl<'a, REG> Chsel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel1::Value1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel1::Value2)
    }
}
#[doc = "Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel2 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    Value2 = 1,
}
impl From<Chsel2> for bool {
    #[inline(always)]
    fn from(variant: Chsel2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL2` reader - Channel Selection"]
pub type Chsel2R = crate::BitReader<Chsel2>;
impl Chsel2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel2 {
        match self.bits {
            false => Chsel2::Value1,
            true => Chsel2::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chsel2::Value1
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chsel2::Value2
    }
}
#[doc = "Field `CHSEL2` writer - Channel Selection"]
pub type Chsel2W<'a, REG> = crate::BitWriter<'a, REG, Chsel2>;
impl<'a, REG> Chsel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel2::Value1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel2::Value2)
    }
}
#[doc = "Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel3 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    Value2 = 1,
}
impl From<Chsel3> for bool {
    #[inline(always)]
    fn from(variant: Chsel3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL3` reader - Channel Selection"]
pub type Chsel3R = crate::BitReader<Chsel3>;
impl Chsel3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel3 {
        match self.bits {
            false => Chsel3::Value1,
            true => Chsel3::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chsel3::Value1
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chsel3::Value2
    }
}
#[doc = "Field `CHSEL3` writer - Channel Selection"]
pub type Chsel3W<'a, REG> = crate::BitWriter<'a, REG, Chsel3>;
impl<'a, REG> Chsel3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel3::Value1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel3::Value2)
    }
}
#[doc = "Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel4 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    Value2 = 1,
}
impl From<Chsel4> for bool {
    #[inline(always)]
    fn from(variant: Chsel4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL4` reader - Channel Selection"]
pub type Chsel4R = crate::BitReader<Chsel4>;
impl Chsel4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel4 {
        match self.bits {
            false => Chsel4::Value1,
            true => Chsel4::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chsel4::Value1
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chsel4::Value2
    }
}
#[doc = "Field `CHSEL4` writer - Channel Selection"]
pub type Chsel4W<'a, REG> = crate::BitWriter<'a, REG, Chsel4>;
impl<'a, REG> Chsel4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel4::Value1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel4::Value2)
    }
}
#[doc = "Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel5 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    Value2 = 1,
}
impl From<Chsel5> for bool {
    #[inline(always)]
    fn from(variant: Chsel5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL5` reader - Channel Selection"]
pub type Chsel5R = crate::BitReader<Chsel5>;
impl Chsel5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel5 {
        match self.bits {
            false => Chsel5::Value1,
            true => Chsel5::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chsel5::Value1
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chsel5::Value2
    }
}
#[doc = "Field `CHSEL5` writer - Channel Selection"]
pub type Chsel5W<'a, REG> = crate::BitWriter<'a, REG, Chsel5>;
impl<'a, REG> Chsel5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel5::Value1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel5::Value2)
    }
}
#[doc = "Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel6 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    Value2 = 1,
}
impl From<Chsel6> for bool {
    #[inline(always)]
    fn from(variant: Chsel6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL6` reader - Channel Selection"]
pub type Chsel6R = crate::BitReader<Chsel6>;
impl Chsel6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel6 {
        match self.bits {
            false => Chsel6::Value1,
            true => Chsel6::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chsel6::Value1
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chsel6::Value2
    }
}
#[doc = "Field `CHSEL6` writer - Channel Selection"]
pub type Chsel6W<'a, REG> = crate::BitWriter<'a, REG, Chsel6>;
impl<'a, REG> Chsel6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel6::Value1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel6::Value2)
    }
}
#[doc = "Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chsel7 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    Value2 = 1,
}
impl From<Chsel7> for bool {
    #[inline(always)]
    fn from(variant: Chsel7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL7` reader - Channel Selection"]
pub type Chsel7R = crate::BitReader<Chsel7>;
impl Chsel7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chsel7 {
        match self.bits {
            false => Chsel7::Value1,
            true => Chsel7::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chsel7::Value1
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chsel7::Value2
    }
}
#[doc = "Field `CHSEL7` writer - Channel Selection"]
pub type Chsel7W<'a, REG> = crate::BitWriter<'a, REG, Chsel7>;
impl<'a, REG> Chsel7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel7::Value1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chsel7::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Channel Selection"]
    #[inline(always)]
    pub fn chsel0(&self) -> Chsel0R {
        Chsel0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Selection"]
    #[inline(always)]
    pub fn chsel1(&self) -> Chsel1R {
        Chsel1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel Selection"]
    #[inline(always)]
    pub fn chsel2(&self) -> Chsel2R {
        Chsel2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel Selection"]
    #[inline(always)]
    pub fn chsel3(&self) -> Chsel3R {
        Chsel3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel Selection"]
    #[inline(always)]
    pub fn chsel4(&self) -> Chsel4R {
        Chsel4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel Selection"]
    #[inline(always)]
    pub fn chsel5(&self) -> Chsel5R {
        Chsel5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel Selection"]
    #[inline(always)]
    pub fn chsel6(&self) -> Chsel6R {
        Chsel6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel Selection"]
    #[inline(always)]
    pub fn chsel7(&self) -> Chsel7R {
        Chsel7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel0(&mut self) -> Chsel0W<AsselSpec> {
        Chsel0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel1(&mut self) -> Chsel1W<AsselSpec> {
        Chsel1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel2(&mut self) -> Chsel2W<AsselSpec> {
        Chsel2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel3(&mut self) -> Chsel3W<AsselSpec> {
        Chsel3W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel4(&mut self) -> Chsel4W<AsselSpec> {
        Chsel4W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel5(&mut self) -> Chsel5W<AsselSpec> {
        Chsel5W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel6(&mut self) -> Chsel6W<AsselSpec> {
        Chsel6W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel7(&mut self) -> Chsel7W<AsselSpec> {
        Chsel7W::new(self, 7)
    }
}
#[doc = "Autoscan Source Channel Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`assel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`assel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AsselSpec;
impl crate::RegisterSpec for AsselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`assel::R`](R) reader structure"]
impl crate::Readable for AsselSpec {}
#[doc = "`write(|w| ..)` method takes [`assel::W`](W) writer structure"]
impl crate::Writable for AsselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ASSEL to value 0"]
impl crate::Resettable for AsselSpec {
    const RESET_VALUE: u32 = 0;
}
