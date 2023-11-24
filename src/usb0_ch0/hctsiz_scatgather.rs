#[doc = "Register `HCTSIZ_SCATGATHER` reader"]
pub type R = crate::R<HCTSIZ_SCATGATHER_SPEC>;
#[doc = "Register `HCTSIZ_SCATGATHER` writer"]
pub type W = crate::W<HCTSIZ_SCATGATHER_SPEC>;
#[doc = "Field `SCHED_INFO` reader - Schedule information"]
pub type SCHED_INFO_R = crate::FieldReader;
#[doc = "Field `SCHED_INFO` writer - Schedule information"]
pub type SCHED_INFO_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NTD` reader - Number of Transfer Descriptors: 0=1 descriptor, 63=64 descriptors, 1=2 descriptors, 3=4 descriptors, 7=8 descriptors, 15=16 descriptors, 31=32 descriptors, 63=64 descriptors,"]
pub type NTD_R = crate::FieldReader;
#[doc = "Field `NTD` writer - Number of Transfer Descriptors: 0=1 descriptor, 63=64 descriptors, 1=2 descriptors, 3=4 descriptors, 7=8 descriptors, 15=16 descriptors, 31=32 descriptors, 63=64 descriptors,"]
pub type NTD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Pid` reader - PID"]
pub type PID_R = crate::FieldReader<PID_A>;
#[doc = "PID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PID_A {
    #[doc = "0: DATA0"]
    VALUE1 = 0,
    #[doc = "1: DATA2"]
    VALUE2 = 1,
    #[doc = "2: DATA1"]
    VALUE3 = 2,
    #[doc = "3: MDATA (non-control)"]
    VALUE4 = 3,
}
impl From<PID_A> for u8 {
    #[inline(always)]
    fn from(variant: PID_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PID_A {
    type Ux = u8;
}
impl PID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PID_A {
        match self.bits {
            0 => PID_A::VALUE1,
            1 => PID_A::VALUE2,
            2 => PID_A::VALUE3,
            3 => PID_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "DATA0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PID_A::VALUE1
    }
    #[doc = "DATA2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PID_A::VALUE2
    }
    #[doc = "DATA1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PID_A::VALUE3
    }
    #[doc = "MDATA (non-control)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PID_A::VALUE4
    }
}
#[doc = "Field `Pid` writer - PID"]
pub type PID_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PID_A>;
impl<'a, REG> PID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DATA0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PID_A::VALUE1)
    }
    #[doc = "DATA2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PID_A::VALUE2)
    }
    #[doc = "DATA1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PID_A::VALUE3)
    }
    #[doc = "MDATA (non-control)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(PID_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:7 - Schedule information"]
    #[inline(always)]
    pub fn sched_info(&self) -> SCHED_INFO_R {
        SCHED_INFO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of Transfer Descriptors: 0=1 descriptor, 63=64 descriptors, 1=2 descriptors, 3=4 descriptors, 7=8 descriptors, 15=16 descriptors, 31=32 descriptors, 63=64 descriptors,"]
    #[inline(always)]
    pub fn ntd(&self) -> NTD_R {
        NTD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 29:30 - PID"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Schedule information"]
    #[inline(always)]
    #[must_use]
    pub fn sched_info(&mut self) -> SCHED_INFO_W<HCTSIZ_SCATGATHER_SPEC> {
        SCHED_INFO_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Number of Transfer Descriptors: 0=1 descriptor, 63=64 descriptors, 1=2 descriptors, 3=4 descriptors, 7=8 descriptors, 15=16 descriptors, 31=32 descriptors, 63=64 descriptors,"]
    #[inline(always)]
    #[must_use]
    pub fn ntd(&mut self) -> NTD_W<HCTSIZ_SCATGATHER_SPEC> {
        NTD_W::new(self, 8)
    }
    #[doc = "Bits 29:30 - PID"]
    #[inline(always)]
    #[must_use]
    pub fn pid(&mut self) -> PID_W<HCTSIZ_SCATGATHER_SPEC> {
        PID_W::new(self, 29)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Host Channel Transfer Size Register \\[SCATGATHER\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz_scatgather::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz_scatgather::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCTSIZ_SCATGATHER_SPEC;
impl crate::RegisterSpec for HCTSIZ_SCATGATHER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hctsiz_scatgather::R`](R) reader structure"]
impl crate::Readable for HCTSIZ_SCATGATHER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hctsiz_scatgather::W`](W) writer structure"]
impl crate::Writable for HCTSIZ_SCATGATHER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCTSIZ_SCATGATHER to value 0"]
impl crate::Resettable for HCTSIZ_SCATGATHER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
