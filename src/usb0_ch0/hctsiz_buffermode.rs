#[doc = "Register `HCTSIZ_BUFFERMODE` reader"]
pub struct R(crate::R<HCTSIZ_BUFFERMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCTSIZ_BUFFERMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCTSIZ_BUFFERMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCTSIZ_BUFFERMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCTSIZ_BUFFERMODE` writer"]
pub struct W(crate::W<HCTSIZ_BUFFERMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCTSIZ_BUFFERMODE_SPEC>;
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
impl From<crate::W<HCTSIZ_BUFFERMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCTSIZ_BUFFERMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XferSize` reader - Transfer Size"]
pub type XFER_SIZE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `XferSize` writer - Transfer Size"]
pub type XFER_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCTSIZ_BUFFERMODE_SPEC, u32, u32, 19, O>;
#[doc = "Field `PktCnt` reader - Packet Count"]
pub type PKT_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PktCnt` writer - Packet Count"]
pub type PKT_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCTSIZ_BUFFERMODE_SPEC, u16, u16, 10, O>;
#[doc = "Field `Pid` reader - PID"]
pub type PID_R = crate::FieldReader<u8, PID_A>;
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
    #[doc = "3: MDATA (non-control)/SETUP (control)"]
    VALUE4 = 3,
}
impl From<PID_A> for u8 {
    #[inline(always)]
    fn from(variant: PID_A) -> Self {
        variant as _
    }
}
impl PID_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `Pid` writer - PID"]
pub type PID_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, HCTSIZ_BUFFERMODE_SPEC, u8, PID_A, 2, O>;
impl<'a, const O: u8> PID_W<'a, O> {
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
    #[doc = "MDATA (non-control)/SETUP (control)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PID_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfer_size(&self) -> XFER_SIZE_R {
        XFER_SIZE_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pkt_cnt(&self) -> PKT_CNT_R {
        PKT_CNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - PID"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_size(&mut self) -> XFER_SIZE_W<0> {
        XFER_SIZE_W::new(self)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_cnt(&mut self) -> PKT_CNT_W<19> {
        PKT_CNT_W::new(self)
    }
    #[doc = "Bits 29:30 - PID"]
    #[inline(always)]
    #[must_use]
    pub fn pid(&mut self) -> PID_W<29> {
        PID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Channel Transfer Size Register \\[BUFFERMODE\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctsiz_buffermode](index.html) module"]
pub struct HCTSIZ_BUFFERMODE_SPEC;
impl crate::RegisterSpec for HCTSIZ_BUFFERMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hctsiz_buffermode::R](R) reader structure"]
impl crate::Readable for HCTSIZ_BUFFERMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hctsiz_buffermode::W](W) writer structure"]
impl crate::Writable for HCTSIZ_BUFFERMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCTSIZ_BUFFERMODE to value 0"]
impl crate::Resettable for HCTSIZ_BUFFERMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
