#[doc = "Reader of register QBUR0"]
pub type R = crate::R<u32, super::QBUR0>;
#[doc = "Reader of field `REQCHNR`"]
pub type REQCHNR_R = crate::R<u8, u8>;
#[doc = "Reader of field `RF`"]
pub type RF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENSI`"]
pub type ENSI_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTR`"]
pub type EXTR_R = crate::R<bool, bool>;
#[doc = "Request Channel Number Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `V`"]
pub type V_R = crate::R<bool, V_A>;
impl V_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> V_A {
        match self.bits {
            false => V_A::VALUE1,
            true => V_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == V_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
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
        RF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable Source Interrupt"]
    #[inline(always)]
    pub fn ensi(&self) -> ENSI_R {
        ENSI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External Trigger"]
    #[inline(always)]
    pub fn extr(&self) -> EXTR_R {
        EXTR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Request Channel Number Valid"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
