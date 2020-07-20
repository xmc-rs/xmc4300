#[doc = "Reader of register PLLSTAT"]
pub type R = crate::R<u32, super::PLLSTAT>;
#[doc = "VCO Bypass Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOBYST_A {
    #[doc = "0: Free-running / Normal Mode is entered"]
    CONST_0 = 0,
    #[doc = "1: Prescaler Mode is entered"]
    CONST_1 = 1,
}
impl From<VCOBYST_A> for bool {
    #[inline(always)]
    fn from(variant: VCOBYST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VCOBYST`"]
pub type VCOBYST_R = crate::R<bool, VCOBYST_A>;
impl VCOBYST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCOBYST_A {
        match self.bits {
            false => VCOBYST_A::CONST_0,
            true => VCOBYST_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == VCOBYST_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == VCOBYST_A::CONST_1
    }
}
#[doc = "PLL Power-saving Mode Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWDSTAT_A {
    #[doc = "0: PLL Power-saving Mode was not entered"]
    CONST_0 = 0,
    #[doc = "1: PLL Power-saving Mode was entered"]
    CONST_1 = 1,
}
impl From<PWDSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: PWDSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWDSTAT`"]
pub type PWDSTAT_R = crate::R<bool, PWDSTAT_A>;
impl PWDSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWDSTAT_A {
        match self.bits {
            false => PWDSTAT_A::CONST_0,
            true => PWDSTAT_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PWDSTAT_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PWDSTAT_A::CONST_1
    }
}
#[doc = "PLL LOCK Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOLOCK_A {
    #[doc = "0: PLL not locked"]
    CONST_0 = 0,
    #[doc = "1: PLL locked"]
    CONST_1 = 1,
}
impl From<VCOLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: VCOLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VCOLOCK`"]
pub type VCOLOCK_R = crate::R<bool, VCOLOCK_A>;
impl VCOLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCOLOCK_A {
        match self.bits {
            false => VCOLOCK_A::CONST_0,
            true => VCOLOCK_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == VCOLOCK_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == VCOLOCK_A::CONST_1
    }
}
#[doc = "K1 Divider Ready Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum K1RDY_A {
    #[doc = "0: K1-Divider does not operate with the new value"]
    CONST_0 = 0,
    #[doc = "1: K1-Divider operate with the new value"]
    CONST_1 = 1,
}
impl From<K1RDY_A> for bool {
    #[inline(always)]
    fn from(variant: K1RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `K1RDY`"]
pub type K1RDY_R = crate::R<bool, K1RDY_A>;
impl K1RDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> K1RDY_A {
        match self.bits {
            false => K1RDY_A::CONST_0,
            true => K1RDY_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == K1RDY_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == K1RDY_A::CONST_1
    }
}
#[doc = "K2 Divider Ready Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum K2RDY_A {
    #[doc = "0: K2-Divider does not operate with the new value"]
    CONST_0 = 0,
    #[doc = "1: K2-Divider operate with the new value"]
    CONST_1 = 1,
}
impl From<K2RDY_A> for bool {
    #[inline(always)]
    fn from(variant: K2RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `K2RDY`"]
pub type K2RDY_R = crate::R<bool, K2RDY_A>;
impl K2RDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> K2RDY_A {
        match self.bits {
            false => K2RDY_A::CONST_0,
            true => K2RDY_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == K2RDY_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == K2RDY_A::CONST_1
    }
}
#[doc = "Bypass Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BY_A {
    #[doc = "0: Bypass Mode is not entered"]
    CONST_0 = 0,
    #[doc = "1: Bypass Mode is entered. Input fOSC is selected as output fPLL."]
    CONST_1 = 1,
}
impl From<BY_A> for bool {
    #[inline(always)]
    fn from(variant: BY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BY`"]
pub type BY_R = crate::R<bool, BY_A>;
impl BY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BY_A {
        match self.bits {
            false => BY_A::CONST_0,
            true => BY_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == BY_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == BY_A::CONST_1
    }
}
#[doc = "Oscillator for PLL Valid Low Status Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLLV_A {
    #[doc = "0: The OSC frequency is not usable. Frequency fREF is too low."]
    CONST_0 = 0,
    #[doc = "1: The OSC frequency is usable"]
    CONST_1 = 1,
}
impl From<PLLLV_A> for bool {
    #[inline(always)]
    fn from(variant: PLLLV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLLV`"]
pub type PLLLV_R = crate::R<bool, PLLLV_A>;
impl PLLLV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLLV_A {
        match self.bits {
            false => PLLLV_A::CONST_0,
            true => PLLLV_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PLLLV_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PLLLV_A::CONST_1
    }
}
#[doc = "Oscillator for PLL Valid High Status Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLHV_A {
    #[doc = "0: The OSC frequency is not usable. Frequency fOSC is too high."]
    CONST_0 = 0,
    #[doc = "1: The OSC frequency is usable"]
    CONST_1 = 1,
}
impl From<PLLHV_A> for bool {
    #[inline(always)]
    fn from(variant: PLLHV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLHV`"]
pub type PLLHV_R = crate::R<bool, PLLHV_A>;
impl PLLHV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLHV_A {
        match self.bits {
            false => PLLHV_A::CONST_0,
            true => PLLHV_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PLLHV_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PLLHV_A::CONST_1
    }
}
#[doc = "Oscillator for PLL Valid Spike Status Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSP_A {
    #[doc = "0: The OSC frequency is not usable. Spikes are detected that disturb a locked operation"]
    CONST_0 = 0,
    #[doc = "1: The OSC frequency is usable"]
    CONST_1 = 1,
}
impl From<PLLSP_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLSP`"]
pub type PLLSP_R = crate::R<bool, PLLSP_A>;
impl PLLSP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSP_A {
        match self.bits {
            false => PLLSP_A::CONST_0,
            true => PLLSP_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PLLSP_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PLLSP_A::CONST_1
    }
}
impl R {
    #[doc = "Bit 0 - VCO Bypass Status"]
    #[inline(always)]
    pub fn vcobyst(&self) -> VCOBYST_R {
        VCOBYST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PLL Power-saving Mode Status"]
    #[inline(always)]
    pub fn pwdstat(&self) -> PWDSTAT_R {
        PWDSTAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PLL LOCK Status"]
    #[inline(always)]
    pub fn vcolock(&self) -> VCOLOCK_R {
        VCOLOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - K1 Divider Ready Status"]
    #[inline(always)]
    pub fn k1rdy(&self) -> K1RDY_R {
        K1RDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - K2 Divider Ready Status"]
    #[inline(always)]
    pub fn k2rdy(&self) -> K2RDY_R {
        K2RDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Bypass Mode Status"]
    #[inline(always)]
    pub fn by(&self) -> BY_R {
        BY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Oscillator for PLL Valid Low Status Bit"]
    #[inline(always)]
    pub fn plllv(&self) -> PLLLV_R {
        PLLLV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Oscillator for PLL Valid High Status Bit"]
    #[inline(always)]
    pub fn pllhv(&self) -> PLLHV_R {
        PLLHV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Oscillator for PLL Valid Spike Status Bit"]
    #[inline(always)]
    pub fn pllsp(&self) -> PLLSP_R {
        PLLSP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
