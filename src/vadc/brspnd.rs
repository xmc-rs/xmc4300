#[doc = "Register `BRSPND[%s]` reader"]
pub type R = crate::R<BrspndSpec>;
#[doc = "Register `BRSPND[%s]` writer"]
pub type W = crate::W<BrspndSpec>;
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chpndg0 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: Request conversion of this channel"]
    Value2 = 1,
}
impl From<Chpndg0> for bool {
    #[inline(always)]
    fn from(variant: Chpndg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHPNDG0` reader - Channels Pending Group x"]
pub type Chpndg0R = crate::BitReader<Chpndg0>;
impl Chpndg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chpndg0 {
        match self.bits {
            false => Chpndg0::Value1,
            true => Chpndg0::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chpndg0::Value1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chpndg0::Value2
    }
}
#[doc = "Field `CHPNDG0` writer - Channels Pending Group x"]
pub type Chpndg0W<'a, REG> = crate::BitWriter<'a, REG, Chpndg0>;
impl<'a, REG> Chpndg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chpndg0::Value1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chpndg0::Value2)
    }
}
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chpndg1 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: Request conversion of this channel"]
    Value2 = 1,
}
impl From<Chpndg1> for bool {
    #[inline(always)]
    fn from(variant: Chpndg1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHPNDG1` reader - Channels Pending Group x"]
pub type Chpndg1R = crate::BitReader<Chpndg1>;
impl Chpndg1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chpndg1 {
        match self.bits {
            false => Chpndg1::Value1,
            true => Chpndg1::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chpndg1::Value1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chpndg1::Value2
    }
}
#[doc = "Field `CHPNDG1` writer - Channels Pending Group x"]
pub type Chpndg1W<'a, REG> = crate::BitWriter<'a, REG, Chpndg1>;
impl<'a, REG> Chpndg1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chpndg1::Value1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chpndg1::Value2)
    }
}
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chpndg2 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: Request conversion of this channel"]
    Value2 = 1,
}
impl From<Chpndg2> for bool {
    #[inline(always)]
    fn from(variant: Chpndg2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHPNDG2` reader - Channels Pending Group x"]
pub type Chpndg2R = crate::BitReader<Chpndg2>;
impl Chpndg2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chpndg2 {
        match self.bits {
            false => Chpndg2::Value1,
            true => Chpndg2::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chpndg2::Value1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chpndg2::Value2
    }
}
#[doc = "Field `CHPNDG2` writer - Channels Pending Group x"]
pub type Chpndg2W<'a, REG> = crate::BitWriter<'a, REG, Chpndg2>;
impl<'a, REG> Chpndg2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chpndg2::Value1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chpndg2::Value2)
    }
}
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chpndg3 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: Request conversion of this channel"]
    Value2 = 1,
}
impl From<Chpndg3> for bool {
    #[inline(always)]
    fn from(variant: Chpndg3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHPNDG3` reader - Channels Pending Group x"]
pub type Chpndg3R = crate::BitReader<Chpndg3>;
impl Chpndg3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chpndg3 {
        match self.bits {
            false => Chpndg3::Value1,
            true => Chpndg3::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chpndg3::Value1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chpndg3::Value2
    }
}
#[doc = "Field `CHPNDG3` writer - Channels Pending Group x"]
pub type Chpndg3W<'a, REG> = crate::BitWriter<'a, REG, Chpndg3>;
impl<'a, REG> Chpndg3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chpndg3::Value1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chpndg3::Value2)
    }
}
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chpndg4 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: Request conversion of this channel"]
    Value2 = 1,
}
impl From<Chpndg4> for bool {
    #[inline(always)]
    fn from(variant: Chpndg4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHPNDG4` reader - Channels Pending Group x"]
pub type Chpndg4R = crate::BitReader<Chpndg4>;
impl Chpndg4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chpndg4 {
        match self.bits {
            false => Chpndg4::Value1,
            true => Chpndg4::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chpndg4::Value1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chpndg4::Value2
    }
}
#[doc = "Field `CHPNDG4` writer - Channels Pending Group x"]
pub type Chpndg4W<'a, REG> = crate::BitWriter<'a, REG, Chpndg4>;
impl<'a, REG> Chpndg4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chpndg4::Value1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chpndg4::Value2)
    }
}
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chpndg5 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: Request conversion of this channel"]
    Value2 = 1,
}
impl From<Chpndg5> for bool {
    #[inline(always)]
    fn from(variant: Chpndg5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHPNDG5` reader - Channels Pending Group x"]
pub type Chpndg5R = crate::BitReader<Chpndg5>;
impl Chpndg5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chpndg5 {
        match self.bits {
            false => Chpndg5::Value1,
            true => Chpndg5::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chpndg5::Value1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chpndg5::Value2
    }
}
#[doc = "Field `CHPNDG5` writer - Channels Pending Group x"]
pub type Chpndg5W<'a, REG> = crate::BitWriter<'a, REG, Chpndg5>;
impl<'a, REG> Chpndg5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chpndg5::Value1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chpndg5::Value2)
    }
}
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chpndg6 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: Request conversion of this channel"]
    Value2 = 1,
}
impl From<Chpndg6> for bool {
    #[inline(always)]
    fn from(variant: Chpndg6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHPNDG6` reader - Channels Pending Group x"]
pub type Chpndg6R = crate::BitReader<Chpndg6>;
impl Chpndg6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chpndg6 {
        match self.bits {
            false => Chpndg6::Value1,
            true => Chpndg6::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chpndg6::Value1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chpndg6::Value2
    }
}
#[doc = "Field `CHPNDG6` writer - Channels Pending Group x"]
pub type Chpndg6W<'a, REG> = crate::BitWriter<'a, REG, Chpndg6>;
impl<'a, REG> Chpndg6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chpndg6::Value1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chpndg6::Value2)
    }
}
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chpndg7 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: Request conversion of this channel"]
    Value2 = 1,
}
impl From<Chpndg7> for bool {
    #[inline(always)]
    fn from(variant: Chpndg7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHPNDG7` reader - Channels Pending Group x"]
pub type Chpndg7R = crate::BitReader<Chpndg7>;
impl Chpndg7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chpndg7 {
        match self.bits {
            false => Chpndg7::Value1,
            true => Chpndg7::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chpndg7::Value1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chpndg7::Value2
    }
}
#[doc = "Field `CHPNDG7` writer - Channels Pending Group x"]
pub type Chpndg7W<'a, REG> = crate::BitWriter<'a, REG, Chpndg7>;
impl<'a, REG> Chpndg7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chpndg7::Value1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chpndg7::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg0(&self) -> Chpndg0R {
        Chpndg0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg1(&self) -> Chpndg1R {
        Chpndg1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg2(&self) -> Chpndg2R {
        Chpndg2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg3(&self) -> Chpndg3R {
        Chpndg3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg4(&self) -> Chpndg4R {
        Chpndg4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg5(&self) -> Chpndg5R {
        Chpndg5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg6(&self) -> Chpndg6R {
        Chpndg6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg7(&self) -> Chpndg7R {
        Chpndg7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channels Pending Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chpndg0(&mut self) -> Chpndg0W<BrspndSpec> {
        Chpndg0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channels Pending Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chpndg1(&mut self) -> Chpndg1W<BrspndSpec> {
        Chpndg1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channels Pending Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chpndg2(&mut self) -> Chpndg2W<BrspndSpec> {
        Chpndg2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channels Pending Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chpndg3(&mut self) -> Chpndg3W<BrspndSpec> {
        Chpndg3W::new(self, 3)
    }
    #[doc = "Bit 4 - Channels Pending Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chpndg4(&mut self) -> Chpndg4W<BrspndSpec> {
        Chpndg4W::new(self, 4)
    }
    #[doc = "Bit 5 - Channels Pending Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chpndg5(&mut self) -> Chpndg5W<BrspndSpec> {
        Chpndg5W::new(self, 5)
    }
    #[doc = "Bit 6 - Channels Pending Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chpndg6(&mut self) -> Chpndg6W<BrspndSpec> {
        Chpndg6W::new(self, 6)
    }
    #[doc = "Bit 7 - Channels Pending Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chpndg7(&mut self) -> Chpndg7W<BrspndSpec> {
        Chpndg7W::new(self, 7)
    }
}
#[doc = "Background Request Source Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brspnd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brspnd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrspndSpec;
impl crate::RegisterSpec for BrspndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brspnd::R`](R) reader structure"]
impl crate::Readable for BrspndSpec {}
#[doc = "`write(|w| ..)` method takes [`brspnd::W`](W) writer structure"]
impl crate::Writable for BrspndSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BRSPND[%s]
to value 0"]
impl crate::Resettable for BrspndSpec {
    const RESET_VALUE: u32 = 0;
}
