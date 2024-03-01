#[doc = "Register `REFLAG` reader"]
pub type R = crate::R<ReflagSpec>;
#[doc = "Register `REFLAG` writer"]
pub type W = crate::W<ReflagSpec>;
#[doc = "Result Event for Result Register 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev0 {
    #[doc = "0: No result event"]
    Value1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    Value2 = 1,
}
impl From<Rev0> for bool {
    #[inline(always)]
    fn from(variant: Rev0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV0` reader - Result Event for Result Register 0"]
pub type Rev0R = crate::BitReader<Rev0>;
impl Rev0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rev0 {
        match self.bits {
            false => Rev0::Value1,
            true => Rev0::Value2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev0::Value1
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev0::Value2
    }
}
#[doc = "Field `REV0` writer - Result Event for Result Register 0"]
pub type Rev0W<'a, REG> = crate::BitWriter<'a, REG, Rev0>;
impl<'a, REG> Rev0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev0::Value1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev0::Value2)
    }
}
#[doc = "Result Event for Result Register 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev1 {
    #[doc = "0: No result event"]
    Value1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    Value2 = 1,
}
impl From<Rev1> for bool {
    #[inline(always)]
    fn from(variant: Rev1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV1` reader - Result Event for Result Register 1"]
pub type Rev1R = crate::BitReader<Rev1>;
impl Rev1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rev1 {
        match self.bits {
            false => Rev1::Value1,
            true => Rev1::Value2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev1::Value1
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev1::Value2
    }
}
#[doc = "Field `REV1` writer - Result Event for Result Register 1"]
pub type Rev1W<'a, REG> = crate::BitWriter<'a, REG, Rev1>;
impl<'a, REG> Rev1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev1::Value1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev1::Value2)
    }
}
#[doc = "Result Event for Result Register 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev2 {
    #[doc = "0: No result event"]
    Value1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    Value2 = 1,
}
impl From<Rev2> for bool {
    #[inline(always)]
    fn from(variant: Rev2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV2` reader - Result Event for Result Register 2"]
pub type Rev2R = crate::BitReader<Rev2>;
impl Rev2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rev2 {
        match self.bits {
            false => Rev2::Value1,
            true => Rev2::Value2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev2::Value1
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev2::Value2
    }
}
#[doc = "Field `REV2` writer - Result Event for Result Register 2"]
pub type Rev2W<'a, REG> = crate::BitWriter<'a, REG, Rev2>;
impl<'a, REG> Rev2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev2::Value1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev2::Value2)
    }
}
#[doc = "Result Event for Result Register 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev3 {
    #[doc = "0: No result event"]
    Value1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    Value2 = 1,
}
impl From<Rev3> for bool {
    #[inline(always)]
    fn from(variant: Rev3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV3` reader - Result Event for Result Register 3"]
pub type Rev3R = crate::BitReader<Rev3>;
impl Rev3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rev3 {
        match self.bits {
            false => Rev3::Value1,
            true => Rev3::Value2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev3::Value1
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev3::Value2
    }
}
#[doc = "Field `REV3` writer - Result Event for Result Register 3"]
pub type Rev3W<'a, REG> = crate::BitWriter<'a, REG, Rev3>;
impl<'a, REG> Rev3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev3::Value1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev3::Value2)
    }
}
#[doc = "Result Event for Result Register 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev4 {
    #[doc = "0: No result event"]
    Value1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    Value2 = 1,
}
impl From<Rev4> for bool {
    #[inline(always)]
    fn from(variant: Rev4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV4` reader - Result Event for Result Register 4"]
pub type Rev4R = crate::BitReader<Rev4>;
impl Rev4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rev4 {
        match self.bits {
            false => Rev4::Value1,
            true => Rev4::Value2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev4::Value1
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev4::Value2
    }
}
#[doc = "Field `REV4` writer - Result Event for Result Register 4"]
pub type Rev4W<'a, REG> = crate::BitWriter<'a, REG, Rev4>;
impl<'a, REG> Rev4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev4::Value1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev4::Value2)
    }
}
#[doc = "Result Event for Result Register 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev5 {
    #[doc = "0: No result event"]
    Value1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    Value2 = 1,
}
impl From<Rev5> for bool {
    #[inline(always)]
    fn from(variant: Rev5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV5` reader - Result Event for Result Register 5"]
pub type Rev5R = crate::BitReader<Rev5>;
impl Rev5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rev5 {
        match self.bits {
            false => Rev5::Value1,
            true => Rev5::Value2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev5::Value1
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev5::Value2
    }
}
#[doc = "Field `REV5` writer - Result Event for Result Register 5"]
pub type Rev5W<'a, REG> = crate::BitWriter<'a, REG, Rev5>;
impl<'a, REG> Rev5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev5::Value1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev5::Value2)
    }
}
#[doc = "Result Event for Result Register 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev6 {
    #[doc = "0: No result event"]
    Value1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    Value2 = 1,
}
impl From<Rev6> for bool {
    #[inline(always)]
    fn from(variant: Rev6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV6` reader - Result Event for Result Register 6"]
pub type Rev6R = crate::BitReader<Rev6>;
impl Rev6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rev6 {
        match self.bits {
            false => Rev6::Value1,
            true => Rev6::Value2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev6::Value1
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev6::Value2
    }
}
#[doc = "Field `REV6` writer - Result Event for Result Register 6"]
pub type Rev6W<'a, REG> = crate::BitWriter<'a, REG, Rev6>;
impl<'a, REG> Rev6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev6::Value1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev6::Value2)
    }
}
#[doc = "Result Event for Result Register 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev7 {
    #[doc = "0: No result event"]
    Value1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    Value2 = 1,
}
impl From<Rev7> for bool {
    #[inline(always)]
    fn from(variant: Rev7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV7` reader - Result Event for Result Register 7"]
pub type Rev7R = crate::BitReader<Rev7>;
impl Rev7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rev7 {
        match self.bits {
            false => Rev7::Value1,
            true => Rev7::Value2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev7::Value1
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev7::Value2
    }
}
#[doc = "Field `REV7` writer - Result Event for Result Register 7"]
pub type Rev7W<'a, REG> = crate::BitWriter<'a, REG, Rev7>;
impl<'a, REG> Rev7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev7::Value1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev7::Value2)
    }
}
#[doc = "Result Event for Result Register 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev8 {
    #[doc = "0: No result event"]
    Value1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    Value2 = 1,
}
impl From<Rev8> for bool {
    #[inline(always)]
    fn from(variant: Rev8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV8` reader - Result Event for Result Register 8"]
pub type Rev8R = crate::BitReader<Rev8>;
impl Rev8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rev8 {
        match self.bits {
            false => Rev8::Value1,
            true => Rev8::Value2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev8::Value1
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev8::Value2
    }
}
#[doc = "Field `REV8` writer - Result Event for Result Register 8"]
pub type Rev8W<'a, REG> = crate::BitWriter<'a, REG, Rev8>;
impl<'a, REG> Rev8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev8::Value1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev8::Value2)
    }
}
#[doc = "Result Event for Result Register 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev9 {
    #[doc = "0: No result event"]
    Value1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    Value2 = 1,
}
impl From<Rev9> for bool {
    #[inline(always)]
    fn from(variant: Rev9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV9` reader - Result Event for Result Register 9"]
pub type Rev9R = crate::BitReader<Rev9>;
impl Rev9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rev9 {
        match self.bits {
            false => Rev9::Value1,
            true => Rev9::Value2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev9::Value1
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev9::Value2
    }
}
#[doc = "Field `REV9` writer - Result Event for Result Register 9"]
pub type Rev9W<'a, REG> = crate::BitWriter<'a, REG, Rev9>;
impl<'a, REG> Rev9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev9::Value1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev9::Value2)
    }
}
#[doc = "Result Event for Result Register 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev10 {
    #[doc = "0: No result event"]
    Value1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    Value2 = 1,
}
impl From<Rev10> for bool {
    #[inline(always)]
    fn from(variant: Rev10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV10` reader - Result Event for Result Register 10"]
pub type Rev10R = crate::BitReader<Rev10>;
impl Rev10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rev10 {
        match self.bits {
            false => Rev10::Value1,
            true => Rev10::Value2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev10::Value1
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev10::Value2
    }
}
#[doc = "Field `REV10` writer - Result Event for Result Register 10"]
pub type Rev10W<'a, REG> = crate::BitWriter<'a, REG, Rev10>;
impl<'a, REG> Rev10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev10::Value1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev10::Value2)
    }
}
#[doc = "Result Event for Result Register 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev11 {
    #[doc = "0: No result event"]
    Value1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    Value2 = 1,
}
impl From<Rev11> for bool {
    #[inline(always)]
    fn from(variant: Rev11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV11` reader - Result Event for Result Register 11"]
pub type Rev11R = crate::BitReader<Rev11>;
impl Rev11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rev11 {
        match self.bits {
            false => Rev11::Value1,
            true => Rev11::Value2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev11::Value1
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev11::Value2
    }
}
#[doc = "Field `REV11` writer - Result Event for Result Register 11"]
pub type Rev11W<'a, REG> = crate::BitWriter<'a, REG, Rev11>;
impl<'a, REG> Rev11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev11::Value1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev11::Value2)
    }
}
#[doc = "Result Event for Result Register 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev12 {
    #[doc = "0: No result event"]
    Value1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    Value2 = 1,
}
impl From<Rev12> for bool {
    #[inline(always)]
    fn from(variant: Rev12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV12` reader - Result Event for Result Register 12"]
pub type Rev12R = crate::BitReader<Rev12>;
impl Rev12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rev12 {
        match self.bits {
            false => Rev12::Value1,
            true => Rev12::Value2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev12::Value1
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev12::Value2
    }
}
#[doc = "Field `REV12` writer - Result Event for Result Register 12"]
pub type Rev12W<'a, REG> = crate::BitWriter<'a, REG, Rev12>;
impl<'a, REG> Rev12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev12::Value1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev12::Value2)
    }
}
#[doc = "Result Event for Result Register 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev13 {
    #[doc = "0: No result event"]
    Value1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    Value2 = 1,
}
impl From<Rev13> for bool {
    #[inline(always)]
    fn from(variant: Rev13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV13` reader - Result Event for Result Register 13"]
pub type Rev13R = crate::BitReader<Rev13>;
impl Rev13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rev13 {
        match self.bits {
            false => Rev13::Value1,
            true => Rev13::Value2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev13::Value1
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev13::Value2
    }
}
#[doc = "Field `REV13` writer - Result Event for Result Register 13"]
pub type Rev13W<'a, REG> = crate::BitWriter<'a, REG, Rev13>;
impl<'a, REG> Rev13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev13::Value1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev13::Value2)
    }
}
#[doc = "Result Event for Result Register 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev14 {
    #[doc = "0: No result event"]
    Value1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    Value2 = 1,
}
impl From<Rev14> for bool {
    #[inline(always)]
    fn from(variant: Rev14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV14` reader - Result Event for Result Register 14"]
pub type Rev14R = crate::BitReader<Rev14>;
impl Rev14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rev14 {
        match self.bits {
            false => Rev14::Value1,
            true => Rev14::Value2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev14::Value1
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev14::Value2
    }
}
#[doc = "Field `REV14` writer - Result Event for Result Register 14"]
pub type Rev14W<'a, REG> = crate::BitWriter<'a, REG, Rev14>;
impl<'a, REG> Rev14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev14::Value1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev14::Value2)
    }
}
#[doc = "Result Event for Result Register 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rev15 {
    #[doc = "0: No result event"]
    Value1 = 0,
    #[doc = "1: New result was stored in register GxRESy"]
    Value2 = 1,
}
impl From<Rev15> for bool {
    #[inline(always)]
    fn from(variant: Rev15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV15` reader - Result Event for Result Register 15"]
pub type Rev15R = crate::BitReader<Rev15>;
impl Rev15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rev15 {
        match self.bits {
            false => Rev15::Value1,
            true => Rev15::Value2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rev15::Value1
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rev15::Value2
    }
}
#[doc = "Field `REV15` writer - Result Event for Result Register 15"]
pub type Rev15W<'a, REG> = crate::BitWriter<'a, REG, Rev15>;
impl<'a, REG> Rev15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rev15::Value1)
    }
    #[doc = "New result was stored in register GxRESy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rev15::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Result Event for Result Register 0"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Result Event for Result Register 1"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Result Event for Result Register 2"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Result Event for Result Register 3"]
    #[inline(always)]
    pub fn rev3(&self) -> Rev3R {
        Rev3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Result Event for Result Register 4"]
    #[inline(always)]
    pub fn rev4(&self) -> Rev4R {
        Rev4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Result Event for Result Register 5"]
    #[inline(always)]
    pub fn rev5(&self) -> Rev5R {
        Rev5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Result Event for Result Register 6"]
    #[inline(always)]
    pub fn rev6(&self) -> Rev6R {
        Rev6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Result Event for Result Register 7"]
    #[inline(always)]
    pub fn rev7(&self) -> Rev7R {
        Rev7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Result Event for Result Register 8"]
    #[inline(always)]
    pub fn rev8(&self) -> Rev8R {
        Rev8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Result Event for Result Register 9"]
    #[inline(always)]
    pub fn rev9(&self) -> Rev9R {
        Rev9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Result Event for Result Register 10"]
    #[inline(always)]
    pub fn rev10(&self) -> Rev10R {
        Rev10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Result Event for Result Register 11"]
    #[inline(always)]
    pub fn rev11(&self) -> Rev11R {
        Rev11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Result Event for Result Register 12"]
    #[inline(always)]
    pub fn rev12(&self) -> Rev12R {
        Rev12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Result Event for Result Register 13"]
    #[inline(always)]
    pub fn rev13(&self) -> Rev13R {
        Rev13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Result Event for Result Register 14"]
    #[inline(always)]
    pub fn rev14(&self) -> Rev14R {
        Rev14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Result Event for Result Register 15"]
    #[inline(always)]
    pub fn rev15(&self) -> Rev15R {
        Rev15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Result Event for Result Register 0"]
    #[inline(always)]
    #[must_use]
    pub fn rev0(&mut self) -> Rev0W<ReflagSpec> {
        Rev0W::new(self, 0)
    }
    #[doc = "Bit 1 - Result Event for Result Register 1"]
    #[inline(always)]
    #[must_use]
    pub fn rev1(&mut self) -> Rev1W<ReflagSpec> {
        Rev1W::new(self, 1)
    }
    #[doc = "Bit 2 - Result Event for Result Register 2"]
    #[inline(always)]
    #[must_use]
    pub fn rev2(&mut self) -> Rev2W<ReflagSpec> {
        Rev2W::new(self, 2)
    }
    #[doc = "Bit 3 - Result Event for Result Register 3"]
    #[inline(always)]
    #[must_use]
    pub fn rev3(&mut self) -> Rev3W<ReflagSpec> {
        Rev3W::new(self, 3)
    }
    #[doc = "Bit 4 - Result Event for Result Register 4"]
    #[inline(always)]
    #[must_use]
    pub fn rev4(&mut self) -> Rev4W<ReflagSpec> {
        Rev4W::new(self, 4)
    }
    #[doc = "Bit 5 - Result Event for Result Register 5"]
    #[inline(always)]
    #[must_use]
    pub fn rev5(&mut self) -> Rev5W<ReflagSpec> {
        Rev5W::new(self, 5)
    }
    #[doc = "Bit 6 - Result Event for Result Register 6"]
    #[inline(always)]
    #[must_use]
    pub fn rev6(&mut self) -> Rev6W<ReflagSpec> {
        Rev6W::new(self, 6)
    }
    #[doc = "Bit 7 - Result Event for Result Register 7"]
    #[inline(always)]
    #[must_use]
    pub fn rev7(&mut self) -> Rev7W<ReflagSpec> {
        Rev7W::new(self, 7)
    }
    #[doc = "Bit 8 - Result Event for Result Register 8"]
    #[inline(always)]
    #[must_use]
    pub fn rev8(&mut self) -> Rev8W<ReflagSpec> {
        Rev8W::new(self, 8)
    }
    #[doc = "Bit 9 - Result Event for Result Register 9"]
    #[inline(always)]
    #[must_use]
    pub fn rev9(&mut self) -> Rev9W<ReflagSpec> {
        Rev9W::new(self, 9)
    }
    #[doc = "Bit 10 - Result Event for Result Register 10"]
    #[inline(always)]
    #[must_use]
    pub fn rev10(&mut self) -> Rev10W<ReflagSpec> {
        Rev10W::new(self, 10)
    }
    #[doc = "Bit 11 - Result Event for Result Register 11"]
    #[inline(always)]
    #[must_use]
    pub fn rev11(&mut self) -> Rev11W<ReflagSpec> {
        Rev11W::new(self, 11)
    }
    #[doc = "Bit 12 - Result Event for Result Register 12"]
    #[inline(always)]
    #[must_use]
    pub fn rev12(&mut self) -> Rev12W<ReflagSpec> {
        Rev12W::new(self, 12)
    }
    #[doc = "Bit 13 - Result Event for Result Register 13"]
    #[inline(always)]
    #[must_use]
    pub fn rev13(&mut self) -> Rev13W<ReflagSpec> {
        Rev13W::new(self, 13)
    }
    #[doc = "Bit 14 - Result Event for Result Register 14"]
    #[inline(always)]
    #[must_use]
    pub fn rev14(&mut self) -> Rev14W<ReflagSpec> {
        Rev14W::new(self, 14)
    }
    #[doc = "Bit 15 - Result Event for Result Register 15"]
    #[inline(always)]
    #[must_use]
    pub fn rev15(&mut self) -> Rev15W<ReflagSpec> {
        Rev15W::new(self, 15)
    }
}
#[doc = "Result Event Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReflagSpec;
impl crate::RegisterSpec for ReflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reflag::R`](R) reader structure"]
impl crate::Readable for ReflagSpec {}
#[doc = "`write(|w| ..)` method takes [`reflag::W`](W) writer structure"]
impl crate::Writable for ReflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REFLAG to value 0"]
impl crate::Resettable for ReflagSpec {
    const RESET_VALUE: u32 = 0;
}
