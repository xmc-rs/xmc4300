#[doc = "Register `SLOT_INT_STATUS` reader"]
pub type R = crate::R<SlotIntStatusSpec>;
#[doc = "Interrupt Signal for Card Slot\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SlotIntStatus {
    #[doc = "0: Slot 1"]
    Value1 = 0,
}
impl From<SlotIntStatus> for u8 {
    #[inline(always)]
    fn from(variant: SlotIntStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SlotIntStatus {
    type Ux = u8;
}
#[doc = "Field `SLOT_INT_STATUS` reader - Interrupt Signal for Card Slot"]
pub type SlotIntStatusR = crate::FieldReader<SlotIntStatus>;
impl SlotIntStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SlotIntStatus> {
        match self.bits {
            0 => Some(SlotIntStatus::Value1),
            _ => None,
        }
    }
    #[doc = "Slot 1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SlotIntStatus::Value1
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt Signal for Card Slot"]
    #[inline(always)]
    pub fn slot_int_status(&self) -> SlotIntStatusR {
        SlotIntStatusR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Slot Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slot_int_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlotIntStatusSpec;
impl crate::RegisterSpec for SlotIntStatusSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`slot_int_status::R`](R) reader structure"]
impl crate::Readable for SlotIntStatusSpec {}
#[doc = "`reset()` method sets SLOT_INT_STATUS to value 0"]
impl crate::Resettable for SlotIntStatusSpec {
    const RESET_VALUE: u16 = 0;
}
