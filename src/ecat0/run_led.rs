#[doc = "Register `RUN_LED` reader"]
pub type R = crate::R<RUN_LED_SPEC>;
#[doc = "Register `RUN_LED` writer"]
pub type W = crate::W<RUN_LED_SPEC>;
#[doc = "LED Code\n\nValue on reset: 14"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LED_CODE_A {
    #[doc = "0: OFF - Init State"]
    VALUE1 = 0,
    #[doc = "1: Flash - SafeOp)"]
    VALUE2 = 1,
    #[doc = "13: Blinking - PreOp"]
    VALUE3 = 13,
    #[doc = "14: Flickering - Bootstrap"]
    VALUE4 = 14,
    #[doc = "15: On - Operational"]
    VALUE5 = 15,
}
impl From<LED_CODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LED_CODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LED_CODE_A {
    type Ux = u8;
}
impl crate::IsEnum for LED_CODE_A {}
#[doc = "Field `LED_CODE` reader - LED Code"]
pub type LED_CODE_R = crate::FieldReader<LED_CODE_A>;
impl LED_CODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LED_CODE_A> {
        match self.bits {
            0 => Some(LED_CODE_A::VALUE1),
            1 => Some(LED_CODE_A::VALUE2),
            13 => Some(LED_CODE_A::VALUE3),
            14 => Some(LED_CODE_A::VALUE4),
            15 => Some(LED_CODE_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "OFF - Init State"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LED_CODE_A::VALUE1
    }
    #[doc = "Flash - SafeOp)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LED_CODE_A::VALUE2
    }
    #[doc = "Blinking - PreOp"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LED_CODE_A::VALUE3
    }
    #[doc = "Flickering - Bootstrap"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == LED_CODE_A::VALUE4
    }
    #[doc = "On - Operational"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == LED_CODE_A::VALUE5
    }
}
#[doc = "Field `LED_CODE` writer - LED Code"]
pub type LED_CODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, LED_CODE_A>;
impl<'a, REG> LED_CODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OFF - Init State"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LED_CODE_A::VALUE1)
    }
    #[doc = "Flash - SafeOp)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LED_CODE_A::VALUE2)
    }
    #[doc = "Blinking - PreOp"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(LED_CODE_A::VALUE3)
    }
    #[doc = "Flickering - Bootstrap"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(LED_CODE_A::VALUE4)
    }
    #[doc = "On - Operational"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(LED_CODE_A::VALUE5)
    }
}
#[doc = "Enable Override\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_OVERR_A {
    #[doc = "0: Override disable"]
    VALUE1 = 0,
    #[doc = "1: Override enable"]
    VALUE2 = 1,
}
impl From<EN_OVERR_A> for bool {
    #[inline(always)]
    fn from(variant: EN_OVERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN_OVERR` reader - Enable Override"]
pub type EN_OVERR_R = crate::BitReader<EN_OVERR_A>;
impl EN_OVERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN_OVERR_A {
        match self.bits {
            false => EN_OVERR_A::VALUE1,
            true => EN_OVERR_A::VALUE2,
        }
    }
    #[doc = "Override disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EN_OVERR_A::VALUE1
    }
    #[doc = "Override enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EN_OVERR_A::VALUE2
    }
}
#[doc = "Field `EN_OVERR` writer - Enable Override"]
pub type EN_OVERR_W<'a, REG> = crate::BitWriter<'a, REG, EN_OVERR_A>;
impl<'a, REG> EN_OVERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EN_OVERR_A::VALUE1)
    }
    #[doc = "Override enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EN_OVERR_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:3 - LED Code"]
    #[inline(always)]
    pub fn led_code(&self) -> LED_CODE_R {
        LED_CODE_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - Enable Override"]
    #[inline(always)]
    pub fn en_overr(&self) -> EN_OVERR_R {
        EN_OVERR_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - LED Code"]
    #[inline(always)]
    pub fn led_code(&mut self) -> LED_CODE_W<RUN_LED_SPEC> {
        LED_CODE_W::new(self, 0)
    }
    #[doc = "Bit 4 - Enable Override"]
    #[inline(always)]
    pub fn en_overr(&mut self) -> EN_OVERR_W<RUN_LED_SPEC> {
        EN_OVERR_W::new(self, 4)
    }
}
#[doc = "RUN LED Override\n\nYou can [`read`](crate::Reg::read) this register and get [`run_led::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`run_led::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RUN_LED_SPEC;
impl crate::RegisterSpec for RUN_LED_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`run_led::R`](R) reader structure"]
impl crate::Readable for RUN_LED_SPEC {}
#[doc = "`write(|w| ..)` method takes [`run_led::W`](W) writer structure"]
impl crate::Writable for RUN_LED_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RUN_LED to value 0x0e"]
impl crate::Resettable for RUN_LED_SPEC {
    const RESET_VALUE: u8 = 0x0e;
}
