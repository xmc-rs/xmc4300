#[doc = "Register `Q0R0` reader"]
pub type R = crate::R<Q0r0Spec>;
#[doc = "Field `REQCHNR` reader - Request Channel Number"]
pub type ReqchnrR = crate::FieldReader;
#[doc = "Refill\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rf {
    #[doc = "0: The request is discarded after the conversion start."]
    Value1 = 0,
    #[doc = "1: The request is automatically refilled into the queue after the conversion start."]
    Value2 = 1,
}
impl From<Rf> for bool {
    #[inline(always)]
    fn from(variant: Rf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF` reader - Refill"]
pub type RfR = crate::BitReader<Rf>;
impl RfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rf {
        match self.bits {
            false => Rf::Value1,
            true => Rf::Value2,
        }
    }
    #[doc = "The request is discarded after the conversion start."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rf::Value1
    }
    #[doc = "The request is automatically refilled into the queue after the conversion start."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rf::Value2
    }
}
#[doc = "Enable Source Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ensi {
    #[doc = "0: No request source interrupt"]
    Value1 = 0,
    #[doc = "1: A request source event interrupt is generated upon a request source event (related conversion is finished)"]
    Value2 = 1,
}
impl From<Ensi> for bool {
    #[inline(always)]
    fn from(variant: Ensi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENSI` reader - Enable Source Interrupt"]
pub type EnsiR = crate::BitReader<Ensi>;
impl EnsiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ensi {
        match self.bits {
            false => Ensi::Value1,
            true => Ensi::Value2,
        }
    }
    #[doc = "No request source interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ensi::Value1
    }
    #[doc = "A request source event interrupt is generated upon a request source event (related conversion is finished)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ensi::Value2
    }
}
#[doc = "External Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extr {
    #[doc = "0: A valid queue entry immediately leads to a conversion request"]
    Value1 = 0,
    #[doc = "1: The request handler waits for a trigger event"]
    Value2 = 1,
}
impl From<Extr> for bool {
    #[inline(always)]
    fn from(variant: Extr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTR` reader - External Trigger"]
pub type ExtrR = crate::BitReader<Extr>;
impl ExtrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extr {
        match self.bits {
            false => Extr::Value1,
            true => Extr::Value2,
        }
    }
    #[doc = "A valid queue entry immediately leads to a conversion request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Extr::Value1
    }
    #[doc = "The request handler waits for a trigger event"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Extr::Value2
    }
}
#[doc = "Request Channel Number Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum V {
    #[doc = "0: No valid queue entry"]
    Value1 = 0,
    #[doc = "1: The queue entry is valid and leads to a conversion request"]
    Value2 = 1,
}
impl From<V> for bool {
    #[inline(always)]
    fn from(variant: V) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `V` reader - Request Channel Number Valid"]
pub type VR = crate::BitReader<V>;
impl VR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> V {
        match self.bits {
            false => V::Value1,
            true => V::Value2,
        }
    }
    #[doc = "No valid queue entry"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == V::Value1
    }
    #[doc = "The queue entry is valid and leads to a conversion request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == V::Value2
    }
}
impl R {
    #[doc = "Bits 0:4 - Request Channel Number"]
    #[inline(always)]
    pub fn reqchnr(&self) -> ReqchnrR {
        ReqchnrR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Refill"]
    #[inline(always)]
    pub fn rf(&self) -> RfR {
        RfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Source Interrupt"]
    #[inline(always)]
    pub fn ensi(&self) -> EnsiR {
        EnsiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Trigger"]
    #[inline(always)]
    pub fn extr(&self) -> ExtrR {
        ExtrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Request Channel Number Valid"]
    #[inline(always)]
    pub fn v(&self) -> VR {
        VR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Queue 0 Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`q0r0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Q0r0Spec;
impl crate::RegisterSpec for Q0r0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`q0r0::R`](R) reader structure"]
impl crate::Readable for Q0r0Spec {}
#[doc = "`reset()` method sets Q0R0 to value 0"]
impl crate::Resettable for Q0r0Spec {
    const RESET_VALUE: u32 = 0;
}
