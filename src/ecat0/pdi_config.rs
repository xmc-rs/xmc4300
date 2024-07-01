#[doc = "Register `PDI_CONFIG` reader"]
pub type R = crate::R<PDI_CONFIG_SPEC>;
#[doc = "On-chip bus clock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for BUS_CLK_A {
    type Ux = u8;
}
impl crate::IsEnum for BUS_CLK_A {}
#[doc = "Field `BUS_CLK` reader - On-chip bus clock"]
pub type BUS_CLK_R = crate::FieldReader<BUS_CLK_A>;
impl BUS_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BUS_CLK_A> {
        match self.bits {
            0 => Some(BUS_CLK_A::VALUE1),
            1 => Some(BUS_CLK_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "asyncronous"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BUS_CLK_A::VALUE1
    }
    #[doc = "values 1-31 is used for synchronous multiplication factor (N*25Mhz)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BUS_CLK_A::VALUE2
    }
}
#[doc = "On-chip bus\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for OC_BUS_A {
    type Ux = u8;
}
impl crate::IsEnum for OC_BUS_A {}
#[doc = "Field `OC_BUS` reader - On-chip bus"]
pub type OC_BUS_R = crate::FieldReader<OC_BUS_A>;
impl OC_BUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OC_BUS_A> {
        match self.bits {
            0 => Some(OC_BUS_A::VALUE1),
            1 => Some(OC_BUS_A::VALUE2),
            2 => Some(OC_BUS_A::VALUE3),
            4 => Some(OC_BUS_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Altera Avalon"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OC_BUS_A::VALUE1
    }
    #[doc = "AXI"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OC_BUS_A::VALUE2
    }
    #[doc = "Xilinx PLB v4.6"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == OC_BUS_A::VALUE3
    }
    #[doc = "Xilinx OPB"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == OC_BUS_A::VALUE4
    }
}
impl R {
    #[doc = "Bits 0:4 - On-chip bus clock"]
    #[inline(always)]
    pub fn bus_clk(&self) -> BUS_CLK_R {
        BUS_CLK_R::new(self.bits & 0x1f)
    }
    #[doc = "Bits 5:7 - On-chip bus"]
    #[inline(always)]
    pub fn oc_bus(&self) -> OC_BUS_R {
        OC_BUS_R::new((self.bits >> 5) & 7)
    }
}
#[doc = "PDI Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pdi_config::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDI_CONFIG_SPEC;
impl crate::RegisterSpec for PDI_CONFIG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pdi_config::R`](R) reader structure"]
impl crate::Readable for PDI_CONFIG_SPEC {}
#[doc = "`reset()` method sets PDI_CONFIG to value 0x81"]
impl crate::Resettable for PDI_CONFIG_SPEC {
    const RESET_VALUE: u8 = 0x81;
}
