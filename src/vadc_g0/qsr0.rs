#[doc = "Register `QSR0` reader"]
pub type R = crate::R<Qsr0Spec>;
#[doc = "Filling Level for Queue 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fill {
    #[doc = "0: There is 1 ( if EMPTY = 0) or no (if EMPTY = 1) valid entry in the queue"]
    Value1 = 0,
    #[doc = "1: There are 2 valid entries in the queue"]
    Value2 = 1,
    #[doc = "2: There are 3 valid entries in the queue"]
    Value3 = 2,
    #[doc = "7: There are 8 valid entries in the queue"]
    Value4 = 7,
}
impl From<Fill> for u8 {
    #[inline(always)]
    fn from(variant: Fill) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fill {
    type Ux = u8;
}
#[doc = "Field `FILL` reader - Filling Level for Queue 2"]
pub type FillR = crate::FieldReader<Fill>;
impl FillR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fill> {
        match self.bits {
            0 => Some(Fill::Value1),
            1 => Some(Fill::Value2),
            2 => Some(Fill::Value3),
            7 => Some(Fill::Value4),
            _ => None,
        }
    }
    #[doc = "There is 1 ( if EMPTY = 0) or no (if EMPTY = 1) valid entry in the queue"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Fill::Value1
    }
    #[doc = "There are 2 valid entries in the queue"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Fill::Value2
    }
    #[doc = "There are 3 valid entries in the queue"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Fill::Value3
    }
    #[doc = "There are 8 valid entries in the queue"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Fill::Value4
    }
}
#[doc = "Queue Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Empty {
    #[doc = "0: There are valid entries in the queue (see FILL)"]
    Value1 = 0,
    #[doc = "1: No valid entries (queue is empty)"]
    Value2 = 1,
}
impl From<Empty> for bool {
    #[inline(always)]
    fn from(variant: Empty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMPTY` reader - Queue Empty"]
pub type EmptyR = crate::BitReader<Empty>;
impl EmptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Empty {
        match self.bits {
            false => Empty::Value1,
            true => Empty::Value2,
        }
    }
    #[doc = "There are valid entries in the queue (see FILL)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Empty::Value1
    }
    #[doc = "No valid entries (queue is empty)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Empty::Value2
    }
}
#[doc = "Request Gate Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reqgt {
    #[doc = "0: The gate input is low"]
    Value1 = 0,
    #[doc = "1: The gate input is high"]
    Value2 = 1,
}
impl From<Reqgt> for bool {
    #[inline(always)]
    fn from(variant: Reqgt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQGT` reader - Request Gate Level"]
pub type ReqgtR = crate::BitReader<Reqgt>;
impl ReqgtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reqgt {
        match self.bits {
            false => Reqgt::Value1,
            true => Reqgt::Value2,
        }
    }
    #[doc = "The gate input is low"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Reqgt::Value1
    }
    #[doc = "The gate input is high"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Reqgt::Value2
    }
}
#[doc = "Event Detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev {
    #[doc = "0: No trigger event"]
    Value1 = 0,
    #[doc = "1: A trigger event has been detected"]
    Value2 = 1,
}
impl From<Ev> for bool {
    #[inline(always)]
    fn from(variant: Ev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV` reader - Event Detected"]
pub type EvR = crate::BitReader<Ev>;
impl EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev {
        match self.bits {
            false => Ev::Value1,
            true => Ev::Value2,
        }
    }
    #[doc = "No trigger event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ev::Value1
    }
    #[doc = "A trigger event has been detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ev::Value2
    }
}
impl R {
    #[doc = "Bits 0:3 - Filling Level for Queue 2"]
    #[inline(always)]
    pub fn fill(&self) -> FillR {
        FillR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - Queue Empty"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Request Gate Level"]
    #[inline(always)]
    pub fn reqgt(&self) -> ReqgtR {
        ReqgtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Event Detected"]
    #[inline(always)]
    pub fn ev(&self) -> EvR {
        EvR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Queue 0 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qsr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Qsr0Spec;
impl crate::RegisterSpec for Qsr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qsr0::R`](R) reader structure"]
impl crate::Readable for Qsr0Spec {}
#[doc = "`reset()` method sets QSR0 to value 0x20"]
impl crate::Resettable for Qsr0Spec {
    const RESET_VALUE: u32 = 0x20;
}
