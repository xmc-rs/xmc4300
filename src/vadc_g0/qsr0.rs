#[doc = "Reader of register QSR0"]
pub type R = crate::R<u32, super::QSR0>;
#[doc = "Filling Level for Queue 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILL_A {
    #[doc = "0: There is 1 ( if EMPTY = 0) or no (if EMPTY = 1) valid entry in the queue"]
    VALUE1,
    #[doc = "1: There are 2 valid entries in the queue"]
    VALUE2,
    #[doc = "2: There are 3 valid entries in the queue"]
    VALUE3,
    #[doc = "7: There are 8 valid entries in the queue"]
    VALUE4,
}
impl From<FILL_A> for u8 {
    #[inline(always)]
    fn from(variant: FILL_A) -> Self {
        match variant {
            FILL_A::VALUE1 => 0,
            FILL_A::VALUE2 => 1,
            FILL_A::VALUE3 => 2,
            FILL_A::VALUE4 => 7,
        }
    }
}
#[doc = "Reader of field `FILL`"]
pub type FILL_R = crate::R<u8, FILL_A>;
impl FILL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FILL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FILL_A::VALUE1),
            1 => Val(FILL_A::VALUE2),
            2 => Val(FILL_A::VALUE3),
            7 => Val(FILL_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FILL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FILL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == FILL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == FILL_A::VALUE4
    }
}
#[doc = "Queue Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMPTY_A {
    #[doc = "0: There are valid entries in the queue (see FILL)"]
    VALUE1,
    #[doc = "1: No valid entries (queue is empty)"]
    VALUE2,
}
impl From<EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: EMPTY_A) -> Self {
        match variant {
            EMPTY_A::VALUE1 => false,
            EMPTY_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `EMPTY`"]
pub type EMPTY_R = crate::R<bool, EMPTY_A>;
impl EMPTY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMPTY_A {
        match self.bits {
            false => EMPTY_A::VALUE1,
            true => EMPTY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EMPTY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EMPTY_A::VALUE2
    }
}
#[doc = "Request Gate Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQGT_A {
    #[doc = "0: The gate input is low"]
    VALUE1,
    #[doc = "1: The gate input is high"]
    VALUE2,
}
impl From<REQGT_A> for bool {
    #[inline(always)]
    fn from(variant: REQGT_A) -> Self {
        match variant {
            REQGT_A::VALUE1 => false,
            REQGT_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `REQGT`"]
pub type REQGT_R = crate::R<bool, REQGT_A>;
impl REQGT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQGT_A {
        match self.bits {
            false => REQGT_A::VALUE1,
            true => REQGT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REQGT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REQGT_A::VALUE2
    }
}
#[doc = "Event Detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV_A {
    #[doc = "0: No trigger event"]
    VALUE1,
    #[doc = "1: A trigger event has been detected"]
    VALUE2,
}
impl From<EV_A> for bool {
    #[inline(always)]
    fn from(variant: EV_A) -> Self {
        match variant {
            EV_A::VALUE1 => false,
            EV_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `EV`"]
pub type EV_R = crate::R<bool, EV_A>;
impl EV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EV_A {
        match self.bits {
            false => EV_A::VALUE1,
            true => EV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:3 - Filling Level for Queue 2"]
    #[inline(always)]
    pub fn fill(&self) -> FILL_R {
        FILL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - Queue Empty"]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Request Gate Level"]
    #[inline(always)]
    pub fn reqgt(&self) -> REQGT_R {
        REQGT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Event Detected"]
    #[inline(always)]
    pub fn ev(&self) -> EV_R {
        EV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
