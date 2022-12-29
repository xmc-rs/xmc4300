#[doc = "Register `RESD[%s]` reader"]
pub struct R(crate::R<RESD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESULT` reader - Result of Most Recent Conversion"]
pub type RESULT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DRC` reader - Data Reduction Counter"]
pub type DRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHNR` reader - Channel Number"]
pub type CHNR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMUX` reader - External Multiplexer Setting"]
pub type EMUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CRS` reader - Converted Request Source"]
pub type CRS_R = crate::FieldReader<u8, CRS_A>;
#[doc = "Converted Request Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CRS_A {
    #[doc = "0: Request source 0"]
    VALUE1 = 0,
    #[doc = "1: Request source 1"]
    VALUE2 = 1,
    #[doc = "2: Request source 2"]
    VALUE3 = 2,
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
    pub fn variant(&self) -> Option<CRS_A> {
        match self.bits {
            0 => Some(CRS_A::VALUE1),
            1 => Some(CRS_A::VALUE2),
            2 => Some(CRS_A::VALUE3),
            _ => None,
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
}
#[doc = "Field `FCR` reader - Fast Compare Result"]
pub type FCR_R = crate::BitReader<FCR_A>;
#[doc = "Fast Compare Result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCR_A {
    #[doc = "0: Signal level was below compare value"]
    VALUE1 = 0,
    #[doc = "1: Signal level was above compare value"]
    VALUE2 = 1,
}
impl From<FCR_A> for bool {
    #[inline(always)]
    fn from(variant: FCR_A) -> Self {
        variant as u8 != 0
    }
}
impl FCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCR_A {
        match self.bits {
            false => FCR_A::VALUE1,
            true => FCR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FCR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FCR_A::VALUE2
    }
}
#[doc = "Field `VF` reader - Valid Flag"]
pub type VF_R = crate::BitReader<VF_A>;
#[doc = "Valid Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VF_A {
    #[doc = "0: No new result available"]
    VALUE1 = 0,
    #[doc = "1: Bitfield RESULT has been updated with new result value and has not yet been read, or bit FCR has been updated"]
    VALUE2 = 1,
}
impl From<VF_A> for bool {
    #[inline(always)]
    fn from(variant: VF_A) -> Self {
        variant as u8 != 0
    }
}
impl VF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VF_A {
        match self.bits {
            false => VF_A::VALUE1,
            true => VF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:15 - Result of Most Recent Conversion"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Data Reduction Counter"]
    #[inline(always)]
    pub fn drc(&self) -> DRC_R {
        DRC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:24 - Channel Number"]
    #[inline(always)]
    pub fn chnr(&self) -> CHNR_R {
        CHNR_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:27 - External Multiplexer Setting"]
    #[inline(always)]
    pub fn emux(&self) -> EMUX_R {
        EMUX_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:29 - Converted Request Source"]
    #[inline(always)]
    pub fn crs(&self) -> CRS_R {
        CRS_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Fast Compare Result"]
    #[inline(always)]
    pub fn fcr(&self) -> FCR_R {
        FCR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Valid Flag"]
    #[inline(always)]
    pub fn vf(&self) -> VF_R {
        VF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Result Register, Debug\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resd](index.html) module"]
pub struct RESD_SPEC;
impl crate::RegisterSpec for RESD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resd::R](R) reader structure"]
impl crate::Readable for RESD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESD[%s]
to value 0"]
impl crate::Resettable for RESD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
