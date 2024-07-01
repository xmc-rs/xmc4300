#[doc = "Register `Q0R0` reader"]
pub type R = crate::R<Q0R0_SPEC>;
#[doc = "Field `REQCHNR` reader - Request Channel Number"]
pub type REQCHNR_R = crate::FieldReader;
#[doc = "Refill\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF_A {
    #[doc = "0: The request is discarded after the conversion start."]
    VALUE1 = 0,
    #[doc = "1: The request is automatically refilled into the queue after the conversion start."]
    VALUE2 = 1,
}
impl From<RF_A> for bool {
    #[inline(always)]
    fn from(variant: RF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF` reader - Refill"]
pub type RF_R = crate::BitReader<RF_A>;
impl RF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RF_A {
        match self.bits {
            false => RF_A::VALUE1,
            true => RF_A::VALUE2,
        }
    }
    #[doc = "The request is discarded after the conversion start."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RF_A::VALUE1
    }
    #[doc = "The request is automatically refilled into the queue after the conversion start."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RF_A::VALUE2
    }
}
#[doc = "Enable Source Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENSI_A {
    #[doc = "0: No request source interrupt"]
    VALUE1 = 0,
    #[doc = "1: A request source event interrupt is generated upon a request source event (related conversion is finished)"]
    VALUE2 = 1,
}
impl From<ENSI_A> for bool {
    #[inline(always)]
    fn from(variant: ENSI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENSI` reader - Enable Source Interrupt"]
pub type ENSI_R = crate::BitReader<ENSI_A>;
impl ENSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENSI_A {
        match self.bits {
            false => ENSI_A::VALUE1,
            true => ENSI_A::VALUE2,
        }
    }
    #[doc = "No request source interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENSI_A::VALUE1
    }
    #[doc = "A request source event interrupt is generated upon a request source event (related conversion is finished)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENSI_A::VALUE2
    }
}
#[doc = "External Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTR_A {
    #[doc = "0: A valid queue entry immediately leads to a conversion request"]
    VALUE1 = 0,
    #[doc = "1: The request handler waits for a trigger event"]
    VALUE2 = 1,
}
impl From<EXTR_A> for bool {
    #[inline(always)]
    fn from(variant: EXTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTR` reader - External Trigger"]
pub type EXTR_R = crate::BitReader<EXTR_A>;
impl EXTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTR_A {
        match self.bits {
            false => EXTR_A::VALUE1,
            true => EXTR_A::VALUE2,
        }
    }
    #[doc = "A valid queue entry immediately leads to a conversion request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EXTR_A::VALUE1
    }
    #[doc = "The request handler waits for a trigger event"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EXTR_A::VALUE2
    }
}
#[doc = "Request Channel Number Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum V_A {
    #[doc = "0: No valid queue entry"]
    VALUE1 = 0,
    #[doc = "1: The queue entry is valid and leads to a conversion request"]
    VALUE2 = 1,
}
impl From<V_A> for bool {
    #[inline(always)]
    fn from(variant: V_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `V` reader - Request Channel Number Valid"]
pub type V_R = crate::BitReader<V_A>;
impl V_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> V_A {
        match self.bits {
            false => V_A::VALUE1,
            true => V_A::VALUE2,
        }
    }
    #[doc = "No valid queue entry"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == V_A::VALUE1
    }
    #[doc = "The queue entry is valid and leads to a conversion request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == V_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:4 - Request Channel Number"]
    #[inline(always)]
    pub fn reqchnr(&self) -> REQCHNR_R {
        REQCHNR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Refill"]
    #[inline(always)]
    pub fn rf(&self) -> RF_R {
        RF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Source Interrupt"]
    #[inline(always)]
    pub fn ensi(&self) -> ENSI_R {
        ENSI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Trigger"]
    #[inline(always)]
    pub fn extr(&self) -> EXTR_R {
        EXTR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Request Channel Number Valid"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Queue 0 Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`q0r0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Q0R0_SPEC;
impl crate::RegisterSpec for Q0R0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`q0r0::R`](R) reader structure"]
impl crate::Readable for Q0R0_SPEC {}
#[doc = "`reset()` method sets Q0R0 to value 0"]
impl crate::Resettable for Q0R0_SPEC {
    const RESET_VALUE: u32 = 0;
}
