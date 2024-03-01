#[doc = "Register `ERR_LED` reader"]
pub type R = crate::R<ErrLedSpec>;
#[doc = "Register `ERR_LED` writer"]
pub type W = crate::W<ErrLedSpec>;
#[doc = "LED Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LedCode {
    #[doc = "0: OFF"]
    Value1 = 0,
    #[doc = "13: Blinking"]
    Value2 = 13,
    #[doc = "14: Flickering"]
    Value3 = 14,
    #[doc = "15: On"]
    Value4 = 15,
}
impl From<LedCode> for u8 {
    #[inline(always)]
    fn from(variant: LedCode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LedCode {
    type Ux = u8;
}
#[doc = "Field `LED_CODE` reader - LED Code"]
pub type LedCodeR = crate::FieldReader<LedCode>;
impl LedCodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LedCode> {
        match self.bits {
            0 => Some(LedCode::Value1),
            13 => Some(LedCode::Value2),
            14 => Some(LedCode::Value3),
            15 => Some(LedCode::Value4),
            _ => None,
        }
    }
    #[doc = "OFF"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LedCode::Value1
    }
    #[doc = "Blinking"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LedCode::Value2
    }
    #[doc = "Flickering"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LedCode::Value3
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == LedCode::Value4
    }
}
#[doc = "Field `LED_CODE` writer - LED Code"]
pub type LedCodeW<'a, REG> = crate::FieldWriter<'a, REG, 4, LedCode>;
impl<'a, REG> LedCodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OFF"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LedCode::Value1)
    }
    #[doc = "Blinking"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LedCode::Value2)
    }
    #[doc = "Flickering"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(LedCode::Value3)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(LedCode::Value4)
    }
}
#[doc = "Enable Override\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnOverr {
    #[doc = "0: Override disable"]
    Value1 = 0,
    #[doc = "1: Override enable"]
    Value2 = 1,
}
impl From<EnOverr> for bool {
    #[inline(always)]
    fn from(variant: EnOverr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN_OVERR` reader - Enable Override"]
pub type EnOverrR = crate::BitReader<EnOverr>;
impl EnOverrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnOverr {
        match self.bits {
            false => EnOverr::Value1,
            true => EnOverr::Value2,
        }
    }
    #[doc = "Override disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EnOverr::Value1
    }
    #[doc = "Override enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EnOverr::Value2
    }
}
#[doc = "Field `EN_OVERR` writer - Enable Override"]
pub type EnOverrW<'a, REG> = crate::BitWriter<'a, REG, EnOverr>;
impl<'a, REG> EnOverrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EnOverr::Value1)
    }
    #[doc = "Override enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EnOverr::Value2)
    }
}
impl R {
    #[doc = "Bits 0:3 - LED Code"]
    #[inline(always)]
    pub fn led_code(&self) -> LedCodeR {
        LedCodeR::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - Enable Override"]
    #[inline(always)]
    pub fn en_overr(&self) -> EnOverrR {
        EnOverrR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - LED Code"]
    #[inline(always)]
    #[must_use]
    pub fn led_code(&mut self) -> LedCodeW<ErrLedSpec> {
        LedCodeW::new(self, 0)
    }
    #[doc = "Bit 4 - Enable Override"]
    #[inline(always)]
    #[must_use]
    pub fn en_overr(&mut self) -> EnOverrW<ErrLedSpec> {
        EnOverrW::new(self, 4)
    }
}
#[doc = "RUN ERR Override\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_led::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_led::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrLedSpec;
impl crate::RegisterSpec for ErrLedSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`err_led::R`](R) reader structure"]
impl crate::Readable for ErrLedSpec {}
#[doc = "`write(|w| ..)` method takes [`err_led::W`](W) writer structure"]
impl crate::Writable for ErrLedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ERR_LED to value 0"]
impl crate::Resettable for ErrLedSpec {
    const RESET_VALUE: u8 = 0;
}
