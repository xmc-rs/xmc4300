#[doc = "Register `USBPLLSTAT` reader"]
pub struct R(crate::R<USBPLLSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPLLSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPLLSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPLLSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VCOBYST` reader - VCO Bypass Status"]
pub type VCOBYST_R = crate::BitReader<VCOBYST_A>;
#[doc = "VCO Bypass Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCOBYST_A {
    #[doc = "0: Normal Mode is entered"]
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
impl VCOBYST_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `PWDSTAT` reader - PLL Power-saving Mode Status"]
pub type PWDSTAT_R = crate::BitReader<PWDSTAT_A>;
#[doc = "PLL Power-saving Mode Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PWDSTAT_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `VCOLOCK` reader - PLL VCO Lock Status"]
pub type VCOLOCK_R = crate::BitReader<VCOLOCK_A>;
#[doc = "PLL VCO Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCOLOCK_A {
    #[doc = "0: The frequency difference of fREF and fDIV is greater than allowed. The VCO part of the PLL can not lock on a target frequency."]
    CONST_0 = 0,
    #[doc = "1: The frequency difference of fREF and fDIV is small enough to enable a stable VCO operation"]
    CONST_1 = 1,
}
impl From<VCOLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: VCOLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl VCOLOCK_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `BY` reader - Bypass Mode Status"]
pub type BY_R = crate::BitReader<BY_A>;
#[doc = "Bypass Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl BY_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `VCOLOCKED` reader - PLL LOCK Status"]
pub type VCOLOCKED_R = crate::BitReader<VCOLOCKED_A>;
#[doc = "PLL LOCK Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCOLOCKED_A {
    #[doc = "0: PLL not locked"]
    CONST_0 = 0,
    #[doc = "1: PLL locked"]
    CONST_1 = 1,
}
impl From<VCOLOCKED_A> for bool {
    #[inline(always)]
    fn from(variant: VCOLOCKED_A) -> Self {
        variant as u8 != 0
    }
}
impl VCOLOCKED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCOLOCKED_A {
        match self.bits {
            false => VCOLOCKED_A::CONST_0,
            true => VCOLOCKED_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == VCOLOCKED_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == VCOLOCKED_A::CONST_1
    }
}
impl R {
    #[doc = "Bit 0 - VCO Bypass Status"]
    #[inline(always)]
    pub fn vcobyst(&self) -> VCOBYST_R {
        VCOBYST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL Power-saving Mode Status"]
    #[inline(always)]
    pub fn pwdstat(&self) -> PWDSTAT_R {
        PWDSTAT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PLL VCO Lock Status"]
    #[inline(always)]
    pub fn vcolock(&self) -> VCOLOCK_R {
        VCOLOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Bypass Mode Status"]
    #[inline(always)]
    pub fn by(&self) -> BY_R {
        BY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL LOCK Status"]
    #[inline(always)]
    pub fn vcolocked(&self) -> VCOLOCKED_R {
        VCOLOCKED_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "USB PLL Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbpllstat](index.html) module"]
pub struct USBPLLSTAT_SPEC;
impl crate::RegisterSpec for USBPLLSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbpllstat::R](R) reader structure"]
impl crate::Readable for USBPLLSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBPLLSTAT to value 0x02"]
impl crate::Resettable for USBPLLSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
