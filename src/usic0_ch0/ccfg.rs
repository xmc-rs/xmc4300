#[doc = "Register `CCFG` reader"]
pub type R = crate::R<CcfgSpec>;
#[doc = "SSC Protocol Available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssc {
    #[doc = "0: The SSC protocol is not available."]
    Value1 = 0,
    #[doc = "1: The SSC protocol is available."]
    Value2 = 1,
}
impl From<Ssc> for bool {
    #[inline(always)]
    fn from(variant: Ssc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSC` reader - SSC Protocol Available"]
pub type SscR = crate::BitReader<Ssc>;
impl SscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssc {
        match self.bits {
            false => Ssc::Value1,
            true => Ssc::Value2,
        }
    }
    #[doc = "The SSC protocol is not available."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ssc::Value1
    }
    #[doc = "The SSC protocol is available."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ssc::Value2
    }
}
#[doc = "ASC Protocol Available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asc {
    #[doc = "0: The ASC protocol is not available."]
    Value1 = 0,
    #[doc = "1: The ASC protocol is available."]
    Value2 = 1,
}
impl From<Asc> for bool {
    #[inline(always)]
    fn from(variant: Asc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASC` reader - ASC Protocol Available"]
pub type AscR = crate::BitReader<Asc>;
impl AscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Asc {
        match self.bits {
            false => Asc::Value1,
            true => Asc::Value2,
        }
    }
    #[doc = "The ASC protocol is not available."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Asc::Value1
    }
    #[doc = "The ASC protocol is available."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Asc::Value2
    }
}
#[doc = "IIC Protocol Available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iic {
    #[doc = "0: The IIC protocol is not available."]
    Value1 = 0,
    #[doc = "1: The IIC protocol is available."]
    Value2 = 1,
}
impl From<Iic> for bool {
    #[inline(always)]
    fn from(variant: Iic) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IIC` reader - IIC Protocol Available"]
pub type IicR = crate::BitReader<Iic>;
impl IicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iic {
        match self.bits {
            false => Iic::Value1,
            true => Iic::Value2,
        }
    }
    #[doc = "The IIC protocol is not available."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Iic::Value1
    }
    #[doc = "The IIC protocol is available."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Iic::Value2
    }
}
#[doc = "IIS Protocol Available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iis {
    #[doc = "0: The IIS protocol is not available."]
    Value1 = 0,
    #[doc = "1: The IIS protocol is available."]
    Value2 = 1,
}
impl From<Iis> for bool {
    #[inline(always)]
    fn from(variant: Iis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IIS` reader - IIS Protocol Available"]
pub type IisR = crate::BitReader<Iis>;
impl IisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iis {
        match self.bits {
            false => Iis::Value1,
            true => Iis::Value2,
        }
    }
    #[doc = "The IIS protocol is not available."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Iis::Value1
    }
    #[doc = "The IIS protocol is available."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Iis::Value2
    }
}
#[doc = "Receive FIFO Buffer Available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rb {
    #[doc = "0: A receive FIFO buffer is not available."]
    Value1 = 0,
    #[doc = "1: A receive FIFO buffer is available."]
    Value2 = 1,
}
impl From<Rb> for bool {
    #[inline(always)]
    fn from(variant: Rb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RB` reader - Receive FIFO Buffer Available"]
pub type RbR = crate::BitReader<Rb>;
impl RbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rb {
        match self.bits {
            false => Rb::Value1,
            true => Rb::Value2,
        }
    }
    #[doc = "A receive FIFO buffer is not available."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rb::Value1
    }
    #[doc = "A receive FIFO buffer is available."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rb::Value2
    }
}
#[doc = "Transmit FIFO Buffer Available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tb {
    #[doc = "0: A transmit FIFO buffer is not available."]
    Value1 = 0,
    #[doc = "1: A transmit FIFO buffer is available."]
    Value2 = 1,
}
impl From<Tb> for bool {
    #[inline(always)]
    fn from(variant: Tb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TB` reader - Transmit FIFO Buffer Available"]
pub type TbR = crate::BitReader<Tb>;
impl TbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tb {
        match self.bits {
            false => Tb::Value1,
            true => Tb::Value2,
        }
    }
    #[doc = "A transmit FIFO buffer is not available."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tb::Value1
    }
    #[doc = "A transmit FIFO buffer is available."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tb::Value2
    }
}
impl R {
    #[doc = "Bit 0 - SSC Protocol Available"]
    #[inline(always)]
    pub fn ssc(&self) -> SscR {
        SscR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ASC Protocol Available"]
    #[inline(always)]
    pub fn asc(&self) -> AscR {
        AscR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IIC Protocol Available"]
    #[inline(always)]
    pub fn iic(&self) -> IicR {
        IicR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IIS Protocol Available"]
    #[inline(always)]
    pub fn iis(&self) -> IisR {
        IisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO Buffer Available"]
    #[inline(always)]
    pub fn rb(&self) -> RbR {
        RbR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Buffer Available"]
    #[inline(always)]
    pub fn tb(&self) -> TbR {
        TbR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcfgSpec;
impl crate::RegisterSpec for CcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccfg::R`](R) reader structure"]
impl crate::Readable for CcfgSpec {}
#[doc = "`reset()` method sets CCFG to value 0xcf"]
impl crate::Resettable for CcfgSpec {
    const RESET_VALUE: u32 = 0xcf;
}
