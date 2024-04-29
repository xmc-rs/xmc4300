#[doc = "Register `DC_SYNC0_CYC_TIME` reader"]
pub type R = crate::R<DC_SYNC0_CYC_TIME_SPEC>;
#[doc = "Register `DC_SYNC0_CYC_TIME` writer"]
pub type W = crate::W<DC_SYNC0_CYC_TIME_SPEC>;
#[doc = "Time between two consecutive SYNC0 pulses\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum TIME_BETWEEN_SYNC0_A {
    #[doc = "0: Single shot mode, generate only one SYNC0 pulse"]
    VALUE1 = 0,
}
impl From<TIME_BETWEEN_SYNC0_A> for u32 {
    #[inline(always)]
    fn from(variant: TIME_BETWEEN_SYNC0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIME_BETWEEN_SYNC0_A {
    type Ux = u32;
}
impl crate::IsEnum for TIME_BETWEEN_SYNC0_A {}
#[doc = "Field `TIME_BETWEEN_SYNC0` reader - Time between two consecutive SYNC0 pulses"]
pub type TIME_BETWEEN_SYNC0_R = crate::FieldReader<TIME_BETWEEN_SYNC0_A>;
impl TIME_BETWEEN_SYNC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TIME_BETWEEN_SYNC0_A> {
        match self.bits {
            0 => Some(TIME_BETWEEN_SYNC0_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Single shot mode, generate only one SYNC0 pulse"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TIME_BETWEEN_SYNC0_A::VALUE1
    }
}
#[doc = "Field `TIME_BETWEEN_SYNC0` writer - Time between two consecutive SYNC0 pulses"]
pub type TIME_BETWEEN_SYNC0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, TIME_BETWEEN_SYNC0_A>;
impl<'a, REG> TIME_BETWEEN_SYNC0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Single shot mode, generate only one SYNC0 pulse"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TIME_BETWEEN_SYNC0_A::VALUE1)
    }
}
impl R {
    #[doc = "Bits 0:31 - Time between two consecutive SYNC0 pulses"]
    #[inline(always)]
    pub fn time_between_sync0(&self) -> TIME_BETWEEN_SYNC0_R {
        TIME_BETWEEN_SYNC0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Time between two consecutive SYNC0 pulses"]
    #[inline(always)]
    #[must_use]
    pub fn time_between_sync0(&mut self) -> TIME_BETWEEN_SYNC0_W<DC_SYNC0_CYC_TIME_SPEC> {
        TIME_BETWEEN_SYNC0_W::new(self, 0)
    }
}
#[doc = "SYNC0 Cycle Time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_sync0_cyc_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc_sync0_cyc_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC_SYNC0_CYC_TIME_SPEC;
impl crate::RegisterSpec for DC_SYNC0_CYC_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_sync0_cyc_time::R`](R) reader structure"]
impl crate::Readable for DC_SYNC0_CYC_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dc_sync0_cyc_time::W`](W) writer structure"]
impl crate::Writable for DC_SYNC0_CYC_TIME_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_SYNC0_CYC_TIME to value 0"]
impl crate::Resettable for DC_SYNC0_CYC_TIME_SPEC {
    const RESET_VALUE: u32 = 0;
}
