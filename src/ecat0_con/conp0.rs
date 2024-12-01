#[doc = "Register `CONP0` reader"]
pub type R = crate::R<CONP0_SPEC>;
#[doc = "Register `CONP0` writer"]
pub type W = crate::W<CONP0_SPEC>;
#[doc = "PORT0 Receive Input 0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXD0_A {
    #[doc = "0: Data input RXD0A is selected"]
    VALUE1 = 0,
    #[doc = "1: Data input RXD0B is selected"]
    VALUE2 = 1,
    #[doc = "2: Data input RXD0C is selected"]
    VALUE3 = 2,
    #[doc = "3: Data input RXD0D is selected"]
    VALUE4 = 3,
}
impl From<RXD0_A> for u8 {
    #[inline(always)]
    fn from(variant: RXD0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXD0_A {
    type Ux = u8;
}
impl crate::IsEnum for RXD0_A {}
#[doc = "Field `RXD0` reader - PORT0 Receive Input 0 Select"]
pub type RXD0_R = crate::FieldReader<RXD0_A>;
impl RXD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXD0_A {
        match self.bits {
            0 => RXD0_A::VALUE1,
            1 => RXD0_A::VALUE2,
            2 => RXD0_A::VALUE3,
            3 => RXD0_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input RXD0A is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXD0_A::VALUE1
    }
    #[doc = "Data input RXD0B is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RXD0_A::VALUE2
    }
    #[doc = "Data input RXD0C is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RXD0_A::VALUE3
    }
    #[doc = "Data input RXD0D is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RXD0_A::VALUE4
    }
}
#[doc = "Field `RXD0` writer - PORT0 Receive Input 0 Select"]
pub type RXD0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RXD0_A, crate::Safe>;
impl<'a, REG> RXD0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input RXD0A is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RXD0_A::VALUE1)
    }
    #[doc = "Data input RXD0B is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RXD0_A::VALUE2)
    }
    #[doc = "Data input RXD0C is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(RXD0_A::VALUE3)
    }
    #[doc = "Data input RXD0D is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(RXD0_A::VALUE4)
    }
}
#[doc = "Port0 Receive Input 1 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXD1_A {
    #[doc = "0: Data input RXD1A is selected"]
    VALUE1 = 0,
    #[doc = "1: Data input RXD1B is selected"]
    VALUE2 = 1,
    #[doc = "2: Data input RXD1C is selected"]
    VALUE3 = 2,
    #[doc = "3: Data input RXD1D is selected"]
    VALUE4 = 3,
}
impl From<RXD1_A> for u8 {
    #[inline(always)]
    fn from(variant: RXD1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXD1_A {
    type Ux = u8;
}
impl crate::IsEnum for RXD1_A {}
#[doc = "Field `RXD1` reader - Port0 Receive Input 1 Select"]
pub type RXD1_R = crate::FieldReader<RXD1_A>;
impl RXD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXD1_A {
        match self.bits {
            0 => RXD1_A::VALUE1,
            1 => RXD1_A::VALUE2,
            2 => RXD1_A::VALUE3,
            3 => RXD1_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input RXD1A is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXD1_A::VALUE1
    }
    #[doc = "Data input RXD1B is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RXD1_A::VALUE2
    }
    #[doc = "Data input RXD1C is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RXD1_A::VALUE3
    }
    #[doc = "Data input RXD1D is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RXD1_A::VALUE4
    }
}
#[doc = "Field `RXD1` writer - Port0 Receive Input 1 Select"]
pub type RXD1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RXD1_A, crate::Safe>;
impl<'a, REG> RXD1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input RXD1A is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RXD1_A::VALUE1)
    }
    #[doc = "Data input RXD1B is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RXD1_A::VALUE2)
    }
    #[doc = "Data input RXD1C is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(RXD1_A::VALUE3)
    }
    #[doc = "Data input RXD1D is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(RXD1_A::VALUE4)
    }
}
#[doc = "Port0 Receive Input 2 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXD2_A {
    #[doc = "0: Data input RXD2A is selected"]
    VALUE1 = 0,
    #[doc = "1: Data input RXD2B is selected"]
    VALUE2 = 1,
    #[doc = "2: Data input RXD2C is selected"]
    VALUE3 = 2,
    #[doc = "3: Data input RXD2D is selected"]
    VALUE4 = 3,
}
impl From<RXD2_A> for u8 {
    #[inline(always)]
    fn from(variant: RXD2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXD2_A {
    type Ux = u8;
}
impl crate::IsEnum for RXD2_A {}
#[doc = "Field `RXD2` reader - Port0 Receive Input 2 Select"]
pub type RXD2_R = crate::FieldReader<RXD2_A>;
impl RXD2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXD2_A {
        match self.bits {
            0 => RXD2_A::VALUE1,
            1 => RXD2_A::VALUE2,
            2 => RXD2_A::VALUE3,
            3 => RXD2_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input RXD2A is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXD2_A::VALUE1
    }
    #[doc = "Data input RXD2B is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RXD2_A::VALUE2
    }
    #[doc = "Data input RXD2C is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RXD2_A::VALUE3
    }
    #[doc = "Data input RXD2D is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RXD2_A::VALUE4
    }
}
#[doc = "Field `RXD2` writer - Port0 Receive Input 2 Select"]
pub type RXD2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RXD2_A, crate::Safe>;
impl<'a, REG> RXD2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input RXD2A is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RXD2_A::VALUE1)
    }
    #[doc = "Data input RXD2B is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RXD2_A::VALUE2)
    }
    #[doc = "Data input RXD2C is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(RXD2_A::VALUE3)
    }
    #[doc = "Data input RXD2D is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(RXD2_A::VALUE4)
    }
}
#[doc = "Port0 Receive Input 3 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXD3_A {
    #[doc = "0: Data input RXD3A is selected"]
    VALUE1 = 0,
    #[doc = "1: Data input RXD3B is selected"]
    VALUE2 = 1,
    #[doc = "2: Data input RXD3C is selected"]
    VALUE3 = 2,
    #[doc = "3: Data input RXD3D is selected"]
    VALUE4 = 3,
}
impl From<RXD3_A> for u8 {
    #[inline(always)]
    fn from(variant: RXD3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXD3_A {
    type Ux = u8;
}
impl crate::IsEnum for RXD3_A {}
#[doc = "Field `RXD3` reader - Port0 Receive Input 3 Select"]
pub type RXD3_R = crate::FieldReader<RXD3_A>;
impl RXD3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXD3_A {
        match self.bits {
            0 => RXD3_A::VALUE1,
            1 => RXD3_A::VALUE2,
            2 => RXD3_A::VALUE3,
            3 => RXD3_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input RXD3A is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXD3_A::VALUE1
    }
    #[doc = "Data input RXD3B is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RXD3_A::VALUE2
    }
    #[doc = "Data input RXD3C is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RXD3_A::VALUE3
    }
    #[doc = "Data input RXD3D is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RXD3_A::VALUE4
    }
}
#[doc = "Field `RXD3` writer - Port0 Receive Input 3 Select"]
pub type RXD3_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RXD3_A, crate::Safe>;
impl<'a, REG> RXD3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input RXD3A is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RXD3_A::VALUE1)
    }
    #[doc = "Data input RXD3B is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RXD3_A::VALUE2)
    }
    #[doc = "Data input RXD3C is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(RXD3_A::VALUE3)
    }
    #[doc = "Data input RXD3D is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(RXD3_A::VALUE4)
    }
}
#[doc = "Port0 MII RX ERROR Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RX_ERR_A {
    #[doc = "0: Data input RX_ERRA is selected"]
    VALUE1 = 0,
    #[doc = "1: Data input RX_ERRB is selected"]
    VALUE2 = 1,
    #[doc = "2: Data input RX_ERRC is selected"]
    VALUE3 = 2,
    #[doc = "3: Data input RX_ERRD is selected"]
    VALUE4 = 3,
}
impl From<RX_ERR_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_ERR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RX_ERR_A {
    type Ux = u8;
}
impl crate::IsEnum for RX_ERR_A {}
#[doc = "Field `RX_ERR` reader - Port0 MII RX ERROR Input Select"]
pub type RX_ERR_R = crate::FieldReader<RX_ERR_A>;
impl RX_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_ERR_A {
        match self.bits {
            0 => RX_ERR_A::VALUE1,
            1 => RX_ERR_A::VALUE2,
            2 => RX_ERR_A::VALUE3,
            3 => RX_ERR_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input RX_ERRA is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RX_ERR_A::VALUE1
    }
    #[doc = "Data input RX_ERRB is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RX_ERR_A::VALUE2
    }
    #[doc = "Data input RX_ERRC is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RX_ERR_A::VALUE3
    }
    #[doc = "Data input RX_ERRD is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RX_ERR_A::VALUE4
    }
}
#[doc = "Field `RX_ERR` writer - Port0 MII RX ERROR Input Select"]
pub type RX_ERR_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RX_ERR_A, crate::Safe>;
impl<'a, REG> RX_ERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input RX_ERRA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RX_ERR_A::VALUE1)
    }
    #[doc = "Data input RX_ERRB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RX_ERR_A::VALUE2)
    }
    #[doc = "Data input RX_ERRC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(RX_ERR_A::VALUE3)
    }
    #[doc = "Data input RX_ERRD is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(RX_ERR_A::VALUE4)
    }
}
#[doc = "Port0 MII RX DV Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RX_DV_A {
    #[doc = "0: Data input RX_DVA is selected"]
    VALUE1 = 0,
    #[doc = "1: Data input RX_DVB is selected"]
    VALUE2 = 1,
    #[doc = "2: Data input RX_DVC is selected"]
    VALUE3 = 2,
    #[doc = "3: Data input RX_DVD is selected"]
    VALUE4 = 3,
}
impl From<RX_DV_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_DV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RX_DV_A {
    type Ux = u8;
}
impl crate::IsEnum for RX_DV_A {}
#[doc = "Field `RX_DV` reader - Port0 MII RX DV Input Select"]
pub type RX_DV_R = crate::FieldReader<RX_DV_A>;
impl RX_DV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_DV_A {
        match self.bits {
            0 => RX_DV_A::VALUE1,
            1 => RX_DV_A::VALUE2,
            2 => RX_DV_A::VALUE3,
            3 => RX_DV_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input RX_DVA is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RX_DV_A::VALUE1
    }
    #[doc = "Data input RX_DVB is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RX_DV_A::VALUE2
    }
    #[doc = "Data input RX_DVC is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RX_DV_A::VALUE3
    }
    #[doc = "Data input RX_DVD is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RX_DV_A::VALUE4
    }
}
#[doc = "Field `RX_DV` writer - Port0 MII RX DV Input Select"]
pub type RX_DV_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RX_DV_A, crate::Safe>;
impl<'a, REG> RX_DV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input RX_DVA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RX_DV_A::VALUE1)
    }
    #[doc = "Data input RX_DVB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RX_DV_A::VALUE2)
    }
    #[doc = "Data input RX_DVC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(RX_DV_A::VALUE3)
    }
    #[doc = "Data input RX_DVD is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(RX_DV_A::VALUE4)
    }
}
#[doc = "Port0 MII RX Clock Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RX_CLK_A {
    #[doc = "0: Clock input RX_CLKA"]
    VALUE1 = 0,
    #[doc = "1: Clock input RX_CLKB"]
    VALUE2 = 1,
    #[doc = "2: Clock input RX_CLKC"]
    VALUE3 = 2,
    #[doc = "3: Clock input RX_CLKD"]
    VALUE4 = 3,
}
impl From<RX_CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_CLK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RX_CLK_A {
    type Ux = u8;
}
impl crate::IsEnum for RX_CLK_A {}
#[doc = "Field `RX_CLK` reader - Port0 MII RX Clock Input Select"]
pub type RX_CLK_R = crate::FieldReader<RX_CLK_A>;
impl RX_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_CLK_A {
        match self.bits {
            0 => RX_CLK_A::VALUE1,
            1 => RX_CLK_A::VALUE2,
            2 => RX_CLK_A::VALUE3,
            3 => RX_CLK_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Clock input RX_CLKA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RX_CLK_A::VALUE1
    }
    #[doc = "Clock input RX_CLKB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RX_CLK_A::VALUE2
    }
    #[doc = "Clock input RX_CLKC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RX_CLK_A::VALUE3
    }
    #[doc = "Clock input RX_CLKD"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RX_CLK_A::VALUE4
    }
}
#[doc = "Field `RX_CLK` writer - Port0 MII RX Clock Input Select"]
pub type RX_CLK_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RX_CLK_A, crate::Safe>;
impl<'a, REG> RX_CLK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock input RX_CLKA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RX_CLK_A::VALUE1)
    }
    #[doc = "Clock input RX_CLKB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RX_CLK_A::VALUE2)
    }
    #[doc = "Clock input RX_CLKC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(RX_CLK_A::VALUE3)
    }
    #[doc = "Clock input RX_CLKD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(RX_CLK_A::VALUE4)
    }
}
#[doc = "Port0 PHY Link Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LINK_A {
    #[doc = "0: PHY LINKA"]
    VALUE1 = 0,
    #[doc = "1: PHY LINKB"]
    VALUE2 = 1,
    #[doc = "2: PHY LINKC"]
    VALUE3 = 2,
    #[doc = "3: PHY LINKD"]
    VALUE4 = 3,
}
impl From<LINK_A> for u8 {
    #[inline(always)]
    fn from(variant: LINK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LINK_A {
    type Ux = u8;
}
impl crate::IsEnum for LINK_A {}
#[doc = "Field `LINK` reader - Port0 PHY Link Input Select"]
pub type LINK_R = crate::FieldReader<LINK_A>;
impl LINK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LINK_A {
        match self.bits {
            0 => LINK_A::VALUE1,
            1 => LINK_A::VALUE2,
            2 => LINK_A::VALUE3,
            3 => LINK_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "PHY LINKA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LINK_A::VALUE1
    }
    #[doc = "PHY LINKB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LINK_A::VALUE2
    }
    #[doc = "PHY LINKC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LINK_A::VALUE3
    }
    #[doc = "PHY LINKD"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == LINK_A::VALUE4
    }
}
#[doc = "Field `LINK` writer - Port0 PHY Link Input Select"]
pub type LINK_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LINK_A, crate::Safe>;
impl<'a, REG> LINK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PHY LINKA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LINK_A::VALUE1)
    }
    #[doc = "PHY LINKB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LINK_A::VALUE2)
    }
    #[doc = "PHY LINKC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(LINK_A::VALUE3)
    }
    #[doc = "PHY LINKD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(LINK_A::VALUE4)
    }
}
#[doc = "Port0 MII TX Clock Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TX_CLK_A {
    #[doc = "0: Clock input TX_CLKA"]
    VALUE1 = 0,
    #[doc = "1: Clock input TX_CLKB"]
    VALUE2 = 1,
    #[doc = "2: Clock input TX_CLKC"]
    VALUE3 = 2,
    #[doc = "3: Clock input TX_CLKD"]
    VALUE4 = 3,
}
impl From<TX_CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_CLK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TX_CLK_A {
    type Ux = u8;
}
impl crate::IsEnum for TX_CLK_A {}
#[doc = "Field `TX_CLK` reader - Port0 MII TX Clock Input Select"]
pub type TX_CLK_R = crate::FieldReader<TX_CLK_A>;
impl TX_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_CLK_A {
        match self.bits {
            0 => TX_CLK_A::VALUE1,
            1 => TX_CLK_A::VALUE2,
            2 => TX_CLK_A::VALUE3,
            3 => TX_CLK_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Clock input TX_CLKA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TX_CLK_A::VALUE1
    }
    #[doc = "Clock input TX_CLKB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TX_CLK_A::VALUE2
    }
    #[doc = "Clock input TX_CLKC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TX_CLK_A::VALUE3
    }
    #[doc = "Clock input TX_CLKD"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TX_CLK_A::VALUE4
    }
}
#[doc = "Field `TX_CLK` writer - Port0 MII TX Clock Input Select"]
pub type TX_CLK_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TX_CLK_A, crate::Safe>;
impl<'a, REG> TX_CLK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock input TX_CLKA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TX_CLK_A::VALUE1)
    }
    #[doc = "Clock input TX_CLKB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TX_CLK_A::VALUE2)
    }
    #[doc = "Clock input TX_CLKC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(TX_CLK_A::VALUE3)
    }
    #[doc = "Clock input TX_CLKD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(TX_CLK_A::VALUE4)
    }
}
#[doc = "Port0 Manual TX Shift configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TX_SHIFT_A {
    #[doc = "0: 0 ns"]
    VALUE1 = 0,
    #[doc = "1: 10 ns"]
    VALUE2 = 1,
    #[doc = "2: 20 ns"]
    VALUE3 = 2,
    #[doc = "3: 30 ns"]
    VALUE4 = 3,
}
impl From<TX_SHIFT_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_SHIFT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TX_SHIFT_A {
    type Ux = u8;
}
impl crate::IsEnum for TX_SHIFT_A {}
#[doc = "Field `TX_SHIFT` reader - Port0 Manual TX Shift configuration"]
pub type TX_SHIFT_R = crate::FieldReader<TX_SHIFT_A>;
impl TX_SHIFT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_SHIFT_A {
        match self.bits {
            0 => TX_SHIFT_A::VALUE1,
            1 => TX_SHIFT_A::VALUE2,
            2 => TX_SHIFT_A::VALUE3,
            3 => TX_SHIFT_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "0 ns"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TX_SHIFT_A::VALUE1
    }
    #[doc = "10 ns"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TX_SHIFT_A::VALUE2
    }
    #[doc = "20 ns"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TX_SHIFT_A::VALUE3
    }
    #[doc = "30 ns"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TX_SHIFT_A::VALUE4
    }
}
#[doc = "Field `TX_SHIFT` writer - Port0 Manual TX Shift configuration"]
pub type TX_SHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TX_SHIFT_A, crate::Safe>;
impl<'a, REG> TX_SHIFT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 ns"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TX_SHIFT_A::VALUE1)
    }
    #[doc = "10 ns"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TX_SHIFT_A::VALUE2)
    }
    #[doc = "20 ns"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(TX_SHIFT_A::VALUE3)
    }
    #[doc = "30 ns"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(TX_SHIFT_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:1 - PORT0 Receive Input 0 Select"]
    #[inline(always)]
    pub fn rxd0(&self) -> RXD0_R {
        RXD0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port0 Receive Input 1 Select"]
    #[inline(always)]
    pub fn rxd1(&self) -> RXD1_R {
        RXD1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port0 Receive Input 2 Select"]
    #[inline(always)]
    pub fn rxd2(&self) -> RXD2_R {
        RXD2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port0 Receive Input 3 Select"]
    #[inline(always)]
    pub fn rxd3(&self) -> RXD3_R {
        RXD3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port0 MII RX ERROR Input Select"]
    #[inline(always)]
    pub fn rx_err(&self) -> RX_ERR_R {
        RX_ERR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port0 MII RX DV Input Select"]
    #[inline(always)]
    pub fn rx_dv(&self) -> RX_DV_R {
        RX_DV_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port0 MII RX Clock Input Select"]
    #[inline(always)]
    pub fn rx_clk(&self) -> RX_CLK_R {
        RX_CLK_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port0 PHY Link Input Select"]
    #[inline(always)]
    pub fn link(&self) -> LINK_R {
        LINK_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port0 MII TX Clock Input Select"]
    #[inline(always)]
    pub fn tx_clk(&self) -> TX_CLK_R {
        TX_CLK_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port0 Manual TX Shift configuration"]
    #[inline(always)]
    pub fn tx_shift(&self) -> TX_SHIFT_R {
        TX_SHIFT_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PORT0 Receive Input 0 Select"]
    #[inline(always)]
    pub fn rxd0(&mut self) -> RXD0_W<CONP0_SPEC> {
        RXD0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port0 Receive Input 1 Select"]
    #[inline(always)]
    pub fn rxd1(&mut self) -> RXD1_W<CONP0_SPEC> {
        RXD1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port0 Receive Input 2 Select"]
    #[inline(always)]
    pub fn rxd2(&mut self) -> RXD2_W<CONP0_SPEC> {
        RXD2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port0 Receive Input 3 Select"]
    #[inline(always)]
    pub fn rxd3(&mut self) -> RXD3_W<CONP0_SPEC> {
        RXD3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port0 MII RX ERROR Input Select"]
    #[inline(always)]
    pub fn rx_err(&mut self) -> RX_ERR_W<CONP0_SPEC> {
        RX_ERR_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port0 MII RX DV Input Select"]
    #[inline(always)]
    pub fn rx_dv(&mut self) -> RX_DV_W<CONP0_SPEC> {
        RX_DV_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port0 MII RX Clock Input Select"]
    #[inline(always)]
    pub fn rx_clk(&mut self) -> RX_CLK_W<CONP0_SPEC> {
        RX_CLK_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - Port0 PHY Link Input Select"]
    #[inline(always)]
    pub fn link(&mut self) -> LINK_W<CONP0_SPEC> {
        LINK_W::new(self, 16)
    }
    #[doc = "Bits 28:29 - Port0 MII TX Clock Input Select"]
    #[inline(always)]
    pub fn tx_clk(&mut self) -> TX_CLK_W<CONP0_SPEC> {
        TX_CLK_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port0 Manual TX Shift configuration"]
    #[inline(always)]
    pub fn tx_shift(&mut self) -> TX_SHIFT_W<CONP0_SPEC> {
        TX_SHIFT_W::new(self, 30)
    }
}
#[doc = "EtherCAT 0 Port 1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`conp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONP0_SPEC;
impl crate::RegisterSpec for CONP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conp0::R`](R) reader structure"]
impl crate::Readable for CONP0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conp0::W`](W) writer structure"]
impl crate::Writable for CONP0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONP0 to value 0"]
impl crate::Resettable for CONP0_SPEC {
    const RESET_VALUE: u32 = 0;
}
