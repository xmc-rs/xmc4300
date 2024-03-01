#[doc = "Register `HCTSIZ_SCATGATHER` reader"]
pub type R = crate::R<HctsizScatgatherSpec>;
#[doc = "Register `HCTSIZ_SCATGATHER` writer"]
pub type W = crate::W<HctsizScatgatherSpec>;
#[doc = "Field `SCHED_INFO` reader - Schedule information"]
pub type SchedInfoR = crate::FieldReader;
#[doc = "Field `SCHED_INFO` writer - Schedule information"]
pub type SchedInfoW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NTD` reader - Number of Transfer Descriptors: 0=1 descriptor, 63=64 descriptors, 1=2 descriptors, 3=4 descriptors, 7=8 descriptors, 15=16 descriptors, 31=32 descriptors, 63=64 descriptors,"]
pub type NtdR = crate::FieldReader;
#[doc = "Field `NTD` writer - Number of Transfer Descriptors: 0=1 descriptor, 63=64 descriptors, 1=2 descriptors, 3=4 descriptors, 7=8 descriptors, 15=16 descriptors, 31=32 descriptors, 63=64 descriptors,"]
pub type NtdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "PID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pid {
    #[doc = "0: DATA0"]
    Value1 = 0,
    #[doc = "1: DATA2"]
    Value2 = 1,
    #[doc = "2: DATA1"]
    Value3 = 2,
    #[doc = "3: MDATA (non-control)"]
    Value4 = 3,
}
impl From<Pid> for u8 {
    #[inline(always)]
    fn from(variant: Pid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pid {
    type Ux = u8;
}
#[doc = "Field `Pid` reader - PID"]
pub type PidR = crate::FieldReader<Pid>;
impl PidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pid {
        match self.bits {
            0 => Pid::Value1,
            1 => Pid::Value2,
            2 => Pid::Value3,
            3 => Pid::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "DATA0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pid::Value1
    }
    #[doc = "DATA2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pid::Value2
    }
    #[doc = "DATA1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pid::Value3
    }
    #[doc = "MDATA (non-control)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pid::Value4
    }
}
#[doc = "Field `Pid` writer - PID"]
pub type PidW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Pid>;
impl<'a, REG> PidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DATA0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::Value1)
    }
    #[doc = "DATA2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::Value2)
    }
    #[doc = "DATA1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::Value3)
    }
    #[doc = "MDATA (non-control)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::Value4)
    }
}
impl R {
    #[doc = "Bits 0:7 - Schedule information"]
    #[inline(always)]
    pub fn sched_info(&self) -> SchedInfoR {
        SchedInfoR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of Transfer Descriptors: 0=1 descriptor, 63=64 descriptors, 1=2 descriptors, 3=4 descriptors, 7=8 descriptors, 15=16 descriptors, 31=32 descriptors, 63=64 descriptors,"]
    #[inline(always)]
    pub fn ntd(&self) -> NtdR {
        NtdR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 29:30 - PID"]
    #[inline(always)]
    pub fn pid(&self) -> PidR {
        PidR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Schedule information"]
    #[inline(always)]
    #[must_use]
    pub fn sched_info(&mut self) -> SchedInfoW<HctsizScatgatherSpec> {
        SchedInfoW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Number of Transfer Descriptors: 0=1 descriptor, 63=64 descriptors, 1=2 descriptors, 3=4 descriptors, 7=8 descriptors, 15=16 descriptors, 31=32 descriptors, 63=64 descriptors,"]
    #[inline(always)]
    #[must_use]
    pub fn ntd(&mut self) -> NtdW<HctsizScatgatherSpec> {
        NtdW::new(self, 8)
    }
    #[doc = "Bits 29:30 - PID"]
    #[inline(always)]
    #[must_use]
    pub fn pid(&mut self) -> PidW<HctsizScatgatherSpec> {
        PidW::new(self, 29)
    }
}
#[doc = "Host Channel Transfer Size Register \\[SCATGATHER\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz_scatgather::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz_scatgather::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HctsizScatgatherSpec;
impl crate::RegisterSpec for HctsizScatgatherSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hctsiz_scatgather::R`](R) reader structure"]
impl crate::Readable for HctsizScatgatherSpec {}
#[doc = "`write(|w| ..)` method takes [`hctsiz_scatgather::W`](W) writer structure"]
impl crate::Writable for HctsizScatgatherSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCTSIZ_SCATGATHER to value 0"]
impl crate::Resettable for HctsizScatgatherSpec {
    const RESET_VALUE: u32 = 0;
}
