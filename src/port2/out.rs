#[doc = "Register `OUT` reader"]
pub type R = crate::R<OutSpec>;
#[doc = "Register `OUT` writer"]
pub type W = crate::W<OutSpec>;
#[doc = "Port n Output Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0 {
    #[doc = "0: The output level of Pn.x is 0."]
    Const0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    Const1 = 1,
}
impl From<P0> for bool {
    #[inline(always)]
    fn from(variant: P0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0` reader - Port n Output Bit 0\n\nThe field is **modified** in some way after a read operation."]
pub type P0R = crate::BitReader<P0>;
impl P0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0 {
        match self.bits {
            false => P0::Const0,
            true => P0::Const1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P0::Const0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P0::Const1
    }
}
#[doc = "Field `P0` writer - Port n Output Bit 0"]
pub type P0W<'a, REG> = crate::BitWriter<'a, REG, P0>;
impl<'a, REG> P0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P0::Const0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P0::Const1)
    }
}
#[doc = "Port n Output Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1 {
    #[doc = "0: The output level of Pn.x is 0."]
    Const0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    Const1 = 1,
}
impl From<P1> for bool {
    #[inline(always)]
    fn from(variant: P1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1` reader - Port n Output Bit 1\n\nThe field is **modified** in some way after a read operation."]
pub type P1R = crate::BitReader<P1>;
impl P1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1 {
        match self.bits {
            false => P1::Const0,
            true => P1::Const1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P1::Const0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P1::Const1
    }
}
#[doc = "Field `P1` writer - Port n Output Bit 1"]
pub type P1W<'a, REG> = crate::BitWriter<'a, REG, P1>;
impl<'a, REG> P1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P1::Const0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P1::Const1)
    }
}
#[doc = "Port n Output Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2 {
    #[doc = "0: The output level of Pn.x is 0."]
    Const0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    Const1 = 1,
}
impl From<P2> for bool {
    #[inline(always)]
    fn from(variant: P2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2` reader - Port n Output Bit 2\n\nThe field is **modified** in some way after a read operation."]
pub type P2R = crate::BitReader<P2>;
impl P2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2 {
        match self.bits {
            false => P2::Const0,
            true => P2::Const1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P2::Const0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P2::Const1
    }
}
#[doc = "Field `P2` writer - Port n Output Bit 2"]
pub type P2W<'a, REG> = crate::BitWriter<'a, REG, P2>;
impl<'a, REG> P2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P2::Const0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P2::Const1)
    }
}
#[doc = "Port n Output Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P3 {
    #[doc = "0: The output level of Pn.x is 0."]
    Const0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    Const1 = 1,
}
impl From<P3> for bool {
    #[inline(always)]
    fn from(variant: P3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P3` reader - Port n Output Bit 3\n\nThe field is **modified** in some way after a read operation."]
pub type P3R = crate::BitReader<P3>;
impl P3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P3 {
        match self.bits {
            false => P3::Const0,
            true => P3::Const1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P3::Const0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P3::Const1
    }
}
#[doc = "Field `P3` writer - Port n Output Bit 3"]
pub type P3W<'a, REG> = crate::BitWriter<'a, REG, P3>;
impl<'a, REG> P3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P3::Const0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P3::Const1)
    }
}
#[doc = "Port n Output Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P4 {
    #[doc = "0: The output level of Pn.x is 0."]
    Const0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    Const1 = 1,
}
impl From<P4> for bool {
    #[inline(always)]
    fn from(variant: P4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P4` reader - Port n Output Bit 4\n\nThe field is **modified** in some way after a read operation."]
pub type P4R = crate::BitReader<P4>;
impl P4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P4 {
        match self.bits {
            false => P4::Const0,
            true => P4::Const1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P4::Const0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P4::Const1
    }
}
#[doc = "Field `P4` writer - Port n Output Bit 4"]
pub type P4W<'a, REG> = crate::BitWriter<'a, REG, P4>;
impl<'a, REG> P4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P4::Const0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P4::Const1)
    }
}
#[doc = "Port n Output Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P5 {
    #[doc = "0: The output level of Pn.x is 0."]
    Const0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    Const1 = 1,
}
impl From<P5> for bool {
    #[inline(always)]
    fn from(variant: P5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P5` reader - Port n Output Bit 5\n\nThe field is **modified** in some way after a read operation."]
pub type P5R = crate::BitReader<P5>;
impl P5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P5 {
        match self.bits {
            false => P5::Const0,
            true => P5::Const1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P5::Const0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P5::Const1
    }
}
#[doc = "Field `P5` writer - Port n Output Bit 5"]
pub type P5W<'a, REG> = crate::BitWriter<'a, REG, P5>;
impl<'a, REG> P5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P5::Const0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P5::Const1)
    }
}
#[doc = "Port n Output Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P6 {
    #[doc = "0: The output level of Pn.x is 0."]
    Const0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    Const1 = 1,
}
impl From<P6> for bool {
    #[inline(always)]
    fn from(variant: P6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P6` reader - Port n Output Bit 6\n\nThe field is **modified** in some way after a read operation."]
pub type P6R = crate::BitReader<P6>;
impl P6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P6 {
        match self.bits {
            false => P6::Const0,
            true => P6::Const1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P6::Const0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P6::Const1
    }
}
#[doc = "Field `P6` writer - Port n Output Bit 6"]
pub type P6W<'a, REG> = crate::BitWriter<'a, REG, P6>;
impl<'a, REG> P6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P6::Const0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P6::Const1)
    }
}
#[doc = "Port n Output Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P7 {
    #[doc = "0: The output level of Pn.x is 0."]
    Const0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    Const1 = 1,
}
impl From<P7> for bool {
    #[inline(always)]
    fn from(variant: P7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P7` reader - Port n Output Bit 7\n\nThe field is **modified** in some way after a read operation."]
pub type P7R = crate::BitReader<P7>;
impl P7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P7 {
        match self.bits {
            false => P7::Const0,
            true => P7::Const1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P7::Const0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P7::Const1
    }
}
#[doc = "Field `P7` writer - Port n Output Bit 7"]
pub type P7W<'a, REG> = crate::BitWriter<'a, REG, P7>;
impl<'a, REG> P7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P7::Const0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P7::Const1)
    }
}
#[doc = "Port n Output Bit 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P8 {
    #[doc = "0: The output level of Pn.x is 0."]
    Const0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    Const1 = 1,
}
impl From<P8> for bool {
    #[inline(always)]
    fn from(variant: P8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P8` reader - Port n Output Bit 8\n\nThe field is **modified** in some way after a read operation."]
pub type P8R = crate::BitReader<P8>;
impl P8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P8 {
        match self.bits {
            false => P8::Const0,
            true => P8::Const1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P8::Const0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P8::Const1
    }
}
#[doc = "Field `P8` writer - Port n Output Bit 8"]
pub type P8W<'a, REG> = crate::BitWriter<'a, REG, P8>;
impl<'a, REG> P8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P8::Const0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P8::Const1)
    }
}
#[doc = "Port n Output Bit 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P9 {
    #[doc = "0: The output level of Pn.x is 0."]
    Const0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    Const1 = 1,
}
impl From<P9> for bool {
    #[inline(always)]
    fn from(variant: P9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P9` reader - Port n Output Bit 9\n\nThe field is **modified** in some way after a read operation."]
pub type P9R = crate::BitReader<P9>;
impl P9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P9 {
        match self.bits {
            false => P9::Const0,
            true => P9::Const1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P9::Const0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P9::Const1
    }
}
#[doc = "Field `P9` writer - Port n Output Bit 9"]
pub type P9W<'a, REG> = crate::BitWriter<'a, REG, P9>;
impl<'a, REG> P9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P9::Const0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P9::Const1)
    }
}
#[doc = "Port n Output Bit 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P10 {
    #[doc = "0: The output level of Pn.x is 0."]
    Const0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    Const1 = 1,
}
impl From<P10> for bool {
    #[inline(always)]
    fn from(variant: P10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P10` reader - Port n Output Bit 10\n\nThe field is **modified** in some way after a read operation."]
pub type P10R = crate::BitReader<P10>;
impl P10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P10 {
        match self.bits {
            false => P10::Const0,
            true => P10::Const1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P10::Const0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P10::Const1
    }
}
#[doc = "Field `P10` writer - Port n Output Bit 10"]
pub type P10W<'a, REG> = crate::BitWriter<'a, REG, P10>;
impl<'a, REG> P10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P10::Const0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P10::Const1)
    }
}
#[doc = "Port n Output Bit 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P11 {
    #[doc = "0: The output level of Pn.x is 0."]
    Const0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    Const1 = 1,
}
impl From<P11> for bool {
    #[inline(always)]
    fn from(variant: P11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P11` reader - Port n Output Bit 11\n\nThe field is **modified** in some way after a read operation."]
pub type P11R = crate::BitReader<P11>;
impl P11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P11 {
        match self.bits {
            false => P11::Const0,
            true => P11::Const1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P11::Const0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P11::Const1
    }
}
#[doc = "Field `P11` writer - Port n Output Bit 11"]
pub type P11W<'a, REG> = crate::BitWriter<'a, REG, P11>;
impl<'a, REG> P11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P11::Const0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P11::Const1)
    }
}
#[doc = "Port n Output Bit 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P12 {
    #[doc = "0: The output level of Pn.x is 0."]
    Const0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    Const1 = 1,
}
impl From<P12> for bool {
    #[inline(always)]
    fn from(variant: P12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P12` reader - Port n Output Bit 12\n\nThe field is **modified** in some way after a read operation."]
pub type P12R = crate::BitReader<P12>;
impl P12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P12 {
        match self.bits {
            false => P12::Const0,
            true => P12::Const1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P12::Const0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P12::Const1
    }
}
#[doc = "Field `P12` writer - Port n Output Bit 12"]
pub type P12W<'a, REG> = crate::BitWriter<'a, REG, P12>;
impl<'a, REG> P12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P12::Const0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P12::Const1)
    }
}
#[doc = "Port n Output Bit 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P13 {
    #[doc = "0: The output level of Pn.x is 0."]
    Const0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    Const1 = 1,
}
impl From<P13> for bool {
    #[inline(always)]
    fn from(variant: P13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P13` reader - Port n Output Bit 13\n\nThe field is **modified** in some way after a read operation."]
pub type P13R = crate::BitReader<P13>;
impl P13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P13 {
        match self.bits {
            false => P13::Const0,
            true => P13::Const1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P13::Const0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P13::Const1
    }
}
#[doc = "Field `P13` writer - Port n Output Bit 13"]
pub type P13W<'a, REG> = crate::BitWriter<'a, REG, P13>;
impl<'a, REG> P13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P13::Const0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P13::Const1)
    }
}
#[doc = "Port n Output Bit 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P14 {
    #[doc = "0: The output level of Pn.x is 0."]
    Const0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    Const1 = 1,
}
impl From<P14> for bool {
    #[inline(always)]
    fn from(variant: P14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P14` reader - Port n Output Bit 14\n\nThe field is **modified** in some way after a read operation."]
pub type P14R = crate::BitReader<P14>;
impl P14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P14 {
        match self.bits {
            false => P14::Const0,
            true => P14::Const1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P14::Const0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P14::Const1
    }
}
#[doc = "Field `P14` writer - Port n Output Bit 14"]
pub type P14W<'a, REG> = crate::BitWriter<'a, REG, P14>;
impl<'a, REG> P14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P14::Const0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P14::Const1)
    }
}
#[doc = "Port n Output Bit 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P15 {
    #[doc = "0: The output level of Pn.x is 0."]
    Const0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    Const1 = 1,
}
impl From<P15> for bool {
    #[inline(always)]
    fn from(variant: P15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P15` reader - Port n Output Bit 15\n\nThe field is **modified** in some way after a read operation."]
pub type P15R = crate::BitReader<P15>;
impl P15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P15 {
        match self.bits {
            false => P15::Const0,
            true => P15::Const1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P15::Const0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P15::Const1
    }
}
#[doc = "Field `P15` writer - Port n Output Bit 15"]
pub type P15W<'a, REG> = crate::BitWriter<'a, REG, P15>;
impl<'a, REG> P15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P15::Const0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P15::Const1)
    }
}
impl R {
    #[doc = "Bit 0 - Port n Output Bit 0"]
    #[inline(always)]
    pub fn p0(&self) -> P0R {
        P0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port n Output Bit 1"]
    #[inline(always)]
    pub fn p1(&self) -> P1R {
        P1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port n Output Bit 2"]
    #[inline(always)]
    pub fn p2(&self) -> P2R {
        P2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port n Output Bit 3"]
    #[inline(always)]
    pub fn p3(&self) -> P3R {
        P3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port n Output Bit 4"]
    #[inline(always)]
    pub fn p4(&self) -> P4R {
        P4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port n Output Bit 5"]
    #[inline(always)]
    pub fn p5(&self) -> P5R {
        P5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port n Output Bit 6"]
    #[inline(always)]
    pub fn p6(&self) -> P6R {
        P6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port n Output Bit 7"]
    #[inline(always)]
    pub fn p7(&self) -> P7R {
        P7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port n Output Bit 8"]
    #[inline(always)]
    pub fn p8(&self) -> P8R {
        P8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port n Output Bit 9"]
    #[inline(always)]
    pub fn p9(&self) -> P9R {
        P9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port n Output Bit 10"]
    #[inline(always)]
    pub fn p10(&self) -> P10R {
        P10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port n Output Bit 11"]
    #[inline(always)]
    pub fn p11(&self) -> P11R {
        P11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port n Output Bit 12"]
    #[inline(always)]
    pub fn p12(&self) -> P12R {
        P12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port n Output Bit 13"]
    #[inline(always)]
    pub fn p13(&self) -> P13R {
        P13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port n Output Bit 14"]
    #[inline(always)]
    pub fn p14(&self) -> P14R {
        P14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port n Output Bit 15"]
    #[inline(always)]
    pub fn p15(&self) -> P15R {
        P15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port n Output Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0W<OutSpec> {
        P0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port n Output Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1W<OutSpec> {
        P1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port n Output Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2W<OutSpec> {
        P2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port n Output Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3W<OutSpec> {
        P3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port n Output Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4W<OutSpec> {
        P4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port n Output Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5W<OutSpec> {
        P5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port n Output Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6W<OutSpec> {
        P6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port n Output Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7W<OutSpec> {
        P7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port n Output Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8W<OutSpec> {
        P8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port n Output Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9W<OutSpec> {
        P9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port n Output Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10W<OutSpec> {
        P10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port n Output Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11W<OutSpec> {
        P11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port n Output Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12W<OutSpec> {
        P12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port n Output Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13W<OutSpec> {
        P13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port n Output Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14W<OutSpec> {
        P14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port n Output Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15W<OutSpec> {
        P15W::new(self, 15)
    }
}
#[doc = "Port 2 Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutSpec;
impl crate::RegisterSpec for OutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out::R`](R) reader structure"]
impl crate::Readable for OutSpec {}
#[doc = "`write(|w| ..)` method takes [`out::W`](W) writer structure"]
impl crate::Writable for OutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT to value 0"]
impl crate::Resettable for OutSpec {
    const RESET_VALUE: u32 = 0;
}
