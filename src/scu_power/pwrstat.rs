#[doc = "Register `PWRSTAT` reader"]
pub struct R(crate::R<PWRSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HIBEN` reader - Hibernate Domain Enable Status"]
pub type HIBEN_R = crate::BitReader<HIBEN_A>;
#[doc = "Hibernate Domain Enable Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIBEN_A {
    #[doc = "0: Inactive"]
    CONST_0 = 0,
    #[doc = "1: Active"]
    CONST_1 = 1,
}
impl From<HIBEN_A> for bool {
    #[inline(always)]
    fn from(variant: HIBEN_A) -> Self {
        variant as u8 != 0
    }
}
impl HIBEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIBEN_A {
        match self.bits {
            false => HIBEN_A::CONST_0,
            true => HIBEN_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == HIBEN_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == HIBEN_A::CONST_1
    }
}
#[doc = "Field `USBPHYPDQ` reader - USB PHY Transceiver State"]
pub type USBPHYPDQ_R = crate::BitReader<USBPHYPDQ_A>;
#[doc = "USB PHY Transceiver State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBPHYPDQ_A {
    #[doc = "0: Power-down"]
    CONST_0 = 0,
    #[doc = "1: Active"]
    CONST_1 = 1,
}
impl From<USBPHYPDQ_A> for bool {
    #[inline(always)]
    fn from(variant: USBPHYPDQ_A) -> Self {
        variant as u8 != 0
    }
}
impl USBPHYPDQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBPHYPDQ_A {
        match self.bits {
            false => USBPHYPDQ_A::CONST_0,
            true => USBPHYPDQ_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == USBPHYPDQ_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == USBPHYPDQ_A::CONST_1
    }
}
#[doc = "Field `USBOTGEN` reader - USB On-The-Go Comparators State"]
pub type USBOTGEN_R = crate::BitReader<USBOTGEN_A>;
#[doc = "USB On-The-Go Comparators State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBOTGEN_A {
    #[doc = "0: Power-down"]
    CONST_0 = 0,
    #[doc = "1: Active"]
    CONST_1 = 1,
}
impl From<USBOTGEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBOTGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl USBOTGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBOTGEN_A {
        match self.bits {
            false => USBOTGEN_A::CONST_0,
            true => USBOTGEN_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == USBOTGEN_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == USBOTGEN_A::CONST_1
    }
}
#[doc = "Field `USBPUWQ` reader - USB Weak Pull-Up at PADN State"]
pub type USBPUWQ_R = crate::BitReader<USBPUWQ_A>;
#[doc = "USB Weak Pull-Up at PADN State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBPUWQ_A {
    #[doc = "0: Pull-up active"]
    CONST_0 = 0,
    #[doc = "1: Pull-up not active"]
    CONST_1 = 1,
}
impl From<USBPUWQ_A> for bool {
    #[inline(always)]
    fn from(variant: USBPUWQ_A) -> Self {
        variant as u8 != 0
    }
}
impl USBPUWQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBPUWQ_A {
        match self.bits {
            false => USBPUWQ_A::CONST_0,
            true => USBPUWQ_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == USBPUWQ_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == USBPUWQ_A::CONST_1
    }
}
impl R {
    #[doc = "Bit 0 - Hibernate Domain Enable Status"]
    #[inline(always)]
    pub fn hiben(&self) -> HIBEN_R {
        HIBEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - USB PHY Transceiver State"]
    #[inline(always)]
    pub fn usbphypdq(&self) -> USBPHYPDQ_R {
        USBPHYPDQ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USB On-The-Go Comparators State"]
    #[inline(always)]
    pub fn usbotgen(&self) -> USBOTGEN_R {
        USBOTGEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USB Weak Pull-Up at PADN State"]
    #[inline(always)]
    pub fn usbpuwq(&self) -> USBPUWQ_R {
        USBPUWQ_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[doc = "PCU Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrstat](index.html) module"]
pub struct PWRSTAT_SPEC;
impl crate::RegisterSpec for PWRSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrstat::R](R) reader structure"]
impl crate::Readable for PWRSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWRSTAT to value 0"]
impl crate::Resettable for PWRSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
