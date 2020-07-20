#[doc = "Reader of register PDI_CONFIG"]
pub type R = crate::R<u8, super::PDI_CONFIG>;
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
#[doc = "Reader of field `BUS_CLK`"]
pub type BUS_CLK_R = crate::R<u8, BUS_CLK_A>;
impl BUS_CLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BUS_CLK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BUS_CLK_A::VALUE1),
            1 => Val(BUS_CLK_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BUS_CLK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BUS_CLK_A::VALUE2
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
#[doc = "Reader of field `OC_BUS`"]
pub type OC_BUS_R = crate::R<u8, OC_BUS_A>;
impl OC_BUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OC_BUS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OC_BUS_A::VALUE1),
            1 => Val(OC_BUS_A::VALUE2),
            2 => Val(OC_BUS_A::VALUE3),
            4 => Val(OC_BUS_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OC_BUS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OC_BUS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == OC_BUS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == OC_BUS_A::VALUE4
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
