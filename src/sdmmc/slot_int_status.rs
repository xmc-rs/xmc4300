#[doc = "Register `SLOT_INT_STATUS` reader"]
pub type R = crate::R<SLOT_INT_STATUS_SPEC>;
#[doc = "Interrupt Signal for Card Slot\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SLOT_INT_STATUS_A {
    #[doc = "0: Slot 1"]
    VALUE1 = 0,
}
impl From<SLOT_INT_STATUS_A> for u8 {
    #[inline(always)]
    fn from(variant: SLOT_INT_STATUS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SLOT_INT_STATUS_A {
    type Ux = u8;
}
impl crate::IsEnum for SLOT_INT_STATUS_A {}
#[doc = "Field `SLOT_INT_STATUS` reader - Interrupt Signal for Card Slot"]
pub type SLOT_INT_STATUS_R = crate::FieldReader<SLOT_INT_STATUS_A>;
impl SLOT_INT_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SLOT_INT_STATUS_A> {
        match self.bits {
            0 => Some(SLOT_INT_STATUS_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Slot 1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SLOT_INT_STATUS_A::VALUE1
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt Signal for Card Slot"]
    #[inline(always)]
    pub fn slot_int_status(&self) -> SLOT_INT_STATUS_R {
        SLOT_INT_STATUS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Slot Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slot_int_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLOT_INT_STATUS_SPEC;
impl crate::RegisterSpec for SLOT_INT_STATUS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`slot_int_status::R`](R) reader structure"]
impl crate::Readable for SLOT_INT_STATUS_SPEC {}
#[doc = "`reset()` method sets SLOT_INT_STATUS to value 0"]
impl crate::Resettable for SLOT_INT_STATUS_SPEC {
    const RESET_VALUE: u16 = 0;
}
