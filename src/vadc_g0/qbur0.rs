#[doc = "Register `QBUR0` reader"]
pub type R = crate::R<Qbur0Spec>;
#[doc = "Field `REQCHNR` reader - Request Channel Number"]
pub type ReqchnrR = crate::FieldReader;
#[doc = "Field `RF` reader - Refill"]
pub type RfR = crate::BitReader;
#[doc = "Field `ENSI` reader - Enable Source Interrupt"]
pub type EnsiR = crate::BitReader;
#[doc = "Field `EXTR` reader - External Trigger"]
pub type ExtrR = crate::BitReader;
#[doc = "Request Channel Number Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum V {
    #[doc = "0: Backup register not valid"]
    Value1 = 0,
    #[doc = "1: Backup register contains a valid entry. This will be requested before a valid entry in queue register 0 (stage 0) will be requested."]
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
    #[doc = "Backup register not valid"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == V::Value1
    }
    #[doc = "Backup register contains a valid entry. This will be requested before a valid entry in queue register 0 (stage 0) will be requested."]
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
#[doc = "Queue 0 Backup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qbur0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Qbur0Spec;
impl crate::RegisterSpec for Qbur0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qbur0::R`](R) reader structure"]
impl crate::Readable for Qbur0Spec {}
#[doc = "`reset()` method sets QBUR0 to value 0"]
impl crate::Resettable for Qbur0Spec {
    const RESET_VALUE: u32 = 0;
}
