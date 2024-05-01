#[doc = "Register `PORT_DESC` reader"]
pub type R = crate::R<PortDescSpec>;
#[doc = "Port Configuration\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port0 {
    #[doc = "0: Not implemented"]
    Value1 = 0,
    #[doc = "1: Not configured (SII EEPROM)"]
    Value2 = 1,
    #[doc = "2: EBUS"]
    Value3 = 2,
    #[doc = "3: MII / RMII / RGMII"]
    Value4 = 3,
}
impl From<Port0> for u8 {
    #[inline(always)]
    fn from(variant: Port0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port0 {
    type Ux = u8;
}
impl crate::IsEnum for Port0 {}
#[doc = "Field `Port0` reader - Port Configuration"]
pub type Port0R = crate::FieldReader<Port0>;
impl Port0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port0 {
        match self.bits {
            0 => Port0::Value1,
            1 => Port0::Value2,
            2 => Port0::Value3,
            3 => Port0::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Not implemented"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Port0::Value1
    }
    #[doc = "Not configured (SII EEPROM)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Port0::Value2
    }
    #[doc = "EBUS"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Port0::Value3
    }
    #[doc = "MII / RMII / RGMII"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Port0::Value4
    }
}
#[doc = "Port Configuration\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port1 {
    #[doc = "0: Not implemented"]
    Value1 = 0,
    #[doc = "1: Not configured (SII EEPROM)"]
    Value2 = 1,
    #[doc = "2: EBUS"]
    Value3 = 2,
    #[doc = "3: MII / RMII / RGMII"]
    Value4 = 3,
}
impl From<Port1> for u8 {
    #[inline(always)]
    fn from(variant: Port1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port1 {
    type Ux = u8;
}
impl crate::IsEnum for Port1 {}
#[doc = "Field `Port1` reader - Port Configuration"]
pub type Port1R = crate::FieldReader<Port1>;
impl Port1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port1 {
        match self.bits {
            0 => Port1::Value1,
            1 => Port1::Value2,
            2 => Port1::Value3,
            3 => Port1::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Not implemented"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Port1::Value1
    }
    #[doc = "Not configured (SII EEPROM)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Port1::Value2
    }
    #[doc = "EBUS"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Port1::Value3
    }
    #[doc = "MII / RMII / RGMII"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Port1::Value4
    }
}
#[doc = "Port Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port2 {
    #[doc = "0: Not implemented"]
    Value1 = 0,
    #[doc = "1: Not configured (SII EEPROM)"]
    Value2 = 1,
    #[doc = "2: EBUS"]
    Value3 = 2,
    #[doc = "3: MII / RMII / RGMII"]
    Value4 = 3,
}
impl From<Port2> for u8 {
    #[inline(always)]
    fn from(variant: Port2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port2 {
    type Ux = u8;
}
impl crate::IsEnum for Port2 {}
#[doc = "Field `Port2` reader - Port Configuration"]
pub type Port2R = crate::FieldReader<Port2>;
impl Port2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port2 {
        match self.bits {
            0 => Port2::Value1,
            1 => Port2::Value2,
            2 => Port2::Value3,
            3 => Port2::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Not implemented"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Port2::Value1
    }
    #[doc = "Not configured (SII EEPROM)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Port2::Value2
    }
    #[doc = "EBUS"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Port2::Value3
    }
    #[doc = "MII / RMII / RGMII"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Port2::Value4
    }
}
#[doc = "Port Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port3 {
    #[doc = "0: Not implemented"]
    Value1 = 0,
    #[doc = "1: Not configured (SII EEPROM)"]
    Value2 = 1,
    #[doc = "2: EBUS"]
    Value3 = 2,
    #[doc = "3: MII / RMII / RGMII"]
    Value4 = 3,
}
impl From<Port3> for u8 {
    #[inline(always)]
    fn from(variant: Port3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port3 {
    type Ux = u8;
}
impl crate::IsEnum for Port3 {}
#[doc = "Field `Port3` reader - Port Configuration"]
pub type Port3R = crate::FieldReader<Port3>;
impl Port3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port3 {
        match self.bits {
            0 => Port3::Value1,
            1 => Port3::Value2,
            2 => Port3::Value3,
            3 => Port3::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Not implemented"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Port3::Value1
    }
    #[doc = "Not configured (SII EEPROM)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Port3::Value2
    }
    #[doc = "EBUS"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Port3::Value3
    }
    #[doc = "MII / RMII / RGMII"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Port3::Value4
    }
}
impl R {
    #[doc = "Bits 0:1 - Port Configuration"]
    #[inline(always)]
    pub fn port0(&self) -> Port0R {
        Port0R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Port Configuration"]
    #[inline(always)]
    pub fn port1(&self) -> Port1R {
        Port1R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Port Configuration"]
    #[inline(always)]
    pub fn port2(&self) -> Port2R {
        Port2R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Port Configuration"]
    #[inline(always)]
    pub fn port3(&self) -> Port3R {
        Port3R::new((self.bits >> 6) & 3)
    }
}
#[doc = "Port Descriptor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`port_desc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PortDescSpec;
impl crate::RegisterSpec for PortDescSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`port_desc::R`](R) reader structure"]
impl crate::Readable for PortDescSpec {}
#[doc = "`reset()` method sets PORT_DESC to value 0x0f"]
impl crate::Resettable for PortDescSpec {
    const RESET_VALUE: u8 = 0x0f;
}
