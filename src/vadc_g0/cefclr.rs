#[doc = "Register `CEFCLR` writer"]
pub type W = crate::W<CefclrSpec>;
#[doc = "Clear Channel Event for Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cev0 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the channel event flag in GxCEFLAG"]
    Value2 = 1,
}
impl From<Cev0> for bool {
    #[inline(always)]
    fn from(variant: Cev0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV0` writer - Clear Channel Event for Channel 0"]
pub type Cev0W<'a, REG> = crate::BitWriter<'a, REG, Cev0>;
impl<'a, REG> Cev0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev0::Value1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev0::Value2)
    }
}
#[doc = "Clear Channel Event for Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cev1 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the channel event flag in GxCEFLAG"]
    Value2 = 1,
}
impl From<Cev1> for bool {
    #[inline(always)]
    fn from(variant: Cev1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV1` writer - Clear Channel Event for Channel 1"]
pub type Cev1W<'a, REG> = crate::BitWriter<'a, REG, Cev1>;
impl<'a, REG> Cev1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev1::Value1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev1::Value2)
    }
}
#[doc = "Clear Channel Event for Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cev2 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the channel event flag in GxCEFLAG"]
    Value2 = 1,
}
impl From<Cev2> for bool {
    #[inline(always)]
    fn from(variant: Cev2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV2` writer - Clear Channel Event for Channel 2"]
pub type Cev2W<'a, REG> = crate::BitWriter<'a, REG, Cev2>;
impl<'a, REG> Cev2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev2::Value1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev2::Value2)
    }
}
#[doc = "Clear Channel Event for Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cev3 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the channel event flag in GxCEFLAG"]
    Value2 = 1,
}
impl From<Cev3> for bool {
    #[inline(always)]
    fn from(variant: Cev3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV3` writer - Clear Channel Event for Channel 3"]
pub type Cev3W<'a, REG> = crate::BitWriter<'a, REG, Cev3>;
impl<'a, REG> Cev3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev3::Value1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev3::Value2)
    }
}
#[doc = "Clear Channel Event for Channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cev4 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the channel event flag in GxCEFLAG"]
    Value2 = 1,
}
impl From<Cev4> for bool {
    #[inline(always)]
    fn from(variant: Cev4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV4` writer - Clear Channel Event for Channel 4"]
pub type Cev4W<'a, REG> = crate::BitWriter<'a, REG, Cev4>;
impl<'a, REG> Cev4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev4::Value1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev4::Value2)
    }
}
#[doc = "Clear Channel Event for Channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cev5 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the channel event flag in GxCEFLAG"]
    Value2 = 1,
}
impl From<Cev5> for bool {
    #[inline(always)]
    fn from(variant: Cev5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV5` writer - Clear Channel Event for Channel 5"]
pub type Cev5W<'a, REG> = crate::BitWriter<'a, REG, Cev5>;
impl<'a, REG> Cev5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev5::Value1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev5::Value2)
    }
}
#[doc = "Clear Channel Event for Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cev6 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the channel event flag in GxCEFLAG"]
    Value2 = 1,
}
impl From<Cev6> for bool {
    #[inline(always)]
    fn from(variant: Cev6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV6` writer - Clear Channel Event for Channel 6"]
pub type Cev6W<'a, REG> = crate::BitWriter<'a, REG, Cev6>;
impl<'a, REG> Cev6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev6::Value1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev6::Value2)
    }
}
#[doc = "Clear Channel Event for Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cev7 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the channel event flag in GxCEFLAG"]
    Value2 = 1,
}
impl From<Cev7> for bool {
    #[inline(always)]
    fn from(variant: Cev7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV7` writer - Clear Channel Event for Channel 7"]
pub type Cev7W<'a, REG> = crate::BitWriter<'a, REG, Cev7>;
impl<'a, REG> Cev7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev7::Value1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev7::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Channel Event for Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn cev0(&mut self) -> Cev0W<CefclrSpec> {
        Cev0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Channel Event for Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn cev1(&mut self) -> Cev1W<CefclrSpec> {
        Cev1W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear Channel Event for Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn cev2(&mut self) -> Cev2W<CefclrSpec> {
        Cev2W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Channel Event for Channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn cev3(&mut self) -> Cev3W<CefclrSpec> {
        Cev3W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear Channel Event for Channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn cev4(&mut self) -> Cev4W<CefclrSpec> {
        Cev4W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear Channel Event for Channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn cev5(&mut self) -> Cev5W<CefclrSpec> {
        Cev5W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear Channel Event for Channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn cev6(&mut self) -> Cev6W<CefclrSpec> {
        Cev6W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear Channel Event for Channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn cev7(&mut self) -> Cev7W<CefclrSpec> {
        Cev7W::new(self, 7)
    }
}
#[doc = "Channel Event Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cefclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CefclrSpec;
impl crate::RegisterSpec for CefclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cefclr::W`](W) writer structure"]
impl crate::Writable for CefclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CEFCLR to value 0"]
impl crate::Resettable for CefclrSpec {
    const RESET_VALUE: u32 = 0;
}
