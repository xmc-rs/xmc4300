#[doc = "Register `KSCFG` reader"]
pub struct R(crate::R<KSCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KSCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KSCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KSCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KSCFG` writer"]
pub struct W(crate::W<KSCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KSCFG_SPEC>;
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
impl From<crate::W<KSCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KSCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODEN` reader - Module Enable"]
pub type MODEN_R = crate::BitReader<MODEN_A>;
#[doc = "Module Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODEN_A {
    #[doc = "0: The module is switched off immediately (without respecting a stop condition). It does not react on mode control actions and the module clock is switched off. The module does not react on read accesses and ignores write accesses (except to KSCFG)."]
    VALUE1 = 0,
    #[doc = "1: The module is switched on and can operate. After writing 1 to MODEN, it is recommended to read register KSCFG to avoid pipeline effects in the control block before accessing other Service Request Processing registers."]
    VALUE2 = 1,
}
impl From<MODEN_A> for bool {
    #[inline(always)]
    fn from(variant: MODEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MODEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODEN_A {
        match self.bits {
            false => MODEN_A::VALUE1,
            true => MODEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MODEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MODEN_A::VALUE2
    }
}
#[doc = "Field `MODEN` writer - Module Enable"]
pub type MODEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, KSCFG_SPEC, MODEN_A, O>;
impl<'a, const O: u8> MODEN_W<'a, O> {
    #[doc = "The module is switched off immediately (without respecting a stop condition). It does not react on mode control actions and the module clock is switched off. The module does not react on read accesses and ignores write accesses (except to KSCFG)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MODEN_A::VALUE1)
    }
    #[doc = "The module is switched on and can operate. After writing 1 to MODEN, it is recommended to read register KSCFG to avoid pipeline effects in the control block before accessing other Service Request Processing registers."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MODEN_A::VALUE2)
    }
}
#[doc = "Bit Protection for MODEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BPMODEN_AW {
    #[doc = "0: MODEN is not changed."]
    VALUE1 = 0,
    #[doc = "1: MODEN is updated with the written value."]
    VALUE2 = 1,
}
impl From<BPMODEN_AW> for bool {
    #[inline(always)]
    fn from(variant: BPMODEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPMODEN` writer - Bit Protection for MODEN"]
pub type BPMODEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, KSCFG_SPEC, BPMODEN_AW, O>;
impl<'a, const O: u8> BPMODEN_W<'a, O> {
    #[doc = "MODEN is not changed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BPMODEN_AW::VALUE1)
    }
    #[doc = "MODEN is updated with the written value."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BPMODEN_AW::VALUE2)
    }
}
#[doc = "Field `NOMCFG` reader - Normal Operation Mode Configuration"]
pub type NOMCFG_R = crate::FieldReader<u8, NOMCFG_A>;
#[doc = "Normal Operation Mode Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NOMCFG_A {
    #[doc = "0: Run mode 0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Run mode 1 is selected."]
    VALUE2 = 1,
    #[doc = "2: Stop mode 0 is selected."]
    VALUE3 = 2,
    #[doc = "3: Stop mode 1 is selected."]
    VALUE4 = 3,
}
impl From<NOMCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: NOMCFG_A) -> Self {
        variant as _
    }
}
impl NOMCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOMCFG_A {
        match self.bits {
            0 => NOMCFG_A::VALUE1,
            1 => NOMCFG_A::VALUE2,
            2 => NOMCFG_A::VALUE3,
            3 => NOMCFG_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NOMCFG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NOMCFG_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == NOMCFG_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == NOMCFG_A::VALUE4
    }
}
#[doc = "Field `NOMCFG` writer - Normal Operation Mode Configuration"]
pub type NOMCFG_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, KSCFG_SPEC, u8, NOMCFG_A, 2, O>;
impl<'a, const O: u8> NOMCFG_W<'a, O> {
    #[doc = "Run mode 0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(NOMCFG_A::VALUE1)
    }
    #[doc = "Run mode 1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(NOMCFG_A::VALUE2)
    }
    #[doc = "Stop mode 0 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(NOMCFG_A::VALUE3)
    }
    #[doc = "Stop mode 1 is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(NOMCFG_A::VALUE4)
    }
}
#[doc = "Bit Protection for NOMCFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BPNOM_AW {
    #[doc = "0: NOMCFG is not changed."]
    VALUE1 = 0,
    #[doc = "1: NOMCFG is updated with the written value."]
    VALUE2 = 1,
}
impl From<BPNOM_AW> for bool {
    #[inline(always)]
    fn from(variant: BPNOM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPNOM` writer - Bit Protection for NOMCFG"]
pub type BPNOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, KSCFG_SPEC, BPNOM_AW, O>;
impl<'a, const O: u8> BPNOM_W<'a, O> {
    #[doc = "NOMCFG is not changed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BPNOM_AW::VALUE1)
    }
    #[doc = "NOMCFG is updated with the written value."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BPNOM_AW::VALUE2)
    }
}
#[doc = "Field `SUMCFG` reader - Suspend Mode Configuration"]
pub type SUMCFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUMCFG` writer - Suspend Mode Configuration"]
pub type SUMCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KSCFG_SPEC, u8, u8, 2, O>;
#[doc = "Bit Protection for SUMCFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BPSUM_AW {
    #[doc = "0: SUMCFG is not changed."]
    VALUE1 = 0,
    #[doc = "1: SUMCFG is updated with the written value."]
    VALUE2 = 1,
}
impl From<BPSUM_AW> for bool {
    #[inline(always)]
    fn from(variant: BPSUM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPSUM` writer - Bit Protection for SUMCFG"]
