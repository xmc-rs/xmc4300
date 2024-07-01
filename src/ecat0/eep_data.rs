#[doc = "Register `EEP_DATA[%s]` reader"]
pub type R = crate::R<EEP_DATA_SPEC>;
#[doc = "Register `EEP_DATA[%s]` writer"]
pub type W = crate::W<EEP_DATA_SPEC>;
#[doc = "Field `EEP_DATA` reader - EEPROM Data"]
pub type EEP_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `EEP_DATA` writer - EEPROM Data"]
pub type EEP_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - EEPROM Data"]
    #[inline(always)]
    pub fn eep_data(&self) -> EEP_DATA_R {
        EEP_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - EEPROM Data"]
    #[inline(always)]
    #[must_use]
    pub fn eep_data(&mut self) -> EEP_DATA_W<EEP_DATA_SPEC> {
        EEP_DATA_W::new(self, 0)
    }
}
#[doc = "EEPROM Read/Write data\n\nYou can [`read`](crate::Reg::read) this register and get [`eep_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eep_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EEP_DATA_SPEC;
impl crate::RegisterSpec for EEP_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eep_data::R`](R) reader structure"]
impl crate::Readable for EEP_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eep_data::W`](W) writer structure"]
impl crate::Writable for EEP_DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EEP_DATA[%s]
to value 0"]
impl crate::Resettable for EEP_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
