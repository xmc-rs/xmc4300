#[doc = "Register `BRSSEL[%s]` reader"]
pub type R = crate::R<BrsselSpec>;
#[doc = "Register `BRSSEL[%s]` writer"]
pub type W = crate::W<BrsselSpec>;
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chselg0 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    Value2 = 1,
}
impl From<Chselg0> for bool {
    #[inline(always)]
    fn from(variant: Chselg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELG0` reader - Channel Selection Group x"]
pub type Chselg0R = crate::BitReader<Chselg0>;
impl Chselg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chselg0 {
        match self.bits {
            false => Chselg0::Value1,
            true => Chselg0::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chselg0::Value1
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chselg0::Value2
    }
}
#[doc = "Field `CHSELG0` writer - Channel Selection Group x"]
pub type Chselg0W<'a, REG> = crate::BitWriter<'a, REG, Chselg0>;
impl<'a, REG> Chselg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chselg0::Value1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chselg0::Value2)
    }
}
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chselg1 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    Value2 = 1,
}
impl From<Chselg1> for bool {
    #[inline(always)]
    fn from(variant: Chselg1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELG1` reader - Channel Selection Group x"]
pub type Chselg1R = crate::BitReader<Chselg1>;
impl Chselg1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chselg1 {
        match self.bits {
            false => Chselg1::Value1,
            true => Chselg1::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chselg1::Value1
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chselg1::Value2
    }
}
#[doc = "Field `CHSELG1` writer - Channel Selection Group x"]
pub type Chselg1W<'a, REG> = crate::BitWriter<'a, REG, Chselg1>;
impl<'a, REG> Chselg1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chselg1::Value1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chselg1::Value2)
    }
}
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chselg2 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    Value2 = 1,
}
impl From<Chselg2> for bool {
    #[inline(always)]
    fn from(variant: Chselg2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELG2` reader - Channel Selection Group x"]
pub type Chselg2R = crate::BitReader<Chselg2>;
impl Chselg2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chselg2 {
        match self.bits {
            false => Chselg2::Value1,
            true => Chselg2::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chselg2::Value1
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chselg2::Value2
    }
}
#[doc = "Field `CHSELG2` writer - Channel Selection Group x"]
pub type Chselg2W<'a, REG> = crate::BitWriter<'a, REG, Chselg2>;
impl<'a, REG> Chselg2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chselg2::Value1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chselg2::Value2)
    }
}
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chselg3 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    Value2 = 1,
}
impl From<Chselg3> for bool {
    #[inline(always)]
    fn from(variant: Chselg3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELG3` reader - Channel Selection Group x"]
pub type Chselg3R = crate::BitReader<Chselg3>;
impl Chselg3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chselg3 {
        match self.bits {
            false => Chselg3::Value1,
            true => Chselg3::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chselg3::Value1
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chselg3::Value2
    }
}
#[doc = "Field `CHSELG3` writer - Channel Selection Group x"]
pub type Chselg3W<'a, REG> = crate::BitWriter<'a, REG, Chselg3>;
impl<'a, REG> Chselg3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chselg3::Value1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chselg3::Value2)
    }
}
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chselg4 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    Value2 = 1,
}
impl From<Chselg4> for bool {
    #[inline(always)]
    fn from(variant: Chselg4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELG4` reader - Channel Selection Group x"]
pub type Chselg4R = crate::BitReader<Chselg4>;
impl Chselg4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chselg4 {
        match self.bits {
            false => Chselg4::Value1,
            true => Chselg4::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chselg4::Value1
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chselg4::Value2
    }
}
#[doc = "Field `CHSELG4` writer - Channel Selection Group x"]
pub type Chselg4W<'a, REG> = crate::BitWriter<'a, REG, Chselg4>;
impl<'a, REG> Chselg4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chselg4::Value1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chselg4::Value2)
    }
}
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chselg5 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    Value2 = 1,
}
impl From<Chselg5> for bool {
    #[inline(always)]
    fn from(variant: Chselg5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELG5` reader - Channel Selection Group x"]
pub type Chselg5R = crate::BitReader<Chselg5>;
impl Chselg5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chselg5 {
        match self.bits {
            false => Chselg5::Value1,
            true => Chselg5::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chselg5::Value1
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chselg5::Value2
    }
}
#[doc = "Field `CHSELG5` writer - Channel Selection Group x"]
pub type Chselg5W<'a, REG> = crate::BitWriter<'a, REG, Chselg5>;
impl<'a, REG> Chselg5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chselg5::Value1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chselg5::Value2)
    }
}
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chselg6 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    Value2 = 1,
}
impl From<Chselg6> for bool {
    #[inline(always)]
    fn from(variant: Chselg6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELG6` reader - Channel Selection Group x"]
pub type Chselg6R = crate::BitReader<Chselg6>;
impl Chselg6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chselg6 {
        match self.bits {
            false => Chselg6::Value1,
            true => Chselg6::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chselg6::Value1
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chselg6::Value2
    }
}
#[doc = "Field `CHSELG6` writer - Channel Selection Group x"]
pub type Chselg6W<'a, REG> = crate::BitWriter<'a, REG, Chselg6>;
impl<'a, REG> Chselg6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chselg6::Value1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chselg6::Value2)
    }
}
#[doc = "Channel Selection Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chselg7 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    Value2 = 1,
}
impl From<Chselg7> for bool {
    #[inline(always)]
    fn from(variant: Chselg7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSELG7` reader - Channel Selection Group x"]
pub type Chselg7R = crate::BitReader<Chselg7>;
impl Chselg7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chselg7 {
        match self.bits {
            false => Chselg7::Value1,
            true => Chselg7::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chselg7::Value1
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chselg7::Value2
    }
}
#[doc = "Field `CHSELG7` writer - Channel Selection Group x"]
pub type Chselg7W<'a, REG> = crate::BitWriter<'a, REG, Chselg7>;
impl<'a, REG> Chselg7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chselg7::Value1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chselg7::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg0(&self) -> Chselg0R {
        Chselg0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg1(&self) -> Chselg1R {
        Chselg1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg2(&self) -> Chselg2R {
        Chselg2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg3(&self) -> Chselg3R {
        Chselg3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg4(&self) -> Chselg4R {
        Chselg4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg5(&self) -> Chselg5R {
        Chselg5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg6(&self) -> Chselg6R {
        Chselg6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel Selection Group x"]
    #[inline(always)]
    pub fn chselg7(&self) -> Chselg7R {
        Chselg7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Selection Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chselg0(&mut self) -> Chselg0W<BrsselSpec> {
        Chselg0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Selection Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chselg1(&mut self) -> Chselg1W<BrsselSpec> {
        Chselg1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel Selection Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chselg2(&mut self) -> Chselg2W<BrsselSpec> {
        Chselg2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel Selection Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chselg3(&mut self) -> Chselg3W<BrsselSpec> {
        Chselg3W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel Selection Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chselg4(&mut self) -> Chselg4W<BrsselSpec> {
        Chselg4W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel Selection Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chselg5(&mut self) -> Chselg5W<BrsselSpec> {
        Chselg5W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel Selection Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chselg6(&mut self) -> Chselg6W<BrsselSpec> {
        Chselg6W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel Selection Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chselg7(&mut self) -> Chselg7W<BrsselSpec> {
        Chselg7W::new(self, 7)
    }
}
#[doc = "Background Request Source Channel Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brssel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brssel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrsselSpec;
impl crate::RegisterSpec for BrsselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brssel::R`](R) reader structure"]
impl crate::Readable for BrsselSpec {}
#[doc = "`write(|w| ..)` method takes [`brssel::W`](W) writer structure"]
impl crate::Writable for BrsselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BRSSEL[%s]
to value 0"]
impl crate::Resettable for BrsselSpec {
    const RESET_VALUE: u32 = 0;
}
