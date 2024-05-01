#[doc = "Register `ASPND` reader"]
pub type R = crate::R<AspndSpec>;
#[doc = "Register `ASPND` writer"]
pub type W = crate::W<AspndSpec>;
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chpnd0 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: Request conversion of this channel"]
    Value2 = 1,
}
impl From<Chpnd0> for bool {
    #[inline(always)]
    fn from(variant: Chpnd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHPND0` reader - Channels Pending"]
pub type Chpnd0R = crate::BitReader<Chpnd0>;
impl Chpnd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chpnd0 {
        match self.bits {
            false => Chpnd0::Value1,
            true => Chpnd0::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chpnd0::Value1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chpnd0::Value2
    }
}
#[doc = "Field `CHPND0` writer - Channels Pending"]
pub type Chpnd0W<'a, REG> = crate::BitWriter<'a, REG, Chpnd0>;
impl<'a, REG> Chpnd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chpnd0::Value1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chpnd0::Value2)
    }
}
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chpnd1 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: Request conversion of this channel"]
    Value2 = 1,
}
impl From<Chpnd1> for bool {
    #[inline(always)]
    fn from(variant: Chpnd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHPND1` reader - Channels Pending"]
pub type Chpnd1R = crate::BitReader<Chpnd1>;
impl Chpnd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chpnd1 {
        match self.bits {
            false => Chpnd1::Value1,
            true => Chpnd1::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chpnd1::Value1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chpnd1::Value2
    }
}
#[doc = "Field `CHPND1` writer - Channels Pending"]
pub type Chpnd1W<'a, REG> = crate::BitWriter<'a, REG, Chpnd1>;
impl<'a, REG> Chpnd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chpnd1::Value1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chpnd1::Value2)
    }
}
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chpnd2 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: Request conversion of this channel"]
    Value2 = 1,
}
impl From<Chpnd2> for bool {
    #[inline(always)]
    fn from(variant: Chpnd2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHPND2` reader - Channels Pending"]
pub type Chpnd2R = crate::BitReader<Chpnd2>;
impl Chpnd2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chpnd2 {
        match self.bits {
            false => Chpnd2::Value1,
            true => Chpnd2::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chpnd2::Value1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chpnd2::Value2
    }
}
#[doc = "Field `CHPND2` writer - Channels Pending"]
pub type Chpnd2W<'a, REG> = crate::BitWriter<'a, REG, Chpnd2>;
impl<'a, REG> Chpnd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chpnd2::Value1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chpnd2::Value2)
    }
}
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chpnd3 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: Request conversion of this channel"]
    Value2 = 1,
}
impl From<Chpnd3> for bool {
    #[inline(always)]
    fn from(variant: Chpnd3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHPND3` reader - Channels Pending"]
pub type Chpnd3R = crate::BitReader<Chpnd3>;
impl Chpnd3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chpnd3 {
        match self.bits {
            false => Chpnd3::Value1,
            true => Chpnd3::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chpnd3::Value1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chpnd3::Value2
    }
}
#[doc = "Field `CHPND3` writer - Channels Pending"]
pub type Chpnd3W<'a, REG> = crate::BitWriter<'a, REG, Chpnd3>;
impl<'a, REG> Chpnd3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chpnd3::Value1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chpnd3::Value2)
    }
}
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chpnd4 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: Request conversion of this channel"]
    Value2 = 1,
}
impl From<Chpnd4> for bool {
    #[inline(always)]
    fn from(variant: Chpnd4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHPND4` reader - Channels Pending"]
pub type Chpnd4R = crate::BitReader<Chpnd4>;
impl Chpnd4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chpnd4 {
        match self.bits {
            false => Chpnd4::Value1,
            true => Chpnd4::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chpnd4::Value1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chpnd4::Value2
    }
}
#[doc = "Field `CHPND4` writer - Channels Pending"]
pub type Chpnd4W<'a, REG> = crate::BitWriter<'a, REG, Chpnd4>;
impl<'a, REG> Chpnd4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chpnd4::Value1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chpnd4::Value2)
    }
}
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chpnd5 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: Request conversion of this channel"]
    Value2 = 1,
}
impl From<Chpnd5> for bool {
    #[inline(always)]
    fn from(variant: Chpnd5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHPND5` reader - Channels Pending"]
pub type Chpnd5R = crate::BitReader<Chpnd5>;
impl Chpnd5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chpnd5 {
        match self.bits {
            false => Chpnd5::Value1,
            true => Chpnd5::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chpnd5::Value1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chpnd5::Value2
    }
}
#[doc = "Field `CHPND5` writer - Channels Pending"]
pub type Chpnd5W<'a, REG> = crate::BitWriter<'a, REG, Chpnd5>;
impl<'a, REG> Chpnd5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chpnd5::Value1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chpnd5::Value2)
    }
}
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chpnd6 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: Request conversion of this channel"]
    Value2 = 1,
}
impl From<Chpnd6> for bool {
    #[inline(always)]
    fn from(variant: Chpnd6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHPND6` reader - Channels Pending"]
pub type Chpnd6R = crate::BitReader<Chpnd6>;
impl Chpnd6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chpnd6 {
        match self.bits {
            false => Chpnd6::Value1,
            true => Chpnd6::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chpnd6::Value1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chpnd6::Value2
    }
}
#[doc = "Field `CHPND6` writer - Channels Pending"]
pub type Chpnd6W<'a, REG> = crate::BitWriter<'a, REG, Chpnd6>;
impl<'a, REG> Chpnd6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chpnd6::Value1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chpnd6::Value2)
    }
}
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chpnd7 {
    #[doc = "0: Ignore this channel"]
    Value1 = 0,
    #[doc = "1: Request conversion of this channel"]
    Value2 = 1,
}
impl From<Chpnd7> for bool {
    #[inline(always)]
    fn from(variant: Chpnd7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHPND7` reader - Channels Pending"]
pub type Chpnd7R = crate::BitReader<Chpnd7>;
impl Chpnd7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chpnd7 {
        match self.bits {
            false => Chpnd7::Value1,
            true => Chpnd7::Value2,
        }
    }
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chpnd7::Value1
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chpnd7::Value2
    }
}
#[doc = "Field `CHPND7` writer - Channels Pending"]
pub type Chpnd7W<'a, REG> = crate::BitWriter<'a, REG, Chpnd7>;
impl<'a, REG> Chpnd7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chpnd7::Value1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chpnd7::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd0(&self) -> Chpnd0R {
        Chpnd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd1(&self) -> Chpnd1R {
        Chpnd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd2(&self) -> Chpnd2R {
        Chpnd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd3(&self) -> Chpnd3R {
        Chpnd3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd4(&self) -> Chpnd4R {
        Chpnd4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd5(&self) -> Chpnd5R {
        Chpnd5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd6(&self) -> Chpnd6R {
        Chpnd6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd7(&self) -> Chpnd7R {
        Chpnd7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channels Pending"]
    #[inline(always)]
    #[must_use]
    pub fn chpnd0(&mut self) -> Chpnd0W<AspndSpec> {
        Chpnd0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channels Pending"]
    #[inline(always)]
    #[must_use]
    pub fn chpnd1(&mut self) -> Chpnd1W<AspndSpec> {
        Chpnd1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channels Pending"]
    #[inline(always)]
    #[must_use]
    pub fn chpnd2(&mut self) -> Chpnd2W<AspndSpec> {
        Chpnd2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channels Pending"]
    #[inline(always)]
    #[must_use]
    pub fn chpnd3(&mut self) -> Chpnd3W<AspndSpec> {
        Chpnd3W::new(self, 3)
    }
    #[doc = "Bit 4 - Channels Pending"]
    #[inline(always)]
    #[must_use]
    pub fn chpnd4(&mut self) -> Chpnd4W<AspndSpec> {
        Chpnd4W::new(self, 4)
    }
    #[doc = "Bit 5 - Channels Pending"]
    #[inline(always)]
    #[must_use]
    pub fn chpnd5(&mut self) -> Chpnd5W<AspndSpec> {
        Chpnd5W::new(self, 5)
    }
    #[doc = "Bit 6 - Channels Pending"]
    #[inline(always)]
    #[must_use]
    pub fn chpnd6(&mut self) -> Chpnd6W<AspndSpec> {
        Chpnd6W::new(self, 6)
    }
    #[doc = "Bit 7 - Channels Pending"]
    #[inline(always)]
    #[must_use]
    pub fn chpnd7(&mut self) -> Chpnd7W<AspndSpec> {
        Chpnd7W::new(self, 7)
    }
}
#[doc = "Autoscan Source Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aspnd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aspnd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AspndSpec;
impl crate::RegisterSpec for AspndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aspnd::R`](R) reader structure"]
impl crate::Readable for AspndSpec {}
#[doc = "`write(|w| ..)` method takes [`aspnd::W`](W) writer structure"]
impl crate::Writable for AspndSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ASPND to value 0"]
impl crate::Resettable for AspndSpec {
    const RESET_VALUE: u32 = 0;
}
