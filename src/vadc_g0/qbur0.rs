#[doc = "Register `QBUR0` reader"]
pub type R = crate::R<QBUR0_SPEC>;
#[doc = "Field `REQCHNR` reader - Request Channel Number"]
pub type REQCHNR_R = crate::FieldReader;
#[doc = "Field `RF` reader - Refill"]
pub type RF_R = crate::BitReader;
#[doc = "Field `ENSI` reader - Enable Source Interrupt"]
pub type ENSI_R = crate::BitReader;
#[doc = "Field `EXTR` reader - External Trigger"]
pub type EXTR_R = crate::BitReader;
#[doc = "Request Channel Number Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum V_A {
    #[doc = "0: Backup register not valid"]
    VALUE1 = 0,
    #[doc = "1: Backup register contains a valid entry. This will be requested before a valid entry in queue register 0 (stage 0) will be requested."]
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
    #[doc = "Backup register not valid"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == V_A::VALUE1
    }
    #[doc = "Backup register contains a valid entry. This will be requested before a valid entry in queue register 0 (stage 0) will be requested."]
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
#[doc = "Queue 0 Backup Register\n\nYou can [`read`](crate::Reg::read) this register and get [`qbur0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QBUR0_SPEC;
impl crate::RegisterSpec for QBUR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qbur0::R`](R) reader structure"]
impl crate::Readable for QBUR0_SPEC {}
#[doc = "`reset()` method sets QBUR0 to value 0"]
impl crate::Resettable for QBUR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
