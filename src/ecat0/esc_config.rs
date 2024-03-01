#[doc = "Register `ESC_CONFIG` reader"]
pub type R = crate::R<EscConfigSpec>;
#[doc = "Device emulation (control of AL status)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Emul {
    #[doc = "0: AL status register has to be set by PDI"]
    Value1 = 0,
    #[doc = "1: AL status register will be set to value written to AL control register"]
    Value2 = 1,
}
impl From<Emul> for bool {
    #[inline(always)]
    fn from(variant: Emul) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMUL` reader - Device emulation (control of AL status)"]
pub type EmulR = crate::BitReader<Emul>;
impl EmulR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Emul {
        match self.bits {
            false => Emul::Value1,
            true => Emul::Value2,
        }
    }
    #[doc = "AL status register has to be set by PDI"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Emul::Value1
    }
    #[doc = "AL status register will be set to value written to AL control register"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Emul::Value2
    }
}
#[doc = "Enhanced Link detection all ports\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ehld {
    #[doc = "0: disabled (if bits \\[7:4\\]=0)"]
    Value1 = 0,
    #[doc = "1: enabled at all ports (overrides bits \\[7:4\\])"]
    Value2 = 1,
}
impl From<Ehld> for bool {
    #[inline(always)]
    fn from(variant: Ehld) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EHLD` reader - Enhanced Link detection all ports"]
pub type EhldR = crate::BitReader<Ehld>;
impl EhldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ehld {
        match self.bits {
            false => Ehld::Value1,
            true => Ehld::Value2,
        }
    }
    #[doc = "disabled (if bits \\[7:4\\]=0)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ehld::Value1
    }
    #[doc = "enabled at all ports (overrides bits \\[7:4\\])"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ehld::Value2
    }
}
#[doc = "Distributed Clocks SYNC Out Unit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClksOut {
    #[doc = "0: disabled (power saving)"]
    Value1 = 0,
    #[doc = "1: enabled"]
    Value2 = 1,
}
impl From<ClksOut> for bool {
    #[inline(always)]
    fn from(variant: ClksOut) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKS_OUT` reader - Distributed Clocks SYNC Out Unit"]
pub type ClksOutR = crate::BitReader<ClksOut>;
impl ClksOutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClksOut {
        match self.bits {
            false => ClksOut::Value1,
            true => ClksOut::Value2,
        }
    }
    #[doc = "disabled (power saving)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ClksOut::Value1
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ClksOut::Value2
    }
}
#[doc = "Distributed Clocks Latch In Unit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClksIn {
    #[doc = "0: disabled (power saving)"]
    Value1 = 0,
    #[doc = "1: enabled"]
    Value2 = 1,
}
impl From<ClksIn> for bool {
    #[inline(always)]
    fn from(variant: ClksIn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKS_IN` reader - Distributed Clocks Latch In Unit"]
pub type ClksInR = crate::BitReader<ClksIn>;
impl ClksInR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClksIn {
        match self.bits {
            false => ClksIn::Value1,
            true => ClksIn::Value2,
        }
    }
    #[doc = "disabled (power saving)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ClksIn::Value1
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ClksIn::Value2
    }
}
#[doc = "Enhanced Link port 0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EhldP0 {
    #[doc = "0: disabled (if bit 1 = 0)"]
    Value1 = 0,
    #[doc = "1: enabled"]
    Value2 = 1,
}
impl From<EhldP0> for bool {
    #[inline(always)]
    fn from(variant: EhldP0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EHLD_P0` reader - Enhanced Link port 0"]
pub type EhldP0R = crate::BitReader<EhldP0>;
impl EhldP0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EhldP0 {
        match self.bits {
            false => EhldP0::Value1,
            true => EhldP0::Value2,
        }
    }
    #[doc = "disabled (if bit 1 = 0)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EhldP0::Value1
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EhldP0::Value2
    }
}
#[doc = "Enhanced Link port 1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EhldP1 {
    #[doc = "0: disabled (if bit 1 = 0)"]
    Value1 = 0,
    #[doc = "1: enabled"]
    Value2 = 1,
}
impl From<EhldP1> for bool {
    #[inline(always)]
    fn from(variant: EhldP1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EHLD_P1` reader - Enhanced Link port 1"]
pub type EhldP1R = crate::BitReader<EhldP1>;
impl EhldP1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EhldP1 {
        match self.bits {
            false => EhldP1::Value1,
            true => EhldP1::Value2,
        }
    }
    #[doc = "disabled (if bit 1 = 0)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EhldP1::Value1
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EhldP1::Value2
    }
}
#[doc = "Enhanced Link port 2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EhldP2 {
    #[doc = "0: disabled (if bit 1 = 0)"]
    Value1 = 0,
    #[doc = "1: enabled"]
    Value2 = 1,
}
impl From<EhldP2> for bool {
    #[inline(always)]
    fn from(variant: EhldP2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EHLD_P2` reader - Enhanced Link port 2"]
pub type EhldP2R = crate::BitReader<EhldP2>;
impl EhldP2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EhldP2 {
        match self.bits {
            false => EhldP2::Value1,
            true => EhldP2::Value2,
        }
    }
    #[doc = "disabled (if bit 1 = 0)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EhldP2::Value1
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EhldP2::Value2
    }
}
#[doc = "Enhanced Link port 3\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EhldP3 {
    #[doc = "0: disabled (if bit 1 = 0)"]
    Value1 = 0,
    #[doc = "1: enabled"]
    Value2 = 1,
}
impl From<EhldP3> for bool {
    #[inline(always)]
    fn from(variant: EhldP3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EHLD_P3` reader - Enhanced Link port 3"]
pub type EhldP3R = crate::BitReader<EhldP3>;
impl EhldP3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EhldP3 {
        match self.bits {
            false => EhldP3::Value1,
            true => EhldP3::Value2,
        }
    }
    #[doc = "disabled (if bit 1 = 0)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EhldP3::Value1
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EhldP3::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Device emulation (control of AL status)"]
    #[inline(always)]
    pub fn emul(&self) -> EmulR {
        EmulR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enhanced Link detection all ports"]
    #[inline(always)]
    pub fn ehld(&self) -> EhldR {
        EhldR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Distributed Clocks SYNC Out Unit"]
    #[inline(always)]
    pub fn clks_out(&self) -> ClksOutR {
        ClksOutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Distributed Clocks Latch In Unit"]
    #[inline(always)]
    pub fn clks_in(&self) -> ClksInR {
        ClksInR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enhanced Link port 0"]
    #[inline(always)]
    pub fn ehld_p0(&self) -> EhldP0R {
        EhldP0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enhanced Link port 1"]
    #[inline(always)]
    pub fn ehld_p1(&self) -> EhldP1R {
        EhldP1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enhanced Link port 2"]
    #[inline(always)]
    pub fn ehld_p2(&self) -> EhldP2R {
        EhldP2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enhanced Link port 3"]
    #[inline(always)]
    pub fn ehld_p3(&self) -> EhldP3R {
        EhldP3R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "ESC Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esc_config::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EscConfigSpec;
impl crate::RegisterSpec for EscConfigSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`esc_config::R`](R) reader structure"]
impl crate::Readable for EscConfigSpec {}
#[doc = "`reset()` method sets ESC_CONFIG to value 0xfe"]
impl crate::Resettable for EscConfigSpec {
    const RESET_VALUE: u8 = 0xfe;
}
