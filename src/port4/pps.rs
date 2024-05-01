#[doc = "Register `PPS` reader"]
pub type R = crate::R<PpsSpec>;
#[doc = "Register `PPS` writer"]
pub type W = crate::W<PpsSpec>;
#[doc = "Port n Pin Power Save Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pps0 {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    Const0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    Const1 = 1,
}
impl From<Pps0> for bool {
    #[inline(always)]
    fn from(variant: Pps0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS0` reader - Port n Pin Power Save Bit 0"]
pub type Pps0R = crate::BitReader<Pps0>;
impl Pps0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pps0 {
        match self.bits {
            false => Pps0::Const0,
            true => Pps0::Const1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pps0::Const0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pps0::Const1
    }
}
#[doc = "Field `PPS0` writer - Port n Pin Power Save Bit 0"]
pub type Pps0W<'a, REG> = crate::BitWriter<'a, REG, Pps0>;
impl<'a, REG> Pps0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pps0::Const0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pps0::Const1)
    }
}
#[doc = "Port n Pin Power Save Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pps1 {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    Const0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    Const1 = 1,
}
impl From<Pps1> for bool {
    #[inline(always)]
    fn from(variant: Pps1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS1` reader - Port n Pin Power Save Bit 1"]
pub type Pps1R = crate::BitReader<Pps1>;
impl Pps1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pps1 {
        match self.bits {
            false => Pps1::Const0,
            true => Pps1::Const1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pps1::Const0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pps1::Const1
    }
}
#[doc = "Field `PPS1` writer - Port n Pin Power Save Bit 1"]
pub type Pps1W<'a, REG> = crate::BitWriter<'a, REG, Pps1>;
impl<'a, REG> Pps1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pps1::Const0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pps1::Const1)
    }
}
#[doc = "Port n Pin Power Save Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pps2 {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    Const0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    Const1 = 1,
}
impl From<Pps2> for bool {
    #[inline(always)]
    fn from(variant: Pps2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS2` reader - Port n Pin Power Save Bit 2"]
pub type Pps2R = crate::BitReader<Pps2>;
impl Pps2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pps2 {
        match self.bits {
            false => Pps2::Const0,
            true => Pps2::Const1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pps2::Const0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pps2::Const1
    }
}
#[doc = "Field `PPS2` writer - Port n Pin Power Save Bit 2"]
pub type Pps2W<'a, REG> = crate::BitWriter<'a, REG, Pps2>;
impl<'a, REG> Pps2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pps2::Const0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pps2::Const1)
    }
}
#[doc = "Port n Pin Power Save Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pps3 {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    Const0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    Const1 = 1,
}
impl From<Pps3> for bool {
    #[inline(always)]
    fn from(variant: Pps3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS3` reader - Port n Pin Power Save Bit 3"]
pub type Pps3R = crate::BitReader<Pps3>;
impl Pps3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pps3 {
        match self.bits {
            false => Pps3::Const0,
            true => Pps3::Const1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pps3::Const0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pps3::Const1
    }
}
#[doc = "Field `PPS3` writer - Port n Pin Power Save Bit 3"]
pub type Pps3W<'a, REG> = crate::BitWriter<'a, REG, Pps3>;
impl<'a, REG> Pps3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pps3::Const0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pps3::Const1)
    }
}
#[doc = "Port n Pin Power Save Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pps4 {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    Const0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    Const1 = 1,
}
impl From<Pps4> for bool {
    #[inline(always)]
    fn from(variant: Pps4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS4` reader - Port n Pin Power Save Bit 4"]
pub type Pps4R = crate::BitReader<Pps4>;
impl Pps4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pps4 {
        match self.bits {
            false => Pps4::Const0,
            true => Pps4::Const1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pps4::Const0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pps4::Const1
    }
}
#[doc = "Field `PPS4` writer - Port n Pin Power Save Bit 4"]
pub type Pps4W<'a, REG> = crate::BitWriter<'a, REG, Pps4>;
impl<'a, REG> Pps4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pps4::Const0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pps4::Const1)
    }
}
#[doc = "Port n Pin Power Save Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pps5 {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    Const0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    Const1 = 1,
}
impl From<Pps5> for bool {
    #[inline(always)]
    fn from(variant: Pps5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS5` reader - Port n Pin Power Save Bit 5"]
pub type Pps5R = crate::BitReader<Pps5>;
impl Pps5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pps5 {
        match self.bits {
            false => Pps5::Const0,
            true => Pps5::Const1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pps5::Const0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pps5::Const1
    }
}
#[doc = "Field `PPS5` writer - Port n Pin Power Save Bit 5"]
pub type Pps5W<'a, REG> = crate::BitWriter<'a, REG, Pps5>;
impl<'a, REG> Pps5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pps5::Const0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pps5::Const1)
    }
}
#[doc = "Port n Pin Power Save Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pps6 {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    Const0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    Const1 = 1,
}
impl From<Pps6> for bool {
    #[inline(always)]
    fn from(variant: Pps6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS6` reader - Port n Pin Power Save Bit 6"]
pub type Pps6R = crate::BitReader<Pps6>;
impl Pps6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pps6 {
        match self.bits {
            false => Pps6::Const0,
            true => Pps6::Const1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pps6::Const0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pps6::Const1
    }
}
#[doc = "Field `PPS6` writer - Port n Pin Power Save Bit 6"]
pub type Pps6W<'a, REG> = crate::BitWriter<'a, REG, Pps6>;
impl<'a, REG> Pps6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pps6::Const0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pps6::Const1)
    }
}
#[doc = "Port n Pin Power Save Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pps7 {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    Const0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    Const1 = 1,
}
impl From<Pps7> for bool {
    #[inline(always)]
    fn from(variant: Pps7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS7` reader - Port n Pin Power Save Bit 7"]
pub type Pps7R = crate::BitReader<Pps7>;
impl Pps7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pps7 {
        match self.bits {
            false => Pps7::Const0,
            true => Pps7::Const1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pps7::Const0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pps7::Const1
    }
}
#[doc = "Field `PPS7` writer - Port n Pin Power Save Bit 7"]
pub type Pps7W<'a, REG> = crate::BitWriter<'a, REG, Pps7>;
impl<'a, REG> Pps7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pps7::Const0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pps7::Const1)
    }
}
#[doc = "Port n Pin Power Save Bit 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pps8 {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    Const0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    Const1 = 1,
}
impl From<Pps8> for bool {
    #[inline(always)]
    fn from(variant: Pps8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS8` reader - Port n Pin Power Save Bit 8"]
pub type Pps8R = crate::BitReader<Pps8>;
impl Pps8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pps8 {
        match self.bits {
            false => Pps8::Const0,
            true => Pps8::Const1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pps8::Const0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pps8::Const1
    }
}
#[doc = "Field `PPS8` writer - Port n Pin Power Save Bit 8"]
pub type Pps8W<'a, REG> = crate::BitWriter<'a, REG, Pps8>;
impl<'a, REG> Pps8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pps8::Const0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pps8::Const1)
    }
}
#[doc = "Port n Pin Power Save Bit 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pps9 {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    Const0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    Const1 = 1,
}
impl From<Pps9> for bool {
    #[inline(always)]
    fn from(variant: Pps9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS9` reader - Port n Pin Power Save Bit 9"]
pub type Pps9R = crate::BitReader<Pps9>;
impl Pps9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pps9 {
        match self.bits {
            false => Pps9::Const0,
            true => Pps9::Const1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pps9::Const0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pps9::Const1
    }
}
#[doc = "Field `PPS9` writer - Port n Pin Power Save Bit 9"]
pub type Pps9W<'a, REG> = crate::BitWriter<'a, REG, Pps9>;
impl<'a, REG> Pps9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pps9::Const0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pps9::Const1)
    }
}
#[doc = "Port n Pin Power Save Bit 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pps10 {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    Const0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    Const1 = 1,
}
impl From<Pps10> for bool {
    #[inline(always)]
    fn from(variant: Pps10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS10` reader - Port n Pin Power Save Bit 10"]
pub type Pps10R = crate::BitReader<Pps10>;
impl Pps10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pps10 {
        match self.bits {
            false => Pps10::Const0,
            true => Pps10::Const1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pps10::Const0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pps10::Const1
    }
}
#[doc = "Field `PPS10` writer - Port n Pin Power Save Bit 10"]
pub type Pps10W<'a, REG> = crate::BitWriter<'a, REG, Pps10>;
impl<'a, REG> Pps10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pps10::Const0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pps10::Const1)
    }
}
#[doc = "Port n Pin Power Save Bit 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pps11 {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    Const0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    Const1 = 1,
}
impl From<Pps11> for bool {
    #[inline(always)]
    fn from(variant: Pps11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS11` reader - Port n Pin Power Save Bit 11"]
pub type Pps11R = crate::BitReader<Pps11>;
impl Pps11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pps11 {
        match self.bits {
            false => Pps11::Const0,
            true => Pps11::Const1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pps11::Const0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pps11::Const1
    }
}
#[doc = "Field `PPS11` writer - Port n Pin Power Save Bit 11"]
pub type Pps11W<'a, REG> = crate::BitWriter<'a, REG, Pps11>;
impl<'a, REG> Pps11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pps11::Const0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pps11::Const1)
    }
}
#[doc = "Port n Pin Power Save Bit 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pps12 {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    Const0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    Const1 = 1,
}
impl From<Pps12> for bool {
    #[inline(always)]
    fn from(variant: Pps12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS12` reader - Port n Pin Power Save Bit 12"]
pub type Pps12R = crate::BitReader<Pps12>;
impl Pps12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pps12 {
        match self.bits {
            false => Pps12::Const0,
            true => Pps12::Const1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pps12::Const0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pps12::Const1
    }
}
#[doc = "Field `PPS12` writer - Port n Pin Power Save Bit 12"]
pub type Pps12W<'a, REG> = crate::BitWriter<'a, REG, Pps12>;
impl<'a, REG> Pps12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pps12::Const0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pps12::Const1)
    }
}
#[doc = "Port n Pin Power Save Bit 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pps13 {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    Const0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    Const1 = 1,
}
impl From<Pps13> for bool {
    #[inline(always)]
    fn from(variant: Pps13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS13` reader - Port n Pin Power Save Bit 13"]
pub type Pps13R = crate::BitReader<Pps13>;
impl Pps13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pps13 {
        match self.bits {
            false => Pps13::Const0,
            true => Pps13::Const1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pps13::Const0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pps13::Const1
    }
}
#[doc = "Field `PPS13` writer - Port n Pin Power Save Bit 13"]
pub type Pps13W<'a, REG> = crate::BitWriter<'a, REG, Pps13>;
impl<'a, REG> Pps13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pps13::Const0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pps13::Const1)
    }
}
#[doc = "Port n Pin Power Save Bit 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pps14 {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    Const0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    Const1 = 1,
}
impl From<Pps14> for bool {
    #[inline(always)]
    fn from(variant: Pps14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS14` reader - Port n Pin Power Save Bit 14"]
pub type Pps14R = crate::BitReader<Pps14>;
impl Pps14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pps14 {
        match self.bits {
            false => Pps14::Const0,
            true => Pps14::Const1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pps14::Const0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pps14::Const1
    }
}
#[doc = "Field `PPS14` writer - Port n Pin Power Save Bit 14"]
pub type Pps14W<'a, REG> = crate::BitWriter<'a, REG, Pps14>;
impl<'a, REG> Pps14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pps14::Const0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pps14::Const1)
    }
}
#[doc = "Port n Pin Power Save Bit 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pps15 {
    #[doc = "0: Pin Power Save of Pn.x is disabled."]
    Const0 = 0,
    #[doc = "1: Pin Power Save of Pn.x is enabled."]
    Const1 = 1,
}
impl From<Pps15> for bool {
    #[inline(always)]
    fn from(variant: Pps15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS15` reader - Port n Pin Power Save Bit 15"]
pub type Pps15R = crate::BitReader<Pps15>;
impl Pps15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pps15 {
        match self.bits {
            false => Pps15::Const0,
            true => Pps15::Const1,
        }
    }
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pps15::Const0
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pps15::Const1
    }
}
#[doc = "Field `PPS15` writer - Port n Pin Power Save Bit 15"]
pub type Pps15W<'a, REG> = crate::BitWriter<'a, REG, Pps15>;
impl<'a, REG> Pps15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Power Save of Pn.x is disabled."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pps15::Const0)
    }
    #[doc = "Pin Power Save of Pn.x is enabled."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pps15::Const1)
    }
}
impl R {
    #[doc = "Bit 0 - Port n Pin Power Save Bit 0"]
    #[inline(always)]
    pub fn pps0(&self) -> Pps0R {
        Pps0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port n Pin Power Save Bit 1"]
    #[inline(always)]
    pub fn pps1(&self) -> Pps1R {
        Pps1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port n Pin Power Save Bit 2"]
    #[inline(always)]
    pub fn pps2(&self) -> Pps2R {
        Pps2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port n Pin Power Save Bit 3"]
    #[inline(always)]
    pub fn pps3(&self) -> Pps3R {
        Pps3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port n Pin Power Save Bit 4"]
    #[inline(always)]
    pub fn pps4(&self) -> Pps4R {
        Pps4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port n Pin Power Save Bit 5"]
    #[inline(always)]
    pub fn pps5(&self) -> Pps5R {
        Pps5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port n Pin Power Save Bit 6"]
    #[inline(always)]
    pub fn pps6(&self) -> Pps6R {
        Pps6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port n Pin Power Save Bit 7"]
    #[inline(always)]
    pub fn pps7(&self) -> Pps7R {
        Pps7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port n Pin Power Save Bit 8"]
    #[inline(always)]
    pub fn pps8(&self) -> Pps8R {
        Pps8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port n Pin Power Save Bit 9"]
    #[inline(always)]
    pub fn pps9(&self) -> Pps9R {
        Pps9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port n Pin Power Save Bit 10"]
    #[inline(always)]
    pub fn pps10(&self) -> Pps10R {
        Pps10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port n Pin Power Save Bit 11"]
    #[inline(always)]
    pub fn pps11(&self) -> Pps11R {
        Pps11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port n Pin Power Save Bit 12"]
    #[inline(always)]
    pub fn pps12(&self) -> Pps12R {
        Pps12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port n Pin Power Save Bit 13"]
    #[inline(always)]
    pub fn pps13(&self) -> Pps13R {
        Pps13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port n Pin Power Save Bit 14"]
    #[inline(always)]
    pub fn pps14(&self) -> Pps14R {
        Pps14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port n Pin Power Save Bit 15"]
    #[inline(always)]
    pub fn pps15(&self) -> Pps15R {
        Pps15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port n Pin Power Save Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pps0(&mut self) -> Pps0W<PpsSpec> {
        Pps0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port n Pin Power Save Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pps1(&mut self) -> Pps1W<PpsSpec> {
        Pps1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port n Pin Power Save Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pps2(&mut self) -> Pps2W<PpsSpec> {
        Pps2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port n Pin Power Save Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pps3(&mut self) -> Pps3W<PpsSpec> {
        Pps3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port n Pin Power Save Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pps4(&mut self) -> Pps4W<PpsSpec> {
        Pps4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port n Pin Power Save Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pps5(&mut self) -> Pps5W<PpsSpec> {
        Pps5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port n Pin Power Save Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pps6(&mut self) -> Pps6W<PpsSpec> {
        Pps6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port n Pin Power Save Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pps7(&mut self) -> Pps7W<PpsSpec> {
        Pps7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port n Pin Power Save Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pps8(&mut self) -> Pps8W<PpsSpec> {
        Pps8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port n Pin Power Save Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pps9(&mut self) -> Pps9W<PpsSpec> {
        Pps9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port n Pin Power Save Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pps10(&mut self) -> Pps10W<PpsSpec> {
        Pps10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port n Pin Power Save Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pps11(&mut self) -> Pps11W<PpsSpec> {
        Pps11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port n Pin Power Save Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn pps12(&mut self) -> Pps12W<PpsSpec> {
        Pps12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port n Pin Power Save Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn pps13(&mut self) -> Pps13W<PpsSpec> {
        Pps13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port n Pin Power Save Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pps14(&mut self) -> Pps14W<PpsSpec> {
        Pps14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port n Pin Power Save Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn pps15(&mut self) -> Pps15W<PpsSpec> {
        Pps15W::new(self, 15)
    }
}
#[doc = "Port 4 Pin Power Save Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pps::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pps::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpsSpec;
impl crate::RegisterSpec for PpsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pps::R`](R) reader structure"]
impl crate::Readable for PpsSpec {}
#[doc = "`write(|w| ..)` method takes [`pps::W`](W) writer structure"]
impl crate::Writable for PpsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PPS to value 0"]
impl crate::Resettable for PpsSpec {
    const RESET_VALUE: u32 = 0;
}
