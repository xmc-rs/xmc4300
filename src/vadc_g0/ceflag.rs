#[doc = "Register `CEFLAG` reader"]
pub type R = crate::R<CeflagSpec>;
#[doc = "Register `CEFLAG` writer"]
pub type W = crate::W<CeflagSpec>;
#[doc = "Channel Event for Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cev0 {
    #[doc = "0: No channel event"]
    Value1 = 0,
    #[doc = "1: A channel event has occurred"]
    Value2 = 1,
}
impl From<Cev0> for bool {
    #[inline(always)]
    fn from(variant: Cev0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV0` reader - Channel Event for Channel 0"]
pub type Cev0R = crate::BitReader<Cev0>;
impl Cev0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cev0 {
        match self.bits {
            false => Cev0::Value1,
            true => Cev0::Value2,
        }
    }
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cev0::Value1
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cev0::Value2
    }
}
#[doc = "Field `CEV0` writer - Channel Event for Channel 0"]
pub type Cev0W<'a, REG> = crate::BitWriter<'a, REG, Cev0>;
impl<'a, REG> Cev0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev0::Value1)
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev0::Value2)
    }
}
#[doc = "Channel Event for Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cev1 {
    #[doc = "0: No channel event"]
    Value1 = 0,
    #[doc = "1: A channel event has occurred"]
    Value2 = 1,
}
impl From<Cev1> for bool {
    #[inline(always)]
    fn from(variant: Cev1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV1` reader - Channel Event for Channel 1"]
pub type Cev1R = crate::BitReader<Cev1>;
impl Cev1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cev1 {
        match self.bits {
            false => Cev1::Value1,
            true => Cev1::Value2,
        }
    }
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cev1::Value1
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cev1::Value2
    }
}
#[doc = "Field `CEV1` writer - Channel Event for Channel 1"]
pub type Cev1W<'a, REG> = crate::BitWriter<'a, REG, Cev1>;
impl<'a, REG> Cev1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev1::Value1)
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev1::Value2)
    }
}
#[doc = "Channel Event for Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cev2 {
    #[doc = "0: No channel event"]
    Value1 = 0,
    #[doc = "1: A channel event has occurred"]
    Value2 = 1,
}
impl From<Cev2> for bool {
    #[inline(always)]
    fn from(variant: Cev2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV2` reader - Channel Event for Channel 2"]
pub type Cev2R = crate::BitReader<Cev2>;
impl Cev2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cev2 {
        match self.bits {
            false => Cev2::Value1,
            true => Cev2::Value2,
        }
    }
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cev2::Value1
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cev2::Value2
    }
}
#[doc = "Field `CEV2` writer - Channel Event for Channel 2"]
pub type Cev2W<'a, REG> = crate::BitWriter<'a, REG, Cev2>;
impl<'a, REG> Cev2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev2::Value1)
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev2::Value2)
    }
}
#[doc = "Channel Event for Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cev3 {
    #[doc = "0: No channel event"]
    Value1 = 0,
    #[doc = "1: A channel event has occurred"]
    Value2 = 1,
}
impl From<Cev3> for bool {
    #[inline(always)]
    fn from(variant: Cev3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV3` reader - Channel Event for Channel 3"]
pub type Cev3R = crate::BitReader<Cev3>;
impl Cev3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cev3 {
        match self.bits {
            false => Cev3::Value1,
            true => Cev3::Value2,
        }
    }
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cev3::Value1
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cev3::Value2
    }
}
#[doc = "Field `CEV3` writer - Channel Event for Channel 3"]
pub type Cev3W<'a, REG> = crate::BitWriter<'a, REG, Cev3>;
impl<'a, REG> Cev3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev3::Value1)
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev3::Value2)
    }
}
#[doc = "Channel Event for Channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cev4 {
    #[doc = "0: No channel event"]
    Value1 = 0,
    #[doc = "1: A channel event has occurred"]
    Value2 = 1,
}
impl From<Cev4> for bool {
    #[inline(always)]
    fn from(variant: Cev4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV4` reader - Channel Event for Channel 4"]
pub type Cev4R = crate::BitReader<Cev4>;
impl Cev4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cev4 {
        match self.bits {
            false => Cev4::Value1,
            true => Cev4::Value2,
        }
    }
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cev4::Value1
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cev4::Value2
    }
}
#[doc = "Field `CEV4` writer - Channel Event for Channel 4"]
pub type Cev4W<'a, REG> = crate::BitWriter<'a, REG, Cev4>;
impl<'a, REG> Cev4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev4::Value1)
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev4::Value2)
    }
}
#[doc = "Channel Event for Channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cev5 {
    #[doc = "0: No channel event"]
    Value1 = 0,
    #[doc = "1: A channel event has occurred"]
    Value2 = 1,
}
impl From<Cev5> for bool {
    #[inline(always)]
    fn from(variant: Cev5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV5` reader - Channel Event for Channel 5"]
pub type Cev5R = crate::BitReader<Cev5>;
impl Cev5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cev5 {
        match self.bits {
            false => Cev5::Value1,
            true => Cev5::Value2,
        }
    }
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cev5::Value1
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cev5::Value2
    }
}
#[doc = "Field `CEV5` writer - Channel Event for Channel 5"]
pub type Cev5W<'a, REG> = crate::BitWriter<'a, REG, Cev5>;
impl<'a, REG> Cev5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev5::Value1)
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev5::Value2)
    }
}
#[doc = "Channel Event for Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cev6 {
    #[doc = "0: No channel event"]
    Value1 = 0,
    #[doc = "1: A channel event has occurred"]
    Value2 = 1,
}
impl From<Cev6> for bool {
    #[inline(always)]
    fn from(variant: Cev6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV6` reader - Channel Event for Channel 6"]
pub type Cev6R = crate::BitReader<Cev6>;
impl Cev6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cev6 {
        match self.bits {
            false => Cev6::Value1,
            true => Cev6::Value2,
        }
    }
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cev6::Value1
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cev6::Value2
    }
}
#[doc = "Field `CEV6` writer - Channel Event for Channel 6"]
pub type Cev6W<'a, REG> = crate::BitWriter<'a, REG, Cev6>;
impl<'a, REG> Cev6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev6::Value1)
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev6::Value2)
    }
}
#[doc = "Channel Event for Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cev7 {
    #[doc = "0: No channel event"]
    Value1 = 0,
    #[doc = "1: A channel event has occurred"]
    Value2 = 1,
}
impl From<Cev7> for bool {
    #[inline(always)]
    fn from(variant: Cev7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV7` reader - Channel Event for Channel 7"]
pub type Cev7R = crate::BitReader<Cev7>;
impl Cev7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cev7 {
        match self.bits {
            false => Cev7::Value1,
            true => Cev7::Value2,
        }
    }
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cev7::Value1
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cev7::Value2
    }
}
#[doc = "Field `CEV7` writer - Channel Event for Channel 7"]
pub type Cev7W<'a, REG> = crate::BitWriter<'a, REG, Cev7>;
impl<'a, REG> Cev7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev7::Value1)
    }
    #[doc = "A channel event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev7::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Channel Event for Channel 0"]
    #[inline(always)]
    pub fn cev0(&self) -> Cev0R {
        Cev0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Event for Channel 1"]
    #[inline(always)]
    pub fn cev1(&self) -> Cev1R {
        Cev1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel Event for Channel 2"]
    #[inline(always)]
    pub fn cev2(&self) -> Cev2R {
        Cev2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel Event for Channel 3"]
    #[inline(always)]
    pub fn cev3(&self) -> Cev3R {
        Cev3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel Event for Channel 4"]
    #[inline(always)]
    pub fn cev4(&self) -> Cev4R {
        Cev4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel Event for Channel 5"]
    #[inline(always)]
    pub fn cev5(&self) -> Cev5R {
        Cev5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel Event for Channel 6"]
    #[inline(always)]
    pub fn cev6(&self) -> Cev6R {
        Cev6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel Event for Channel 7"]
    #[inline(always)]
    pub fn cev7(&self) -> Cev7R {
        Cev7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Event for Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn cev0(&mut self) -> Cev0W<CeflagSpec> {
        Cev0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Event for Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn cev1(&mut self) -> Cev1W<CeflagSpec> {
        Cev1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel Event for Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn cev2(&mut self) -> Cev2W<CeflagSpec> {
        Cev2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel Event for Channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn cev3(&mut self) -> Cev3W<CeflagSpec> {
        Cev3W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel Event for Channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn cev4(&mut self) -> Cev4W<CeflagSpec> {
        Cev4W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel Event for Channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn cev5(&mut self) -> Cev5W<CeflagSpec> {
        Cev5W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel Event for Channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn cev6(&mut self) -> Cev6W<CeflagSpec> {
        Cev6W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel Event for Channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn cev7(&mut self) -> Cev7W<CeflagSpec> {
        Cev7W::new(self, 7)
    }
}
#[doc = "Channel Event Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ceflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ceflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CeflagSpec;
impl crate::RegisterSpec for CeflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ceflag::R`](R) reader structure"]
impl crate::Readable for CeflagSpec {}
#[doc = "`write(|w| ..)` method takes [`ceflag::W`](W) writer structure"]
impl crate::Writable for CeflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CEFLAG to value 0"]
impl crate::Resettable for CeflagSpec {
    const RESET_VALUE: u32 = 0;
}
