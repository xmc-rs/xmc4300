#[doc = "Register `EEP_STATE` reader"]
pub type R = crate::R<EepStateSpec>;
#[doc = "Register `EEP_STATE` writer"]
pub type W = crate::W<EepStateSpec>;
#[doc = "Access to EEPROM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Access {
    #[doc = "0: PDI releases EEPROM access"]
    Value1 = 0,
    #[doc = "1: PDI takes EEPROM access (PDI has EEPROM control)"]
    Value2 = 1,
}
impl From<Access> for bool {
    #[inline(always)]
    fn from(variant: Access) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCESS` reader - Access to EEPROM"]
pub type AccessR = crate::BitReader<Access>;
impl AccessR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Access {
        match self.bits {
            false => Access::Value1,
            true => Access::Value2,
        }
    }
    #[doc = "PDI releases EEPROM access"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Access::Value1
    }
    #[doc = "PDI takes EEPROM access (PDI has EEPROM control)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Access::Value2
    }
}
#[doc = "Field `ACCESS` writer - Access to EEPROM"]
pub type AccessW<'a, REG> = crate::BitWriter<'a, REG, Access>;
impl<'a, REG> AccessW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PDI releases EEPROM access"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Access::Value1)
    }
    #[doc = "PDI takes EEPROM access (PDI has EEPROM control)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Access::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Access to EEPROM"]
    #[inline(always)]
    pub fn access(&self) -> AccessR {
        AccessR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access to EEPROM"]
    #[inline(always)]
    #[must_use]
    pub fn access(&mut self) -> AccessW<EepStateSpec> {
        AccessW::new(self, 0)
    }
}
#[doc = "EEPROM PDI Access State\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eep_state::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eep_state::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EepStateSpec;
impl crate::RegisterSpec for EepStateSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eep_state::R`](R) reader structure"]
impl crate::Readable for EepStateSpec {}
#[doc = "`write(|w| ..)` method takes [`eep_state::W`](W) writer structure"]
impl crate::Writable for EepStateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets EEP_STATE to value 0"]
impl crate::Resettable for EepStateSpec {
    const RESET_VALUE: u8 = 0;
}
