#[doc = "Reader of register PRSTAT1"]
pub type R = crate::R<u32, super::PRSTAT1>;
#[doc = "LEDTS Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEDTSCU0RS_A {
    #[doc = "0: Reset de-asserted"]
    CONST_0,
    #[doc = "1: Reset asserted"]
    CONST_1,
}
impl From<LEDTSCU0RS_A> for bool {
    #[inline(always)]
    fn from(variant: LEDTSCU0RS_A) -> Self {
        match variant {
            LEDTSCU0RS_A::CONST_0 => false,
            LEDTSCU0RS_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `LEDTSCU0RS`"]
pub type LEDTSCU0RS_R = crate::R<bool, LEDTSCU0RS_A>;
impl LEDTSCU0RS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEDTSCU0RS_A {
        match self.bits {
            false => LEDTSCU0RS_A::CONST_0,
            true => LEDTSCU0RS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == LEDTSCU0RS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == LEDTSCU0RS_A::CONST_1
    }
}
#[doc = "MultiCAN Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCAN0RS_A {
    #[doc = "0: Reset de-asserted"]
    CONST_0,
    #[doc = "1: Reset asserted"]
    CONST_1,
}
impl From<MCAN0RS_A> for bool {
    #[inline(always)]
    fn from(variant: MCAN0RS_A) -> Self {
        match variant {
            MCAN0RS_A::CONST_0 => false,
            MCAN0RS_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `MCAN0RS`"]
pub type MCAN0RS_R = crate::R<bool, MCAN0RS_A>;
impl MCAN0RS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCAN0RS_A {
        match self.bits {
            false => MCAN0RS_A::CONST_0,
            true => MCAN0RS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == MCAN0RS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == MCAN0RS_A::CONST_1
    }
}
#[doc = "DAC Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACRS_A {
    #[doc = "0: Reset de-asserted"]
    CONST_0,
    #[doc = "1: Reset asserted"]
    CONST_1,
}
impl From<DACRS_A> for bool {
    #[inline(always)]
    fn from(variant: DACRS_A) -> Self {
        match variant {
            DACRS_A::CONST_0 => false,
            DACRS_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `DACRS`"]
pub type DACRS_R = crate::R<bool, DACRS_A>;
impl DACRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACRS_A {
        match self.bits {
            false => DACRS_A::CONST_0,
            true => DACRS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == DACRS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == DACRS_A::CONST_1
    }
}
#[doc = "MMC Interface Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMCIRS_A {
    #[doc = "0: Reset de-asserted"]
    CONST_0,
    #[doc = "1: Reset asserted"]
    CONST_1,
}
impl From<MMCIRS_A> for bool {
    #[inline(always)]
    fn from(variant: MMCIRS_A) -> Self {
        match variant {
            MMCIRS_A::CONST_0 => false,
            MMCIRS_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `MMCIRS`"]
pub type MMCIRS_R = crate::R<bool, MMCIRS_A>;
impl MMCIRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMCIRS_A {
        match self.bits {
            false => MMCIRS_A::CONST_0,
            true => MMCIRS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == MMCIRS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == MMCIRS_A::CONST_1
    }
}
#[doc = "USIC1 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC1RS_A {
    #[doc = "0: Reset de-asserted"]
    CONST_0,
    #[doc = "1: Reset asserted"]
    CONST_1,
}
impl From<USIC1RS_A> for bool {
    #[inline(always)]
    fn from(variant: USIC1RS_A) -> Self {
        match variant {
            USIC1RS_A::CONST_0 => false,
            USIC1RS_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `USIC1RS`"]
pub type USIC1RS_R = crate::R<bool, USIC1RS_A>;
impl USIC1RS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USIC1RS_A {
        match self.bits {
            false => USIC1RS_A::CONST_0,
            true => USIC1RS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == USIC1RS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == USIC1RS_A::CONST_1
    }
}
#[doc = "PORTS Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPORTSRS_A {
    #[doc = "0: Reset de-asserted"]
    CONST_0,
    #[doc = "1: Reset asserted"]
    CONST_1,
}
impl From<PPORTSRS_A> for bool {
    #[inline(always)]
    fn from(variant: PPORTSRS_A) -> Self {
        match variant {
            PPORTSRS_A::CONST_0 => false,
            PPORTSRS_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `PPORTSRS`"]
pub type PPORTSRS_R = crate::R<bool, PPORTSRS_A>;
impl PPORTSRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPORTSRS_A {
        match self.bits {
            false => PPORTSRS_A::CONST_0,
            true => PPORTSRS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PPORTSRS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PPORTSRS_A::CONST_1
    }
}
impl R {
    #[doc = "Bit 3 - LEDTS Reset Status"]
    #[inline(always)]
    pub fn ledtscu0rs(&self) -> LEDTSCU0RS_R {
        LEDTSCU0RS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MultiCAN Reset Status"]
    #[inline(always)]
    pub fn mcan0rs(&self) -> MCAN0RS_R {
        MCAN0RS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DAC Reset Status"]
    #[inline(always)]
    pub fn dacrs(&self) -> DACRS_R {
        DACRS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MMC Interface Reset Status"]
    #[inline(always)]
    pub fn mmcirs(&self) -> MMCIRS_R {
        MMCIRS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USIC1 Reset Status"]
    #[inline(always)]
    pub fn usic1rs(&self) -> USIC1RS_R {
        USIC1RS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PORTS Reset Status"]
    #[inline(always)]
    pub fn pportsrs(&self) -> PPORTSRS_R {
        PPORTSRS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
