#[doc = "Register `PDI_EXT_CONFIG` reader"]
pub type R = crate::R<PdiExtConfigSpec>;
#[doc = "Read Prefetch Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RPref {
    #[doc = "0: 4 cycles"]
    Value1 = 0,
    #[doc = "1: 1 cycle (typical)"]
    Value2 = 1,
    #[doc = "2: 2 cycles"]
    Value3 = 2,
}
impl From<RPref> for u8 {
    #[inline(always)]
    fn from(variant: RPref) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RPref {
    type Ux = u8;
}
#[doc = "Field `R_Pref` reader - Read Prefetch Size"]
pub type RPrefR = crate::FieldReader<RPref>;
impl RPrefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RPref> {
        match self.bits {
            0 => Some(RPref::Value1),
            1 => Some(RPref::Value2),
            2 => Some(RPref::Value3),
            _ => None,
        }
    }
    #[doc = "4 cycles"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RPref::Value1
    }
    #[doc = "1 cycle (typical)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RPref::Value2
    }
    #[doc = "2 cycles"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RPref::Value3
    }
}
#[doc = "On-chip Sub Type for AXI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SubType {
    #[doc = "0: AXI3"]
    Value1 = 0,
    #[doc = "1: AXI4"]
    Value2 = 1,
    #[doc = "2: AXI4 Lite"]
    Value3 = 2,
}
impl From<SubType> for u8 {
    #[inline(always)]
    fn from(variant: SubType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SubType {
    type Ux = u8;
}
#[doc = "Field `SUB_TYPE` reader - On-chip Sub Type for AXI"]
pub type SubTypeR = crate::FieldReader<SubType>;
impl SubTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SubType> {
        match self.bits {
            0 => Some(SubType::Value1),
            1 => Some(SubType::Value2),
            2 => Some(SubType::Value3),
            _ => None,
        }
    }
    #[doc = "AXI3"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SubType::Value1
    }
    #[doc = "AXI4"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SubType::Value2
    }
    #[doc = "AXI4 Lite"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SubType::Value3
    }
}
impl R {
    #[doc = "Bits 0:1 - Read Prefetch Size"]
    #[inline(always)]
    pub fn r_pref(&self) -> RPrefR {
        RPrefR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:10 - On-chip Sub Type for AXI"]
    #[inline(always)]
    pub fn sub_type(&self) -> SubTypeR {
        SubTypeR::new(((self.bits >> 8) & 7) as u8)
    }
}
#[doc = "PDI Synchronous Microcontroller extended Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdi_ext_config::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdiExtConfigSpec;
impl crate::RegisterSpec for PdiExtConfigSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pdi_ext_config::R`](R) reader structure"]
impl crate::Readable for PdiExtConfigSpec {}
#[doc = "`reset()` method sets PDI_EXT_CONFIG to value 0"]
impl crate::Resettable for PdiExtConfigSpec {
    const RESET_VALUE: u16 = 0;
}
