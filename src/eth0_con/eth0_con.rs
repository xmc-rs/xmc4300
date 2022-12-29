#[doc = "Register `ETH0_CON` reader"]
pub struct R(crate::R<ETH0_CON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH0_CON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH0_CON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH0_CON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH0_CON` writer"]
pub struct W(crate::W<ETH0_CON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH0_CON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ETH0_CON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH0_CON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXD0` reader - MAC Receive Input 0"]
pub type RXD0_R = crate::FieldReader<u8, RXD0_A>;
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
impl RXD0_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `RXD0` writer - MAC Receive Input 0"]
pub type RXD0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ETH0_CON_SPEC, u8, RXD0_A, 2, O>;
impl<'a, const O: u8> RXD0_W<'a, O> {
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
}
#[doc = "Field `RXD1` reader - MAC Receive Input 1"]
pub type RXD1_R = crate::FieldReader<u8, RXD1_A>;
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
impl RXD1_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `RXD1` writer - MAC Receive Input 1"]
pub type RXD1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ETH0_CON_SPEC, u8, RXD1_A, 2, O>;
impl<'a, const O: u8> RXD1_W<'a, O> {
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
}
#[doc = "Field `RXD2` reader - MAC Receive Input 2"]
pub type RXD2_R = crate::FieldReader<u8, RXD2_A>;
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
impl RXD2_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `RXD2` writer - MAC Receive Input 2"]
pub type RXD2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ETH0_CON_SPEC, u8, RXD2_A, 2, O>;
impl<'a, const O: u8> RXD2_W<'a, O> {
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
}
#[doc = "Field `RXD3` reader - MAC Receive Input 3"]
pub type RXD3_R = crate::FieldReader<u8, RXD3_A>;
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
impl RXD3_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `RXD3` writer - MAC Receive Input 3"]
pub type RXD3_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ETH0_CON_SPEC, u8, RXD3_A, 2, O>;
impl<'a, const O: u8> RXD3_W<'a, O> {
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
}
#[doc = "Field `CLK_RMII` reader - RMII clock input"]
pub type CLK_RMII_R = crate::FieldReader<u8, CLK_RMII_A>;
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
impl CLK_RMII_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_RMII_A {
        match self.bits {
            0 => CLK_RMII_A::VALUE1,
            1 => CLK_RMII_A::VALUE2,
            2 => CLK_RMII_A::VALUE3,
            3 => CLK_RMII_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CLK_RMII_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CLK_RMII_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CLK_RMII_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CLK_RMII_A::VALUE4
    }
}
#[doc = "Field `CLK_RMII` writer - RMII clock input"]
pub type CLK_RMII_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ETH0_CON_SPEC, u8, CLK_RMII_A, 2, O>;
impl<'a, const O: u8> CLK_RMII_W<'a, O> {
    #[doc = "Data input RMIIA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLK_RMII_A::VALUE1)
    }
    #[doc = "Data input RMIIB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CLK_RMII_A::VALUE2)
    }
    #[doc = "Data input RMIIC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CLK_RMII_A::VALUE3)
    }
    #[doc = "Data input RMIID is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CLK_RMII_A::VALUE4)
    }
}
#[doc = "Field `CRS_DV` reader - CRS_DV input"]
pub type CRS_DV_R = crate::FieldReader<u8, CRS_DV_A>;
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
impl CRS_DV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRS_DV_A {
        match self.bits {
            0 => CRS_DV_A::VALUE1,
            1 => CRS_DV_A::VALUE2,
            2 => CRS_DV_A::VALUE3,
            3 => CRS_DV_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CRS_DV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CRS_DV_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CRS_DV_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CRS_DV_A::VALUE4
    }
}
#[doc = "Field `CRS_DV` writer - CRS_DV input"]
pub type CRS_DV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ETH0_CON_SPEC, u8, CRS_DV_A, 2, O>;
impl<'a, const O: u8> CRS_DV_W<'a, O> {
    #[doc = "Data input CRS_DVA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CRS_DV_A::VALUE1)
    }
    #[doc = "Data input CRS_DVB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CRS_DV_A::VALUE2)
    }
    #[doc = "Data input CRS_DVC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CRS_DV_A::VALUE3)
    }
    #[doc = "Data input CRS_DVD is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CRS_DV_A::VALUE4)
    }
}
#[doc = "Field `CRS` reader - CRS input"]
pub type CRS_R = crate::FieldReader<u8, CRS_A>;
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
impl CRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRS_A {
        match self.bits {
            0 => CRS_A::VALUE1,
            1 => CRS_A::VALUE2,
            2 => CRS_A::VALUE3,
            3 => CRS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CRS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CRS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CRS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CRS_A::VALUE4
    }
}
#[doc = "Field `CRS` writer - CRS input"]
pub type CRS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ETH0_CON_SPEC, u8, CRS_A, 2, O>;
impl<'a, const O: u8> CRS_W<'a, O> {
    #[doc = "Data input CRSA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CRS_A::VALUE1)
    }
    #[doc = "Data input CRSB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CRS_A::VALUE2)
    }
    #[doc = "Data input CRSC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CRS_A::VALUE3)
    }
    #[doc = "Data input CRSD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CRS_A::VALUE4)
    }
}
#[doc = "Field `RXER` reader - RXER Input"]
pub type RXER_R = crate::FieldReader<u8, RXER_A>;
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
impl RXER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXER_A {
        match self.bits {
            0 => RXER_A::VALUE1,
            1 => RXER_A::VALUE2,
            2 => RXER_A::VALUE3,
            3 => RXER_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXER_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RXER_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RXER_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RXER_A::VALUE4
    }
}
#[doc = "Field `RXER` writer - RXER Input"]
pub type RXER_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ETH0_CON_SPEC, u8, RXER_A, 2, O>;
impl<'a, const O: u8> RXER_W<'a, O> {
    #[doc = "Data input RXERA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXER_A::VALUE1)
    }
    #[doc = "Data input RXERB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXER_A::VALUE2)
    }
    #[doc = "Data input RXERC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(RXER_A::VALUE3)
    }
    #[doc = "Data input RXERD is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(RXER_A::VALUE4)
    }
}
#[doc = "Field `COL` reader - COL input"]
pub type COL_R = crate::FieldReader<u8, COL_A>;
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
impl COL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COL_A {
        match self.bits {
            0 => COL_A::VALUE1,
            1 => COL_A::VALUE2,
            2 => COL_A::VALUE3,
            3 => COL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == COL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == COL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == COL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == COL_A::VALUE4
    }
}
#[doc = "Field `COL` writer - COL input"]
pub type COL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ETH0_CON_SPEC, u8, COL_A, 2, O>;
impl<'a, const O: u8> COL_W<'a, O> {
    #[doc = "Data input COLA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(COL_A::VALUE1)
    }
    #[doc = "Data input COLB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(COL_A::VALUE2)
    }
    #[doc = "Data input COLC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(COL_A::VALUE3)
    }
    #[doc = "Data input COLD is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(COL_A::VALUE4)
    }
}
#[doc = "Field `CLK_TX` reader - CLK_TX input"]
pub type CLK_TX_R = crate::FieldReader<u8, CLK_TX_A>;
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
impl CLK_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_TX_A {
        match self.bits {
            0 => CLK_TX_A::VALUE1,
            1 => CLK_TX_A::VALUE2,
            2 => CLK_TX_A::VALUE3,
            3 => CLK_TX_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CLK_TX_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CLK_TX_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CLK_TX_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CLK_TX_A::VALUE4
    }
}
#[doc = "Field `CLK_TX` writer - CLK_TX input"]
pub type CLK_TX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ETH0_CON_SPEC, u8, CLK_TX_A, 2, O>;
impl<'a, const O: u8> CLK_TX_W<'a, O> {
    #[doc = "Data input CLK_TXA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLK_TX_A::VALUE1)
    }
    #[doc = "Data input CLK_TXB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CLK_TX_A::VALUE2)
    }
    #[doc = "Data input CLK_TXC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CLK_TX_A::VALUE3)
    }
    #[doc = "Data input CLK_TXD is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CLK_TX_A::VALUE4)
    }
}
#[doc = "Field `MDIO` reader - MDIO Input Select"]
pub type MDIO_R = crate::FieldReader<u8, MDIO_A>;
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
impl MDIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIO_A {
        match self.bits {
            0 => MDIO_A::VALUE1,
            1 => MDIO_A::VALUE2,
            2 => MDIO_A::VALUE3,
            3 => MDIO_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MDIO_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MDIO_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == MDIO_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == MDIO_A::VALUE4
    }
}
#[doc = "Field `MDIO` writer - MDIO Input Select"]
pub type MDIO_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ETH0_CON_SPEC, u8, MDIO_A, 2, O>;
impl<'a, const O: u8> MDIO_W<'a, O> {
    #[doc = "Data input MDIA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MDIO_A::VALUE1)
    }
    #[doc = "Data input MDIB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MDIO_A::VALUE2)
    }
    #[doc = "Data input MDIC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(MDIO_A::VALUE3)
    }
    #[doc = "Data input MDID is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(MDIO_A::VALUE4)
    }
}
#[doc = "Field `INFSEL` reader - Ethernet MAC Interface Selection"]
pub type INFSEL_R = crate::BitReader<INFSEL_A>;
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
impl INFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INFSEL_A {
        match self.bits {
            false => INFSEL_A::VALUE1,
            true => INFSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == INFSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INFSEL_A::VALUE2
    }
}
#[doc = "Field `INFSEL` writer - Ethernet MAC Interface Selection"]
pub type INFSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH0_CON_SPEC, INFSEL_A, O>;
impl<'a, const O: u8> INFSEL_W<'a, O> {
    #[doc = "MII"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(INFSEL_A::VALUE1)
    }
    #[doc = "RMII"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
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
    pub fn rxd0(&mut self) -> RXD0_W<0> {
        RXD0_W::new(self)
    }
    #[doc = "Bits 2:3 - MAC Receive Input 1"]
    #[inline(always)]
    #[must_use]
    pub fn rxd1(&mut self) -> RXD1_W<2> {
        RXD1_W::new(self)
    }
    #[doc = "Bits 4:5 - MAC Receive Input 2"]
    #[inline(always)]
    #[must_use]
    pub fn rxd2(&mut self) -> RXD2_W<4> {
        RXD2_W::new(self)
    }
    #[doc = "Bits 6:7 - MAC Receive Input 3"]
    #[inline(always)]
    #[must_use]
    pub fn rxd3(&mut self) -> RXD3_W<6> {
        RXD3_W::new(self)
    }
    #[doc = "Bits 8:9 - RMII clock input"]
    #[inline(always)]
    #[must_use]
    pub fn clk_rmii(&mut self) -> CLK_RMII_W<8> {
        CLK_RMII_W::new(self)
    }
    #[doc = "Bits 10:11 - CRS_DV input"]
    #[inline(always)]
    #[must_use]
    pub fn crs_dv(&mut self) -> CRS_DV_W<10> {
        CRS_DV_W::new(self)
    }
    #[doc = "Bits 12:13 - CRS input"]
    #[inline(always)]
    #[must_use]
    pub fn crs(&mut self) -> CRS_W<12> {
        CRS_W::new(self)
    }
    #[doc = "Bits 14:15 - RXER Input"]
    #[inline(always)]
    #[must_use]
    pub fn rxer(&mut self) -> RXER_W<14> {
        RXER_W::new(self)
    }
    #[doc = "Bits 16:17 - COL input"]
    #[inline(always)]
    #[must_use]
    pub fn col(&mut self) -> COL_W<16> {
        COL_W::new(self)
    }
    #[doc = "Bits 18:19 - CLK_TX input"]
    #[inline(always)]
    #[must_use]
    pub fn clk_tx(&mut self) -> CLK_TX_W<18> {
        CLK_TX_W::new(self)
    }
    #[doc = "Bits 22:23 - MDIO Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn mdio(&mut self) -> MDIO_W<22> {
        MDIO_W::new(self)
    }
    #[doc = "Bit 26 - Ethernet MAC Interface Selection"]
    #[inline(always)]
    #[must_use]
    pub fn infsel(&mut self) -> INFSEL_W<26> {
        INFSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet 0 Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth0_con](index.html) module"]
pub struct ETH0_CON_SPEC;
impl crate::RegisterSpec for ETH0_CON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth0_con::R](R) reader structure"]
impl crate::Readable for ETH0_CON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth0_con::W](W) writer structure"]
impl crate::Writable for ETH0_CON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH0_CON to value 0"]
impl crate::Resettable for ETH0_CON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