pub type BPSUM_W<'a, const O: u8> = crate::BitWriter<'a, u32, KSCFG_SPEC, BPSUM_AW, O>;
impl<'a, const O: u8> BPSUM_W<'a, O> {
    #[doc = "SUMCFG is not changed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BPSUM_AW::VALUE1)
    }
    #[doc = "SUMCFG is updated with the written value."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BPSUM_AW::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    pub fn moden(&self) -> MODEN_R {
        MODEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Normal Operation Mode Configuration"]
    #[inline(always)]
    pub fn nomcfg(&self) -> NOMCFG_R {
        NOMCFG_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Suspend Mode Configuration"]
    #[inline(always)]
    pub fn sumcfg(&self) -> SUMCFG_R {
        SUMCFG_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moden(&mut self) -> MODEN_W<0> {
        MODEN_W::new(self)
    }
    #[doc = "Bit 1 - Bit Protection for MODEN"]
    #[inline(always)]
    #[must_use]
    pub fn bpmoden(&mut self) -> BPMODEN_W<1> {
        BPMODEN_W::new(self)
    }
    #[doc = "Bits 4:5 - Normal Operation Mode Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn nomcfg(&mut self) -> NOMCFG_W<4> {
        NOMCFG_W::new(self)
    }
    #[doc = "Bit 7 - Bit Protection for NOMCFG"]
    #[inline(always)]
    #[must_use]
    pub fn bpnom(&mut self) -> BPNOM_W<7> {
        BPNOM_W::new(self)
    }
    #[doc = "Bits 8:9 - Suspend Mode Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn sumcfg(&mut self) -> SUMCFG_W<8> {
        SUMCFG_W::new(self)
    }
    #[doc = "Bit 11 - Bit Protection for SUMCFG"]
    #[inline(always)]
    #[must_use]
    pub fn bpsum(&mut self) -> BPSUM_W<11> {
        BPSUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Kernel State Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kscfg](index.html) module"]
pub struct KSCFG_SPEC;
impl crate::RegisterSpec for KSCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [kscfg::R](R) reader structure"]
impl crate::Readable for KSCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [kscfg::W](W) writer structure"]
impl crate::Writable for KSCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KSCFG to value 0"]
impl crate::Resettable for KSCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
