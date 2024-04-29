#[doc = "Register `ETH0_CON` reader"]
pub type R = crate::R<ETH0_CON_SPEC>;
#[doc = "Register `ETH0_CON` writer"]
pub type W = crate::W<ETH0_CON_SPEC>;
#[doc = "MAC Receive Input 0\n\nValue on reset: 0"]
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
#[doc = "Field `RXD0` reader - MAC Receive Input 0"]
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
#[doc = "Field `RXD0` writer - MAC Receive Input 0"]
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
#[doc = "MAC Receive Input 1\n\nValue on reset: 0"]
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
#[doc = "Field `RXD1` reader - MAC Receive Input 1"]
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
#[doc = "Field `RXD1` writer - MAC Receive Input 1"]
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
#[doc = "MAC Receive Input 2\n\nValue on reset: 0"]
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
#[doc = "Field `RXD2` reader - MAC Receive Input 2"]
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
#[doc = "Field `RXD2` writer - MAC Receive Input 2"]
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
#[doc = "MAC Receive Input 3\n\nValue on reset: 0"]
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
#[doc = "Field `RXD3` reader - MAC Receive Input 3"]
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
#[doc = "Field `RXD3` writer - MAC Receive Input 3"]
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
#[doc = "RMII clock input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK_RMII_A {
    #[doc = "0: Data input RMIIA is selected"]
    VALUE1 = 0,
    #[doc = "1: Data input RMIIB is selected"]
    VALUE2 = 1,
    #[doc = "2: Data input RMIIC is selected"]
    VALUE3 = 2,
    #[doc = "3: Data input RMIID is selected"]
    VALUE4 = 3,
}
impl From<CLK_RMII_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_RMII_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLK_RMII_A {
    type Ux = u8;
}
impl crate::IsEnum for CLK_RMII_A {}
#[doc = "Field `CLK_RMII` reader - RMII clock input"]
pub type CLK_RMII_R = crate::FieldReader<CLK_RMII_A>;
impl CLK_RMII_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLK_RMII_A {
        match self.bits {
            0 => CLK_RMII_A::VALUE1,
            1 => CLK_RMII_A::VALUE2,
            2 => CLK_RMII_A::VALUE3,
            3 => CLK_RMII_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input RMIIA is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CLK_RMII_A::VALUE1
    }
    #[doc = "Data input RMIIB is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CLK_RMII_A::VALUE2
    }
    #[doc = "Data input RMIIC is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CLK_RMII_A::VALUE3
    }
    #[doc = "Data input RMIID is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CLK_RMII_A::VALUE4
    }
}
#[doc = "Field `CLK_RMII` writer - RMII clock input"]
pub type CLK_RMII_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CLK_RMII_A, crate::Safe>;
impl<'a, REG> CLK_RMII_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input RMIIA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_RMII_A::VALUE1)
    }
    #[doc = "Data input RMIIB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_RMII_A::VALUE2)
    }
    #[doc = "Data input RMIIC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_RMII_A::VALUE3)
    }
    #[doc = "Data input RMIID is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_RMII_A::VALUE4)
    }
}
#[doc = "CRS_DV input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CRS_DV_A {
    #[doc = "0: Data input CRS_DVA is selected"]
    VALUE1 = 0,
    #[doc = "1: Data input CRS_DVB is selected"]
    VALUE2 = 1,
    #[doc = "2: Data input CRS_DVC is selected"]
    VALUE3 = 2,
    #[doc = "3: Data input CRS_DVD is selected"]
    VALUE4 = 3,
}
impl From<CRS_DV_A> for u8 {
    #[inline(always)]
    fn from(variant: CRS_DV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CRS_DV_A {
    type Ux = u8;
}
impl crate::IsEnum for CRS_DV_A {}
#[doc = "Field `CRS_DV` reader - CRS_DV input"]
pub type CRS_DV_R = crate::FieldReader<CRS_DV_A>;
impl CRS_DV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRS_DV_A {
        match self.bits {
            0 => CRS_DV_A::VALUE1,
            1 => CRS_DV_A::VALUE2,
            2 => CRS_DV_A::VALUE3,
            3 => CRS_DV_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input CRS_DVA is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CRS_DV_A::VALUE1
    }
    #[doc = "Data input CRS_DVB is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CRS_DV_A::VALUE2
    }
    #[doc = "Data input CRS_DVC is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CRS_DV_A::VALUE3
    }
    #[doc = "Data input CRS_DVD is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CRS_DV_A::VALUE4
    }
}
#[doc = "Field `CRS_DV` writer - CRS_DV input"]
pub type CRS_DV_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CRS_DV_A, crate::Safe>;
impl<'a, REG> CRS_DV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input CRS_DVA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CRS_DV_A::VALUE1)
    }
    #[doc = "Data input CRS_DVB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CRS_DV_A::VALUE2)
    }
    #[doc = "Data input CRS_DVC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CRS_DV_A::VALUE3)
    }
    #[doc = "Data input CRS_DVD is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CRS_DV_A::VALUE4)
    }
}
#[doc = "CRS input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CRS_A {
    #[doc = "0: Data input CRSA"]
    VALUE1 = 0,
    #[doc = "1: Data input CRSB"]
    VALUE2 = 1,
    #[doc = "2: Data input CRSC"]
    VALUE3 = 2,
    #[doc = "3: Data input CRSD"]
    VALUE4 = 3,
}
impl From<CRS_A> for u8 {
    #[inline(always)]
    fn from(variant: CRS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CRS_A {
    type Ux = u8;
}
impl crate::IsEnum for CRS_A {}
#[doc = "Field `CRS` reader - CRS input"]
pub type CRS_R = crate::FieldReader<CRS_A>;
impl CRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRS_A {
        match self.bits {
            0 => CRS_A::VALUE1,
            1 => CRS_A::VALUE2,
            2 => CRS_A::VALUE3,
            3 => CRS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input CRSA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CRS_A::VALUE1
    }
    #[doc = "Data input CRSB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CRS_A::VALUE2
    }
    #[doc = "Data input CRSC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CRS_A::VALUE3
    }
    #[doc = "Data input CRSD"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CRS_A::VALUE4
    }
}
#[doc = "Field `CRS` writer - CRS input"]
pub type CRS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CRS_A, crate::Safe>;
impl<'a, REG> CRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input CRSA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CRS_A::VALUE1)
    }
    #[doc = "Data input CRSB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CRS_A::VALUE2)
    }
    #[doc = "Data input CRSC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CRS_A::VALUE3)
    }
    #[doc = "Data input CRSD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CRS_A::VALUE4)
    }
}
#[doc = "RXER Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXER_A {
    #[doc = "0: Data input RXERA is selected"]
    VALUE1 = 0,
    #[doc = "1: Data input RXERB is selected"]
    VALUE2 = 1,
    #[doc = "2: Data input RXERC is selected"]
    VALUE3 = 2,
    #[doc = "3: Data input RXERD is selected"]
    VALUE4 = 3,
}
impl From<RXER_A> for u8 {
    #[inline(always)]
    fn from(variant: RXER_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXER_A {
    type Ux = u8;
}
impl crate::IsEnum for RXER_A {}
#[doc = "Field `RXER` reader - RXER Input"]
pub type RXER_R = crate::FieldReader<RXER_A>;
impl RXER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXER_A {
        match self.bits {
            0 => RXER_A::VALUE1,
            1 => RXER_A::VALUE2,
            2 => RXER_A::VALUE3,
            3 => RXER_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input RXERA is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXER_A::VALUE1
    }
    #[doc = "Data input RXERB is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RXER_A::VALUE2
    }
    #[doc = "Data input RXERC is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RXER_A::VALUE3
    }
    #[doc = "Data input RXERD is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RXER_A::VALUE4
    }
}
#[doc = "Field `RXER` writer - RXER Input"]
pub type RXER_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RXER_A, crate::Safe>;
impl<'a, REG> RXER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input RXERA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RXER_A::VALUE1)
    }
    #[doc = "Data input RXERB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RXER_A::VALUE2)
    }
    #[doc = "Data input RXERC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(RXER_A::VALUE3)
    }
    #[doc = "Data input RXERD is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(RXER_A::VALUE4)
    }
}
#[doc = "COL input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COL_A {
    #[doc = "0: Data input COLA is selected"]
    VALUE1 = 0,
    #[doc = "1: Data input COLB is selected"]
    VALUE2 = 1,
    #[doc = "2: Data input COLC is selected"]
    VALUE3 = 2,
    #[doc = "3: Data input COLD is selected"]
    VALUE4 = 3,
}
impl From<COL_A> for u8 {
    #[inline(always)]
    fn from(variant: COL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COL_A {
    type Ux = u8;
}
impl crate::IsEnum for COL_A {}
#[doc = "Field `COL` reader - COL input"]
pub type COL_R = crate::FieldReader<COL_A>;
impl COL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COL_A {
        match self.bits {
            0 => COL_A::VALUE1,
            1 => COL_A::VALUE2,
            2 => COL_A::VALUE3,
            3 => COL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input COLA is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == COL_A::VALUE1
    }
    #[doc = "Data input COLB is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == COL_A::VALUE2
    }
    #[doc = "Data input COLC is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == COL_A::VALUE3
    }
    #[doc = "Data input COLD is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == COL_A::VALUE4
    }
}
#[doc = "Field `COL` writer - COL input"]
pub type COL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, COL_A, crate::Safe>;
impl<'a, REG> COL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input COLA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(COL_A::VALUE1)
    }
    #[doc = "Data input COLB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(COL_A::VALUE2)
    }
    #[doc = "Data input COLC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(COL_A::VALUE3)
    }
    #[doc = "Data input COLD is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(COL_A::VALUE4)
    }
}
#[doc = "CLK_TX input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK_TX_A {
    #[doc = "0: Data input CLK_TXA is selected"]
    VALUE1 = 0,
    #[doc = "1: Data input CLK_TXB is selected"]
    VALUE2 = 1,
    #[doc = "2: Data input CLK_TXC is selected"]
    VALUE3 = 2,
    #[doc = "3: Data input CLK_TXD is selected"]
    VALUE4 = 3,
}
impl From<CLK_TX_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_TX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLK_TX_A {
    type Ux = u8;
}
impl crate::IsEnum for CLK_TX_A {}
#[doc = "Field `CLK_TX` reader - CLK_TX input"]
pub type CLK_TX_R = crate::FieldReader<CLK_TX_A>;
impl CLK_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLK_TX_A {
        match self.bits {
            0 => CLK_TX_A::VALUE1,
            1 => CLK_TX_A::VALUE2,
            2 => CLK_TX_A::VALUE3,
            3 => CLK_TX_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input CLK_TXA is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CLK_TX_A::VALUE1
    }
    #[doc = "Data input CLK_TXB is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CLK_TX_A::VALUE2
    }
    #[doc = "Data input CLK_TXC is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CLK_TX_A::VALUE3
    }
    #[doc = "Data input CLK_TXD is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CLK_TX_A::VALUE4
    }
}
#[doc = "Field `CLK_TX` writer - CLK_TX input"]
pub type CLK_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CLK_TX_A, crate::Safe>;
impl<'a, REG> CLK_TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input CLK_TXA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_TX_A::VALUE1)
    }
    #[doc = "Data input CLK_TXB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_TX_A::VALUE2)
    }
    #[doc = "Data input CLK_TXC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_TX_A::VALUE3)
    }
    #[doc = "Data input CLK_TXD is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_TX_A::VALUE4)
    }
}
#[doc = "MDIO Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MDIO_A {
    #[doc = "0: Data input MDIA is selected"]
    VALUE1 = 0,
    #[doc = "1: Data input MDIB is selected"]
    VALUE2 = 1,
    #[doc = "2: Data input MDIC is selected"]
    VALUE3 = 2,
    #[doc = "3: Data input MDID is selected"]
    VALUE4 = 3,
}
impl From<MDIO_A> for u8 {
    #[inline(always)]
    fn from(variant: MDIO_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MDIO_A {
    type Ux = u8;
}
impl crate::IsEnum for MDIO_A {}
#[doc = "Field `MDIO` reader - MDIO Input Select"]
pub type MDIO_R = crate::FieldReader<MDIO_A>;
impl MDIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MDIO_A {
        match self.bits {
            0 => MDIO_A::VALUE1,
            1 => MDIO_A::VALUE2,
            2 => MDIO_A::VALUE3,
            3 => MDIO_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input MDIA is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MDIO_A::VALUE1
    }
    #[doc = "Data input MDIB is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MDIO_A::VALUE2
    }
    #[doc = "Data input MDIC is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == MDIO_A::VALUE3
    }
    #[doc = "Data input MDID is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == MDIO_A::VALUE4
    }
}
#[doc = "Field `MDIO` writer - MDIO Input Select"]
pub type MDIO_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MDIO_A, crate::Safe>;
impl<'a, REG> MDIO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input MDIA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MDIO_A::VALUE1)
    }
    #[doc = "Data input MDIB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MDIO_A::VALUE2)
    }
    #[doc = "Data input MDIC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(MDIO_A::VALUE3)
    }
    #[doc = "Data input MDID is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(MDIO_A::VALUE4)
    }
}
#[doc = "Ethernet MAC Interface Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INFSEL_A {
    #[doc = "0: MII"]
    VALUE1 = 0,
    #[doc = "1: RMII"]
    VALUE2 = 1,
}
impl From<INFSEL_A> for bool {
    #[inline(always)]
    fn from(variant: INFSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INFSEL` reader - Ethernet MAC Interface Selection"]
pub type INFSEL_R = crate::BitReader<INFSEL_A>;
impl INFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INFSEL_A {
        match self.bits {
            false => INFSEL_A::VALUE1,
            true => INFSEL_A::VALUE2,
        }
    }
    #[doc = "MII"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == INFSEL_A::VALUE1
    }
    #[doc = "RMII"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INFSEL_A::VALUE2
    }
}
#[doc = "Field `INFSEL` writer - Ethernet MAC Interface Selection"]
pub type INFSEL_W<'a, REG> = crate::BitWriter<'a, REG, INFSEL_A>;
impl<'a, REG> INFSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MII"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(INFSEL_A::VALUE1)
    }
    #[doc = "RMII"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(INFSEL_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:1 - MAC Receive Input 0"]
    #[inline(always)]
    pub fn rxd0(&self) -> RXD0_R {
        RXD0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - MAC Receive Input 1"]
    #[inline(always)]
    pub fn rxd1(&self) -> RXD1_R {
        RXD1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - MAC Receive Input 2"]
    #[inline(always)]
    pub fn rxd2(&self) -> RXD2_R {
        RXD2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - MAC Receive Input 3"]
    #[inline(always)]
    pub fn rxd3(&self) -> RXD3_R {
        RXD3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - RMII clock input"]
    #[inline(always)]
    pub fn clk_rmii(&self) -> CLK_RMII_R {
        CLK_RMII_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - CRS_DV input"]
    #[inline(always)]
    pub fn crs_dv(&self) -> CRS_DV_R {
        CRS_DV_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - CRS input"]
    #[inline(always)]
    pub fn crs(&self) -> CRS_R {
        CRS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - RXER Input"]
    #[inline(always)]
    pub fn rxer(&self) -> RXER_R {
        RXER_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - COL input"]
    #[inline(always)]
    pub fn col(&self) -> COL_R {
        COL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - CLK_TX input"]
    #[inline(always)]
    pub fn clk_tx(&self) -> CLK_TX_R {
        CLK_TX_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 22:23 - MDIO Input Select"]
    #[inline(always)]
    pub fn mdio(&self) -> MDIO_R {
        MDIO_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 26 - Ethernet MAC Interface Selection"]
    #[inline(always)]
    pub fn infsel(&self) -> INFSEL_R {
        INFSEL_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - MAC Receive Input 0"]
    #[inline(always)]
    #[must_use]
    pub fn rxd0(&mut self) -> RXD0_W<ETH0_CON_SPEC> {
        RXD0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - MAC Receive Input 1"]
    #[inline(always)]
    #[must_use]
    pub fn rxd1(&mut self) -> RXD1_W<ETH0_CON_SPEC> {
        RXD1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - MAC Receive Input 2"]
    #[inline(always)]
    #[must_use]
    pub fn rxd2(&mut self) -> RXD2_W<ETH0_CON_SPEC> {
        RXD2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - MAC Receive Input 3"]
    #[inline(always)]
    #[must_use]
    pub fn rxd3(&mut self) -> RXD3_W<ETH0_CON_SPEC> {
        RXD3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - RMII clock input"]
    #[inline(always)]
    #[must_use]
    pub fn clk_rmii(&mut self) -> CLK_RMII_W<ETH0_CON_SPEC> {
        CLK_RMII_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - CRS_DV input"]
    #[inline(always)]
    #[must_use]
    pub fn crs_dv(&mut self) -> CRS_DV_W<ETH0_CON_SPEC> {
        CRS_DV_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - CRS input"]
    #[inline(always)]
    #[must_use]
    pub fn crs(&mut self) -> CRS_W<ETH0_CON_SPEC> {
        CRS_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - RXER Input"]
    #[inline(always)]
    #[must_use]
    pub fn rxer(&mut self) -> RXER_W<ETH0_CON_SPEC> {
        RXER_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - COL input"]
    #[inline(always)]
    #[must_use]
    pub fn col(&mut self) -> COL_W<ETH0_CON_SPEC> {
        COL_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - CLK_TX input"]
    #[inline(always)]
    #[must_use]
    pub fn clk_tx(&mut self) -> CLK_TX_W<ETH0_CON_SPEC> {
        CLK_TX_W::new(self, 18)
    }
    #[doc = "Bits 22:23 - MDIO Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn mdio(&mut self) -> MDIO_W<ETH0_CON_SPEC> {
        MDIO_W::new(self, 22)
    }
    #[doc = "Bit 26 - Ethernet MAC Interface Selection"]
    #[inline(always)]
    #[must_use]
    pub fn infsel(&mut self) -> INFSEL_W<ETH0_CON_SPEC> {
        INFSEL_W::new(self, 26)
    }
}
#[doc = "Ethernet 0 Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth0_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth0_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH0_CON_SPEC;
impl crate::RegisterSpec for ETH0_CON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth0_con::R`](R) reader structure"]
impl crate::Readable for ETH0_CON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eth0_con::W`](W) writer structure"]
impl crate::Writable for ETH0_CON_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH0_CON to value 0"]
impl crate::Resettable for ETH0_CON_SPEC {
    const RESET_VALUE: u32 = 0;
}
