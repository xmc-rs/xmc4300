#[doc = "Register `HCTSIZ_SCATGATHER` reader"]
pub struct R(crate::R<HCTSIZ_SCATGATHER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCTSIZ_SCATGATHER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCTSIZ_SCATGATHER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCTSIZ_SCATGATHER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCTSIZ_SCATGATHER` writer"]
pub struct W(crate::W<HCTSIZ_SCATGATHER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCTSIZ_SCATGATHER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<HCTSIZ_SCATGATHER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCTSIZ_SCATGATHER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCHED_INFO` reader - Schedule information"]
pub struct SCHED_INFO_R(crate::FieldReader<u8, u8>);
impl SCHED_INFO_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCHED_INFO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHED_INFO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHED_INFO` writer - Schedule information"]
pub struct SCHED_INFO_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHED_INFO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `NTD` reader - Number of Transfer Descriptors: 0=1 descriptor, 63=64 descriptors, 1=2 descriptors, 3=4 descriptors, 7=8 descriptors, 15=16 descriptors, 31=32 descriptors, 63=64 descriptors,"]
pub struct NTD_R(crate::FieldReader<u8, u8>);
impl NTD_R {
    pub(crate) fn new(bits: u8) -> Self {
        NTD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NTD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NTD` writer - Number of Transfer Descriptors: 0=1 descriptor, 63=64 descriptors, 1=2 descriptors, 3=4 descriptors, 7=8 descriptors, 15=16 descriptors, 31=32 descriptors, 63=64 descriptors,"]
pub struct NTD_W<'a> {
    w: &'a mut W,
}
impl<'a> NTD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "PID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `Pid` reader - PID"]
pub struct PID_R(crate::FieldReader<u8, PID_A>);
impl PID_R {
    pub(crate) fn new(bits: u8) -> Self {
        PID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID_A {
        match self.bits {
            0 => PID_A::VALUE1,
            1 => PID_A::VALUE2,
            2 => PID_A::VALUE3,
            3 => PID_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PID_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PID_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PID_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == PID_A::VALUE4
    }
}
impl core::ops::Deref for PID_R {
    type Target = crate::FieldReader<u8, PID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Pid` writer - PID"]
pub struct PID_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DATA0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PID_A::VALUE1)
    }
    #[doc = "DATA2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PID_A::VALUE2)
    }
    #[doc = "DATA1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PID_A::VALUE3)
    }
    #[doc = "MDATA (non-control)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PID_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | ((value as u32 & 0x03) << 29);
        self.w
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
        PID_R::new(((self.bits >> 29) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Schedule information"]
    #[inline(always)]
    pub fn sched_info(&mut self) -> SCHED_INFO_W {
        SCHED_INFO_W { w: self }
    }
    #[doc = "Bits 8:15 - Number of Transfer Descriptors: 0=1 descriptor, 63=64 descriptors, 1=2 descriptors, 3=4 descriptors, 7=8 descriptors, 15=16 descriptors, 31=32 descriptors, 63=64 descriptors,"]
    #[inline(always)]
    pub fn ntd(&mut self) -> NTD_W {
        NTD_W { w: self }
    }
    #[doc = "Bits 29:30 - PID"]
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W {
        PID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Channel Transfer Size Register \\[SCATGATHER\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctsiz_scatgather](index.html) module"]
pub struct HCTSIZ_SCATGATHER_SPEC;
impl crate::RegisterSpec for HCTSIZ_SCATGATHER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hctsiz_scatgather::R](R) reader structure"]
impl crate::Readable for HCTSIZ_SCATGATHER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hctsiz_scatgather::W](W) writer structure"]
impl crate::Writable for HCTSIZ_SCATGATHER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCTSIZ_SCATGATHER to value 0"]
impl crate::Resettable for HCTSIZ_SCATGATHER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
