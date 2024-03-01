#[doc = "Register `CHASS` reader"]
pub type R = crate::R<ChassSpec>;
#[doc = "Register `CHASS` writer"]
pub type W = crate::W<ChassSpec>;
#[doc = "Assignment for Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Assch0 {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    Value1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    Value2 = 1,
}
impl From<Assch0> for bool {
    #[inline(always)]
    fn from(variant: Assch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSCH0` reader - Assignment for Channel 0"]
pub type Assch0R = crate::BitReader<Assch0>;
impl Assch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Assch0 {
        match self.bits {
            false => Assch0::Value1,
            true => Assch0::Value2,
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Assch0::Value1
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Assch0::Value2
    }
}
#[doc = "Field `ASSCH0` writer - Assignment for Channel 0"]
pub type Assch0W<'a, REG> = crate::BitWriter<'a, REG, Assch0>;
impl<'a, REG> Assch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Assch0::Value1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Assch0::Value2)
    }
}
#[doc = "Assignment for Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Assch1 {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    Value1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    Value2 = 1,
}
impl From<Assch1> for bool {
    #[inline(always)]
    fn from(variant: Assch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSCH1` reader - Assignment for Channel 1"]
pub type Assch1R = crate::BitReader<Assch1>;
impl Assch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Assch1 {
        match self.bits {
            false => Assch1::Value1,
            true => Assch1::Value2,
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Assch1::Value1
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Assch1::Value2
    }
}
#[doc = "Field `ASSCH1` writer - Assignment for Channel 1"]
pub type Assch1W<'a, REG> = crate::BitWriter<'a, REG, Assch1>;
impl<'a, REG> Assch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Assch1::Value1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Assch1::Value2)
    }
}
#[doc = "Assignment for Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Assch2 {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    Value1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    Value2 = 1,
}
impl From<Assch2> for bool {
    #[inline(always)]
    fn from(variant: Assch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSCH2` reader - Assignment for Channel 2"]
pub type Assch2R = crate::BitReader<Assch2>;
impl Assch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Assch2 {
        match self.bits {
            false => Assch2::Value1,
            true => Assch2::Value2,
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Assch2::Value1
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Assch2::Value2
    }
}
#[doc = "Field `ASSCH2` writer - Assignment for Channel 2"]
pub type Assch2W<'a, REG> = crate::BitWriter<'a, REG, Assch2>;
impl<'a, REG> Assch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Assch2::Value1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Assch2::Value2)
    }
}
#[doc = "Assignment for Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Assch3 {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    Value1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    Value2 = 1,
}
impl From<Assch3> for bool {
    #[inline(always)]
    fn from(variant: Assch3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSCH3` reader - Assignment for Channel 3"]
pub type Assch3R = crate::BitReader<Assch3>;
impl Assch3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Assch3 {
        match self.bits {
            false => Assch3::Value1,
            true => Assch3::Value2,
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Assch3::Value1
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Assch3::Value2
    }
}
#[doc = "Field `ASSCH3` writer - Assignment for Channel 3"]
pub type Assch3W<'a, REG> = crate::BitWriter<'a, REG, Assch3>;
impl<'a, REG> Assch3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Assch3::Value1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Assch3::Value2)
    }
}
#[doc = "Assignment for Channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Assch4 {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    Value1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    Value2 = 1,
}
impl From<Assch4> for bool {
    #[inline(always)]
    fn from(variant: Assch4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSCH4` reader - Assignment for Channel 4"]
pub type Assch4R = crate::BitReader<Assch4>;
impl Assch4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Assch4 {
        match self.bits {
            false => Assch4::Value1,
            true => Assch4::Value2,
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Assch4::Value1
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Assch4::Value2
    }
}
#[doc = "Field `ASSCH4` writer - Assignment for Channel 4"]
pub type Assch4W<'a, REG> = crate::BitWriter<'a, REG, Assch4>;
impl<'a, REG> Assch4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Assch4::Value1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Assch4::Value2)
    }
}
#[doc = "Assignment for Channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Assch5 {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    Value1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    Value2 = 1,
}
impl From<Assch5> for bool {
    #[inline(always)]
    fn from(variant: Assch5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSCH5` reader - Assignment for Channel 5"]
pub type Assch5R = crate::BitReader<Assch5>;
impl Assch5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Assch5 {
        match self.bits {
            false => Assch5::Value1,
            true => Assch5::Value2,
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Assch5::Value1
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Assch5::Value2
    }
}
#[doc = "Field `ASSCH5` writer - Assignment for Channel 5"]
pub type Assch5W<'a, REG> = crate::BitWriter<'a, REG, Assch5>;
impl<'a, REG> Assch5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Assch5::Value1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Assch5::Value2)
    }
}
#[doc = "Assignment for Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Assch6 {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    Value1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    Value2 = 1,
}
impl From<Assch6> for bool {
    #[inline(always)]
    fn from(variant: Assch6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSCH6` reader - Assignment for Channel 6"]
pub type Assch6R = crate::BitReader<Assch6>;
impl Assch6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Assch6 {
        match self.bits {
            false => Assch6::Value1,
            true => Assch6::Value2,
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Assch6::Value1
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Assch6::Value2
    }
}
#[doc = "Field `ASSCH6` writer - Assignment for Channel 6"]
pub type Assch6W<'a, REG> = crate::BitWriter<'a, REG, Assch6>;
impl<'a, REG> Assch6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Assch6::Value1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Assch6::Value2)
    }
}
#[doc = "Assignment for Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Assch7 {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    Value1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    Value2 = 1,
}
impl From<Assch7> for bool {
    #[inline(always)]
    fn from(variant: Assch7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSCH7` reader - Assignment for Channel 7"]
pub type Assch7R = crate::BitReader<Assch7>;
impl Assch7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Assch7 {
        match self.bits {
            false => Assch7::Value1,
            true => Assch7::Value2,
        }
    }
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Assch7::Value1
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Assch7::Value2
    }
}
#[doc = "Field `ASSCH7` writer - Assignment for Channel 7"]
pub type Assch7W<'a, REG> = crate::BitWriter<'a, REG, Assch7>;
impl<'a, REG> Assch7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Assch7::Value1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Assch7::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Assignment for Channel 0"]
    #[inline(always)]
    pub fn assch0(&self) -> Assch0R {
        Assch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Assignment for Channel 1"]
    #[inline(always)]
    pub fn assch1(&self) -> Assch1R {
        Assch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Assignment for Channel 2"]
    #[inline(always)]
    pub fn assch2(&self) -> Assch2R {
        Assch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Assignment for Channel 3"]
    #[inline(always)]
    pub fn assch3(&self) -> Assch3R {
        Assch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Assignment for Channel 4"]
    #[inline(always)]
    pub fn assch4(&self) -> Assch4R {
        Assch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Assignment for Channel 5"]
    #[inline(always)]
    pub fn assch5(&self) -> Assch5R {
        Assch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Assignment for Channel 6"]
    #[inline(always)]
    pub fn assch6(&self) -> Assch6R {
        Assch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Assignment for Channel 7"]
    #[inline(always)]
    pub fn assch7(&self) -> Assch7R {
        Assch7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Assignment for Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn assch0(&mut self) -> Assch0W<ChassSpec> {
        Assch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Assignment for Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn assch1(&mut self) -> Assch1W<ChassSpec> {
        Assch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Assignment for Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn assch2(&mut self) -> Assch2W<ChassSpec> {
        Assch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Assignment for Channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn assch3(&mut self) -> Assch3W<ChassSpec> {
        Assch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Assignment for Channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn assch4(&mut self) -> Assch4W<ChassSpec> {
        Assch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Assignment for Channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn assch5(&mut self) -> Assch5W<ChassSpec> {
        Assch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Assignment for Channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn assch6(&mut self) -> Assch6W<ChassSpec> {
        Assch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Assignment for Channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn assch7(&mut self) -> Assch7W<ChassSpec> {
        Assch7W::new(self, 7)
    }
}
#[doc = "Channel Assignment Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chass::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chass::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChassSpec;
impl crate::RegisterSpec for ChassSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chass::R`](R) reader structure"]
impl crate::Readable for ChassSpec {}
#[doc = "`write(|w| ..)` method takes [`chass::W`](W) writer structure"]
impl crate::Writable for ChassSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHASS to value 0"]
impl crate::Resettable for ChassSpec {
    const RESET_VALUE: u32 = 0;
}
