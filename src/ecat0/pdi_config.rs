#[doc = "Register `PDI_CONFIG` reader"]
pub struct R(crate::R<PDI_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDI_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDI_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDI_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "On-chip bus clock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BUS_CLK_A {
    #[doc = "0: asyncronous"]
    VALUE1 = 0,
    #[doc = "1: values 1-31 is used for synchronous multiplication factor (N*25Mhz)"]
    VALUE2 = 1,
}
impl From<BUS_CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: BUS_CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BUS_CLK` reader - On-chip bus clock"]
pub struct BUS_CLK_R(crate::FieldReader<u8, BUS_CLK_A>);
impl BUS_CLK_R {
    pub(crate) fn new(bits: u8) -> Self {
        BUS_CLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BUS_CLK_A> {
        match self.bits {
            0 => Some(BUS_CLK_A::VALUE1),
            1 => Some(BUS_CLK_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BUS_CLK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BUS_CLK_A::VALUE2
    }
}
impl core::ops::Deref for BUS_CLK_R {
    type Target = crate::FieldReader<u8, BUS_CLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "On-chip bus\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OC_BUS_A {
    #[doc = "0: Altera Avalon"]
    VALUE1 = 0,
    #[doc = "1: AXI"]
    VALUE2 = 1,
    #[doc = "2: Xilinx PLB v4.6"]
    VALUE3 = 2,
    #[doc = "4: Xilinx OPB"]
    VALUE4 = 4,
}
impl From<OC_BUS_A> for u8 {
    #[inline(always)]
    fn from(variant: OC_BUS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OC_BUS` reader - On-chip bus"]
pub struct OC_BUS_R(crate::FieldReader<u8, OC_BUS_A>);
impl OC_BUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        OC_BUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OC_BUS_A> {
        match self.bits {
            0 => Some(OC_BUS_A::VALUE1),
            1 => Some(OC_BUS_A::VALUE2),
            2 => Some(OC_BUS_A::VALUE3),
            4 => Some(OC_BUS_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == OC_BUS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == OC_BUS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == OC_BUS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == OC_BUS_A::VALUE4
    }
}
impl core::ops::Deref for OC_BUS_R {
    type Target = crate::FieldReader<u8, OC_BUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - On-chip bus clock"]
    #[inline(always)]
    pub fn bus_clk(&self) -> BUS_CLK_R {
        BUS_CLK_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - On-chip bus"]
    #[inline(always)]
    pub fn oc_bus(&self) -> OC_BUS_R {
        OC_BUS_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
#[doc = "PDI Control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdi_config](index.html) module"]
pub struct PDI_CONFIG_SPEC;
impl crate::RegisterSpec for PDI_CONFIG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pdi_config::R](R) reader structure"]
impl crate::Readable for PDI_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PDI_CONFIG to value 0x81"]
impl crate::Resettable for PDI_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x81
    }
}
