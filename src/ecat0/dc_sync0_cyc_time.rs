#[doc = "Register `DC_SYNC0_CYC_TIME` reader"]
pub type R = crate::R<DcSync0CycTimeSpec>;
#[doc = "Register `DC_SYNC0_CYC_TIME` writer"]
pub type W = crate::W<DcSync0CycTimeSpec>;
#[doc = "Time between two consecutive SYNC0 pulses\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum TimeBetweenSync0 {
    #[doc = "0: Single shot mode, generate only one SYNC0 pulse"]
    Value1 = 0,
}
impl From<TimeBetweenSync0> for u32 {
    #[inline(always)]
    fn from(variant: TimeBetweenSync0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TimeBetweenSync0 {
    type Ux = u32;
}
#[doc = "Field `TIME_BETWEEN_SYNC0` reader - Time between two consecutive SYNC0 pulses"]
pub type TimeBetweenSync0R = crate::FieldReader<TimeBetweenSync0>;
impl TimeBetweenSync0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TimeBetweenSync0> {
        match self.bits {
            0 => Some(TimeBetweenSync0::Value1),
            _ => None,
        }
    }
    #[doc = "Single shot mode, generate only one SYNC0 pulse"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TimeBetweenSync0::Value1
    }
}
#[doc = "Field `TIME_BETWEEN_SYNC0` writer - Time between two consecutive SYNC0 pulses"]
pub type TimeBetweenSync0W<'a, REG> = crate::FieldWriter<'a, REG, 32, TimeBetweenSync0>;
impl<'a, REG> TimeBetweenSync0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Single shot mode, generate only one SYNC0 pulse"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TimeBetweenSync0::Value1)
    }
}
impl R {
    #[doc = "Bits 0:31 - Time between two consecutive SYNC0 pulses"]
    #[inline(always)]
    pub fn time_between_sync0(&self) -> TimeBetweenSync0R {
        TimeBetweenSync0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Time between two consecutive SYNC0 pulses"]
    #[inline(always)]
    #[must_use]
    pub fn time_between_sync0(&mut self) -> TimeBetweenSync0W<DcSync0CycTimeSpec> {
        TimeBetweenSync0W::new(self, 0)
    }
}
#[doc = "SYNC0 Cycle Time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_sync0_cyc_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc_sync0_cyc_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcSync0CycTimeSpec;
impl crate::RegisterSpec for DcSync0CycTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_sync0_cyc_time::R`](R) reader structure"]
impl crate::Readable for DcSync0CycTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_sync0_cyc_time::W`](W) writer structure"]
impl crate::Writable for DcSync0CycTimeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_SYNC0_CYC_TIME to value 0"]
impl crate::Resettable for DcSync0CycTimeSpec {
    const RESET_VALUE: u32 = 0;
}
