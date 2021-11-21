#[doc = "Register `PORT_DESC` reader"]
pub struct R(crate::R<PORT_DESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORT_DESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORT_DESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORT_DESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Port Configuration\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PORT0_A {
    #[doc = "0: Not implemented"]
    VALUE1 = 0,
    #[doc = "1: Not configured (SII EEPROM)"]
    VALUE2 = 1,
    #[doc = "2: EBUS"]
    VALUE3 = 2,
    #[doc = "3: MII / RMII / RGMII"]
    VALUE4 = 3,
}
impl From<PORT0_A> for u8 {
    #[inline(always)]
    fn from(variant: PORT0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `Port0` reader - Port Configuration"]
pub struct PORT0_R(crate::FieldReader<u8, PORT0_A>);
impl PORT0_R {
    pub(crate) fn new(bits: u8) -> Self {
        PORT0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT0_A {
        match self.bits {
            0 => PORT0_A::VALUE1,
            1 => PORT0_A::VALUE2,
            2 => PORT0_A::VALUE3,
            3 => PORT0_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PORT0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PORT0_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PORT0_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == PORT0_A::VALUE4
    }
}
impl core::ops::Deref for PORT0_R {
    type Target = crate::FieldReader<u8, PORT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Port Configuration\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PORT1_A {
    #[doc = "0: Not implemented"]
    VALUE1 = 0,
    #[doc = "1: Not configured (SII EEPROM)"]
    VALUE2 = 1,
    #[doc = "2: EBUS"]
    VALUE3 = 2,
    #[doc = "3: MII / RMII / RGMII"]
    VALUE4 = 3,
}
impl From<PORT1_A> for u8 {
    #[inline(always)]
    fn from(variant: PORT1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `Port1` reader - Port Configuration"]
pub struct PORT1_R(crate::FieldReader<u8, PORT1_A>);
impl PORT1_R {
    pub(crate) fn new(bits: u8) -> Self {
        PORT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT1_A {
        match self.bits {
            0 => PORT1_A::VALUE1,
            1 => PORT1_A::VALUE2,
            2 => PORT1_A::VALUE3,
            3 => PORT1_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PORT1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PORT1_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PORT1_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == PORT1_A::VALUE4
    }
}
impl core::ops::Deref for PORT1_R {
    type Target = crate::FieldReader<u8, PORT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Port Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PORT2_A {
    #[doc = "0: Not implemented"]
    VALUE1 = 0,
    #[doc = "1: Not configured (SII EEPROM)"]
    VALUE2 = 1,
    #[doc = "2: EBUS"]
    VALUE3 = 2,
    #[doc = "3: MII / RMII / RGMII"]
    VALUE4 = 3,
}
impl From<PORT2_A> for u8 {
    #[inline(always)]
    fn from(variant: PORT2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `Port2` reader - Port Configuration"]
pub struct PORT2_R(crate::FieldReader<u8, PORT2_A>);
impl PORT2_R {
    pub(crate) fn new(bits: u8) -> Self {
        PORT2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT2_A {
        match self.bits {
            0 => PORT2_A::VALUE1,
            1 => PORT2_A::VALUE2,
            2 => PORT2_A::VALUE3,
            3 => PORT2_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PORT2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PORT2_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PORT2_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == PORT2_A::VALUE4
    }
}
impl core::ops::Deref for PORT2_R {
    type Target = crate::FieldReader<u8, PORT2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Port Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PORT3_A {
    #[doc = "0: Not implemented"]
    VALUE1 = 0,
    #[doc = "1: Not configured (SII EEPROM)"]
    VALUE2 = 1,
    #[doc = "2: EBUS"]
    VALUE3 = 2,
    #[doc = "3: MII / RMII / RGMII"]
    VALUE4 = 3,
}
impl From<PORT3_A> for u8 {
    #[inline(always)]
    fn from(variant: PORT3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `Port3` reader - Port Configuration"]
pub struct PORT3_R(crate::FieldReader<u8, PORT3_A>);
impl PORT3_R {
    pub(crate) fn new(bits: u8) -> Self {
        PORT3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT3_A {
        match self.bits {
            0 => PORT3_A::VALUE1,
            1 => PORT3_A::VALUE2,
            2 => PORT3_A::VALUE3,
            3 => PORT3_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PORT3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PORT3_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PORT3_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == PORT3_A::VALUE4
    }
}
impl core::ops::Deref for PORT3_R {
    type Target = crate::FieldReader<u8, PORT3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Port Configuration"]
    #[inline(always)]
    pub fn port0(&self) -> PORT0_R {
        PORT0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port Configuration"]
    #[inline(always)]
    pub fn port1(&self) -> PORT1_R {
        PORT1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port Configuration"]
    #[inline(always)]
    pub fn port2(&self) -> PORT2_R {
        PORT2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port Configuration"]
    #[inline(always)]
    pub fn port3(&self) -> PORT3_R {
        PORT3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
#[doc = "Port Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port_desc](index.html) module"]
pub struct PORT_DESC_SPEC;
impl crate::RegisterSpec for PORT_DESC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [port_desc::R](R) reader structure"]
impl crate::Readable for PORT_DESC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PORT_DESC to value 0x0f"]
impl crate::Resettable for PORT_DESC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
