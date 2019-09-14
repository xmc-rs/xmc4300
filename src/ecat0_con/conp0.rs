#[doc = "Reader of register CONP0"]
pub type R = crate::R<u32, super::CONP0>;
#[doc = "Writer for register CONP0"]
pub type W = crate::W<u32, super::CONP0>;
#[doc = "Register CONP0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONP0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PORT0 Receive Input 0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXD0_A {
    #[doc = "0: Data input RXD0A is selected"]
    VALUE1,
    #[doc = "1: Data input RXD0B is selected"]
    VALUE2,
    #[doc = "2: Data input RXD0C is selected"]
    VALUE3,
    #[doc = "3: Data input RXD0D is selected"]
    VALUE4,
}
impl From<RXD0_A> for u8 {
    #[inline(always)]
    fn from(variant: RXD0_A) -> Self {
        match variant {
            RXD0_A::VALUE1 => 0,
            RXD0_A::VALUE2 => 1,
            RXD0_A::VALUE3 => 2,
            RXD0_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `RXD0`"]
pub type RXD0_R = crate::R<u8, RXD0_A>;
impl RXD0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXD0_A {
        match self.bits {
            0 => RXD0_A::VALUE1,
            1 => RXD0_A::VALUE2,
            2 => RXD0_A::VALUE3,
            3 => RXD0_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXD0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RXD0_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RXD0_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RXD0_A::VALUE4
    }
}
#[doc = "Write proxy for field `RXD0`"]
pub struct RXD0_W<'a> {
    w: &'a mut W,
}
impl<'a> RXD0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXD0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Data input RXD0A is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXD0_A::VALUE1)
    }
    #[doc = "Data input RXD0B is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXD0_A::VALUE2)
    }
    #[doc = "Data input RXD0C is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(RXD0_A::VALUE3)
    }
    #[doc = "Data input RXD0D is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(RXD0_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Port0 Receive Input 1 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXD1_A {
    #[doc = "0: Data input RXD1A is selected"]
    VALUE1,
    #[doc = "1: Data input RXD1B is selected"]
    VALUE2,
    #[doc = "2: Data input RXD1C is selected"]
    VALUE3,
    #[doc = "3: Data input RXD1D is selected"]
    VALUE4,
}
impl From<RXD1_A> for u8 {
    #[inline(always)]
    fn from(variant: RXD1_A) -> Self {
        match variant {
            RXD1_A::VALUE1 => 0,
            RXD1_A::VALUE2 => 1,
            RXD1_A::VALUE3 => 2,
            RXD1_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `RXD1`"]
pub type RXD1_R = crate::R<u8, RXD1_A>;
impl RXD1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXD1_A {
        match self.bits {
            0 => RXD1_A::VALUE1,
            1 => RXD1_A::VALUE2,
            2 => RXD1_A::VALUE3,
            3 => RXD1_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXD1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RXD1_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RXD1_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RXD1_A::VALUE4
    }
}
#[doc = "Write proxy for field `RXD1`"]
pub struct RXD1_W<'a> {
    w: &'a mut W,
}
impl<'a> RXD1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXD1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Data input RXD1A is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXD1_A::VALUE1)
    }
    #[doc = "Data input RXD1B is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXD1_A::VALUE2)
    }
    #[doc = "Data input RXD1C is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(RXD1_A::VALUE3)
    }
    #[doc = "Data input RXD1D is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(RXD1_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Port0 Receive Input 2 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXD2_A {
    #[doc = "0: Data input RXD2A is selected"]
    VALUE1,
    #[doc = "1: Data input RXD2B is selected"]
    VALUE2,
    #[doc = "2: Data input RXD2C is selected"]
    VALUE3,
    #[doc = "3: Data input RXD2D is selected"]
    VALUE4,
}
impl From<RXD2_A> for u8 {
    #[inline(always)]
    fn from(variant: RXD2_A) -> Self {
        match variant {
            RXD2_A::VALUE1 => 0,
            RXD2_A::VALUE2 => 1,
            RXD2_A::VALUE3 => 2,
            RXD2_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `RXD2`"]
pub type RXD2_R = crate::R<u8, RXD2_A>;
impl RXD2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXD2_A {
        match self.bits {
            0 => RXD2_A::VALUE1,
            1 => RXD2_A::VALUE2,
            2 => RXD2_A::VALUE3,
            3 => RXD2_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXD2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RXD2_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RXD2_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RXD2_A::VALUE4
    }
}
#[doc = "Write proxy for field `RXD2`"]
pub struct RXD2_W<'a> {
    w: &'a mut W,
}
impl<'a> RXD2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXD2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Data input RXD2A is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXD2_A::VALUE1)
    }
    #[doc = "Data input RXD2B is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXD2_A::VALUE2)
    }
    #[doc = "Data input RXD2C is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(RXD2_A::VALUE3)
    }
    #[doc = "Data input RXD2D is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(RXD2_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Port0 Receive Input 3 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXD3_A {
    #[doc = "0: Data input RXD3A is selected"]
    VALUE1,
    #[doc = "1: Data input RXD3B is selected"]
    VALUE2,
    #[doc = "2: Data input RXD3C is selected"]
    VALUE3,
    #[doc = "3: Data input RXD3D is selected"]
    VALUE4,
}
impl From<RXD3_A> for u8 {
    #[inline(always)]
    fn from(variant: RXD3_A) -> Self {
        match variant {
            RXD3_A::VALUE1 => 0,
            RXD3_A::VALUE2 => 1,
            RXD3_A::VALUE3 => 2,
            RXD3_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `RXD3`"]
pub type RXD3_R = crate::R<u8, RXD3_A>;
impl RXD3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXD3_A {
        match self.bits {
            0 => RXD3_A::VALUE1,
            1 => RXD3_A::VALUE2,
            2 => RXD3_A::VALUE3,
            3 => RXD3_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXD3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RXD3_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RXD3_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RXD3_A::VALUE4
    }
}
#[doc = "Write proxy for field `RXD3`"]
pub struct RXD3_W<'a> {
    w: &'a mut W,
}
impl<'a> RXD3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXD3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Data input RXD3A is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXD3_A::VALUE1)
    }
    #[doc = "Data input RXD3B is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXD3_A::VALUE2)
    }
    #[doc = "Data input RXD3C is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(RXD3_A::VALUE3)
    }
    #[doc = "Data input RXD3D is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(RXD3_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Port0 MII RX ERROR Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_ERR_A {
    #[doc = "0: Data input RX_ERRA is selected"]
    VALUE1,
    #[doc = "1: Data input RX_ERRB is selected"]
    VALUE2,
    #[doc = "2: Data input RX_ERRC is selected"]
    VALUE3,
    #[doc = "3: Data input RX_ERRD is selected"]
    VALUE4,
}
impl From<RX_ERR_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_ERR_A) -> Self {
        match variant {
            RX_ERR_A::VALUE1 => 0,
            RX_ERR_A::VALUE2 => 1,
            RX_ERR_A::VALUE3 => 2,
            RX_ERR_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `RX_ERR`"]
pub type RX_ERR_R = crate::R<u8, RX_ERR_A>;
impl RX_ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_ERR_A {
        match self.bits {
            0 => RX_ERR_A::VALUE1,
            1 => RX_ERR_A::VALUE2,
            2 => RX_ERR_A::VALUE3,
            3 => RX_ERR_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RX_ERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RX_ERR_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RX_ERR_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RX_ERR_A::VALUE4
    }
}
#[doc = "Write proxy for field `RX_ERR`"]
pub struct RX_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_ERR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Data input RX_ERRA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RX_ERR_A::VALUE1)
    }
    #[doc = "Data input RX_ERRB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RX_ERR_A::VALUE2)
    }
    #[doc = "Data input RX_ERRC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(RX_ERR_A::VALUE3)
    }
    #[doc = "Data input RX_ERRD is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(RX_ERR_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Port0 MII RX DV Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DV_A {
    #[doc = "0: Data input RX_DVA is selected"]
    VALUE1,
    #[doc = "1: Data input RX_DVB is selected"]
    VALUE2,
    #[doc = "2: Data input RX_DVC is selected"]
    VALUE3,
    #[doc = "3: Data input RX_DVD is selected"]
    VALUE4,
}
impl From<RX_DV_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_DV_A) -> Self {
        match variant {
            RX_DV_A::VALUE1 => 0,
            RX_DV_A::VALUE2 => 1,
            RX_DV_A::VALUE3 => 2,
            RX_DV_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `RX_DV`"]
pub type RX_DV_R = crate::R<u8, RX_DV_A>;
impl RX_DV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_DV_A {
        match self.bits {
            0 => RX_DV_A::VALUE1,
            1 => RX_DV_A::VALUE2,
            2 => RX_DV_A::VALUE3,
            3 => RX_DV_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RX_DV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RX_DV_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RX_DV_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RX_DV_A::VALUE4
    }
}
#[doc = "Write proxy for field `RX_DV`"]
pub struct RX_DV_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_DV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Data input RX_DVA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RX_DV_A::VALUE1)
    }
    #[doc = "Data input RX_DVB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RX_DV_A::VALUE2)
    }
    #[doc = "Data input RX_DVC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(RX_DV_A::VALUE3)
    }
    #[doc = "Data input RX_DVD is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(RX_DV_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Port0 MII RX Clock Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_CLK_A {
    #[doc = "0: Clock input RX_CLKA"]
    VALUE1,
    #[doc = "1: Clock input RX_CLKB"]
    VALUE2,
    #[doc = "2: Clock input RX_CLKC"]
    VALUE3,
    #[doc = "3: Clock input RX_CLKD"]
    VALUE4,
}
impl From<RX_CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_CLK_A) -> Self {
        match variant {
            RX_CLK_A::VALUE1 => 0,
            RX_CLK_A::VALUE2 => 1,
            RX_CLK_A::VALUE3 => 2,
            RX_CLK_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `RX_CLK`"]
pub type RX_CLK_R = crate::R<u8, RX_CLK_A>;
impl RX_CLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_CLK_A {
        match self.bits {
            0 => RX_CLK_A::VALUE1,
            1 => RX_CLK_A::VALUE2,
            2 => RX_CLK_A::VALUE3,
            3 => RX_CLK_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RX_CLK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RX_CLK_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RX_CLK_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RX_CLK_A::VALUE4
    }
}
#[doc = "Write proxy for field `RX_CLK`"]
pub struct RX_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_CLK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clock input RX_CLKA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RX_CLK_A::VALUE1)
    }
    #[doc = "Clock input RX_CLKB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RX_CLK_A::VALUE2)
    }
    #[doc = "Clock input RX_CLKC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(RX_CLK_A::VALUE3)
    }
    #[doc = "Clock input RX_CLKD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(RX_CLK_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Port0 PHY Link Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINK_A {
    #[doc = "0: PHY LINKA"]
    VALUE1,
    #[doc = "1: PHY LINKB"]
    VALUE2,
    #[doc = "2: PHY LINKC"]
    VALUE3,
    #[doc = "3: PHY LINKD"]
    VALUE4,
}
impl From<LINK_A> for u8 {
    #[inline(always)]
    fn from(variant: LINK_A) -> Self {
        match variant {
            LINK_A::VALUE1 => 0,
            LINK_A::VALUE2 => 1,
            LINK_A::VALUE3 => 2,
            LINK_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `LINK`"]
pub type LINK_R = crate::R<u8, LINK_A>;
impl LINK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINK_A {
        match self.bits {
            0 => LINK_A::VALUE1,
            1 => LINK_A::VALUE2,
            2 => LINK_A::VALUE3,
            3 => LINK_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LINK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LINK_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LINK_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == LINK_A::VALUE4
    }
}
#[doc = "Write proxy for field `LINK`"]
pub struct LINK_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PHY LINKA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LINK_A::VALUE1)
    }
    #[doc = "PHY LINKB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LINK_A::VALUE2)
    }
    #[doc = "PHY LINKC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(LINK_A::VALUE3)
    }
    #[doc = "PHY LINKD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(LINK_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Port0 MII TX Clock Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_CLK_A {
    #[doc = "0: Clock input TX_CLKA"]
    VALUE1,
    #[doc = "1: Clock input TX_CLKB"]
    VALUE2,
    #[doc = "2: Clock input TX_CLKC"]
    VALUE3,
    #[doc = "3: Clock input TX_CLKD"]
    VALUE4,
}
impl From<TX_CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_CLK_A) -> Self {
        match variant {
            TX_CLK_A::VALUE1 => 0,
            TX_CLK_A::VALUE2 => 1,
            TX_CLK_A::VALUE3 => 2,
            TX_CLK_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `TX_CLK`"]
pub type TX_CLK_R = crate::R<u8, TX_CLK_A>;
impl TX_CLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_CLK_A {
        match self.bits {
            0 => TX_CLK_A::VALUE1,
            1 => TX_CLK_A::VALUE2,
            2 => TX_CLK_A::VALUE3,
            3 => TX_CLK_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TX_CLK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TX_CLK_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TX_CLK_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TX_CLK_A::VALUE4
    }
}
#[doc = "Write proxy for field `TX_CLK`"]
pub struct TX_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_CLK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clock input TX_CLKA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TX_CLK_A::VALUE1)
    }
    #[doc = "Clock input TX_CLKB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TX_CLK_A::VALUE2)
    }
    #[doc = "Clock input TX_CLKC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TX_CLK_A::VALUE3)
    }
    #[doc = "Clock input TX_CLKD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(TX_CLK_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Port0 Manual TX Shift configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_SHIFT_A {
    #[doc = "0: 0 ns"]
    VALUE1,
    #[doc = "1: 10 ns"]
    VALUE2,
    #[doc = "2: 20 ns"]
    VALUE3,
    #[doc = "3: 30 ns"]
    VALUE4,
}
impl From<TX_SHIFT_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_SHIFT_A) -> Self {
        match variant {
            TX_SHIFT_A::VALUE1 => 0,
            TX_SHIFT_A::VALUE2 => 1,
            TX_SHIFT_A::VALUE3 => 2,
            TX_SHIFT_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `TX_SHIFT`"]
pub type TX_SHIFT_R = crate::R<u8, TX_SHIFT_A>;
impl TX_SHIFT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_SHIFT_A {
        match self.bits {
            0 => TX_SHIFT_A::VALUE1,
            1 => TX_SHIFT_A::VALUE2,
            2 => TX_SHIFT_A::VALUE3,
            3 => TX_SHIFT_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TX_SHIFT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TX_SHIFT_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TX_SHIFT_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TX_SHIFT_A::VALUE4
    }
}
#[doc = "Write proxy for field `TX_SHIFT`"]
pub struct TX_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SHIFT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_SHIFT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "0 ns"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TX_SHIFT_A::VALUE1)
    }
    #[doc = "10 ns"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TX_SHIFT_A::VALUE2)
    }
    #[doc = "20 ns"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TX_SHIFT_A::VALUE3)
    }
    #[doc = "30 ns"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(TX_SHIFT_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - PORT0 Receive Input 0 Select"]
    #[inline(always)]
    pub fn rxd0(&self) -> RXD0_R {
        RXD0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port0 Receive Input 1 Select"]
    #[inline(always)]
    pub fn rxd1(&self) -> RXD1_R {
        RXD1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port0 Receive Input 2 Select"]
    #[inline(always)]
    pub fn rxd2(&self) -> RXD2_R {
        RXD2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port0 Receive Input 3 Select"]
    #[inline(always)]
    pub fn rxd3(&self) -> RXD3_R {
        RXD3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port0 MII RX ERROR Input Select"]
    #[inline(always)]
    pub fn rx_err(&self) -> RX_ERR_R {
        RX_ERR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Port0 MII RX DV Input Select"]
    #[inline(always)]
    pub fn rx_dv(&self) -> RX_DV_R {
        RX_DV_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Port0 MII RX Clock Input Select"]
    #[inline(always)]
    pub fn rx_clk(&self) -> RX_CLK_R {
        RX_CLK_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Port0 PHY Link Input Select"]
    #[inline(always)]
    pub fn link(&self) -> LINK_R {
        LINK_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Port0 MII TX Clock Input Select"]
    #[inline(always)]
    pub fn tx_clk(&self) -> TX_CLK_R {
        TX_CLK_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Port0 Manual TX Shift configuration"]
    #[inline(always)]
    pub fn tx_shift(&self) -> TX_SHIFT_R {
        TX_SHIFT_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PORT0 Receive Input 0 Select"]
    #[inline(always)]
    pub fn rxd0(&mut self) -> RXD0_W {
        RXD0_W { w: self }
    }
    #[doc = "Bits 2:3 - Port0 Receive Input 1 Select"]
    #[inline(always)]
    pub fn rxd1(&mut self) -> RXD1_W {
        RXD1_W { w: self }
    }
    #[doc = "Bits 4:5 - Port0 Receive Input 2 Select"]
    #[inline(always)]
    pub fn rxd2(&mut self) -> RXD2_W {
        RXD2_W { w: self }
    }
    #[doc = "Bits 6:7 - Port0 Receive Input 3 Select"]
    #[inline(always)]
    pub fn rxd3(&mut self) -> RXD3_W {
        RXD3_W { w: self }
    }
    #[doc = "Bits 8:9 - Port0 MII RX ERROR Input Select"]
    #[inline(always)]
    pub fn rx_err(&mut self) -> RX_ERR_W {
        RX_ERR_W { w: self }
    }
    #[doc = "Bits 10:11 - Port0 MII RX DV Input Select"]
    #[inline(always)]
    pub fn rx_dv(&mut self) -> RX_DV_W {
        RX_DV_W { w: self }
    }
    #[doc = "Bits 12:13 - Port0 MII RX Clock Input Select"]
    #[inline(always)]
    pub fn rx_clk(&mut self) -> RX_CLK_W {
        RX_CLK_W { w: self }
    }
    #[doc = "Bits 16:17 - Port0 PHY Link Input Select"]
    #[inline(always)]
    pub fn link(&mut self) -> LINK_W {
        LINK_W { w: self }
    }
    #[doc = "Bits 28:29 - Port0 MII TX Clock Input Select"]
    #[inline(always)]
    pub fn tx_clk(&mut self) -> TX_CLK_W {
        TX_CLK_W { w: self }
    }
    #[doc = "Bits 30:31 - Port0 Manual TX Shift configuration"]
    #[inline(always)]
    pub fn tx_shift(&mut self) -> TX_SHIFT_W {
        TX_SHIFT_W { w: self }
    }
}
