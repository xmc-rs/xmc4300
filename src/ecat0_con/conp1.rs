#[doc = "Register `CONP1` reader"]
pub type R = crate::R<Conp1Spec>;
#[doc = "Register `CONP1` writer"]
pub type W = crate::W<Conp1Spec>;
#[doc = "Port1 Receive Input 0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxd0 {
    #[doc = "0: Data input RXD0A is selected"]
    Value1 = 0,
    #[doc = "1: Data input RXD0B is selected"]
    Value2 = 1,
    #[doc = "2: Data input RXD0C is selected"]
    Value3 = 2,
    #[doc = "3: Data input RXD0D is selected"]
    Value4 = 3,
}
impl From<Rxd0> for u8 {
    #[inline(always)]
    fn from(variant: Rxd0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxd0 {
    type Ux = u8;
}
#[doc = "Field `RXD0` reader - Port1 Receive Input 0 Select"]
pub type Rxd0R = crate::FieldReader<Rxd0>;
impl Rxd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxd0 {
        match self.bits {
            0 => Rxd0::Value1,
            1 => Rxd0::Value2,
            2 => Rxd0::Value3,
            3 => Rxd0::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input RXD0A is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rxd0::Value1
    }
    #[doc = "Data input RXD0B is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rxd0::Value2
    }
    #[doc = "Data input RXD0C is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rxd0::Value3
    }
    #[doc = "Data input RXD0D is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rxd0::Value4
    }
}
#[doc = "Field `RXD0` writer - Port1 Receive Input 0 Select"]
pub type Rxd0W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Rxd0>;
impl<'a, REG> Rxd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input RXD0A is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxd0::Value1)
    }
    #[doc = "Data input RXD0B is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rxd0::Value2)
    }
    #[doc = "Data input RXD0C is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rxd0::Value3)
    }
    #[doc = "Data input RXD0D is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rxd0::Value4)
    }
}
#[doc = "Port1 Receive Input 1 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxd1 {
    #[doc = "0: Data input RXD1A is selected"]
    Value1 = 0,
    #[doc = "1: Data input RXD1B is selected"]
    Value2 = 1,
    #[doc = "2: Data input RXD1C is selected"]
    Value3 = 2,
    #[doc = "3: Data input RXD1D is selected"]
    Value4 = 3,
}
impl From<Rxd1> for u8 {
    #[inline(always)]
    fn from(variant: Rxd1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxd1 {
    type Ux = u8;
}
#[doc = "Field `RXD1` reader - Port1 Receive Input 1 Select"]
pub type Rxd1R = crate::FieldReader<Rxd1>;
impl Rxd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxd1 {
        match self.bits {
            0 => Rxd1::Value1,
            1 => Rxd1::Value2,
            2 => Rxd1::Value3,
            3 => Rxd1::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input RXD1A is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rxd1::Value1
    }
    #[doc = "Data input RXD1B is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rxd1::Value2
    }
    #[doc = "Data input RXD1C is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rxd1::Value3
    }
    #[doc = "Data input RXD1D is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rxd1::Value4
    }
}
#[doc = "Field `RXD1` writer - Port1 Receive Input 1 Select"]
pub type Rxd1W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Rxd1>;
impl<'a, REG> Rxd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input RXD1A is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxd1::Value1)
    }
    #[doc = "Data input RXD1B is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rxd1::Value2)
    }
    #[doc = "Data input RXD1C is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rxd1::Value3)
    }
    #[doc = "Data input RXD1D is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rxd1::Value4)
    }
}
#[doc = "Port1 Receive Input 2 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxd2 {
    #[doc = "0: Data input RXD2A is selected"]
    Value1 = 0,
    #[doc = "1: Data input RXD2B is selected"]
    Value2 = 1,
    #[doc = "2: Data input RXD2C is selected"]
    Value3 = 2,
    #[doc = "3: Data input RXD2D is selected"]
    Value4 = 3,
}
impl From<Rxd2> for u8 {
    #[inline(always)]
    fn from(variant: Rxd2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxd2 {
    type Ux = u8;
}
#[doc = "Field `RXD2` reader - Port1 Receive Input 2 Select"]
pub type Rxd2R = crate::FieldReader<Rxd2>;
impl Rxd2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxd2 {
        match self.bits {
            0 => Rxd2::Value1,
            1 => Rxd2::Value2,
            2 => Rxd2::Value3,
            3 => Rxd2::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input RXD2A is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rxd2::Value1
    }
    #[doc = "Data input RXD2B is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rxd2::Value2
    }
    #[doc = "Data input RXD2C is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rxd2::Value3
    }
    #[doc = "Data input RXD2D is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rxd2::Value4
    }
}
#[doc = "Field `RXD2` writer - Port1 Receive Input 2 Select"]
pub type Rxd2W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Rxd2>;
impl<'a, REG> Rxd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input RXD2A is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxd2::Value1)
    }
    #[doc = "Data input RXD2B is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rxd2::Value2)
    }
    #[doc = "Data input RXD2C is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rxd2::Value3)
    }
    #[doc = "Data input RXD2D is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rxd2::Value4)
    }
}
#[doc = "Port1 Receive Input 3 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxd3 {
    #[doc = "0: Data input RXD3A is selected"]
    Value1 = 0,
    #[doc = "1: Data input RXD3B is selected"]
    Value2 = 1,
    #[doc = "2: Data input RXD3C is selected"]
    Value3 = 2,
    #[doc = "3: Data input RXD3D is selected"]
    Value4 = 3,
}
impl From<Rxd3> for u8 {
    #[inline(always)]
    fn from(variant: Rxd3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxd3 {
    type Ux = u8;
}
#[doc = "Field `RXD3` reader - Port1 Receive Input 3 Select"]
pub type Rxd3R = crate::FieldReader<Rxd3>;
impl Rxd3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxd3 {
        match self.bits {
            0 => Rxd3::Value1,
            1 => Rxd3::Value2,
            2 => Rxd3::Value3,
            3 => Rxd3::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input RXD3A is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rxd3::Value1
    }
    #[doc = "Data input RXD3B is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rxd3::Value2
    }
    #[doc = "Data input RXD3C is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rxd3::Value3
    }
    #[doc = "Data input RXD3D is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rxd3::Value4
    }
}
#[doc = "Field `RXD3` writer - Port1 Receive Input 3 Select"]
pub type Rxd3W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Rxd3>;
impl<'a, REG> Rxd3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input RXD3A is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxd3::Value1)
    }
    #[doc = "Data input RXD3B is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rxd3::Value2)
    }
    #[doc = "Data input RXD3C is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rxd3::Value3)
    }
    #[doc = "Data input RXD3D is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rxd3::Value4)
    }
}
#[doc = "Port1 MII RX ERROR Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RxErr {
    #[doc = "0: Data input RX_ERRA is selected"]
    Value1 = 0,
    #[doc = "1: Data input RX_ERRB is selected"]
    Value2 = 1,
    #[doc = "2: Data input RX_ERRC is selected"]
    Value3 = 2,
    #[doc = "3: Data input RX_ERRD is selected"]
    Value4 = 3,
}
impl From<RxErr> for u8 {
    #[inline(always)]
    fn from(variant: RxErr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RxErr {
    type Ux = u8;
}
#[doc = "Field `RX_ERR` reader - Port1 MII RX ERROR Input Select"]
pub type RxErrR = crate::FieldReader<RxErr>;
impl RxErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxErr {
        match self.bits {
            0 => RxErr::Value1,
            1 => RxErr::Value2,
            2 => RxErr::Value3,
            3 => RxErr::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input RX_ERRA is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RxErr::Value1
    }
    #[doc = "Data input RX_ERRB is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RxErr::Value2
    }
    #[doc = "Data input RX_ERRC is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RxErr::Value3
    }
    #[doc = "Data input RX_ERRD is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RxErr::Value4
    }
}
#[doc = "Field `RX_ERR` writer - Port1 MII RX ERROR Input Select"]
pub type RxErrW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RxErr>;
impl<'a, REG> RxErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input RX_ERRA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RxErr::Value1)
    }
    #[doc = "Data input RX_ERRB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RxErr::Value2)
    }
    #[doc = "Data input RX_ERRC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(RxErr::Value3)
    }
    #[doc = "Data input RX_ERRD is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(RxErr::Value4)
    }
}
#[doc = "Port1 MII RX DV Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RxDv {
    #[doc = "0: Data input RX_DVA is selected"]
    Value1 = 0,
    #[doc = "1: Data input RX_DVB is selected"]
    Value2 = 1,
    #[doc = "2: Data input RX_DVC is selected"]
    Value3 = 2,
    #[doc = "3: Data input RX_DVD is selected"]
    Value4 = 3,
}
impl From<RxDv> for u8 {
    #[inline(always)]
    fn from(variant: RxDv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RxDv {
    type Ux = u8;
}
#[doc = "Field `RX_DV` reader - Port1 MII RX DV Input Select"]
pub type RxDvR = crate::FieldReader<RxDv>;
impl RxDvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxDv {
        match self.bits {
            0 => RxDv::Value1,
            1 => RxDv::Value2,
            2 => RxDv::Value3,
            3 => RxDv::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input RX_DVA is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RxDv::Value1
    }
    #[doc = "Data input RX_DVB is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RxDv::Value2
    }
    #[doc = "Data input RX_DVC is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RxDv::Value3
    }
    #[doc = "Data input RX_DVD is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RxDv::Value4
    }
}
#[doc = "Field `RX_DV` writer - Port1 MII RX DV Input Select"]
pub type RxDvW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RxDv>;
impl<'a, REG> RxDvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input RX_DVA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RxDv::Value1)
    }
    #[doc = "Data input RX_DVB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RxDv::Value2)
    }
    #[doc = "Data input RX_DVC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(RxDv::Value3)
    }
    #[doc = "Data input RX_DVD is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(RxDv::Value4)
    }
}
#[doc = "Port1 MII RX Clock Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RxClk {
    #[doc = "0: Clock input RX_CLKA"]
    Value1 = 0,
    #[doc = "1: Clock input RX_CLKB"]
    Value2 = 1,
    #[doc = "2: Clock input RX_CLKC"]
    Value3 = 2,
    #[doc = "3: Clock input RX_CLKD"]
    Value4 = 3,
}
impl From<RxClk> for u8 {
    #[inline(always)]
    fn from(variant: RxClk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RxClk {
    type Ux = u8;
}
#[doc = "Field `RX_CLK` reader - Port1 MII RX Clock Input Select"]
pub type RxClkR = crate::FieldReader<RxClk>;
impl RxClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxClk {
        match self.bits {
            0 => RxClk::Value1,
            1 => RxClk::Value2,
            2 => RxClk::Value3,
            3 => RxClk::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Clock input RX_CLKA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RxClk::Value1
    }
    #[doc = "Clock input RX_CLKB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RxClk::Value2
    }
    #[doc = "Clock input RX_CLKC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RxClk::Value3
    }
    #[doc = "Clock input RX_CLKD"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RxClk::Value4
    }
}
#[doc = "Field `RX_CLK` writer - Port1 MII RX Clock Input Select"]
pub type RxClkW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RxClk>;
impl<'a, REG> RxClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock input RX_CLKA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RxClk::Value1)
    }
    #[doc = "Clock input RX_CLKB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RxClk::Value2)
    }
    #[doc = "Clock input RX_CLKC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(RxClk::Value3)
    }
    #[doc = "Clock input RX_CLKD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(RxClk::Value4)
    }
}
#[doc = "Port1 PHY Link Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Link {
    #[doc = "0: PHY LINKA"]
    Value1 = 0,
    #[doc = "1: PHY LINKB"]
    Value2 = 1,
    #[doc = "2: PHY LINKC"]
    Value3 = 2,
    #[doc = "3: PHY LINKD"]
    Value4 = 3,
}
impl From<Link> for u8 {
    #[inline(always)]
    fn from(variant: Link) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Link {
    type Ux = u8;
}
#[doc = "Field `LINK` reader - Port1 PHY Link Input Select"]
pub type LinkR = crate::FieldReader<Link>;
impl LinkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Link {
        match self.bits {
            0 => Link::Value1,
            1 => Link::Value2,
            2 => Link::Value3,
            3 => Link::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "PHY LINKA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Link::Value1
    }
    #[doc = "PHY LINKB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Link::Value2
    }
    #[doc = "PHY LINKC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Link::Value3
    }
    #[doc = "PHY LINKD"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Link::Value4
    }
}
#[doc = "Field `LINK` writer - Port1 PHY Link Input Select"]
pub type LinkW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Link>;
impl<'a, REG> LinkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PHY LINKA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Link::Value1)
    }
    #[doc = "PHY LINKB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Link::Value2)
    }
    #[doc = "PHY LINKC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Link::Value3)
    }
    #[doc = "PHY LINKD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Link::Value4)
    }
}
#[doc = "Port1 MII TX Clock Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TxClk {
    #[doc = "0: Clock input TX_CLKA"]
    Value1 = 0,
    #[doc = "1: Clock input TX_CLKB"]
    Value2 = 1,
    #[doc = "2: Clock input TX_CLKC"]
    Value3 = 2,
    #[doc = "3: Clock input TX_CLKD"]
    Value4 = 3,
}
impl From<TxClk> for u8 {
    #[inline(always)]
    fn from(variant: TxClk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TxClk {
    type Ux = u8;
}
#[doc = "Field `TX_CLK` reader - Port1 MII TX Clock Input Select"]
pub type TxClkR = crate::FieldReader<TxClk>;
impl TxClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxClk {
        match self.bits {
            0 => TxClk::Value1,
            1 => TxClk::Value2,
            2 => TxClk::Value3,
            3 => TxClk::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Clock input TX_CLKA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TxClk::Value1
    }
    #[doc = "Clock input TX_CLKB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TxClk::Value2
    }
    #[doc = "Clock input TX_CLKC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TxClk::Value3
    }
    #[doc = "Clock input TX_CLKD"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TxClk::Value4
    }
}
#[doc = "Field `TX_CLK` writer - Port1 MII TX Clock Input Select"]
pub type TxClkW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TxClk>;
impl<'a, REG> TxClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock input TX_CLKA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TxClk::Value1)
    }
    #[doc = "Clock input TX_CLKB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TxClk::Value2)
    }
    #[doc = "Clock input TX_CLKC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(TxClk::Value3)
    }
    #[doc = "Clock input TX_CLKD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(TxClk::Value4)
    }
}
#[doc = "Port1 Manual TX Shift configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TxShift {
    #[doc = "0: 0 ns"]
    Value1 = 0,
    #[doc = "1: 10 ns"]
    Value2 = 1,
    #[doc = "2: 20 ns"]
    Value3 = 2,
    #[doc = "3: 30 ns"]
    Value4 = 3,
}
impl From<TxShift> for u8 {
    #[inline(always)]
    fn from(variant: TxShift) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TxShift {
    type Ux = u8;
}
#[doc = "Field `TX_SHIFT` reader - Port1 Manual TX Shift configuration"]
pub type TxShiftR = crate::FieldReader<TxShift>;
impl TxShiftR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxShift {
        match self.bits {
            0 => TxShift::Value1,
            1 => TxShift::Value2,
            2 => TxShift::Value3,
            3 => TxShift::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "0 ns"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TxShift::Value1
    }
    #[doc = "10 ns"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TxShift::Value2
    }
    #[doc = "20 ns"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TxShift::Value3
    }
    #[doc = "30 ns"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TxShift::Value4
    }
}
#[doc = "Field `TX_SHIFT` writer - Port1 Manual TX Shift configuration"]
pub type TxShiftW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TxShift>;
impl<'a, REG> TxShiftW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 ns"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TxShift::Value1)
    }
    #[doc = "10 ns"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TxShift::Value2)
    }
    #[doc = "20 ns"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(TxShift::Value3)
    }
    #[doc = "30 ns"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(TxShift::Value4)
    }
}
impl R {
    #[doc = "Bits 0:1 - Port1 Receive Input 0 Select"]
    #[inline(always)]
    pub fn rxd0(&self) -> Rxd0R {
        Rxd0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port1 Receive Input 1 Select"]
    #[inline(always)]
    pub fn rxd1(&self) -> Rxd1R {
        Rxd1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port1 Receive Input 2 Select"]
    #[inline(always)]
    pub fn rxd2(&self) -> Rxd2R {
        Rxd2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port1 Receive Input 3 Select"]
    #[inline(always)]
    pub fn rxd3(&self) -> Rxd3R {
        Rxd3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port1 MII RX ERROR Input Select"]
    #[inline(always)]
    pub fn rx_err(&self) -> RxErrR {
        RxErrR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port1 MII RX DV Input Select"]
    #[inline(always)]
    pub fn rx_dv(&self) -> RxDvR {
        RxDvR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port1 MII RX Clock Input Select"]
    #[inline(always)]
    pub fn rx_clk(&self) -> RxClkR {
        RxClkR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port1 PHY Link Input Select"]
    #[inline(always)]
    pub fn link(&self) -> LinkR {
        LinkR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port1 MII TX Clock Input Select"]
    #[inline(always)]
    pub fn tx_clk(&self) -> TxClkR {
        TxClkR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port1 Manual TX Shift configuration"]
    #[inline(always)]
    pub fn tx_shift(&self) -> TxShiftR {
        TxShiftR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port1 Receive Input 0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn rxd0(&mut self) -> Rxd0W<Conp1Spec> {
        Rxd0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port1 Receive Input 1 Select"]
    #[inline(always)]
    #[must_use]
    pub fn rxd1(&mut self) -> Rxd1W<Conp1Spec> {
        Rxd1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port1 Receive Input 2 Select"]
    #[inline(always)]
    #[must_use]
    pub fn rxd2(&mut self) -> Rxd2W<Conp1Spec> {
        Rxd2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port1 Receive Input 3 Select"]
    #[inline(always)]
    #[must_use]
    pub fn rxd3(&mut self) -> Rxd3W<Conp1Spec> {
        Rxd3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port1 MII RX ERROR Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn rx_err(&mut self) -> RxErrW<Conp1Spec> {
        RxErrW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port1 MII RX DV Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn rx_dv(&mut self) -> RxDvW<Conp1Spec> {
        RxDvW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port1 MII RX Clock Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn rx_clk(&mut self) -> RxClkW<Conp1Spec> {
        RxClkW::new(self, 12)
    }
    #[doc = "Bits 16:17 - Port1 PHY Link Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn link(&mut self) -> LinkW<Conp1Spec> {
        LinkW::new(self, 16)
    }
    #[doc = "Bits 28:29 - Port1 MII TX Clock Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn tx_clk(&mut self) -> TxClkW<Conp1Spec> {
        TxClkW::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port1 Manual TX Shift configuration"]
    #[inline(always)]
    #[must_use]
    pub fn tx_shift(&mut self) -> TxShiftW<Conp1Spec> {
        TxShiftW::new(self, 30)
    }
}
#[doc = "EtherCAT 0 Port 1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conp1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conp1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Conp1Spec;
impl crate::RegisterSpec for Conp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conp1::R`](R) reader structure"]
impl crate::Readable for Conp1Spec {}
#[doc = "`write(|w| ..)` method takes [`conp1::W`](W) writer structure"]
impl crate::Writable for Conp1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONP1 to value 0"]
impl crate::Resettable for Conp1Spec {
    const RESET_VALUE: u32 = 0;
}
