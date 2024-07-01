#[doc = "Register `OUT` reader"]
pub type R = crate::R<OUT_SPEC>;
#[doc = "Register `OUT` writer"]
pub type W = crate::W<OUT_SPEC>;
#[doc = "Port n Output Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P0_A> for bool {
    #[inline(always)]
    fn from(variant: P0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0` reader - Port n Output Bit 0\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P0_R = crate::BitReader<P0_A>;
impl P0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_A {
        match self.bits {
            false => P0_A::CONST_0,
            true => P0_A::CONST_1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P0_A::CONST_0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P0_A::CONST_1
    }
}
#[doc = "Field `P0` writer - Port n Output Bit 0"]
pub type P0_W<'a, REG> = crate::BitWriter<'a, REG, P0_A>;
impl<'a, REG> P0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P0_A::CONST_1)
    }
}
#[doc = "Port n Output Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P1_A> for bool {
    #[inline(always)]
    fn from(variant: P1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1` reader - Port n Output Bit 1\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P1_R = crate::BitReader<P1_A>;
impl P1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_A {
        match self.bits {
            false => P1_A::CONST_0,
            true => P1_A::CONST_1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P1_A::CONST_0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P1_A::CONST_1
    }
}
#[doc = "Field `P1` writer - Port n Output Bit 1"]
pub type P1_W<'a, REG> = crate::BitWriter<'a, REG, P1_A>;
impl<'a, REG> P1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_A::CONST_1)
    }
}
#[doc = "Port n Output Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P2_A> for bool {
    #[inline(always)]
    fn from(variant: P2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2` reader - Port n Output Bit 2\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P2_R = crate::BitReader<P2_A>;
impl P2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_A {
        match self.bits {
            false => P2_A::CONST_0,
            true => P2_A::CONST_1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P2_A::CONST_0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P2_A::CONST_1
    }
}
#[doc = "Field `P2` writer - Port n Output Bit 2"]
pub type P2_W<'a, REG> = crate::BitWriter<'a, REG, P2_A>;
impl<'a, REG> P2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P2_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_A::CONST_1)
    }
}
#[doc = "Port n Output Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P3_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P3_A> for bool {
    #[inline(always)]
    fn from(variant: P3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P3` reader - Port n Output Bit 3\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P3_R = crate::BitReader<P3_A>;
impl P3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P3_A {
        match self.bits {
            false => P3_A::CONST_0,
            true => P3_A::CONST_1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P3_A::CONST_0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P3_A::CONST_1
    }
}
#[doc = "Field `P3` writer - Port n Output Bit 3"]
pub type P3_W<'a, REG> = crate::BitWriter<'a, REG, P3_A>;
impl<'a, REG> P3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P3_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P3_A::CONST_1)
    }
}
#[doc = "Port n Output Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P4_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P4_A> for bool {
    #[inline(always)]
    fn from(variant: P4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P4` reader - Port n Output Bit 4\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P4_R = crate::BitReader<P4_A>;
impl P4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P4_A {
        match self.bits {
            false => P4_A::CONST_0,
            true => P4_A::CONST_1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P4_A::CONST_0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P4_A::CONST_1
    }
}
#[doc = "Field `P4` writer - Port n Output Bit 4"]
pub type P4_W<'a, REG> = crate::BitWriter<'a, REG, P4_A>;
impl<'a, REG> P4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P4_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P4_A::CONST_1)
    }
}
#[doc = "Port n Output Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P5_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P5_A> for bool {
    #[inline(always)]
    fn from(variant: P5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P5` reader - Port n Output Bit 5\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P5_R = crate::BitReader<P5_A>;
impl P5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P5_A {
        match self.bits {
            false => P5_A::CONST_0,
            true => P5_A::CONST_1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P5_A::CONST_0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P5_A::CONST_1
    }
}
#[doc = "Field `P5` writer - Port n Output Bit 5"]
pub type P5_W<'a, REG> = crate::BitWriter<'a, REG, P5_A>;
impl<'a, REG> P5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P5_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P5_A::CONST_1)
    }
}
#[doc = "Port n Output Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P6_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P6_A> for bool {
    #[inline(always)]
    fn from(variant: P6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P6` reader - Port n Output Bit 6\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P6_R = crate::BitReader<P6_A>;
impl P6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P6_A {
        match self.bits {
            false => P6_A::CONST_0,
            true => P6_A::CONST_1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P6_A::CONST_0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P6_A::CONST_1
    }
}
#[doc = "Field `P6` writer - Port n Output Bit 6"]
pub type P6_W<'a, REG> = crate::BitWriter<'a, REG, P6_A>;
impl<'a, REG> P6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P6_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P6_A::CONST_1)
    }
}
#[doc = "Port n Output Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P7_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P7_A> for bool {
    #[inline(always)]
    fn from(variant: P7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P7` reader - Port n Output Bit 7\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P7_R = crate::BitReader<P7_A>;
impl P7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P7_A {
        match self.bits {
            false => P7_A::CONST_0,
            true => P7_A::CONST_1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P7_A::CONST_0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P7_A::CONST_1
    }
}
#[doc = "Field `P7` writer - Port n Output Bit 7"]
pub type P7_W<'a, REG> = crate::BitWriter<'a, REG, P7_A>;
impl<'a, REG> P7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P7_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P7_A::CONST_1)
    }
}
#[doc = "Port n Output Bit 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P8_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P8_A> for bool {
    #[inline(always)]
    fn from(variant: P8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P8` reader - Port n Output Bit 8\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P8_R = crate::BitReader<P8_A>;
impl P8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P8_A {
        match self.bits {
            false => P8_A::CONST_0,
            true => P8_A::CONST_1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P8_A::CONST_0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P8_A::CONST_1
    }
}
#[doc = "Field `P8` writer - Port n Output Bit 8"]
pub type P8_W<'a, REG> = crate::BitWriter<'a, REG, P8_A>;
impl<'a, REG> P8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P8_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P8_A::CONST_1)
    }
}
#[doc = "Port n Output Bit 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P9_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P9_A> for bool {
    #[inline(always)]
    fn from(variant: P9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P9` reader - Port n Output Bit 9\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P9_R = crate::BitReader<P9_A>;
impl P9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P9_A {
        match self.bits {
            false => P9_A::CONST_0,
            true => P9_A::CONST_1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P9_A::CONST_0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P9_A::CONST_1
    }
}
#[doc = "Field `P9` writer - Port n Output Bit 9"]
pub type P9_W<'a, REG> = crate::BitWriter<'a, REG, P9_A>;
impl<'a, REG> P9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P9_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P9_A::CONST_1)
    }
}
#[doc = "Port n Output Bit 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P10_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P10_A> for bool {
    #[inline(always)]
    fn from(variant: P10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P10` reader - Port n Output Bit 10\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P10_R = crate::BitReader<P10_A>;
impl P10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P10_A {
        match self.bits {
            false => P10_A::CONST_0,
            true => P10_A::CONST_1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P10_A::CONST_0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P10_A::CONST_1
    }
}
#[doc = "Field `P10` writer - Port n Output Bit 10"]
pub type P10_W<'a, REG> = crate::BitWriter<'a, REG, P10_A>;
impl<'a, REG> P10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P10_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P10_A::CONST_1)
    }
}
#[doc = "Port n Output Bit 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P11_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P11_A> for bool {
    #[inline(always)]
    fn from(variant: P11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P11` reader - Port n Output Bit 11\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P11_R = crate::BitReader<P11_A>;
impl P11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P11_A {
        match self.bits {
            false => P11_A::CONST_0,
            true => P11_A::CONST_1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P11_A::CONST_0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P11_A::CONST_1
    }
}
#[doc = "Field `P11` writer - Port n Output Bit 11"]
pub type P11_W<'a, REG> = crate::BitWriter<'a, REG, P11_A>;
impl<'a, REG> P11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P11_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P11_A::CONST_1)
    }
}
#[doc = "Port n Output Bit 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P12_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P12_A> for bool {
    #[inline(always)]
    fn from(variant: P12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P12` reader - Port n Output Bit 12\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P12_R = crate::BitReader<P12_A>;
impl P12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P12_A {
        match self.bits {
            false => P12_A::CONST_0,
            true => P12_A::CONST_1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P12_A::CONST_0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P12_A::CONST_1
    }
}
#[doc = "Field `P12` writer - Port n Output Bit 12"]
pub type P12_W<'a, REG> = crate::BitWriter<'a, REG, P12_A>;
impl<'a, REG> P12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P12_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P12_A::CONST_1)
    }
}
#[doc = "Port n Output Bit 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P13_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P13_A> for bool {
    #[inline(always)]
    fn from(variant: P13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P13` reader - Port n Output Bit 13\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P13_R = crate::BitReader<P13_A>;
impl P13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P13_A {
        match self.bits {
            false => P13_A::CONST_0,
            true => P13_A::CONST_1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P13_A::CONST_0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P13_A::CONST_1
    }
}
#[doc = "Field `P13` writer - Port n Output Bit 13"]
pub type P13_W<'a, REG> = crate::BitWriter<'a, REG, P13_A>;
impl<'a, REG> P13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P13_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P13_A::CONST_1)
    }
}
#[doc = "Port n Output Bit 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P14_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P14_A> for bool {
    #[inline(always)]
    fn from(variant: P14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P14` reader - Port n Output Bit 14\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P14_R = crate::BitReader<P14_A>;
impl P14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P14_A {
        match self.bits {
            false => P14_A::CONST_0,
            true => P14_A::CONST_1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P14_A::CONST_0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P14_A::CONST_1
    }
}
#[doc = "Field `P14` writer - Port n Output Bit 14"]
pub type P14_W<'a, REG> = crate::BitWriter<'a, REG, P14_A>;
impl<'a, REG> P14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P14_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P14_A::CONST_1)
    }
}
#[doc = "Port n Output Bit 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P15_A {
    #[doc = "0: The output level of Pn.x is 0."]
    CONST_0 = 0,
    #[doc = "1: The output level of Pn.x is 1."]
    CONST_1 = 1,
}
impl From<P15_A> for bool {
    #[inline(always)]
    fn from(variant: P15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P15` reader - Port n Output Bit 15\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type P15_R = crate::BitReader<P15_A>;
impl P15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P15_A {
        match self.bits {
            false => P15_A::CONST_0,
            true => P15_A::CONST_1,
        }
    }
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == P15_A::CONST_0
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == P15_A::CONST_1
    }
}
#[doc = "Field `P15` writer - Port n Output Bit 15"]
pub type P15_W<'a, REG> = crate::BitWriter<'a, REG, P15_A>;
impl<'a, REG> P15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output level of Pn.x is 0."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(P15_A::CONST_0)
    }
    #[doc = "The output level of Pn.x is 1."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(P15_A::CONST_1)
    }
}
impl R {
    #[doc = "Bit 0 - Port n Output Bit 0"]
    #[inline(always)]
    pub fn p0(&self) -> P0_R {
        P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port n Output Bit 1"]
    #[inline(always)]
    pub fn p1(&self) -> P1_R {
        P1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port n Output Bit 2"]
    #[inline(always)]
    pub fn p2(&self) -> P2_R {
        P2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port n Output Bit 3"]
    #[inline(always)]
    pub fn p3(&self) -> P3_R {
        P3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port n Output Bit 4"]
    #[inline(always)]
    pub fn p4(&self) -> P4_R {
        P4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port n Output Bit 5"]
    #[inline(always)]
    pub fn p5(&self) -> P5_R {
        P5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port n Output Bit 6"]
    #[inline(always)]
    pub fn p6(&self) -> P6_R {
        P6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port n Output Bit 7"]
    #[inline(always)]
    pub fn p7(&self) -> P7_R {
        P7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port n Output Bit 8"]
    #[inline(always)]
    pub fn p8(&self) -> P8_R {
        P8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port n Output Bit 9"]
    #[inline(always)]
    pub fn p9(&self) -> P9_R {
        P9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port n Output Bit 10"]
    #[inline(always)]
    pub fn p10(&self) -> P10_R {
        P10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port n Output Bit 11"]
    #[inline(always)]
    pub fn p11(&self) -> P11_R {
        P11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port n Output Bit 12"]
    #[inline(always)]
    pub fn p12(&self) -> P12_R {
        P12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port n Output Bit 13"]
    #[inline(always)]
    pub fn p13(&self) -> P13_R {
        P13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port n Output Bit 14"]
    #[inline(always)]
    pub fn p14(&self) -> P14_R {
        P14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port n Output Bit 15"]
    #[inline(always)]
    pub fn p15(&self) -> P15_R {
        P15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port n Output Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0_W<OUT_SPEC> {
        P0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port n Output Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1_W<OUT_SPEC> {
        P1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port n Output Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2_W<OUT_SPEC> {
        P2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port n Output Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3_W<OUT_SPEC> {
        P3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port n Output Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4_W<OUT_SPEC> {
        P4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port n Output Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5_W<OUT_SPEC> {
        P5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port n Output Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6_W<OUT_SPEC> {
        P6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port n Output Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7_W<OUT_SPEC> {
        P7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port n Output Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8_W<OUT_SPEC> {
        P8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port n Output Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9_W<OUT_SPEC> {
        P9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port n Output Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10_W<OUT_SPEC> {
        P10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port n Output Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11_W<OUT_SPEC> {
        P11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port n Output Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12_W<OUT_SPEC> {
        P12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port n Output Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13_W<OUT_SPEC> {
        P13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port n Output Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14_W<OUT_SPEC> {
        P14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port n Output Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15_W<OUT_SPEC> {
        P15_W::new(self, 15)
    }
}
#[doc = "Port 15 Output Register\n\nYou can [`read`](crate::Reg::read) this register and get [`out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_SPEC;
impl crate::RegisterSpec for OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out::R`](R) reader structure"]
impl crate::Readable for OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out::W`](W) writer structure"]
impl crate::Writable for OUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT to value 0"]
impl crate::Resettable for OUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
