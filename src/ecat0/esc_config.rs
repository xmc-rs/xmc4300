#[doc = "Register `ESC_CONFIG` reader"]
pub struct R(crate::R<ESC_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESC_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESC_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESC_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Device emulation (control of AL status)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMUL_A {
    #[doc = "0: AL status register has to be set by PDI"]
    VALUE1 = 0,
    #[doc = "1: AL status register will be set to value written to AL control register"]
    VALUE2 = 1,
}
impl From<EMUL_A> for bool {
    #[inline(always)]
    fn from(variant: EMUL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMUL` reader - Device emulation (control of AL status)"]
pub struct EMUL_R(crate::FieldReader<bool, EMUL_A>);
impl EMUL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EMUL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMUL_A {
        match self.bits {
            false => EMUL_A::VALUE1,
            true => EMUL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EMUL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EMUL_A::VALUE2
    }
}
impl core::ops::Deref for EMUL_R {
    type Target = crate::FieldReader<bool, EMUL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enhanced Link detection all ports\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EHLD_A {
    #[doc = "0: disabled (if bits \\[7:4\\]=0)"]
    VALUE1 = 0,
    #[doc = "1: enabled at all ports (overrides bits \\[7:4\\])"]
    VALUE2 = 1,
}
impl From<EHLD_A> for bool {
    #[inline(always)]
    fn from(variant: EHLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EHLD` reader - Enhanced Link detection all ports"]
pub struct EHLD_R(crate::FieldReader<bool, EHLD_A>);
impl EHLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        EHLD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EHLD_A {
        match self.bits {
            false => EHLD_A::VALUE1,
            true => EHLD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EHLD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EHLD_A::VALUE2
    }
}
impl core::ops::Deref for EHLD_R {
    type Target = crate::FieldReader<bool, EHLD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Distributed Clocks SYNC Out Unit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKS_OUT_A {
    #[doc = "0: disabled (power saving)"]
    VALUE1 = 0,
    #[doc = "1: enabled"]
    VALUE2 = 1,
}
impl From<CLKS_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: CLKS_OUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKS_OUT` reader - Distributed Clocks SYNC Out Unit"]
pub struct CLKS_OUT_R(crate::FieldReader<bool, CLKS_OUT_A>);
impl CLKS_OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKS_OUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKS_OUT_A {
        match self.bits {
            false => CLKS_OUT_A::VALUE1,
            true => CLKS_OUT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CLKS_OUT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CLKS_OUT_A::VALUE2
    }
}
impl core::ops::Deref for CLKS_OUT_R {
    type Target = crate::FieldReader<bool, CLKS_OUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Distributed Clocks Latch In Unit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKS_IN_A {
    #[doc = "0: disabled (power saving)"]
    VALUE1 = 0,
    #[doc = "1: enabled"]
    VALUE2 = 1,
}
impl From<CLKS_IN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKS_IN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKS_IN` reader - Distributed Clocks Latch In Unit"]
pub struct CLKS_IN_R(crate::FieldReader<bool, CLKS_IN_A>);
impl CLKS_IN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKS_IN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKS_IN_A {
        match self.bits {
            false => CLKS_IN_A::VALUE1,
            true => CLKS_IN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CLKS_IN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CLKS_IN_A::VALUE2
    }
}
impl core::ops::Deref for CLKS_IN_R {
    type Target = crate::FieldReader<bool, CLKS_IN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enhanced Link port 0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EHLD_P0_A {
    #[doc = "0: disabled (if bit 1 = 0)"]
    VALUE1 = 0,
    #[doc = "1: enabled"]
    VALUE2 = 1,
}
impl From<EHLD_P0_A> for bool {
    #[inline(always)]
    fn from(variant: EHLD_P0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EHLD_P0` reader - Enhanced Link port 0"]
pub struct EHLD_P0_R(crate::FieldReader<bool, EHLD_P0_A>);
impl EHLD_P0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EHLD_P0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EHLD_P0_A {
        match self.bits {
            false => EHLD_P0_A::VALUE1,
            true => EHLD_P0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EHLD_P0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EHLD_P0_A::VALUE2
    }
}
impl core::ops::Deref for EHLD_P0_R {
    type Target = crate::FieldReader<bool, EHLD_P0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enhanced Link port 1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EHLD_P1_A {
    #[doc = "0: disabled (if bit 1 = 0)"]
    VALUE1 = 0,
    #[doc = "1: enabled"]
    VALUE2 = 1,
}
impl From<EHLD_P1_A> for bool {
    #[inline(always)]
    fn from(variant: EHLD_P1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EHLD_P1` reader - Enhanced Link port 1"]
pub struct EHLD_P1_R(crate::FieldReader<bool, EHLD_P1_A>);
impl EHLD_P1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EHLD_P1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EHLD_P1_A {
        match self.bits {
            false => EHLD_P1_A::VALUE1,
            true => EHLD_P1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EHLD_P1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EHLD_P1_A::VALUE2
    }
}
impl core::ops::Deref for EHLD_P1_R {
    type Target = crate::FieldReader<bool, EHLD_P1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enhanced Link port 2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EHLD_P2_A {
    #[doc = "0: disabled (if bit 1 = 0)"]
    VALUE1 = 0,
    #[doc = "1: enabled"]
    VALUE2 = 1,
}
impl From<EHLD_P2_A> for bool {
    #[inline(always)]
    fn from(variant: EHLD_P2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EHLD_P2` reader - Enhanced Link port 2"]
pub struct EHLD_P2_R(crate::FieldReader<bool, EHLD_P2_A>);
impl EHLD_P2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EHLD_P2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EHLD_P2_A {
        match self.bits {
            false => EHLD_P2_A::VALUE1,
            true => EHLD_P2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EHLD_P2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EHLD_P2_A::VALUE2
    }
}
impl core::ops::Deref for EHLD_P2_R {
    type Target = crate::FieldReader<bool, EHLD_P2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enhanced Link port 3\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EHLD_P3_A {
    #[doc = "0: disabled (if bit 1 = 0)"]
    VALUE1 = 0,
    #[doc = "1: enabled"]
    VALUE2 = 1,
}
impl From<EHLD_P3_A> for bool {
    #[inline(always)]
    fn from(variant: EHLD_P3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EHLD_P3` reader - Enhanced Link port 3"]
pub struct EHLD_P3_R(crate::FieldReader<bool, EHLD_P3_A>);
impl EHLD_P3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EHLD_P3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EHLD_P3_A {
        match self.bits {
            false => EHLD_P3_A::VALUE1,
            true => EHLD_P3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EHLD_P3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EHLD_P3_A::VALUE2
    }
}
impl core::ops::Deref for EHLD_P3_R {
    type Target = crate::FieldReader<bool, EHLD_P3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Device emulation (control of AL status)"]
    #[inline(always)]
    pub fn emul(&self) -> EMUL_R {
        EMUL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enhanced Link detection all ports"]
    #[inline(always)]
    pub fn ehld(&self) -> EHLD_R {
        EHLD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Distributed Clocks SYNC Out Unit"]
    #[inline(always)]
    pub fn clks_out(&self) -> CLKS_OUT_R {
        CLKS_OUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Distributed Clocks Latch In Unit"]
    #[inline(always)]
    pub fn clks_in(&self) -> CLKS_IN_R {
        CLKS_IN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enhanced Link port 0"]
    #[inline(always)]
    pub fn ehld_p0(&self) -> EHLD_P0_R {
        EHLD_P0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enhanced Link port 1"]
    #[inline(always)]
    pub fn ehld_p1(&self) -> EHLD_P1_R {
        EHLD_P1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enhanced Link port 2"]
    #[inline(always)]
    pub fn ehld_p2(&self) -> EHLD_P2_R {
        EHLD_P2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enhanced Link port 3"]
    #[inline(always)]
    pub fn ehld_p3(&self) -> EHLD_P3_R {
        EHLD_P3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "ESC Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esc_config](index.html) module"]
pub struct ESC_CONFIG_SPEC;
impl crate::RegisterSpec for ESC_CONFIG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [esc_config::R](R) reader structure"]
impl crate::Readable for ESC_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ESC_CONFIG to value 0xfe"]
impl crate::Resettable for ESC_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfe
    }
}
