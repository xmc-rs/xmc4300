#[doc = "Register `ETH0_CON` reader"]
pub type R = crate::R<Eth0ConSpec>;
#[doc = "Register `ETH0_CON` writer"]
pub type W = crate::W<Eth0ConSpec>;
#[doc = "MAC Receive Input 0\n\nValue on reset: 0"]
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
impl crate::IsEnum for Rxd0 {}
#[doc = "Field `RXD0` reader - MAC Receive Input 0"]
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
#[doc = "Field `RXD0` writer - MAC Receive Input 0"]
pub type Rxd0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Rxd0, crate::Safe>;
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
#[doc = "MAC Receive Input 1\n\nValue on reset: 0"]
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
impl crate::IsEnum for Rxd1 {}
#[doc = "Field `RXD1` reader - MAC Receive Input 1"]
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
#[doc = "Field `RXD1` writer - MAC Receive Input 1"]
pub type Rxd1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Rxd1, crate::Safe>;
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
#[doc = "MAC Receive Input 2\n\nValue on reset: 0"]
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
impl crate::IsEnum for Rxd2 {}
#[doc = "Field `RXD2` reader - MAC Receive Input 2"]
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
#[doc = "Field `RXD2` writer - MAC Receive Input 2"]
pub type Rxd2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Rxd2, crate::Safe>;
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
#[doc = "MAC Receive Input 3\n\nValue on reset: 0"]
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
impl crate::IsEnum for Rxd3 {}
#[doc = "Field `RXD3` reader - MAC Receive Input 3"]
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
#[doc = "Field `RXD3` writer - MAC Receive Input 3"]
pub type Rxd3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Rxd3, crate::Safe>;
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
#[doc = "RMII clock input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkRmii {
    #[doc = "0: Data input RMIIA is selected"]
    Value1 = 0,
    #[doc = "1: Data input RMIIB is selected"]
    Value2 = 1,
    #[doc = "2: Data input RMIIC is selected"]
    Value3 = 2,
    #[doc = "3: Data input RMIID is selected"]
    Value4 = 3,
}
impl From<ClkRmii> for u8 {
    #[inline(always)]
    fn from(variant: ClkRmii) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkRmii {
    type Ux = u8;
}
impl crate::IsEnum for ClkRmii {}
#[doc = "Field `CLK_RMII` reader - RMII clock input"]
pub type ClkRmiiR = crate::FieldReader<ClkRmii>;
impl ClkRmiiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkRmii {
        match self.bits {
            0 => ClkRmii::Value1,
            1 => ClkRmii::Value2,
            2 => ClkRmii::Value3,
            3 => ClkRmii::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input RMIIA is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ClkRmii::Value1
    }
    #[doc = "Data input RMIIB is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ClkRmii::Value2
    }
    #[doc = "Data input RMIIC is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ClkRmii::Value3
    }
    #[doc = "Data input RMIID is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ClkRmii::Value4
    }
}
#[doc = "Field `CLK_RMII` writer - RMII clock input"]
pub type ClkRmiiW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkRmii, crate::Safe>;
impl<'a, REG> ClkRmiiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input RMIIA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkRmii::Value1)
    }
    #[doc = "Data input RMIIB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ClkRmii::Value2)
    }
    #[doc = "Data input RMIIC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(ClkRmii::Value3)
    }
    #[doc = "Data input RMIID is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(ClkRmii::Value4)
    }
}
#[doc = "CRS_DV input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CrsDv {
    #[doc = "0: Data input CRS_DVA is selected"]
    Value1 = 0,
    #[doc = "1: Data input CRS_DVB is selected"]
    Value2 = 1,
    #[doc = "2: Data input CRS_DVC is selected"]
    Value3 = 2,
    #[doc = "3: Data input CRS_DVD is selected"]
    Value4 = 3,
}
impl From<CrsDv> for u8 {
    #[inline(always)]
    fn from(variant: CrsDv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CrsDv {
    type Ux = u8;
}
impl crate::IsEnum for CrsDv {}
#[doc = "Field `CRS_DV` reader - CRS_DV input"]
pub type CrsDvR = crate::FieldReader<CrsDv>;
impl CrsDvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CrsDv {
        match self.bits {
            0 => CrsDv::Value1,
            1 => CrsDv::Value2,
            2 => CrsDv::Value3,
            3 => CrsDv::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input CRS_DVA is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CrsDv::Value1
    }
    #[doc = "Data input CRS_DVB is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CrsDv::Value2
    }
    #[doc = "Data input CRS_DVC is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CrsDv::Value3
    }
    #[doc = "Data input CRS_DVD is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CrsDv::Value4
    }
}
#[doc = "Field `CRS_DV` writer - CRS_DV input"]
pub type CrsDvW<'a, REG> = crate::FieldWriter<'a, REG, 2, CrsDv, crate::Safe>;
impl<'a, REG> CrsDvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input CRS_DVA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CrsDv::Value1)
    }
    #[doc = "Data input CRS_DVB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CrsDv::Value2)
    }
    #[doc = "Data input CRS_DVC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CrsDv::Value3)
    }
    #[doc = "Data input CRS_DVD is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CrsDv::Value4)
    }
}
#[doc = "CRS input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Crs {
    #[doc = "0: Data input CRSA"]
    Value1 = 0,
    #[doc = "1: Data input CRSB"]
    Value2 = 1,
    #[doc = "2: Data input CRSC"]
    Value3 = 2,
    #[doc = "3: Data input CRSD"]
    Value4 = 3,
}
impl From<Crs> for u8 {
    #[inline(always)]
    fn from(variant: Crs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Crs {
    type Ux = u8;
}
impl crate::IsEnum for Crs {}
#[doc = "Field `CRS` reader - CRS input"]
pub type CrsR = crate::FieldReader<Crs>;
impl CrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crs {
        match self.bits {
            0 => Crs::Value1,
            1 => Crs::Value2,
            2 => Crs::Value3,
            3 => Crs::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input CRSA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Crs::Value1
    }
    #[doc = "Data input CRSB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Crs::Value2
    }
    #[doc = "Data input CRSC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Crs::Value3
    }
    #[doc = "Data input CRSD"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Crs::Value4
    }
}
#[doc = "Field `CRS` writer - CRS input"]
pub type CrsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Crs, crate::Safe>;
impl<'a, REG> CrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input CRSA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Crs::Value1)
    }
    #[doc = "Data input CRSB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Crs::Value2)
    }
    #[doc = "Data input CRSC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Crs::Value3)
    }
    #[doc = "Data input CRSD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Crs::Value4)
    }
}
#[doc = "RXER Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxer {
    #[doc = "0: Data input RXERA is selected"]
    Value1 = 0,
    #[doc = "1: Data input RXERB is selected"]
    Value2 = 1,
    #[doc = "2: Data input RXERC is selected"]
    Value3 = 2,
    #[doc = "3: Data input RXERD is selected"]
    Value4 = 3,
}
impl From<Rxer> for u8 {
    #[inline(always)]
    fn from(variant: Rxer) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxer {
    type Ux = u8;
}
impl crate::IsEnum for Rxer {}
#[doc = "Field `RXER` reader - RXER Input"]
pub type RxerR = crate::FieldReader<Rxer>;
impl RxerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxer {
        match self.bits {
            0 => Rxer::Value1,
            1 => Rxer::Value2,
            2 => Rxer::Value3,
            3 => Rxer::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input RXERA is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rxer::Value1
    }
    #[doc = "Data input RXERB is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rxer::Value2
    }
    #[doc = "Data input RXERC is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rxer::Value3
    }
    #[doc = "Data input RXERD is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rxer::Value4
    }
}
#[doc = "Field `RXER` writer - RXER Input"]
pub type RxerW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rxer, crate::Safe>;
impl<'a, REG> RxerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input RXERA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxer::Value1)
    }
    #[doc = "Data input RXERB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rxer::Value2)
    }
    #[doc = "Data input RXERC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rxer::Value3)
    }
    #[doc = "Data input RXERD is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rxer::Value4)
    }
}
#[doc = "COL input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Col {
    #[doc = "0: Data input COLA is selected"]
    Value1 = 0,
    #[doc = "1: Data input COLB is selected"]
    Value2 = 1,
    #[doc = "2: Data input COLC is selected"]
    Value3 = 2,
    #[doc = "3: Data input COLD is selected"]
    Value4 = 3,
}
impl From<Col> for u8 {
    #[inline(always)]
    fn from(variant: Col) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Col {
    type Ux = u8;
}
impl crate::IsEnum for Col {}
#[doc = "Field `COL` reader - COL input"]
pub type ColR = crate::FieldReader<Col>;
impl ColR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Col {
        match self.bits {
            0 => Col::Value1,
            1 => Col::Value2,
            2 => Col::Value3,
            3 => Col::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input COLA is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Col::Value1
    }
    #[doc = "Data input COLB is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Col::Value2
    }
    #[doc = "Data input COLC is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Col::Value3
    }
    #[doc = "Data input COLD is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Col::Value4
    }
}
#[doc = "Field `COL` writer - COL input"]
pub type ColW<'a, REG> = crate::FieldWriter<'a, REG, 2, Col, crate::Safe>;
impl<'a, REG> ColW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input COLA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Col::Value1)
    }
    #[doc = "Data input COLB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Col::Value2)
    }
    #[doc = "Data input COLC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Col::Value3)
    }
    #[doc = "Data input COLD is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Col::Value4)
    }
}
#[doc = "CLK_TX input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkTx {
    #[doc = "0: Data input CLK_TXA is selected"]
    Value1 = 0,
    #[doc = "1: Data input CLK_TXB is selected"]
    Value2 = 1,
    #[doc = "2: Data input CLK_TXC is selected"]
    Value3 = 2,
    #[doc = "3: Data input CLK_TXD is selected"]
    Value4 = 3,
}
impl From<ClkTx> for u8 {
    #[inline(always)]
    fn from(variant: ClkTx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkTx {
    type Ux = u8;
}
impl crate::IsEnum for ClkTx {}
#[doc = "Field `CLK_TX` reader - CLK_TX input"]
pub type ClkTxR = crate::FieldReader<ClkTx>;
impl ClkTxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkTx {
        match self.bits {
            0 => ClkTx::Value1,
            1 => ClkTx::Value2,
            2 => ClkTx::Value3,
            3 => ClkTx::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input CLK_TXA is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ClkTx::Value1
    }
    #[doc = "Data input CLK_TXB is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ClkTx::Value2
    }
    #[doc = "Data input CLK_TXC is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ClkTx::Value3
    }
    #[doc = "Data input CLK_TXD is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ClkTx::Value4
    }
}
#[doc = "Field `CLK_TX` writer - CLK_TX input"]
pub type ClkTxW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkTx, crate::Safe>;
impl<'a, REG> ClkTxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input CLK_TXA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkTx::Value1)
    }
    #[doc = "Data input CLK_TXB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ClkTx::Value2)
    }
    #[doc = "Data input CLK_TXC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(ClkTx::Value3)
    }
    #[doc = "Data input CLK_TXD is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(ClkTx::Value4)
    }
}
#[doc = "MDIO Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mdio {
    #[doc = "0: Data input MDIA is selected"]
    Value1 = 0,
    #[doc = "1: Data input MDIB is selected"]
    Value2 = 1,
    #[doc = "2: Data input MDIC is selected"]
    Value3 = 2,
    #[doc = "3: Data input MDID is selected"]
    Value4 = 3,
}
impl From<Mdio> for u8 {
    #[inline(always)]
    fn from(variant: Mdio) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mdio {
    type Ux = u8;
}
impl crate::IsEnum for Mdio {}
#[doc = "Field `MDIO` reader - MDIO Input Select"]
pub type MdioR = crate::FieldReader<Mdio>;
impl MdioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mdio {
        match self.bits {
            0 => Mdio::Value1,
            1 => Mdio::Value2,
            2 => Mdio::Value3,
            3 => Mdio::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input MDIA is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mdio::Value1
    }
    #[doc = "Data input MDIB is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mdio::Value2
    }
    #[doc = "Data input MDIC is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Mdio::Value3
    }
    #[doc = "Data input MDID is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Mdio::Value4
    }
}
#[doc = "Field `MDIO` writer - MDIO Input Select"]
pub type MdioW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mdio, crate::Safe>;
impl<'a, REG> MdioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input MDIA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mdio::Value1)
    }
    #[doc = "Data input MDIB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mdio::Value2)
    }
    #[doc = "Data input MDIC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Mdio::Value3)
    }
    #[doc = "Data input MDID is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Mdio::Value4)
    }
}
#[doc = "Ethernet MAC Interface Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Infsel {
    #[doc = "0: MII"]
    Value1 = 0,
    #[doc = "1: RMII"]
    Value2 = 1,
}
impl From<Infsel> for bool {
    #[inline(always)]
    fn from(variant: Infsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INFSEL` reader - Ethernet MAC Interface Selection"]
pub type InfselR = crate::BitReader<Infsel>;
impl InfselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Infsel {
        match self.bits {
            false => Infsel::Value1,
            true => Infsel::Value2,
        }
    }
    #[doc = "MII"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Infsel::Value1
    }
    #[doc = "RMII"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Infsel::Value2
    }
}
#[doc = "Field `INFSEL` writer - Ethernet MAC Interface Selection"]
pub type InfselW<'a, REG> = crate::BitWriter<'a, REG, Infsel>;
impl<'a, REG> InfselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MII"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Infsel::Value1)
    }
    #[doc = "RMII"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Infsel::Value2)
    }
}
impl R {
    #[doc = "Bits 0:1 - MAC Receive Input 0"]
    #[inline(always)]
    pub fn rxd0(&self) -> Rxd0R {
        Rxd0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - MAC Receive Input 1"]
    #[inline(always)]
    pub fn rxd1(&self) -> Rxd1R {
        Rxd1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - MAC Receive Input 2"]
    #[inline(always)]
    pub fn rxd2(&self) -> Rxd2R {
        Rxd2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - MAC Receive Input 3"]
    #[inline(always)]
    pub fn rxd3(&self) -> Rxd3R {
        Rxd3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - RMII clock input"]
    #[inline(always)]
    pub fn clk_rmii(&self) -> ClkRmiiR {
        ClkRmiiR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - CRS_DV input"]
    #[inline(always)]
    pub fn crs_dv(&self) -> CrsDvR {
        CrsDvR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - CRS input"]
    #[inline(always)]
    pub fn crs(&self) -> CrsR {
        CrsR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - RXER Input"]
    #[inline(always)]
    pub fn rxer(&self) -> RxerR {
        RxerR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - COL input"]
    #[inline(always)]
    pub fn col(&self) -> ColR {
        ColR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - CLK_TX input"]
    #[inline(always)]
    pub fn clk_tx(&self) -> ClkTxR {
        ClkTxR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 22:23 - MDIO Input Select"]
    #[inline(always)]
    pub fn mdio(&self) -> MdioR {
        MdioR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 26 - Ethernet MAC Interface Selection"]
    #[inline(always)]
    pub fn infsel(&self) -> InfselR {
        InfselR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - MAC Receive Input 0"]
    #[inline(always)]
    #[must_use]
    pub fn rxd0(&mut self) -> Rxd0W<Eth0ConSpec> {
        Rxd0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - MAC Receive Input 1"]
    #[inline(always)]
    #[must_use]
    pub fn rxd1(&mut self) -> Rxd1W<Eth0ConSpec> {
        Rxd1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - MAC Receive Input 2"]
    #[inline(always)]
    #[must_use]
    pub fn rxd2(&mut self) -> Rxd2W<Eth0ConSpec> {
        Rxd2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - MAC Receive Input 3"]
    #[inline(always)]
    #[must_use]
    pub fn rxd3(&mut self) -> Rxd3W<Eth0ConSpec> {
        Rxd3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - RMII clock input"]
    #[inline(always)]
    #[must_use]
    pub fn clk_rmii(&mut self) -> ClkRmiiW<Eth0ConSpec> {
        ClkRmiiW::new(self, 8)
    }
    #[doc = "Bits 10:11 - CRS_DV input"]
    #[inline(always)]
    #[must_use]
    pub fn crs_dv(&mut self) -> CrsDvW<Eth0ConSpec> {
        CrsDvW::new(self, 10)
    }
    #[doc = "Bits 12:13 - CRS input"]
    #[inline(always)]
    #[must_use]
    pub fn crs(&mut self) -> CrsW<Eth0ConSpec> {
        CrsW::new(self, 12)
    }
    #[doc = "Bits 14:15 - RXER Input"]
    #[inline(always)]
    #[must_use]
    pub fn rxer(&mut self) -> RxerW<Eth0ConSpec> {
        RxerW::new(self, 14)
    }
    #[doc = "Bits 16:17 - COL input"]
    #[inline(always)]
    #[must_use]
    pub fn col(&mut self) -> ColW<Eth0ConSpec> {
        ColW::new(self, 16)
    }
    #[doc = "Bits 18:19 - CLK_TX input"]
    #[inline(always)]
    #[must_use]
    pub fn clk_tx(&mut self) -> ClkTxW<Eth0ConSpec> {
        ClkTxW::new(self, 18)
    }
    #[doc = "Bits 22:23 - MDIO Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn mdio(&mut self) -> MdioW<Eth0ConSpec> {
        MdioW::new(self, 22)
    }
    #[doc = "Bit 26 - Ethernet MAC Interface Selection"]
    #[inline(always)]
    #[must_use]
    pub fn infsel(&mut self) -> InfselW<Eth0ConSpec> {
        InfselW::new(self, 26)
    }
}
#[doc = "Ethernet 0 Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth0_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth0_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eth0ConSpec;
impl crate::RegisterSpec for Eth0ConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth0_con::R`](R) reader structure"]
impl crate::Readable for Eth0ConSpec {}
#[doc = "`write(|w| ..)` method takes [`eth0_con::W`](W) writer structure"]
impl crate::Writable for Eth0ConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH0_CON to value 0"]
impl crate::Resettable for Eth0ConSpec {
    const RESET_VALUE: u32 = 0;
}
