#[doc = "Register `PDI_CONFIG` reader"]
pub type R = crate::R<PdiConfigSpec>;
#[doc = "On-chip bus clock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BusClk {
    #[doc = "0: asyncronous"]
    Value1 = 0,
    #[doc = "1: values 1-31 is used for synchronous multiplication factor (N*25Mhz)"]
    Value2 = 1,
}
impl From<BusClk> for u8 {
    #[inline(always)]
    fn from(variant: BusClk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BusClk {
    type Ux = u8;
}
impl crate::IsEnum for BusClk {}
#[doc = "Field `BUS_CLK` reader - On-chip bus clock"]
pub type BusClkR = crate::FieldReader<BusClk>;
impl BusClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BusClk> {
        match self.bits {
            0 => Some(BusClk::Value1),
            1 => Some(BusClk::Value2),
            _ => None,
        }
    }
    #[doc = "asyncronous"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BusClk::Value1
    }
    #[doc = "values 1-31 is used for synchronous multiplication factor (N*25Mhz)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BusClk::Value2
    }
}
#[doc = "On-chip bus\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OcBus {
    #[doc = "0: Altera Avalon"]
    Value1 = 0,
    #[doc = "1: AXI"]
    Value2 = 1,
    #[doc = "2: Xilinx PLB v4.6"]
    Value3 = 2,
    #[doc = "4: Xilinx OPB"]
    Value4 = 4,
}
impl From<OcBus> for u8 {
    #[inline(always)]
    fn from(variant: OcBus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OcBus {
    type Ux = u8;
}
impl crate::IsEnum for OcBus {}
#[doc = "Field `OC_BUS` reader - On-chip bus"]
pub type OcBusR = crate::FieldReader<OcBus>;
impl OcBusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OcBus> {
        match self.bits {
            0 => Some(OcBus::Value1),
            1 => Some(OcBus::Value2),
            2 => Some(OcBus::Value3),
            4 => Some(OcBus::Value4),
            _ => None,
        }
    }
    #[doc = "Altera Avalon"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OcBus::Value1
    }
    #[doc = "AXI"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OcBus::Value2
    }
    #[doc = "Xilinx PLB v4.6"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == OcBus::Value3
    }
    #[doc = "Xilinx OPB"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == OcBus::Value4
    }
}
impl R {
    #[doc = "Bits 0:4 - On-chip bus clock"]
    #[inline(always)]
    pub fn bus_clk(&self) -> BusClkR {
        BusClkR::new(self.bits & 0x1f)
    }
    #[doc = "Bits 5:7 - On-chip bus"]
    #[inline(always)]
    pub fn oc_bus(&self) -> OcBusR {
        OcBusR::new((self.bits >> 5) & 7)
    }
}
#[doc = "PDI Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdi_config::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdiConfigSpec;
impl crate::RegisterSpec for PdiConfigSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pdi_config::R`](R) reader structure"]
impl crate::Readable for PdiConfigSpec {}
#[doc = "`reset()` method sets PDI_CONFIG to value 0x81"]
impl crate::Resettable for PdiConfigSpec {
    const RESET_VALUE: u8 = 0x81;
}
