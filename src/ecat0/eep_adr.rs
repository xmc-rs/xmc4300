#[doc = "Register `EEP_ADR` reader"]
pub type R = crate::R<EepAdrSpec>;
#[doc = "Register `EEP_ADR` writer"]
pub type W = crate::W<EepAdrSpec>;
#[doc = "EEPROM Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum EepromAddr {
    #[doc = "0: First word (= 16 bits)"]
    Value1 = 0,
    #[doc = "1: Second word"]
    Value2 = 1,
}
impl From<EepromAddr> for u32 {
    #[inline(always)]
    fn from(variant: EepromAddr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EepromAddr {
    type Ux = u32;
}
#[doc = "Field `EEPROM_ADDR` reader - EEPROM Address"]
pub type EepromAddrR = crate::FieldReader<EepromAddr>;
impl EepromAddrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EepromAddr> {
        match self.bits {
            0 => Some(EepromAddr::Value1),
            1 => Some(EepromAddr::Value2),
            _ => None,
        }
    }
    #[doc = "First word (= 16 bits)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EepromAddr::Value1
    }
    #[doc = "Second word"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EepromAddr::Value2
    }
}
#[doc = "Field `EEPROM_ADDR` writer - EEPROM Address"]
pub type EepromAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, EepromAddr>;
impl<'a, REG> EepromAddrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "First word (= 16 bits)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EepromAddr::Value1)
    }
    #[doc = "Second word"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EepromAddr::Value2)
    }
}
impl R {
    #[doc = "Bits 0:31 - EEPROM Address"]
    #[inline(always)]
    pub fn eeprom_addr(&self) -> EepromAddrR {
        EepromAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - EEPROM Address"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_addr(&mut self) -> EepromAddrW<EepAdrSpec> {
        EepromAddrW::new(self, 0)
    }
}
#[doc = "EEPROM Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eep_adr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eep_adr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EepAdrSpec;
impl crate::RegisterSpec for EepAdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eep_adr::R`](R) reader structure"]
impl crate::Readable for EepAdrSpec {}
#[doc = "`write(|w| ..)` method takes [`eep_adr::W`](W) writer structure"]
impl crate::Writable for EepAdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EEP_ADR to value 0"]
impl crate::Resettable for EepAdrSpec {
    const RESET_VALUE: u32 = 0;
}
