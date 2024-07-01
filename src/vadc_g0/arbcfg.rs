#[doc = "Register `ARBCFG` reader"]
pub type R = crate::R<ARBCFG_SPEC>;
#[doc = "Register `ARBCFG` writer"]
pub type W = crate::W<ARBCFG_SPEC>;
#[doc = "Field `ANONC` reader - Analog Converter Control"]
pub type ANONC_R = crate::FieldReader;
#[doc = "Field `ANONC` writer - Analog Converter Control"]
pub type ANONC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Arbitration Round Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ARBRND_A {
    #[doc = "0: 4 arbitration slots per round (tARB = 4 / fADCD)"]
    VALUE1 = 0,
    #[doc = "1: 8 arbitration slots per round (tARB = 8 / fADCD)"]
    VALUE2 = 1,
    #[doc = "2: 16 arbitration slots per round (tARB = 16 / fADCD)"]
    VALUE3 = 2,
    #[doc = "3: 20 arbitration slots per round (tARB = 20 / fADCD)"]
    VALUE4 = 3,
}
impl From<ARBRND_A> for u8 {
    #[inline(always)]
    fn from(variant: ARBRND_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ARBRND_A {
    type Ux = u8;
}
impl crate::IsEnum for ARBRND_A {}
#[doc = "Field `ARBRND` reader - Arbitration Round Length"]
pub type ARBRND_R = crate::FieldReader<ARBRND_A>;
impl ARBRND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ARBRND_A {
        match self.bits {
            0 => ARBRND_A::VALUE1,
            1 => ARBRND_A::VALUE2,
            2 => ARBRND_A::VALUE3,
            3 => ARBRND_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "4 arbitration slots per round (tARB = 4 / fADCD)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ARBRND_A::VALUE1
    }
    #[doc = "8 arbitration slots per round (tARB = 8 / fADCD)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ARBRND_A::VALUE2
    }
    #[doc = "16 arbitration slots per round (tARB = 16 / fADCD)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ARBRND_A::VALUE3
    }
    #[doc = "20 arbitration slots per round (tARB = 20 / fADCD)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ARBRND_A::VALUE4
    }
}
#[doc = "Field `ARBRND` writer - Arbitration Round Length"]
pub type ARBRND_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ARBRND_A, crate::Safe>;
impl<'a, REG> ARBRND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 arbitration slots per round (tARB = 4 / fADCD)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ARBRND_A::VALUE1)
    }
    #[doc = "8 arbitration slots per round (tARB = 8 / fADCD)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ARBRND_A::VALUE2)
    }
    #[doc = "16 arbitration slots per round (tARB = 16 / fADCD)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(ARBRND_A::VALUE3)
    }
    #[doc = "20 arbitration slots per round (tARB = 20 / fADCD)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(ARBRND_A::VALUE4)
    }
}
#[doc = "Arbitration Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARBM_A {
    #[doc = "0: The arbiter runs permanently. This setting is required for a synchronization slave (see ) and for equidistant sampling using the signal ARBCNT (see )."]
    VALUE1 = 0,
    #[doc = "1: The arbiter only runs if at least one conversion request of an enabled request source is pending. This setting ensures a reproducible latency from an incoming request to the conversion start, if the converter is idle. Synchronized conversions are not supported."]
    VALUE2 = 1,
}
impl From<ARBM_A> for bool {
    #[inline(always)]
    fn from(variant: ARBM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARBM` reader - Arbitration Mode"]
pub type ARBM_R = crate::BitReader<ARBM_A>;
impl ARBM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ARBM_A {
        match self.bits {
            false => ARBM_A::VALUE1,
            true => ARBM_A::VALUE2,
        }
    }
    #[doc = "The arbiter runs permanently. This setting is required for a synchronization slave (see ) and for equidistant sampling using the signal ARBCNT (see )."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ARBM_A::VALUE1
    }
    #[doc = "The arbiter only runs if at least one conversion request of an enabled request source is pending. This setting ensures a reproducible latency from an incoming request to the conversion start, if the converter is idle. Synchronized conversions are not supported."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ARBM_A::VALUE2
    }
}
#[doc = "Field `ARBM` writer - Arbitration Mode"]
pub type ARBM_W<'a, REG> = crate::BitWriter<'a, REG, ARBM_A>;
impl<'a, REG> ARBM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The arbiter runs permanently. This setting is required for a synchronization slave (see ) and for equidistant sampling using the signal ARBCNT (see )."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ARBM_A::VALUE1)
    }
    #[doc = "The arbiter only runs if at least one conversion request of an enabled request source is pending. This setting ensures a reproducible latency from an incoming request to the conversion start, if the converter is idle. Synchronized conversions are not supported."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ARBM_A::VALUE2)
    }
}
#[doc = "Analog Converter Control Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ANONS_A {
    #[doc = "0: Analog converter off"]
    VALUE1 = 0,
    #[doc = "3: Normal operation (permanently on)"]
    VALUE4 = 3,
}
impl From<ANONS_A> for u8 {
    #[inline(always)]
    fn from(variant: ANONS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ANONS_A {
    type Ux = u8;
}
impl crate::IsEnum for ANONS_A {}
#[doc = "Field `ANONS` reader - Analog Converter Control Status"]
pub type ANONS_R = crate::FieldReader<ANONS_A>;
impl ANONS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ANONS_A> {
        match self.bits {
            0 => Some(ANONS_A::VALUE1),
            3 => Some(ANONS_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Analog converter off"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ANONS_A::VALUE1
    }
    #[doc = "Normal operation (permanently on)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ANONS_A::VALUE4
    }
}
#[doc = "Start-Up Calibration Active Indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAL_A {
    #[doc = "0: Completed or not yet started"]
    VALUE1 = 0,
    #[doc = "1: Start-up calibration phase is active"]
    VALUE2 = 1,
}
impl From<CAL_A> for bool {
    #[inline(always)]
    fn from(variant: CAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAL` reader - Start-Up Calibration Active Indication"]
pub type CAL_R = crate::BitReader<CAL_A>;
impl CAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAL_A {
        match self.bits {
            false => CAL_A::VALUE1,
            true => CAL_A::VALUE2,
        }
    }
    #[doc = "Completed or not yet started"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CAL_A::VALUE1
    }
    #[doc = "Start-up calibration phase is active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CAL_A::VALUE2
    }
}
#[doc = "Converter Busy Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: Not busy"]
    VALUE1 = 0,
    #[doc = "1: Converter is busy with a conversion"]
    VALUE2 = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Converter Busy Flag"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::VALUE1,
            true => BUSY_A::VALUE2,
        }
    }
    #[doc = "Not busy"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BUSY_A::VALUE1
    }
    #[doc = "Converter is busy with a conversion"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BUSY_A::VALUE2
    }
}
#[doc = "Sample Phase Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAMPLE_A {
    #[doc = "0: Converting or idle"]
    VALUE1 = 0,
    #[doc = "1: Input signal is currently sampled"]
    VALUE2 = 1,
}
impl From<SAMPLE_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPLE` reader - Sample Phase Flag"]
pub type SAMPLE_R = crate::BitReader<SAMPLE_A>;
impl SAMPLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SAMPLE_A {
        match self.bits {
            false => SAMPLE_A::VALUE1,
            true => SAMPLE_A::VALUE2,
        }
    }
    #[doc = "Converting or idle"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SAMPLE_A::VALUE1
    }
    #[doc = "Input signal is currently sampled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SAMPLE_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:1 - Analog Converter Control"]
    #[inline(always)]
    pub fn anonc(&self) -> ANONC_R {
        ANONC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Arbitration Round Length"]
    #[inline(always)]
    pub fn arbrnd(&self) -> ARBRND_R {
        ARBRND_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Arbitration Mode"]
    #[inline(always)]
    pub fn arbm(&self) -> ARBM_R {
        ARBM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Analog Converter Control Status"]
    #[inline(always)]
    pub fn anons(&self) -> ANONS_R {
        ANONS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 28 - Start-Up Calibration Active Indication"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Converter Busy Flag"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Sample Phase Flag"]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Analog Converter Control"]
    #[inline(always)]
    #[must_use]
    pub fn anonc(&mut self) -> ANONC_W<ARBCFG_SPEC> {
        ANONC_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Arbitration Round Length"]
    #[inline(always)]
    #[must_use]
    pub fn arbrnd(&mut self) -> ARBRND_W<ARBCFG_SPEC> {
        ARBRND_W::new(self, 4)
    }
    #[doc = "Bit 7 - Arbitration Mode"]
    #[inline(always)]
    #[must_use]
    pub fn arbm(&mut self) -> ARBM_W<ARBCFG_SPEC> {
        ARBM_W::new(self, 7)
    }
}
#[doc = "Arbitration Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arbcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arbcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARBCFG_SPEC;
impl crate::RegisterSpec for ARBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arbcfg::R`](R) reader structure"]
impl crate::Readable for ARBCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arbcfg::W`](W) writer structure"]
impl crate::Writable for ARBCFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARBCFG to value 0"]
impl crate::Resettable for ARBCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
