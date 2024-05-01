#[doc = "Register `REFCLR` writer"]
pub type W = crate::W<RefclrSpec>;
#[doc = "Clear Result Event for Result Register 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev0 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    Value2 = 1,
}
impl From<Rev0> for bool {
    #[inline(always)]
    fn from(variant: Rev0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV0` writer - Clear Result Event for Result Register 0"]
pub type Rev0W<'a, REG> = crate::BitWriter<'a, REG, Rev0>;
impl<'a, REG> Rev0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev0::Value1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev0::Value2)
    }
}
#[doc = "Clear Result Event for Result Register 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev1 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    Value2 = 1,
}
impl From<Rev1> for bool {
    #[inline(always)]
    fn from(variant: Rev1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV1` writer - Clear Result Event for Result Register 1"]
pub type Rev1W<'a, REG> = crate::BitWriter<'a, REG, Rev1>;
impl<'a, REG> Rev1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev1::Value1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev1::Value2)
    }
}
#[doc = "Clear Result Event for Result Register 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev2 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    Value2 = 1,
}
impl From<Rev2> for bool {
    #[inline(always)]
    fn from(variant: Rev2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV2` writer - Clear Result Event for Result Register 2"]
pub type Rev2W<'a, REG> = crate::BitWriter<'a, REG, Rev2>;
impl<'a, REG> Rev2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev2::Value1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev2::Value2)
    }
}
#[doc = "Clear Result Event for Result Register 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev3 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    Value2 = 1,
}
impl From<Rev3> for bool {
    #[inline(always)]
    fn from(variant: Rev3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV3` writer - Clear Result Event for Result Register 3"]
pub type Rev3W<'a, REG> = crate::BitWriter<'a, REG, Rev3>;
impl<'a, REG> Rev3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev3::Value1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev3::Value2)
    }
}
#[doc = "Clear Result Event for Result Register 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev4 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    Value2 = 1,
}
impl From<Rev4> for bool {
    #[inline(always)]
    fn from(variant: Rev4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV4` writer - Clear Result Event for Result Register 4"]
pub type Rev4W<'a, REG> = crate::BitWriter<'a, REG, Rev4>;
impl<'a, REG> Rev4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev4::Value1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev4::Value2)
    }
}
#[doc = "Clear Result Event for Result Register 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev5 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    Value2 = 1,
}
impl From<Rev5> for bool {
    #[inline(always)]
    fn from(variant: Rev5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV5` writer - Clear Result Event for Result Register 5"]
pub type Rev5W<'a, REG> = crate::BitWriter<'a, REG, Rev5>;
impl<'a, REG> Rev5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev5::Value1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev5::Value2)
    }
}
#[doc = "Clear Result Event for Result Register 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev6 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    Value2 = 1,
}
impl From<Rev6> for bool {
    #[inline(always)]
    fn from(variant: Rev6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV6` writer - Clear Result Event for Result Register 6"]
pub type Rev6W<'a, REG> = crate::BitWriter<'a, REG, Rev6>;
impl<'a, REG> Rev6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev6::Value1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev6::Value2)
    }
}
#[doc = "Clear Result Event for Result Register 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev7 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    Value2 = 1,
}
impl From<Rev7> for bool {
    #[inline(always)]
    fn from(variant: Rev7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV7` writer - Clear Result Event for Result Register 7"]
pub type Rev7W<'a, REG> = crate::BitWriter<'a, REG, Rev7>;
impl<'a, REG> Rev7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev7::Value1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev7::Value2)
    }
}
#[doc = "Clear Result Event for Result Register 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev8 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    Value2 = 1,
}
impl From<Rev8> for bool {
    #[inline(always)]
    fn from(variant: Rev8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV8` writer - Clear Result Event for Result Register 8"]
pub type Rev8W<'a, REG> = crate::BitWriter<'a, REG, Rev8>;
impl<'a, REG> Rev8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev8::Value1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev8::Value2)
    }
}
#[doc = "Clear Result Event for Result Register 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev9 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    Value2 = 1,
}
impl From<Rev9> for bool {
    #[inline(always)]
    fn from(variant: Rev9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV9` writer - Clear Result Event for Result Register 9"]
pub type Rev9W<'a, REG> = crate::BitWriter<'a, REG, Rev9>;
impl<'a, REG> Rev9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev9::Value1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev9::Value2)
    }
}
#[doc = "Clear Result Event for Result Register 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev10 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    Value2 = 1,
}
impl From<Rev10> for bool {
    #[inline(always)]
    fn from(variant: Rev10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV10` writer - Clear Result Event for Result Register 10"]
pub type Rev10W<'a, REG> = crate::BitWriter<'a, REG, Rev10>;
impl<'a, REG> Rev10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev10::Value1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev10::Value2)
    }
}
#[doc = "Clear Result Event for Result Register 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev11 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    Value2 = 1,
}
impl From<Rev11> for bool {
    #[inline(always)]
    fn from(variant: Rev11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV11` writer - Clear Result Event for Result Register 11"]
pub type Rev11W<'a, REG> = crate::BitWriter<'a, REG, Rev11>;
impl<'a, REG> Rev11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev11::Value1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev11::Value2)
    }
}
#[doc = "Clear Result Event for Result Register 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev12 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    Value2 = 1,
}
impl From<Rev12> for bool {
    #[inline(always)]
    fn from(variant: Rev12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV12` writer - Clear Result Event for Result Register 12"]
pub type Rev12W<'a, REG> = crate::BitWriter<'a, REG, Rev12>;
impl<'a, REG> Rev12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev12::Value1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev12::Value2)
    }
}
#[doc = "Clear Result Event for Result Register 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev13 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    Value2 = 1,
}
impl From<Rev13> for bool {
    #[inline(always)]
    fn from(variant: Rev13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV13` writer - Clear Result Event for Result Register 13"]
pub type Rev13W<'a, REG> = crate::BitWriter<'a, REG, Rev13>;
impl<'a, REG> Rev13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev13::Value1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev13::Value2)
    }
}
#[doc = "Clear Result Event for Result Register 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev14 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    Value2 = 1,
}
impl From<Rev14> for bool {
    #[inline(always)]
    fn from(variant: Rev14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV14` writer - Clear Result Event for Result Register 14"]
pub type Rev14W<'a, REG> = crate::BitWriter<'a, REG, Rev14>;
impl<'a, REG> Rev14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev14::Value1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev14::Value2)
    }
}
#[doc = "Clear Result Event for Result Register 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev15 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    Value2 = 1,
}
impl From<Rev15> for bool {
    #[inline(always)]
    fn from(variant: Rev15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV15` writer - Clear Result Event for Result Register 15"]
pub type Rev15W<'a, REG> = crate::BitWriter<'a, REG, Rev15>;
impl<'a, REG> Rev15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev15::Value1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev15::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Result Event for Result Register 0"]
    #[inline(always)]
    #[must_use]
    pub fn rev0(&mut self) -> Rev0W<RefclrSpec> {
        Rev0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Result Event for Result Register 1"]
    #[inline(always)]
    #[must_use]
    pub fn rev1(&mut self) -> Rev1W<RefclrSpec> {
        Rev1W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear Result Event for Result Register 2"]
    #[inline(always)]
    #[must_use]
    pub fn rev2(&mut self) -> Rev2W<RefclrSpec> {
        Rev2W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Result Event for Result Register 3"]
    #[inline(always)]
    #[must_use]
    pub fn rev3(&mut self) -> Rev3W<RefclrSpec> {
        Rev3W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear Result Event for Result Register 4"]
    #[inline(always)]
    #[must_use]
    pub fn rev4(&mut self) -> Rev4W<RefclrSpec> {
        Rev4W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear Result Event for Result Register 5"]
    #[inline(always)]
    #[must_use]
    pub fn rev5(&mut self) -> Rev5W<RefclrSpec> {
        Rev5W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear Result Event for Result Register 6"]
    #[inline(always)]
    #[must_use]
    pub fn rev6(&mut self) -> Rev6W<RefclrSpec> {
        Rev6W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear Result Event for Result Register 7"]
    #[inline(always)]
    #[must_use]
    pub fn rev7(&mut self) -> Rev7W<RefclrSpec> {
        Rev7W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear Result Event for Result Register 8"]
    #[inline(always)]
    #[must_use]
    pub fn rev8(&mut self) -> Rev8W<RefclrSpec> {
        Rev8W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear Result Event for Result Register 9"]
    #[inline(always)]
    #[must_use]
    pub fn rev9(&mut self) -> Rev9W<RefclrSpec> {
        Rev9W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear Result Event for Result Register 10"]
    #[inline(always)]
    #[must_use]
    pub fn rev10(&mut self) -> Rev10W<RefclrSpec> {
        Rev10W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear Result Event for Result Register 11"]
    #[inline(always)]
    #[must_use]
    pub fn rev11(&mut self) -> Rev11W<RefclrSpec> {
        Rev11W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear Result Event for Result Register 12"]
    #[inline(always)]
    #[must_use]
    pub fn rev12(&mut self) -> Rev12W<RefclrSpec> {
        Rev12W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear Result Event for Result Register 13"]
    #[inline(always)]
    #[must_use]
    pub fn rev13(&mut self) -> Rev13W<RefclrSpec> {
        Rev13W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear Result Event for Result Register 14"]
    #[inline(always)]
    #[must_use]
    pub fn rev14(&mut self) -> Rev14W<RefclrSpec> {
        Rev14W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear Result Event for Result Register 15"]
    #[inline(always)]
    #[must_use]
    pub fn rev15(&mut self) -> Rev15W<RefclrSpec> {
        Rev15W::new(self, 15)
    }
}
#[doc = "Result Event Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`refclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefclrSpec;
impl crate::RegisterSpec for RefclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`refclr::W`](W) writer structure"]
impl crate::Writable for RefclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REFCLR to value 0"]
impl crate::Resettable for RefclrSpec {
    const RESET_VALUE: u32 = 0;
}
