#[doc = "Register `EEP_ADR` reader"]
pub type R = crate::R<EEP_ADR_SPEC>;
#[doc = "Register `EEP_ADR` writer"]
pub type W = crate::W<EEP_ADR_SPEC>;
#[doc = "EEPROM Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum EEPROM_ADDR_A {
    #[doc = "0: First word (= 16 bits)"]
    VALUE1 = 0,
    #[doc = "1: Second word"]
    VALUE2 = 1,
}
impl From<EEPROM_ADDR_A> for u32 {
    #[inline(always)]
    fn from(variant: EEPROM_ADDR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EEPROM_ADDR_A {
    type Ux = u32;
}
impl crate::IsEnum for EEPROM_ADDR_A {}
#[doc = "Field `EEPROM_ADDR` reader - EEPROM Address"]
pub type EEPROM_ADDR_R = crate::FieldReader<EEPROM_ADDR_A>;
impl EEPROM_ADDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EEPROM_ADDR_A> {
        match self.bits {
            0 => Some(EEPROM_ADDR_A::VALUE1),
            1 => Some(EEPROM_ADDR_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "First word (= 16 bits)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EEPROM_ADDR_A::VALUE1
    }
    #[doc = "Second word"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EEPROM_ADDR_A::VALUE2
    }
}
#[doc = "Field `EEPROM_ADDR` writer - EEPROM Address"]
pub type EEPROM_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, EEPROM_ADDR_A>;
impl<'a, REG> EEPROM_ADDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "First word (= 16 bits)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EEPROM_ADDR_A::VALUE1)
    }
    #[doc = "Second word"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EEPROM_ADDR_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:31 - EEPROM Address"]
    #[inline(always)]
    pub fn eeprom_addr(&self) -> EEPROM_ADDR_R {
        EEPROM_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - EEPROM Address"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_addr(&mut self) -> EEPROM_ADDR_W<EEP_ADR_SPEC> {
        EEPROM_ADDR_W::new(self, 0)
    }
}
#[doc = "EEPROM Address\n\nYou can [`read`](crate::Reg::read) this register and get [`eep_adr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eep_adr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EEP_ADR_SPEC;
impl crate::RegisterSpec for EEP_ADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eep_adr::R`](R) reader structure"]
impl crate::Readable for EEP_ADR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eep_adr::W`](W) writer structure"]
impl crate::Writable for EEP_ADR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EEP_ADR to value 0"]
impl crate::Resettable for EEP_ADR_SPEC {
    const RESET_VALUE: u32 = 0;
}
