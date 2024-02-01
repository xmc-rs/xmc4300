#[doc = "Register `QSR0` reader"]
pub type R = crate::R<QSR0_SPEC>;
#[doc = "Field `FILL` reader - Filling Level for Queue 2"]
pub type FILL_R = crate::FieldReader<FILL_A>;
#[doc = "Filling Level for Queue 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FILL_A {
    #[doc = "0: There is 1 ( if EMPTY = 0) or no (if EMPTY = 1) valid entry in the queue"]
    VALUE1 = 0,
    #[doc = "1: There are 2 valid entries in the queue"]
    VALUE2 = 1,
    #[doc = "2: There are 3 valid entries in the queue"]
    VALUE3 = 2,
    #[doc = "7: There are 8 valid entries in the queue"]
    VALUE4 = 7,
}
impl From<FILL_A> for u8 {
    #[inline(always)]
    fn from(variant: FILL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FILL_A {
    type Ux = u8;
}
impl FILL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FILL_A> {
        match self.bits {
            0 => Some(FILL_A::VALUE1),
            1 => Some(FILL_A::VALUE2),
            2 => Some(FILL_A::VALUE3),
            7 => Some(FILL_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "There is 1 ( if EMPTY = 0) or no (if EMPTY = 1) valid entry in the queue"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FILL_A::VALUE1
    }
    #[doc = "There are 2 valid entries in the queue"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FILL_A::VALUE2
    }
    #[doc = "There are 3 valid entries in the queue"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == FILL_A::VALUE3
    }
    #[doc = "There are 8 valid entries in the queue"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == FILL_A::VALUE4
    }
}
#[doc = "Field `EMPTY` reader - Queue Empty"]
pub type EMPTY_R = crate::BitReader<EMPTY_A>;
#[doc = "Queue Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EMPTY_A {
    #[doc = "0: There are valid entries in the queue (see FILL)"]
    VALUE1 = 0,
    #[doc = "1: No valid entries (queue is empty)"]
    VALUE2 = 1,
}
impl From<EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: EMPTY_A) -> Self {
        variant as u8 != 0
    }
}
impl EMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EMPTY_A {
        match self.bits {
            false => EMPTY_A::VALUE1,
            true => EMPTY_A::VALUE2,
        }
    }
    #[doc = "There are valid entries in the queue (see FILL)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EMPTY_A::VALUE1
    }
    #[doc = "No valid entries (queue is empty)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EMPTY_A::VALUE2
    }
}
#[doc = "Field `REQGT` reader - Request Gate Level"]
pub type REQGT_R = crate::BitReader<REQGT_A>;
#[doc = "Request Gate Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REQGT_A {
    #[doc = "0: The gate input is low"]
    VALUE1 = 0,
    #[doc = "1: The gate input is high"]
    VALUE2 = 1,
}
impl From<REQGT_A> for bool {
    #[inline(always)]
    fn from(variant: REQGT_A) -> Self {
        variant as u8 != 0
    }
}
impl REQGT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REQGT_A {
        match self.bits {
            false => REQGT_A::VALUE1,
            true => REQGT_A::VALUE2,
        }
    }
    #[doc = "The gate input is low"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REQGT_A::VALUE1
    }
    #[doc = "The gate input is high"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REQGT_A::VALUE2
    }
}
#[doc = "Field `EV` reader - Event Detected"]
pub type EV_R = crate::BitReader<EV_A>;
#[doc = "Event Detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EV_A {
    #[doc = "0: No trigger event"]
    VALUE1 = 0,
    #[doc = "1: A trigger event has been detected"]
    VALUE2 = 1,
}
impl From<EV_A> for bool {
    #[inline(always)]
    fn from(variant: EV_A) -> Self {
        variant as u8 != 0
    }
}
impl EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EV_A {
        match self.bits {
            false => EV_A::VALUE1,
            true => EV_A::VALUE2,
        }
    }
    #[doc = "No trigger event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV_A::VALUE1
    }
    #[doc = "A trigger event has been detected"]
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
        EMPTY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Request Gate Level"]
    #[inline(always)]
    pub fn reqgt(&self) -> REQGT_R {
        REQGT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Event Detected"]
    #[inline(always)]
    pub fn ev(&self) -> EV_R {
        EV_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Queue 0 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qsr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QSR0_SPEC;
impl crate::RegisterSpec for QSR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qsr0::R`](R) reader structure"]
impl crate::Readable for QSR0_SPEC {}
#[doc = "`reset()` method sets QSR0 to value 0x20"]
impl crate::Resettable for QSR0_SPEC {
    const RESET_VALUE: u32 = 0x20;
}
