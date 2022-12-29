#[doc = "Register `HPTXSTS` reader"]
pub struct R(crate::R<HPTXSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPTXSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPTXSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPTXSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPTXSTS` writer"]
pub struct W(crate::W<HPTXSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPTXSTS_SPEC>;
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
impl From<crate::W<HPTXSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPTXSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTxFSpcAvail` reader - Periodic Transmit Data FIFO Space Available"]
pub type PTX_FSPC_AVAIL_R = crate::FieldReader<u16, PTX_FSPC_AVAIL_A>;
#[doc = "Periodic Transmit Data FIFO Space Available\n\nValue on reset: 256"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PTX_FSPC_AVAIL_A {
    #[doc = "0: Periodic TxFIFO is full"]
    VALUE1 = 0,
    #[doc = "1: 1 word available"]
    VALUE2 = 1,
    #[doc = "2: 2 words available"]
    VALUE3 = 2,
}
impl From<PTX_FSPC_AVAIL_A> for u16 {
    #[inline(always)]
    fn from(variant: PTX_FSPC_AVAIL_A) -> Self {
        variant as _
    }
}
impl PTX_FSPC_AVAIL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PTX_FSPC_AVAIL_A> {
        match self.bits {
            0 => Some(PTX_FSPC_AVAIL_A::VALUE1),
            1 => Some(PTX_FSPC_AVAIL_A::VALUE2),
            2 => Some(PTX_FSPC_AVAIL_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PTX_FSPC_AVAIL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PTX_FSPC_AVAIL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PTX_FSPC_AVAIL_A::VALUE3
    }
}
#[doc = "Field `PTxFSpcAvail` writer - Periodic Transmit Data FIFO Space Available"]
pub type PTX_FSPC_AVAIL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HPTXSTS_SPEC, u16, PTX_FSPC_AVAIL_A, 16, O>;
impl<'a, const O: u8> PTX_FSPC_AVAIL_W<'a, O> {
    #[doc = "Periodic TxFIFO is full"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PTX_FSPC_AVAIL_A::VALUE1)
    }
    #[doc = "1 word available"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PTX_FSPC_AVAIL_A::VALUE2)
    }
    #[doc = "2 words available"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PTX_FSPC_AVAIL_A::VALUE3)
    }
}
#[doc = "Field `PTxQSpcAvail` reader - Periodic Transmit Request Queue Space Available"]
pub type PTX_QSPC_AVAIL_R = crate::FieldReader<u8, PTX_QSPC_AVAIL_A>;
#[doc = "Periodic Transmit Request Queue Space Available\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PTX_QSPC_AVAIL_A {
    #[doc = "0: Periodic Transmit Request Queue is full"]
    VALUE1 = 0,
    #[doc = "1: 1 location available"]
    VALUE2 = 1,
    #[doc = "2: 2 locations available"]
    VALUE3 = 2,
}
impl From<PTX_QSPC_AVAIL_A> for u8 {
    #[inline(always)]
    fn from(variant: PTX_QSPC_AVAIL_A) -> Self {
        variant as _
    }
}
impl PTX_QSPC_AVAIL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PTX_QSPC_AVAIL_A> {
        match self.bits {
            0 => Some(PTX_QSPC_AVAIL_A::VALUE1),
            1 => Some(PTX_QSPC_AVAIL_A::VALUE2),
            2 => Some(PTX_QSPC_AVAIL_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PTX_QSPC_AVAIL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PTX_QSPC_AVAIL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PTX_QSPC_AVAIL_A::VALUE3
    }
}
#[doc = "Field `PTxQTop` reader - Top of the Periodic Transmit Request Queue"]
pub type PTX_QTOP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Periodic Transmit Data FIFO Space Available"]
    #[inline(always)]
    pub fn ptx_fspc_avail(&self) -> PTX_FSPC_AVAIL_R {
        PTX_FSPC_AVAIL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Periodic Transmit Request Queue Space Available"]
    #[inline(always)]
    pub fn ptx_qspc_avail(&self) -> PTX_QSPC_AVAIL_R {
        PTX_QSPC_AVAIL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Top of the Periodic Transmit Request Queue"]
    #[inline(always)]
    pub fn ptx_qtop(&self) -> PTX_QTOP_R {
        PTX_QTOP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Periodic Transmit Data FIFO Space Available"]
    #[inline(always)]
    #[must_use]
    pub fn ptx_fspc_avail(&mut self) -> PTX_FSPC_AVAIL_W<0> {
        PTX_FSPC_AVAIL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Periodic Transmit FIFO/ Queue Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hptxsts](index.html) module"]
pub struct HPTXSTS_SPEC;
impl crate::RegisterSpec for HPTXSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hptxsts::R](R) reader structure"]
impl crate::Readable for HPTXSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hptxsts::W](W) writer structure"]
impl crate::Writable for HPTXSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPTXSTS to value 0x0008_0100"]
impl crate::Resettable for HPTXSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0100;
}
