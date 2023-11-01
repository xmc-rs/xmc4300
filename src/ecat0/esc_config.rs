#[doc = "Register `ESC_CONFIG` reader"]
pub type R = crate::R<ESC_CONFIG_SPEC>;
#[doc = "Field `EMUL` reader - Device emulation (control of AL status)"]
pub type EMUL_R = crate::BitReader<EMUL_A>;
#[doc = "Device emulation (control of AL status)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl EMUL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EMUL_A {
        match self.bits {
            false => EMUL_A::VALUE1,
            true => EMUL_A::VALUE2,
        }
    }
    #[doc = "AL status register has to be set by PDI"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EMUL_A::VALUE1
    }
    #[doc = "AL status register will be set to value written to AL control register"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EMUL_A::VALUE2
    }
}
#[doc = "Field `EHLD` reader - Enhanced Link detection all ports"]
pub type EHLD_R = crate::BitReader<EHLD_A>;
#[doc = "Enhanced Link detection all ports\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl EHLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EHLD_A {
        match self.bits {
            false => EHLD_A::VALUE1,
            true => EHLD_A::VALUE2,
        }
    }
    #[doc = "disabled (if bits \\[7:4\\]=0)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EHLD_A::VALUE1
    }
    #[doc = "enabled at all ports (overrides bits \\[7:4\\])"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EHLD_A::VALUE2
    }
}
#[doc = "Field `CLKS_OUT` reader - Distributed Clocks SYNC Out Unit"]
pub type CLKS_OUT_R = crate::BitReader<CLKS_OUT_A>;
#[doc = "Distributed Clocks SYNC Out Unit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CLKS_OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKS_OUT_A {
        match self.bits {
            false => CLKS_OUT_A::VALUE1,
            true => CLKS_OUT_A::VALUE2,
        }
    }
    #[doc = "disabled (power saving)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CLKS_OUT_A::VALUE1
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CLKS_OUT_A::VALUE2
    }
}
#[doc = "Field `CLKS_IN` reader - Distributed Clocks Latch In Unit"]
pub type CLKS_IN_R = crate::BitReader<CLKS_IN_A>;
#[doc = "Distributed Clocks Latch In Unit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CLKS_IN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKS_IN_A {
        match self.bits {
            false => CLKS_IN_A::VALUE1,
            true => CLKS_IN_A::VALUE2,
        }
    }
    #[doc = "disabled (power saving)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CLKS_IN_A::VALUE1
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CLKS_IN_A::VALUE2
    }
}
#[doc = "Field `EHLD_P0` reader - Enhanced Link port 0"]
pub type EHLD_P0_R = crate::BitReader<EHLD_P0_A>;
#[doc = "Enhanced Link port 0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl EHLD_P0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EHLD_P0_A {
        match self.bits {
            false => EHLD_P0_A::VALUE1,
            true => EHLD_P0_A::VALUE2,
        }
    }
    #[doc = "disabled (if bit 1 = 0)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EHLD_P0_A::VALUE1
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EHLD_P0_A::VALUE2
    }
}
#[doc = "Field `EHLD_P1` reader - Enhanced Link port 1"]
pub type EHLD_P1_R = crate::BitReader<EHLD_P1_A>;
#[doc = "Enhanced Link port 1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl EHLD_P1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EHLD_P1_A {
        match self.bits {
            false => EHLD_P1_A::VALUE1,
            true => EHLD_P1_A::VALUE2,
        }
    }
    #[doc = "disabled (if bit 1 = 0)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EHLD_P1_A::VALUE1
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EHLD_P1_A::VALUE2
    }
}
#[doc = "Field `EHLD_P2` reader - Enhanced Link port 2"]
pub type EHLD_P2_R = crate::BitReader<EHLD_P2_A>;
#[doc = "Enhanced Link port 2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl EHLD_P2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EHLD_P2_A {
        match self.bits {
            false => EHLD_P2_A::VALUE1,
            true => EHLD_P2_A::VALUE2,
        }
    }
    #[doc = "disabled (if bit 1 = 0)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EHLD_P2_A::VALUE1
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EHLD_P2_A::VALUE2
    }
}
#[doc = "Field `EHLD_P3` reader - Enhanced Link port 3"]
pub type EHLD_P3_R = crate::BitReader<EHLD_P3_A>;
#[doc = "Enhanced Link port 3\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl EHLD_P3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EHLD_P3_A {
        match self.bits {
            false => EHLD_P3_A::VALUE1,
            true => EHLD_P3_A::VALUE2,
        }
    }
    #[doc = "disabled (if bit 1 = 0)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EHLD_P3_A::VALUE1
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EHLD_P3_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Device emulation (control of AL status)"]
    #[inline(always)]
    pub fn emul(&self) -> EMUL_R {
        EMUL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enhanced Link detection all ports"]
    #[inline(always)]
    pub fn ehld(&self) -> EHLD_R {
        EHLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Distributed Clocks SYNC Out Unit"]
    #[inline(always)]
    pub fn clks_out(&self) -> CLKS_OUT_R {
        CLKS_OUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Distributed Clocks Latch In Unit"]
    #[inline(always)]
    pub fn clks_in(&self) -> CLKS_IN_R {
        CLKS_IN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enhanced Link port 0"]
    #[inline(always)]
    pub fn ehld_p0(&self) -> EHLD_P0_R {
        EHLD_P0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enhanced Link port 1"]
    #[inline(always)]
    pub fn ehld_p1(&self) -> EHLD_P1_R {
        EHLD_P1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enhanced Link port 2"]
    #[inline(always)]
    pub fn ehld_p2(&self) -> EHLD_P2_R {
        EHLD_P2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enhanced Link port 3"]
    #[inline(always)]
    pub fn ehld_p3(&self) -> EHLD_P3_R {
        EHLD_P3_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "ESC Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esc_config::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ESC_CONFIG_SPEC;
impl crate::RegisterSpec for ESC_CONFIG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`esc_config::R`](R) reader structure"]
impl crate::Readable for ESC_CONFIG_SPEC {}
#[doc = "`reset()` method sets ESC_CONFIG to value 0xfe"]
impl crate::Resettable for ESC_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0xfe;
}
