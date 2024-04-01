#[doc = "Register `RESD[%s]` reader"]
pub type R = crate::R<ResdSpec>;
#[doc = "Field `RESULT` reader - Result of Most Recent Conversion"]
pub type ResultR = crate::FieldReader<u16>;
#[doc = "Field `DRC` reader - Data Reduction Counter"]
pub type DrcR = crate::FieldReader;
#[doc = "Field `CHNR` reader - Channel Number"]
pub type ChnrR = crate::FieldReader;
#[doc = "Field `EMUX` reader - External Multiplexer Setting"]
pub type EmuxR = crate::FieldReader;
#[doc = "Converted Request Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Crs {
    #[doc = "0: Request source 0"]
    Value1 = 0,
    #[doc = "1: Request source 1"]
    Value2 = 1,
    #[doc = "2: Request source 2"]
    Value3 = 2,
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
#[doc = "Field `CRS` reader - Converted Request Source"]
pub type CrsR = crate::FieldReader<Crs>;
impl CrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Crs> {
        match self.bits {
            0 => Some(Crs::Value1),
            1 => Some(Crs::Value2),
            2 => Some(Crs::Value3),
            _ => None,
        }
    }
    #[doc = "Request source 0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Crs::Value1
    }
    #[doc = "Request source 1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Crs::Value2
    }
    #[doc = "Request source 2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Crs::Value3
    }
}
#[doc = "Fast Compare Result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcr {
    #[doc = "0: Signal level was below compare value"]
    Value1 = 0,
    #[doc = "1: Signal level was above compare value"]
    Value2 = 1,
}
impl From<Fcr> for bool {
    #[inline(always)]
    fn from(variant: Fcr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCR` reader - Fast Compare Result"]
pub type FcrR = crate::BitReader<Fcr>;
impl FcrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcr {
        match self.bits {
            false => Fcr::Value1,
            true => Fcr::Value2,
        }
    }
    #[doc = "Signal level was below compare value"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Fcr::Value1
    }
    #[doc = "Signal level was above compare value"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Fcr::Value2
    }
}
#[doc = "Valid Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vf {
    #[doc = "0: No new result available"]
    Value1 = 0,
    #[doc = "1: Bitfield RESULT has been updated with new result value and has not yet been read, or bit FCR has been updated"]
    Value2 = 1,
}
impl From<Vf> for bool {
    #[inline(always)]
    fn from(variant: Vf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VF` reader - Valid Flag"]
pub type VfR = crate::BitReader<Vf>;
impl VfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vf {
        match self.bits {
            false => Vf::Value1,
            true => Vf::Value2,
        }
    }
    #[doc = "No new result available"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vf::Value1
    }
    #[doc = "Bitfield RESULT has been updated with new result value and has not yet been read, or bit FCR has been updated"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vf::Value2
    }
}
impl R {
    #[doc = "Bits 0:15 - Result of Most Recent Conversion"]
    #[inline(always)]
    pub fn result(&self) -> ResultR {
        ResultR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Data Reduction Counter"]
    #[inline(always)]
    pub fn drc(&self) -> DrcR {
        DrcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:24 - Channel Number"]
    #[inline(always)]
    pub fn chnr(&self) -> ChnrR {
        ChnrR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:27 - External Multiplexer Setting"]
    #[inline(always)]
    pub fn emux(&self) -> EmuxR {
        EmuxR::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:29 - Converted Request Source"]
    #[inline(always)]
    pub fn crs(&self) -> CrsR {
        CrsR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Fast Compare Result"]
    #[inline(always)]
    pub fn fcr(&self) -> FcrR {
        FcrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Valid Flag"]
    #[inline(always)]
    pub fn vf(&self) -> VfR {
        VfR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Result Register, Debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResdSpec;
impl crate::RegisterSpec for ResdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resd::R`](R) reader structure"]
impl crate::Readable for ResdSpec {}
#[doc = "`reset()` method sets RESD[%s]
to value 0"]
impl crate::Resettable for ResdSpec {
    const RESET_VALUE: u32 = 0;
}
