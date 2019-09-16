#[doc = "Reader of register HCTSIZ_SCATGATHER"]
pub type R = crate::R<u32, super::HCTSIZ_SCATGATHER>;
#[doc = "Writer for register HCTSIZ_SCATGATHER"]
pub type W = crate::W<u32, super::HCTSIZ_SCATGATHER>;
#[doc = "Register HCTSIZ_SCATGATHER `reset()`'s with value 0"]
impl crate::ResetValue for super::HCTSIZ_SCATGATHER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCHED_INFO`"]
pub type SCHED_INFO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCHED_INFO`"]
pub struct SCHED_INFO_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHED_INFO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `NTD`"]
pub type NTD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NTD`"]
pub struct NTD_W<'a> {
    w: &'a mut W,
}
impl<'a> NTD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "PID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PID_A {
    #[doc = "0: DATA0"]
    VALUE1,
    #[doc = "1: DATA2"]
    VALUE2,
    #[doc = "2: DATA1"]
    VALUE3,
    #[doc = "3: MDATA (non-control)"]
    VALUE4,
}
impl From<PID_A> for u8 {
    #[inline(always)]
    fn from(variant: PID_A) -> Self {
        match variant {
            PID_A::VALUE1 => 0,
            PID_A::VALUE2 => 1,
            PID_A::VALUE3 => 2,
            PID_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `Pid`"]
pub type PID_R = crate::R<u8, PID_A>;
impl PID_R {
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
        *self == PID_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PID_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PID_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PID_A::VALUE4
    }
}
#[doc = "Write proxy for field `Pid`"]
pub struct PID_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
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
}
