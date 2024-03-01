#[doc = "Register `LSTDSTREG` reader"]
pub type R = crate::R<LstdstregSpec>;
#[doc = "Register `LSTDSTREG` writer"]
pub type W = crate::W<LstdstregSpec>;
#[doc = "Destination last request for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0 {
    #[doc = "0: Not last transaction in current block"]
    Value1 = 0,
    #[doc = "1: Last transaction in current block"]
    Value2 = 1,
}
impl From<Ch0> for bool {
    #[inline(always)]
    fn from(variant: Ch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0` reader - Destination last request for channel 0"]
pub type Ch0R = crate::BitReader<Ch0>;
impl Ch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0 {
        match self.bits {
            false => Ch0::Value1,
            true => Ch0::Value2,
        }
    }
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ch0::Value1
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ch0::Value2
    }
}
#[doc = "Field `CH0` writer - Destination last request for channel 0"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG, Ch0>;
impl<'a, REG> Ch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Value1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Value2)
    }
}
#[doc = "Destination last request for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1 {
    #[doc = "0: Not last transaction in current block"]
    Value1 = 0,
    #[doc = "1: Last transaction in current block"]
    Value2 = 1,
}
impl From<Ch1> for bool {
    #[inline(always)]
    fn from(variant: Ch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1` reader - Destination last request for channel 1"]
pub type Ch1R = crate::BitReader<Ch1>;
impl Ch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1 {
        match self.bits {
            false => Ch1::Value1,
            true => Ch1::Value2,
        }
    }
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ch1::Value1
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ch1::Value2
    }
}
#[doc = "Field `CH1` writer - Destination last request for channel 1"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG, Ch1>;
impl<'a, REG> Ch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1::Value1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1::Value2)
    }
}
#[doc = "Destination last request for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2 {
    #[doc = "0: Not last transaction in current block"]
    Value1 = 0,
    #[doc = "1: Last transaction in current block"]
    Value2 = 1,
}
impl From<Ch2> for bool {
    #[inline(always)]
    fn from(variant: Ch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2` reader - Destination last request for channel 2"]
pub type Ch2R = crate::BitReader<Ch2>;
impl Ch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2 {
        match self.bits {
            false => Ch2::Value1,
            true => Ch2::Value2,
        }
    }
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ch2::Value1
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ch2::Value2
    }
}
#[doc = "Field `CH2` writer - Destination last request for channel 2"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG, Ch2>;
impl<'a, REG> Ch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2::Value1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2::Value2)
    }
}
#[doc = "Destination last request for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3 {
    #[doc = "0: Not last transaction in current block"]
    Value1 = 0,
    #[doc = "1: Last transaction in current block"]
    Value2 = 1,
}
impl From<Ch3> for bool {
    #[inline(always)]
    fn from(variant: Ch3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3` reader - Destination last request for channel 3"]
pub type Ch3R = crate::BitReader<Ch3>;
impl Ch3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3 {
        match self.bits {
            false => Ch3::Value1,
            true => Ch3::Value2,
        }
    }
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ch3::Value1
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ch3::Value2
    }
}
#[doc = "Field `CH3` writer - Destination last request for channel 3"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG, Ch3>;
impl<'a, REG> Ch3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3::Value1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3::Value2)
    }
}
#[doc = "Destination last request for channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch4 {
    #[doc = "0: Not last transaction in current block"]
    Value1 = 0,
    #[doc = "1: Last transaction in current block"]
    Value2 = 1,
}
impl From<Ch4> for bool {
    #[inline(always)]
    fn from(variant: Ch4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4` reader - Destination last request for channel 4"]
pub type Ch4R = crate::BitReader<Ch4>;
impl Ch4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch4 {
        match self.bits {
            false => Ch4::Value1,
            true => Ch4::Value2,
        }
    }
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ch4::Value1
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ch4::Value2
    }
}
#[doc = "Field `CH4` writer - Destination last request for channel 4"]
pub type Ch4W<'a, REG> = crate::BitWriter<'a, REG, Ch4>;
impl<'a, REG> Ch4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4::Value1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4::Value2)
    }
}
#[doc = "Destination last request for channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch5 {
    #[doc = "0: Not last transaction in current block"]
    Value1 = 0,
    #[doc = "1: Last transaction in current block"]
    Value2 = 1,
}
impl From<Ch5> for bool {
    #[inline(always)]
    fn from(variant: Ch5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5` reader - Destination last request for channel 5"]
pub type Ch5R = crate::BitReader<Ch5>;
impl Ch5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch5 {
        match self.bits {
            false => Ch5::Value1,
            true => Ch5::Value2,
        }
    }
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ch5::Value1
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ch5::Value2
    }
}
#[doc = "Field `CH5` writer - Destination last request for channel 5"]
pub type Ch5W<'a, REG> = crate::BitWriter<'a, REG, Ch5>;
impl<'a, REG> Ch5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5::Value1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5::Value2)
    }
}
#[doc = "Destination last request for channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch6 {
    #[doc = "0: Not last transaction in current block"]
    Value1 = 0,
    #[doc = "1: Last transaction in current block"]
    Value2 = 1,
}
impl From<Ch6> for bool {
    #[inline(always)]
    fn from(variant: Ch6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6` reader - Destination last request for channel 6"]
pub type Ch6R = crate::BitReader<Ch6>;
impl Ch6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch6 {
        match self.bits {
            false => Ch6::Value1,
            true => Ch6::Value2,
        }
    }
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ch6::Value1
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ch6::Value2
    }
}
#[doc = "Field `CH6` writer - Destination last request for channel 6"]
pub type Ch6W<'a, REG> = crate::BitWriter<'a, REG, Ch6>;
impl<'a, REG> Ch6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6::Value1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6::Value2)
    }
}
#[doc = "Destination last request for channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch7 {
    #[doc = "0: Not last transaction in current block"]
    Value1 = 0,
    #[doc = "1: Last transaction in current block"]
    Value2 = 1,
}
impl From<Ch7> for bool {
    #[inline(always)]
    fn from(variant: Ch7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7` reader - Destination last request for channel 7"]
pub type Ch7R = crate::BitReader<Ch7>;
impl Ch7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch7 {
        match self.bits {
            false => Ch7::Value1,
            true => Ch7::Value2,
        }
    }
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ch7::Value1
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ch7::Value2
    }
}
#[doc = "Field `CH7` writer - Destination last request for channel 7"]
pub type Ch7W<'a, REG> = crate::BitWriter<'a, REG, Ch7>;
impl<'a, REG> Ch7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7::Value1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7::Value2)
    }
}
#[doc = "Destination last transaction request write enable for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeCh0 {
    #[doc = "0: write disabled"]
    Value1 = 0,
    #[doc = "1: write enabled"]
    Value2 = 1,
}
impl From<WeCh0> for bool {
    #[inline(always)]
    fn from(variant: WeCh0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH0` writer - Destination last transaction request write enable for channel 0"]
pub type WeCh0W<'a, REG> = crate::BitWriter<'a, REG, WeCh0>;
impl<'a, REG> WeCh0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WeCh0::Value1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WeCh0::Value2)
    }
}
#[doc = "Destination last transaction request write enable for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeCh1 {
    #[doc = "0: write disabled"]
    Value1 = 0,
    #[doc = "1: write enabled"]
    Value2 = 1,
}
impl From<WeCh1> for bool {
    #[inline(always)]
    fn from(variant: WeCh1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH1` writer - Destination last transaction request write enable for channel 1"]
pub type WeCh1W<'a, REG> = crate::BitWriter<'a, REG, WeCh1>;
impl<'a, REG> WeCh1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WeCh1::Value1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WeCh1::Value2)
    }
}
#[doc = "Destination last transaction request write enable for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeCh2 {
    #[doc = "0: write disabled"]
    Value1 = 0,
    #[doc = "1: write enabled"]
    Value2 = 1,
}
impl From<WeCh2> for bool {
    #[inline(always)]
    fn from(variant: WeCh2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH2` writer - Destination last transaction request write enable for channel 2"]
pub type WeCh2W<'a, REG> = crate::BitWriter<'a, REG, WeCh2>;
impl<'a, REG> WeCh2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WeCh2::Value1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WeCh2::Value2)
    }
}
#[doc = "Destination last transaction request write enable for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeCh3 {
    #[doc = "0: write disabled"]
    Value1 = 0,
    #[doc = "1: write enabled"]
    Value2 = 1,
}
impl From<WeCh3> for bool {
    #[inline(always)]
    fn from(variant: WeCh3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH3` writer - Destination last transaction request write enable for channel 3"]
pub type WeCh3W<'a, REG> = crate::BitWriter<'a, REG, WeCh3>;
impl<'a, REG> WeCh3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WeCh3::Value1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WeCh3::Value2)
    }
}
#[doc = "Destination last transaction request write enable for channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeCh4 {
    #[doc = "0: write disabled"]
    Value1 = 0,
    #[doc = "1: write enabled"]
    Value2 = 1,
}
impl From<WeCh4> for bool {
    #[inline(always)]
    fn from(variant: WeCh4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH4` writer - Destination last transaction request write enable for channel 4"]
pub type WeCh4W<'a, REG> = crate::BitWriter<'a, REG, WeCh4>;
impl<'a, REG> WeCh4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WeCh4::Value1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WeCh4::Value2)
    }
}
#[doc = "Destination last transaction request write enable for channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeCh5 {
    #[doc = "0: write disabled"]
    Value1 = 0,
    #[doc = "1: write enabled"]
    Value2 = 1,
}
impl From<WeCh5> for bool {
    #[inline(always)]
    fn from(variant: WeCh5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH5` writer - Destination last transaction request write enable for channel 5"]
pub type WeCh5W<'a, REG> = crate::BitWriter<'a, REG, WeCh5>;
impl<'a, REG> WeCh5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WeCh5::Value1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WeCh5::Value2)
    }
}
#[doc = "Destination last transaction request write enable for channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeCh6 {
    #[doc = "0: write disabled"]
    Value1 = 0,
    #[doc = "1: write enabled"]
    Value2 = 1,
}
impl From<WeCh6> for bool {
    #[inline(always)]
    fn from(variant: WeCh6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH6` writer - Destination last transaction request write enable for channel 6"]
pub type WeCh6W<'a, REG> = crate::BitWriter<'a, REG, WeCh6>;
impl<'a, REG> WeCh6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WeCh6::Value1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WeCh6::Value2)
    }
}
#[doc = "Destination last transaction request write enable for channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeCh7 {
    #[doc = "0: write disabled"]
    Value1 = 0,
    #[doc = "1: write enabled"]
    Value2 = 1,
}
impl From<WeCh7> for bool {
    #[inline(always)]
    fn from(variant: WeCh7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH7` writer - Destination last transaction request write enable for channel 7"]
pub type WeCh7W<'a, REG> = crate::BitWriter<'a, REG, WeCh7>;
impl<'a, REG> WeCh7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WeCh7::Value1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WeCh7::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Destination last request for channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Destination last request for channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Destination last request for channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Destination last request for channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Destination last request for channel 4"]
    #[inline(always)]
    pub fn ch4(&self) -> Ch4R {
        Ch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Destination last request for channel 5"]
    #[inline(always)]
    pub fn ch5(&self) -> Ch5R {
        Ch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Destination last request for channel 6"]
    #[inline(always)]
    pub fn ch6(&self) -> Ch6R {
        Ch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Destination last request for channel 7"]
    #[inline(always)]
    pub fn ch7(&self) -> Ch7R {
        Ch7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Destination last request for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> Ch0W<LstdstregSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Destination last request for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> Ch1W<LstdstregSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Destination last request for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> Ch2W<LstdstregSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Destination last request for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> Ch3W<LstdstregSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Destination last request for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> Ch4W<LstdstregSpec> {
        Ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Destination last request for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> Ch5W<LstdstregSpec> {
        Ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Destination last request for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> Ch6W<LstdstregSpec> {
        Ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Destination last request for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> Ch7W<LstdstregSpec> {
        Ch7W::new(self, 7)
    }
    #[doc = "Bit 8 - Destination last transaction request write enable for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch0(&mut self) -> WeCh0W<LstdstregSpec> {
        WeCh0W::new(self, 8)
    }
    #[doc = "Bit 9 - Destination last transaction request write enable for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch1(&mut self) -> WeCh1W<LstdstregSpec> {
        WeCh1W::new(self, 9)
    }
    #[doc = "Bit 10 - Destination last transaction request write enable for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch2(&mut self) -> WeCh2W<LstdstregSpec> {
        WeCh2W::new(self, 10)
    }
    #[doc = "Bit 11 - Destination last transaction request write enable for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch3(&mut self) -> WeCh3W<LstdstregSpec> {
        WeCh3W::new(self, 11)
    }
    #[doc = "Bit 12 - Destination last transaction request write enable for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch4(&mut self) -> WeCh4W<LstdstregSpec> {
        WeCh4W::new(self, 12)
    }
    #[doc = "Bit 13 - Destination last transaction request write enable for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch5(&mut self) -> WeCh5W<LstdstregSpec> {
        WeCh5W::new(self, 13)
    }
    #[doc = "Bit 14 - Destination last transaction request write enable for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch6(&mut self) -> WeCh6W<LstdstregSpec> {
        WeCh6W::new(self, 14)
    }
    #[doc = "Bit 15 - Destination last transaction request write enable for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch7(&mut self) -> WeCh7W<LstdstregSpec> {
        WeCh7W::new(self, 15)
    }
}
#[doc = "Last Destination Transaction Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lstdstreg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lstdstreg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LstdstregSpec;
impl crate::RegisterSpec for LstdstregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lstdstreg::R`](R) reader structure"]
impl crate::Readable for LstdstregSpec {}
#[doc = "`write(|w| ..)` method takes [`lstdstreg::W`](W) writer structure"]
impl crate::Writable for LstdstregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSTDSTREG to value 0"]
impl crate::Resettable for LstdstregSpec {
    const RESET_VALUE: u32 = 0;
}
