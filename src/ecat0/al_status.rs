#[doc = "Register `AL_STATUS` reader"]
pub struct R(crate::R<AL_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AL_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AL_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AL_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AL_STATUS` writer"]
pub struct W(crate::W<AL_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AL_STATUS_SPEC>;
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
impl From<crate::W<AL_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AL_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATE` reader - Actual State of the Device State Machine"]
pub type STATE_R = crate::FieldReader<u8, STATE_A>;
#[doc = "Actual State of the Device State Machine\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATE_A {
    #[doc = "1: Init State"]
    VALUE1 = 1,
    #[doc = "2: Pre-Operational State"]
    VALUE2 = 2,
    #[doc = "3: Bootstrap State"]
    VALUE3 = 3,
    #[doc = "4: Safe-Operational State"]
    VALUE4 = 4,
    #[doc = "8: Operational State"]
    VALUE5 = 8,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as _
    }
}
impl STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STATE_A> {
        match self.bits {
            1 => Some(STATE_A::VALUE1),
            2 => Some(STATE_A::VALUE2),
            3 => Some(STATE_A::VALUE3),
            4 => Some(STATE_A::VALUE4),
            8 => Some(STATE_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STATE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STATE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == STATE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == STATE_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == STATE_A::VALUE5
    }
}
#[doc = "Field `STATE` writer - Actual State of the Device State Machine"]
pub type STATE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, AL_STATUS_SPEC, u8, STATE_A, 4, O>;
impl<'a, const O: u8> STATE_W<'a, O> {
    #[doc = "Init State"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STATE_A::VALUE1)
    }
    #[doc = "Pre-Operational State"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STATE_A::VALUE2)
    }
    #[doc = "Bootstrap State"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(STATE_A::VALUE3)
    }
    #[doc = "Safe-Operational State"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(STATE_A::VALUE4)
    }
    #[doc = "Operational State"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(STATE_A::VALUE5)
    }
}
#[doc = "Field `ERRI` reader - Error Ind"]
pub type ERRI_R = crate::BitReader<ERRI_A>;
#[doc = "Error Ind\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRI_A {
    #[doc = "0: Device is in State as requested or Flag cleared by command"]
    VALUE1 = 0,
    #[doc = "1: Device has not entered requested State or changed State as result of a local action"]
    VALUE2 = 1,
}
impl From<ERRI_A> for bool {
    #[inline(always)]
    fn from(variant: ERRI_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRI_A {
        match self.bits {
            false => ERRI_A::VALUE1,
            true => ERRI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ERRI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ERRI_A::VALUE2
    }
}
#[doc = "Field `ERRI` writer - Error Ind"]
pub type ERRI_W<'a, const O: u8> = crate::BitWriter<'a, u16, AL_STATUS_SPEC, ERRI_A, O>;
impl<'a, const O: u8> ERRI_W<'a, O> {
    #[doc = "Device is in State as requested or Flag cleared by command"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ERRI_A::VALUE1)
    }
    #[doc = "Device has not entered requested State or changed State as result of a local action"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ERRI_A::VALUE2)
    }
}
#[doc = "Field `DID` reader - Device Identification"]
pub type DID_R = crate::BitReader<DID_A>;
#[doc = "Device Identification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DID_A {
    #[doc = "0: Device Identification not valid"]
    VALUE1 = 0,
    #[doc = "1: Device Identification loaded"]
    VALUE2 = 1,
}
impl From<DID_A> for bool {
    #[inline(always)]
    fn from(variant: DID_A) -> Self {
        variant as u8 != 0
    }
}
impl DID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DID_A {
        match self.bits {
            false => DID_A::VALUE1,
            true => DID_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DID_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DID_A::VALUE2
    }
}
#[doc = "Field `DID` writer - Device Identification"]
pub type DID_W<'a, const O: u8> = crate::BitWriter<'a, u16, AL_STATUS_SPEC, DID_A, O>;
impl<'a, const O: u8> DID_W<'a, O> {
    #[doc = "Device Identification not valid"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DID_A::VALUE1)
    }
    #[doc = "Device Identification loaded"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DID_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:3 - Actual State of the Device State Machine"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Error Ind"]
    #[inline(always)]
    pub fn erri(&self) -> ERRI_R {
        ERRI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Device Identification"]
    #[inline(always)]
    pub fn did(&self) -> DID_R {
        DID_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Actual State of the Device State Machine"]
    #[inline(always)]
    #[must_use]
    pub fn state(&mut self) -> STATE_W<0> {
        STATE_W::new(self)
    }
    #[doc = "Bit 4 - Error Ind"]
    #[inline(always)]
    #[must_use]
    pub fn erri(&mut self) -> ERRI_W<4> {
        ERRI_W::new(self)
    }
    #[doc = "Bit 5 - Device Identification"]
    #[inline(always)]
    #[must_use]
    pub fn did(&mut self) -> DID_W<5> {
        DID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AL Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [al_status](index.html) module"]
pub struct AL_STATUS_SPEC;
impl crate::RegisterSpec for AL_STATUS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [al_status::R](R) reader structure"]
impl crate::Readable for AL_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [al_status::W](W) writer structure"]
impl crate::Writable for AL_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AL_STATUS to value 0x01"]
impl crate::Resettable for AL_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
