#[doc = "Register `NFCR` reader"]
pub struct R(crate::R<NFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NFCR` writer"]
pub struct W(crate::W<NFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NFCR_SPEC>;
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
impl From<crate::W<NFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFC` reader - CAN Frame Counter"]
pub type CFC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CFC` writer - CAN Frame Counter"]
pub type CFC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NFCR_SPEC, u16, u16, 16, O>;
#[doc = "Field `CFSEL` reader - CAN Frame Count Selection"]
pub type CFSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFSEL` writer - CAN Frame Count Selection"]
pub type CFSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NFCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `CFMOD` reader - CAN Frame Counter Mode"]
pub type CFMOD_R = crate::FieldReader<u8, CFMOD_A>;
#[doc = "CAN Frame Counter Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFMOD_A {
    #[doc = "0: Frame Count Mode: The frame counter is incremented upon the reception and transmission of frames."]
    VALUE1 = 0,
    #[doc = "1: Time Stamp Mode: The frame counter is used to count bit times."]
    VALUE2 = 1,
    #[doc = "2: Bit Timing Mode: The frame counter is used for analysis of the bit timing."]
    VALUE3 = 2,
    #[doc = "3: Error Count Mode: The frame counter is used for counting when an error frame is received or an error is detected by the node."]
    VALUE4 = 3,
}
impl From<CFMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: CFMOD_A) -> Self {
        variant as _
    }
}
impl CFMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFMOD_A {
        match self.bits {
            0 => CFMOD_A::VALUE1,
            1 => CFMOD_A::VALUE2,
            2 => CFMOD_A::VALUE3,
            3 => CFMOD_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CFMOD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CFMOD_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CFMOD_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CFMOD_A::VALUE4
    }
}
#[doc = "Field `CFMOD` writer - CAN Frame Counter Mode"]
pub type CFMOD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, NFCR_SPEC, u8, CFMOD_A, 2, O>;
impl<'a, const O: u8> CFMOD_W<'a, O> {
    #[doc = "Frame Count Mode: The frame counter is incremented upon the reception and transmission of frames."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFMOD_A::VALUE1)
    }
    #[doc = "Time Stamp Mode: The frame counter is used to count bit times."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFMOD_A::VALUE2)
    }
    #[doc = "Bit Timing Mode: The frame counter is used for analysis of the bit timing."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CFMOD_A::VALUE3)
    }
    #[doc = "Error Count Mode: The frame counter is used for counting when an error frame is received or an error is detected by the node."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CFMOD_A::VALUE4)
    }
}
#[doc = "Field `CFCIE` reader - CAN Frame Count Interrupt Enable"]
pub type CFCIE_R = crate::BitReader<CFCIE_A>;
#[doc = "CAN Frame Count Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFCIE_A {
    #[doc = "0: CAN frame counter overflow interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: CAN frame counter overflow interrupt is enabled."]
    VALUE2 = 1,
}
impl From<CFCIE_A> for bool {
    #[inline(always)]
    fn from(variant: CFCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CFCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFCIE_A {
        match self.bits {
            false => CFCIE_A::VALUE1,
            true => CFCIE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CFCIE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CFCIE_A::VALUE2
    }
}
#[doc = "Field `CFCIE` writer - CAN Frame Count Interrupt Enable"]
pub type CFCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NFCR_SPEC, CFCIE_A, O>;
impl<'a, const O: u8> CFCIE_W<'a, O> {
    #[doc = "CAN frame counter overflow interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFCIE_A::VALUE1)
    }
    #[doc = "CAN frame counter overflow interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFCIE_A::VALUE2)
    }
}
#[doc = "Field `CFCOV` reader - CAN Frame Counter Overflow Flag"]
pub type CFCOV_R = crate::BitReader<CFCOV_A>;
#[doc = "CAN Frame Counter Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFCOV_A {
    #[doc = "0: No overflow has occurred since last flag reset."]
    VALUE1 = 0,
    #[doc = "1: An overflow has occurred since last flag reset."]
    VALUE2 = 1,
}
impl From<CFCOV_A> for bool {
    #[inline(always)]
    fn from(variant: CFCOV_A) -> Self {
        variant as u8 != 0
    }
}
impl CFCOV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFCOV_A {
        match self.bits {
            false => CFCOV_A::VALUE1,
            true => CFCOV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CFCOV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CFCOV_A::VALUE2
    }
}
#[doc = "Field `CFCOV` writer - CAN Frame Counter Overflow Flag"]
pub type CFCOV_W<'a, const O: u8> = crate::BitWriter<'a, u32, NFCR_SPEC, CFCOV_A, O>;
impl<'a, const O: u8> CFCOV_W<'a, O> {
    #[doc = "No overflow has occurred since last flag reset."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFCOV_A::VALUE1)
    }
    #[doc = "An overflow has occurred since last flag reset."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFCOV_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:15 - CAN Frame Counter"]
    #[inline(always)]
    pub fn cfc(&self) -> CFC_R {
        CFC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - CAN Frame Count Selection"]
    #[inline(always)]
    pub fn cfsel(&self) -> CFSEL_R {
        CFSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:20 - CAN Frame Counter Mode"]
    #[inline(always)]
    pub fn cfmod(&self) -> CFMOD_R {
        CFMOD_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 22 - CAN Frame Count Interrupt Enable"]
    #[inline(always)]
    pub fn cfcie(&self) -> CFCIE_R {
        CFCIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CAN Frame Counter Overflow Flag"]
    #[inline(always)]
    pub fn cfcov(&self) -> CFCOV_R {
        CFCOV_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CAN Frame Counter"]
    #[inline(always)]
    #[must_use]
    pub fn cfc(&mut self) -> CFC_W<0> {
        CFC_W::new(self)
    }
    #[doc = "Bits 16:18 - CAN Frame Count Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cfsel(&mut self) -> CFSEL_W<16> {
        CFSEL_W::new(self)
    }
    #[doc = "Bits 19:20 - CAN Frame Counter Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cfmod(&mut self) -> CFMOD_W<19> {
        CFMOD_W::new(self)
    }
    #[doc = "Bit 22 - CAN Frame Count Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfcie(&mut self) -> CFCIE_W<22> {
        CFCIE_W::new(self)
    }
    #[doc = "Bit 23 - CAN Frame Counter Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cfcov(&mut self) -> CFCOV_W<23> {
        CFCOV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Node Frame Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nfcr](index.html) module"]
pub struct NFCR_SPEC;
impl crate::RegisterSpec for NFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nfcr::R](R) reader structure"]
impl crate::Readable for NFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nfcr::W](W) writer structure"]
impl crate::Writable for NFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NFCR to value 0"]
impl crate::Resettable for NFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
