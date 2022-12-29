#[doc = "Register `TC` reader"]
pub struct R(crate::R<TC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TC` writer"]
pub struct W(crate::W<TC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TC_SPEC>;
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
impl From<crate::W<TC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCM` reader - Timer Counting Mode"]
pub type TCM_R = crate::BitReader<TCM_A>;
#[doc = "Timer Counting Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCM_A {
    #[doc = "0: Edge aligned mode"]
    VALUE1 = 0,
    #[doc = "1: Center aligned mode"]
    VALUE2 = 1,
}
impl From<TCM_A> for bool {
    #[inline(always)]
    fn from(variant: TCM_A) -> Self {
        variant as u8 != 0
    }
}
impl TCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCM_A {
        match self.bits {
            false => TCM_A::VALUE1,
            true => TCM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TCM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TCM_A::VALUE2
    }
}
#[doc = "Field `TCM` writer - Timer Counting Mode"]
pub type TCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TC_SPEC, TCM_A, O>;
impl<'a, const O: u8> TCM_W<'a, O> {
    #[doc = "Edge aligned mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TCM_A::VALUE1)
    }
    #[doc = "Center aligned mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TCM_A::VALUE2)
    }
}
#[doc = "Field `TSSM` reader - Timer Single Shot Mode"]
pub type TSSM_R = crate::BitReader<TSSM_A>;
#[doc = "Timer Single Shot Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSSM_A {
    #[doc = "0: Single shot mode is disabled"]
    VALUE1 = 0,
    #[doc = "1: Single shot mode is enabled"]
    VALUE2 = 1,
}
impl From<TSSM_A> for bool {
    #[inline(always)]
    fn from(variant: TSSM_A) -> Self {
        variant as u8 != 0
    }
}
impl TSSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSSM_A {
        match self.bits {
            false => TSSM_A::VALUE1,
            true => TSSM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSSM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSSM_A::VALUE2
    }
}
#[doc = "Field `TSSM` writer - Timer Single Shot Mode"]
pub type TSSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TC_SPEC, TSSM_A, O>;
impl<'a, const O: u8> TSSM_W<'a, O> {
    #[doc = "Single shot mode is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSSM_A::VALUE1)
    }
    #[doc = "Single shot mode is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSSM_A::VALUE2)
    }
}
#[doc = "Field `CLST` reader - Shadow Transfer on Clear"]
pub type CLST_R = crate::BitReader<bool>;
#[doc = "Field `CLST` writer - Shadow Transfer on Clear"]
pub type CLST_W<'a, const O: u8> = crate::BitWriter<'a, u32, TC_SPEC, bool, O>;
#[doc = "Field `CMOD` reader - Capture Compare Mode"]
pub type CMOD_R = crate::BitReader<CMOD_A>;
#[doc = "Capture Compare Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMOD_A {
    #[doc = "0: Compare Mode"]
    VALUE1 = 0,
    #[doc = "1: Capture Mode"]
    VALUE2 = 1,
}
impl From<CMOD_A> for bool {
    #[inline(always)]
    fn from(variant: CMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl CMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMOD_A {
        match self.bits {
            false => CMOD_A::VALUE1,
            true => CMOD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMOD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMOD_A::VALUE2
    }
}
#[doc = "Field `ECM` reader - Extended Capture Mode"]
pub type ECM_R = crate::BitReader<ECM_A>;
#[doc = "Extended Capture Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECM_A {
    #[doc = "0: Normal Capture Mode. Clear of the Full Flag of each capture register is done by accessing the registers individually only."]
    VALUE1 = 0,
    #[doc = "1: Extended Capture Mode. Clear of the Full Flag of each capture register is done not only by accessing the individual registers but also by accessing the ECRD register. When reading the ECRD register, only the capture register register full flag pointed by the VPTR is cleared"]
    VALUE2 = 1,
}
impl From<ECM_A> for bool {
    #[inline(always)]
    fn from(variant: ECM_A) -> Self {
        variant as u8 != 0
    }
}
impl ECM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECM_A {
        match self.bits {
            false => ECM_A::VALUE1,
            true => ECM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ECM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ECM_A::VALUE2
    }
}
#[doc = "Field `ECM` writer - Extended Capture Mode"]
pub type ECM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TC_SPEC, ECM_A, O>;
impl<'a, const O: u8> ECM_W<'a, O> {
    #[doc = "Normal Capture Mode. Clear of the Full Flag of each capture register is done by accessing the registers individually only."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECM_A::VALUE1)
    }
    #[doc = "Extended Capture Mode. Clear of the Full Flag of each capture register is done not only by accessing the individual registers but also by accessing the ECRD register. When reading the ECRD register, only the capture register register full flag pointed by the VPTR is cleared"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECM_A::VALUE2)
    }
}
#[doc = "Field `CAPC` reader - Clear on Capture Control"]
pub type CAPC_R = crate::FieldReader<u8, CAPC_A>;
#[doc = "Clear on Capture Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAPC_A {
    #[doc = "0: Timer is never cleared on a capture event"]
    VALUE1 = 0,
    #[doc = "1: Timer is cleared on a capture event into capture registers 2 and 3. (When SCE = 1#, Timer is always cleared in a capture event)"]
    VALUE2 = 1,
    #[doc = "2: Timer is cleared on a capture event into capture registers 0 and 1. (When SCE = 1#, Timer is always cleared in a capture event)"]
    VALUE3 = 2,
    #[doc = "3: Timer is always cleared in a capture event."]
    VALUE4 = 3,
}
impl From<CAPC_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPC_A) -> Self {
        variant as _
    }
}
impl CAPC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPC_A {
        match self.bits {
            0 => CAPC_A::VALUE1,
            1 => CAPC_A::VALUE2,
            2 => CAPC_A::VALUE3,
            3 => CAPC_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CAPC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CAPC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CAPC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CAPC_A::VALUE4
    }
}
#[doc = "Field `CAPC` writer - Clear on Capture Control"]
pub type CAPC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TC_SPEC, u8, CAPC_A, 2, O>;
impl<'a, const O: u8> CAPC_W<'a, O> {
    #[doc = "Timer is never cleared on a capture event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CAPC_A::VALUE1)
    }
    #[doc = "Timer is cleared on a capture event into capture registers 2 and 3. (When SCE = 1#, Timer is always cleared in a capture event)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CAPC_A::VALUE2)
    }
    #[doc = "Timer is cleared on a capture event into capture registers 0 and 1. (When SCE = 1#, Timer is always cleared in a capture event)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CAPC_A::VALUE3)
    }
    #[doc = "Timer is always cleared in a capture event."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CAPC_A::VALUE4)
    }
}
#[doc = "Field `TLS` reader - Timer Load selector"]
pub type TLS_R = crate::BitReader<TLS_A>;
#[doc = "Timer Load selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TLS_A {
    #[doc = "0: Timer is loaded with the value of CR1"]
    VALUE1 = 0,
    #[doc = "1: Timer is loaded with the value of CR2"]
    VALUE2 = 1,
}
impl From<TLS_A> for bool {
    #[inline(always)]
    fn from(variant: TLS_A) -> Self {
        variant as u8 != 0
    }
}
impl TLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TLS_A {
        match self.bits {
            false => TLS_A::VALUE1,
            true => TLS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TLS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TLS_A::VALUE2
    }
}
#[doc = "Field `TLS` writer - Timer Load selector"]
pub type TLS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TC_SPEC, TLS_A, O>;
impl<'a, const O: u8> TLS_W<'a, O> {
    #[doc = "Timer is loaded with the value of CR1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TLS_A::VALUE1)
    }
    #[doc = "Timer is loaded with the value of CR2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TLS_A::VALUE2)
    }
}
#[doc = "Field `ENDM` reader - Extended Stop Function Control"]
pub type ENDM_R = crate::FieldReader<u8, ENDM_A>;
#[doc = "Extended Stop Function Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENDM_A {
    #[doc = "0: Clears the timer run bit only (default stop)"]
    VALUE1 = 0,
    #[doc = "1: Clears the timer only (flush)"]
    VALUE2 = 1,
    #[doc = "2: Clears the timer and run bit (flush/stop)"]
    VALUE3 = 2,
}
impl From<ENDM_A> for u8 {
    #[inline(always)]
    fn from(variant: ENDM_A) -> Self {
        variant as _
    }
}
impl ENDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ENDM_A> {
        match self.bits {
            0 => Some(ENDM_A::VALUE1),
            1 => Some(ENDM_A::VALUE2),
            2 => Some(ENDM_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENDM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENDM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ENDM_A::VALUE3
    }
}
#[doc = "Field `ENDM` writer - Extended Stop Function Control"]
pub type ENDM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TC_SPEC, u8, ENDM_A, 2, O>;
impl<'a, const O: u8> ENDM_W<'a, O> {
    #[doc = "Clears the timer run bit only (default stop)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENDM_A::VALUE1)
    }
    #[doc = "Clears the timer only (flush)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENDM_A::VALUE2)
    }
    #[doc = "Clears the timer and run bit (flush/stop)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ENDM_A::VALUE3)
    }
}
#[doc = "Field `STRM` reader - Extended Start Function Control"]
pub type STRM_R = crate::BitReader<STRM_A>;
#[doc = "Extended Start Function Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRM_A {
    #[doc = "0: Sets run bit only (default start)"]
    VALUE1 = 0,
    #[doc = "1: Clears the timer and sets run bit, if not set (flush/start)"]
    VALUE2 = 1,
}
impl From<STRM_A> for bool {
    #[inline(always)]
    fn from(variant: STRM_A) -> Self {
        variant as u8 != 0
    }
}
impl STRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STRM_A {
        match self.bits {
            false => STRM_A::VALUE1,
            true => STRM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STRM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STRM_A::VALUE2
    }
}
#[doc = "Field `STRM` writer - Extended Start Function Control"]
pub type STRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TC_SPEC, STRM_A, O>;
impl<'a, const O: u8> STRM_W<'a, O> {
    #[doc = "Sets run bit only (default start)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STRM_A::VALUE1)
    }
    #[doc = "Clears the timer and sets run bit, if not set (flush/start)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STRM_A::VALUE2)
    }
}
#[doc = "Field `SCE` reader - Equal Capture Event enable"]
pub type SCE_R = crate::BitReader<SCE_A>;
#[doc = "Equal Capture Event enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCE_A {
    #[doc = "0: Capture into CC8yC0VThis register contains the values associated with the Capture 0 field./CC8yC1VThis register contains the values associated with the Capture 1 field. registers control by CCycapt0 and capture into CC8yC3VThis register contains the values associated with the Capture 3 field./CC8yC2VThis register contains the values associated with the Capture 2 field. control by CCycapt1"]
    VALUE1 = 0,
    #[doc = "1: Capture into CC8yC0VThis register contains the values associated with the Capture 0 field./CC8yC1VThis register contains the values associated with the Capture 1 field. and CC8yC3VThis register contains the values associated with the Capture 3 field./CC8yC2VThis register contains the values associated with the Capture 2 field. control by CCycapt1"]
    VALUE2 = 1,
}
impl From<SCE_A> for bool {
    #[inline(always)]
    fn from(variant: SCE_A) -> Self {
        variant as u8 != 0
    }
}
impl SCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCE_A {
        match self.bits {
            false => SCE_A::VALUE1,
            true => SCE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SCE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SCE_A::VALUE2
    }
}
#[doc = "Field `SCE` writer - Equal Capture Event enable"]
pub type SCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TC_SPEC, SCE_A, O>;
impl<'a, const O: u8> SCE_W<'a, O> {
    #[doc = "Capture into CC8yC0VThis register contains the values associated with the Capture 0 field./CC8yC1VThis register contains the values associated with the Capture 1 field. registers control by CCycapt0 and capture into CC8yC3VThis register contains the values associated with the Capture 3 field./CC8yC2VThis register contains the values associated with the Capture 2 field. control by CCycapt1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCE_A::VALUE1)
    }
    #[doc = "Capture into CC8yC0VThis register contains the values associated with the Capture 0 field./CC8yC1VThis register contains the values associated with the Capture 1 field. and CC8yC3VThis register contains the values associated with the Capture 3 field./CC8yC2VThis register contains the values associated with the Capture 2 field. control by CCycapt1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCE_A::VALUE2)
    }
}
#[doc = "Field `CCS` reader - Continuous Capture Enable"]
pub type CCS_R = crate::BitReader<CCS_A>;
#[doc = "Continuous Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCS_A {
    #[doc = "0: The capture into a specific capture register is done with the rules linked with the full flags, described at ."]
    VALUE1 = 0,
    #[doc = "1: The capture into the capture registers is always done regardless of the full flag status (even if the register has not been read back)."]
    VALUE2 = 1,
}
impl From<CCS_A> for bool {
    #[inline(always)]
    fn from(variant: CCS_A) -> Self {
        variant as u8 != 0
    }
}
impl CCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCS_A {
        match self.bits {
            false => CCS_A::VALUE1,
            true => CCS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCS_A::VALUE2
    }
}
#[doc = "Field `CCS` writer - Continuous Capture Enable"]
pub type CCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TC_SPEC, CCS_A, O>;
impl<'a, const O: u8> CCS_W<'a, O> {
    #[doc = "The capture into a specific capture register is done with the rules linked with the full flags, described at ."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCS_A::VALUE1)
    }
    #[doc = "The capture into the capture registers is always done regardless of the full flag status (even if the register has not been read back)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCS_A::VALUE2)
    }
}
#[doc = "Field `DITHE` reader - Dither Enable"]
pub type DITHE_R = crate::FieldReader<u8, DITHE_A>;
#[doc = "Dither Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DITHE_A {
    #[doc = "0: Dither is disabled"]
    VALUE1 = 0,
    #[doc = "1: Dither is applied to the Period"]
    VALUE2 = 1,
    #[doc = "2: Dither is applied to the Compare"]
    VALUE3 = 2,
    #[doc = "3: Dither is applied to the Period and Compare"]
    VALUE4 = 3,
}
impl From<DITHE_A> for u8 {
    #[inline(always)]
    fn from(variant: DITHE_A) -> Self {
        variant as _
    }
}
impl DITHE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DITHE_A {
        match self.bits {
            0 => DITHE_A::VALUE1,
            1 => DITHE_A::VALUE2,
            2 => DITHE_A::VALUE3,
            3 => DITHE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DITHE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DITHE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DITHE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DITHE_A::VALUE4
    }
}
#[doc = "Field `DITHE` writer - Dither Enable"]
pub type DITHE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TC_SPEC, u8, DITHE_A, 2, O>;
impl<'a, const O: u8> DITHE_W<'a, O> {
    #[doc = "Dither is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DITHE_A::VALUE1)
    }
    #[doc = "Dither is applied to the Period"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DITHE_A::VALUE2)
    }
    #[doc = "Dither is applied to the Compare"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DITHE_A::VALUE3)
    }
    #[doc = "Dither is applied to the Period and Compare"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(DITHE_A::VALUE4)
    }
}
#[doc = "Field `DIM` reader - Dither input selector"]
pub type DIM_R = crate::BitReader<DIM_A>;
#[doc = "Dither input selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIM_A {
    #[doc = "0: Slice is using it own dither unit"]
    VALUE1 = 0,
    #[doc = "1: Slice is connected to the dither unit of slice 0."]
    VALUE2 = 1,
}
impl From<DIM_A> for bool {
    #[inline(always)]
    fn from(variant: DIM_A) -> Self {
        variant as u8 != 0
    }
}
impl DIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIM_A {
        match self.bits {
            false => DIM_A::VALUE1,
            true => DIM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIM_A::VALUE2
    }
}
#[doc = "Field `DIM` writer - Dither input selector"]
pub type DIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TC_SPEC, DIM_A, O>;
impl<'a, const O: u8> DIM_W<'a, O> {
    #[doc = "Slice is using it own dither unit"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIM_A::VALUE1)
    }
    #[doc = "Slice is connected to the dither unit of slice 0."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIM_A::VALUE2)
    }
}
#[doc = "Field `FPE` reader - Floating Prescaler enable"]
pub type FPE_R = crate::BitReader<FPE_A>;
#[doc = "Floating Prescaler enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPE_A {
    #[doc = "0: Floating prescaler mode is disabled"]
    VALUE1 = 0,
    #[doc = "1: Floating prescaler mode is enabled"]
    VALUE2 = 1,
}
impl From<FPE_A> for bool {
    #[inline(always)]
    fn from(variant: FPE_A) -> Self {
        variant as u8 != 0
    }
}
impl FPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPE_A {
        match self.bits {
            false => FPE_A::VALUE1,
            true => FPE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FPE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FPE_A::VALUE2
    }
}
#[doc = "Field `FPE` writer - Floating Prescaler enable"]
pub type FPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TC_SPEC, FPE_A, O>;
impl<'a, const O: u8> FPE_W<'a, O> {
    #[doc = "Floating prescaler mode is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FPE_A::VALUE1)
    }
    #[doc = "Floating prescaler mode is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FPE_A::VALUE2)
    }
}
#[doc = "Field `TRAPE0` reader - TRAP enable for CCU8x.OUTy0"]
pub type TRAPE0_R = crate::BitReader<TRAPE0_A>;
#[doc = "TRAP enable for CCU8x.OUTy0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRAPE0_A {
    #[doc = "0: TRAP functionality has no effect on the CCU8x.OUTy0 output"]
    VALUE1 = 0,
    #[doc = "1: TRAP functionality affects the CCU8x.OUTy0 output"]
    VALUE2 = 1,
}
impl From<TRAPE0_A> for bool {
    #[inline(always)]
    fn from(variant: TRAPE0_A) -> Self {
        variant as u8 != 0
    }
}
impl TRAPE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRAPE0_A {
        match self.bits {
            false => TRAPE0_A::VALUE1,
            true => TRAPE0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRAPE0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRAPE0_A::VALUE2
    }
}
#[doc = "Field `TRAPE0` writer - TRAP enable for CCU8x.OUTy0"]
pub type TRAPE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TC_SPEC, TRAPE0_A, O>;
impl<'a, const O: u8> TRAPE0_W<'a, O> {
    #[doc = "TRAP functionality has no effect on the CCU8x.OUTy0 output"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRAPE0_A::VALUE1)
    }
    #[doc = "TRAP functionality affects the CCU8x.OUTy0 output"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRAPE0_A::VALUE2)
    }
}
#[doc = "Field `TRAPE1` reader - TRAP enable for CCU8x.OUTy1"]
pub type TRAPE1_R = crate::BitReader<bool>;
#[doc = "Field `TRAPE1` writer - TRAP enable for CCU8x.OUTy1"]
pub type TRAPE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TC_SPEC, bool, O>;
#[doc = "Field `TRAPE2` reader - TRAP enable for CCU8x.OUTy2"]
pub type TRAPE2_R = crate::BitReader<bool>;
#[doc = "Field `TRAPE2` writer - TRAP enable for CCU8x.OUTy2"]
pub type TRAPE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TC_SPEC, bool, O>;
#[doc = "Field `TRAPE3` reader - TRAP enable for CCU8x.OUTy3"]
pub type TRAPE3_R = crate::BitReader<bool>;
#[doc = "Field `TRAPE3` writer - TRAP enable for CCU8x.OUTy3"]
pub type TRAPE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TC_SPEC, bool, O>;
#[doc = "Field `TRPSE` reader - TRAP Synchronization Enable"]
pub type TRPSE_R = crate::BitReader<TRPSE_A>;
#[doc = "TRAP Synchronization Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRPSE_A {
    #[doc = "0: Exiting from TRAP state isn't synchronized with the PWM signal"]
    VALUE1 = 0,
    #[doc = "1: Exiting from TRAP state is synchronized with the PWM signal"]
    VALUE2 = 1,
}
impl From<TRPSE_A> for bool {
    #[inline(always)]
    fn from(variant: TRPSE_A) -> Self {
        variant as u8 != 0
    }
}
impl TRPSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRPSE_A {
        match self.bits {
            false => TRPSE_A::VALUE1,
            true => TRPSE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRPSE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRPSE_A::VALUE2
    }
}
#[doc = "Field `TRPSE` writer - TRAP Synchronization Enable"]
pub type TRPSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TC_SPEC, TRPSE_A, O>;
impl<'a, const O: u8> TRPSE_W<'a, O> {
    #[doc = "Exiting from TRAP state isn't synchronized with the PWM signal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRPSE_A::VALUE1)
    }
    #[doc = "Exiting from TRAP state is synchronized with the PWM signal"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRPSE_A::VALUE2)
    }
}
#[doc = "Field `TRPSW` reader - TRAP State Clear Control"]
pub type TRPSW_R = crate::BitReader<TRPSW_A>;
#[doc = "TRAP State Clear Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRPSW_A {
    #[doc = "0: The slice exits the TRAP state automatically when the TRAP condition is not present (Trap state cleared by HW and SW)"]
    VALUE1 = 0,
    #[doc = "1: The TRAP state can only be exited by a SW request."]
    VALUE2 = 1,
}
impl From<TRPSW_A> for bool {
    #[inline(always)]
    fn from(variant: TRPSW_A) -> Self {
        variant as u8 != 0
    }
}
impl TRPSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRPSW_A {
        match self.bits {
            false => TRPSW_A::VALUE1,
            true => TRPSW_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRPSW_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRPSW_A::VALUE2
    }
}
#[doc = "Field `TRPSW` writer - TRAP State Clear Control"]
pub type TRPSW_W<'a, const O: u8> = crate::BitWriter<'a, u32, TC_SPEC, TRPSW_A, O>;
impl<'a, const O: u8> TRPSW_W<'a, O> {
    #[doc = "The slice exits the TRAP state automatically when the TRAP condition is not present (Trap state cleared by HW and SW)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRPSW_A::VALUE1)
    }
    #[doc = "The TRAP state can only be exited by a SW request."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRPSW_A::VALUE2)
    }
}
#[doc = "Field `EMS` reader - External Modulation Synchronization"]
pub type EMS_R = crate::BitReader<EMS_A>;
#[doc = "External Modulation Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EMS_A {
    #[doc = "0: External Modulation functionality is not synchronized with the PWM signal"]
    VALUE1 = 0,
    #[doc = "1: External Modulation functionality is synchronized with the PWM signal"]
    VALUE2 = 1,
}
impl From<EMS_A> for bool {
    #[inline(always)]
    fn from(variant: EMS_A) -> Self {
        variant as u8 != 0
    }
}
impl EMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMS_A {
        match self.bits {
            false => EMS_A::VALUE1,
            true => EMS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EMS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EMS_A::VALUE2
    }
}
#[doc = "Field `EMS` writer - External Modulation Synchronization"]
pub type EMS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TC_SPEC, EMS_A, O>;
impl<'a, const O: u8> EMS_W<'a, O> {
    #[doc = "External Modulation functionality is not synchronized with the PWM signal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EMS_A::VALUE1)
    }
    #[doc = "External Modulation functionality is synchronized with the PWM signal"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EMS_A::VALUE2)
    }
}
#[doc = "Field `EMT` reader - External Modulation Type"]
pub type EMT_R = crate::BitReader<EMT_A>;
#[doc = "External Modulation Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EMT_A {
    #[doc = "0: External Modulation functionality is clearing the CC8ySTx bits."]
    VALUE1 = 0,
    #[doc = "1: External Modulation functionality is gating the outputs."]
    VALUE2 = 1,
}
impl From<EMT_A> for bool {
    #[inline(always)]
    fn from(variant: EMT_A) -> Self {
        variant as u8 != 0
    }
}
impl EMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMT_A {
        match self.bits {
            false => EMT_A::VALUE1,
            true => EMT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EMT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EMT_A::VALUE2
    }
}
#[doc = "Field `EMT` writer - External Modulation Type"]
pub type EMT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TC_SPEC, EMT_A, O>;
impl<'a, const O: u8> EMT_W<'a, O> {
    #[doc = "External Modulation functionality is clearing the CC8ySTx bits."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EMT_A::VALUE1)
    }
    #[doc = "External Modulation functionality is gating the outputs."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EMT_A::VALUE2)
    }
}
#[doc = "Field `MCME1` reader - Multi Channel Mode Enable for Channel 1"]
pub type MCME1_R = crate::BitReader<MCME1_A>;
#[doc = "Multi Channel Mode Enable for Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCME1_A {
    #[doc = "0: Multi Channel Mode in Channel 1 is disabled"]
    VALUE1 = 0,
    #[doc = "1: Multi Channel Mode in Channel 1 is enabled"]
    VALUE2 = 1,
}
impl From<MCME1_A> for bool {
    #[inline(always)]
    fn from(variant: MCME1_A) -> Self {
        variant as u8 != 0
    }
}
impl MCME1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCME1_A {
        match self.bits {
            false => MCME1_A::VALUE1,
            true => MCME1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MCME1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MCME1_A::VALUE2
    }
}
#[doc = "Field `MCME1` writer - Multi Channel Mode Enable for Channel 1"]
pub type MCME1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TC_SPEC, MCME1_A, O>;
impl<'a, const O: u8> MCME1_W<'a, O> {
    #[doc = "Multi Channel Mode in Channel 1 is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MCME1_A::VALUE1)
    }
    #[doc = "Multi Channel Mode in Channel 1 is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MCME1_A::VALUE2)
    }
}
#[doc = "Field `MCME2` reader - Multi Channel Mode Enable for Channel 2"]
pub type MCME2_R = crate::BitReader<MCME2_A>;
#[doc = "Multi Channel Mode Enable for Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCME2_A {
    #[doc = "0: Multi Channel Mode in Channel 2 is disabled"]
    VALUE1 = 0,
    #[doc = "1: Multi Channel Mode in Channel 2 is enabled"]
    VALUE2 = 1,
}
impl From<MCME2_A> for bool {
    #[inline(always)]
    fn from(variant: MCME2_A) -> Self {
        variant as u8 != 0
    }
}
impl MCME2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCME2_A {
        match self.bits {
            false => MCME2_A::VALUE1,
            true => MCME2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MCME2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MCME2_A::VALUE2
    }
}
#[doc = "Field `MCME2` writer - Multi Channel Mode Enable for Channel 2"]
pub type MCME2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TC_SPEC, MCME2_A, O>;
impl<'a, const O: u8> MCME2_W<'a, O> {
    #[doc = "Multi Channel Mode in Channel 2 is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MCME2_A::VALUE1)
    }
    #[doc = "Multi Channel Mode in Channel 2 is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MCME2_A::VALUE2)
    }
}
#[doc = "Field `EME` reader - External Modulation Channel enable"]
pub type EME_R = crate::FieldReader<u8, EME_A>;
#[doc = "External Modulation Channel enable\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EME_A {
    #[doc = "0: External Modulation functionality doesn't affect any channel"]
    VALUE1 = 0,
    #[doc = "1: External Modulation only applied on channel 1"]
    VALUE2 = 1,
    #[doc = "2: External Modulation only applied on channel 2"]
    VALUE3 = 2,
    #[doc = "3: External Modulation applied on both channels"]
    VALUE4 = 3,
}
impl From<EME_A> for u8 {
    #[inline(always)]
    fn from(variant: EME_A) -> Self {
        variant as _
    }
}
impl EME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EME_A {
        match self.bits {
            0 => EME_A::VALUE1,
            1 => EME_A::VALUE2,
            2 => EME_A::VALUE3,
            3 => EME_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EME_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EME_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EME_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EME_A::VALUE4
    }
}
#[doc = "Field `EME` writer - External Modulation Channel enable"]
pub type EME_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TC_SPEC, u8, EME_A, 2, O>;
impl<'a, const O: u8> EME_W<'a, O> {
    #[doc = "External Modulation functionality doesn't affect any channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EME_A::VALUE1)
    }
    #[doc = "External Modulation only applied on channel 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EME_A::VALUE2)
    }
    #[doc = "External Modulation only applied on channel 2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(EME_A::VALUE3)
    }
    #[doc = "External Modulation applied on both channels"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(EME_A::VALUE4)
    }
}
#[doc = "Field `STOS` reader - Status bit output selector"]
pub type STOS_R = crate::FieldReader<u8, STOS_A>;
#[doc = "Status bit output selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STOS_A {
    #[doc = "0: CC8yST1 forward to CCU8x.STy"]
    VALUE1 = 0,
    #[doc = "1: CC8yST2 forward to CCU8x.STy"]
    VALUE2 = 1,
    #[doc = "2: CC8yST1 AND CC8yST2 forward to CCU8x.STy"]
    VALUE3 = 2,
    #[doc = "3: CC8yST1 OR CC8yST2 forward to CCU8x.STy"]
    VALUE4 = 3,
}
impl From<STOS_A> for u8 {
    #[inline(always)]
    fn from(variant: STOS_A) -> Self {
        variant as _
    }
}
impl STOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOS_A {
        match self.bits {
            0 => STOS_A::VALUE1,
            1 => STOS_A::VALUE2,
            2 => STOS_A::VALUE3,
            3 => STOS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STOS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STOS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == STOS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == STOS_A::VALUE4
    }
}
#[doc = "Field `STOS` writer - Status bit output selector"]
pub type STOS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TC_SPEC, u8, STOS_A, 2, O>;
impl<'a, const O: u8> STOS_W<'a, O> {
    #[doc = "CC8yST1 forward to CCU8x.STy"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STOS_A::VALUE1)
    }
    #[doc = "CC8yST2 forward to CCU8x.STy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STOS_A::VALUE2)
    }
    #[doc = "CC8yST1 AND CC8yST2 forward to CCU8x.STy"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(STOS_A::VALUE3)
    }
    #[doc = "CC8yST1 OR CC8yST2 forward to CCU8x.STy"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(STOS_A::VALUE4)
    }
}
impl R {
    #[doc = "Bit 0 - Timer Counting Mode"]
    #[inline(always)]
    pub fn tcm(&self) -> TCM_R {
        TCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer Single Shot Mode"]
    #[inline(always)]
    pub fn tssm(&self) -> TSSM_R {
        TSSM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Shadow Transfer on Clear"]
    #[inline(always)]
    pub fn clst(&self) -> CLST_R {
        CLST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture Compare Mode"]
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Extended Capture Mode"]
    #[inline(always)]
    pub fn ecm(&self) -> ECM_R {
        ECM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Clear on Capture Control"]
    #[inline(always)]
    pub fn capc(&self) -> CAPC_R {
        CAPC_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Timer Load selector"]
    #[inline(always)]
    pub fn tls(&self) -> TLS_R {
        TLS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Extended Stop Function Control"]
    #[inline(always)]
    pub fn endm(&self) -> ENDM_R {
        ENDM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Extended Start Function Control"]
    #[inline(always)]
    pub fn strm(&self) -> STRM_R {
        STRM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Equal Capture Event enable"]
    #[inline(always)]
    pub fn sce(&self) -> SCE_R {
        SCE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Continuous Capture Enable"]
    #[inline(always)]
    pub fn ccs(&self) -> CCS_R {
        CCS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Dither Enable"]
    #[inline(always)]
    pub fn dithe(&self) -> DITHE_R {
        DITHE_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Dither input selector"]
    #[inline(always)]
    pub fn dim(&self) -> DIM_R {
        DIM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Floating Prescaler enable"]
    #[inline(always)]
    pub fn fpe(&self) -> FPE_R {
        FPE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TRAP enable for CCU8x.OUTy0"]
    #[inline(always)]
    pub fn trape0(&self) -> TRAPE0_R {
        TRAPE0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TRAP enable for CCU8x.OUTy1"]
    #[inline(always)]
    pub fn trape1(&self) -> TRAPE1_R {
        TRAPE1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TRAP enable for CCU8x.OUTy2"]
    #[inline(always)]
    pub fn trape2(&self) -> TRAPE2_R {
        TRAPE2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TRAP enable for CCU8x.OUTy3"]
    #[inline(always)]
    pub fn trape3(&self) -> TRAPE3_R {
        TRAPE3_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TRAP Synchronization Enable"]
    #[inline(always)]
    pub fn trpse(&self) -> TRPSE_R {
        TRPSE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TRAP State Clear Control"]
    #[inline(always)]
    pub fn trpsw(&self) -> TRPSW_R {
        TRPSW_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - External Modulation Synchronization"]
    #[inline(always)]
    pub fn ems(&self) -> EMS_R {
        EMS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - External Modulation Type"]
    #[inline(always)]
    pub fn emt(&self) -> EMT_R {
        EMT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Multi Channel Mode Enable for Channel 1"]
    #[inline(always)]
    pub fn mcme1(&self) -> MCME1_R {
        MCME1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Multi Channel Mode Enable for Channel 2"]
    #[inline(always)]
    pub fn mcme2(&self) -> MCME2_R {
        MCME2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - External Modulation Channel enable"]
    #[inline(always)]
    pub fn eme(&self) -> EME_R {
        EME_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:30 - Status bit output selector"]
    #[inline(always)]
    pub fn stos(&self) -> STOS_R {
        STOS_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Timer Counting Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tcm(&mut self) -> TCM_W<0> {
        TCM_W::new(self)
    }
    #[doc = "Bit 1 - Timer Single Shot Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tssm(&mut self) -> TSSM_W<1> {
        TSSM_W::new(self)
    }
    #[doc = "Bit 2 - Shadow Transfer on Clear"]
    #[inline(always)]
    #[must_use]
    pub fn clst(&mut self) -> CLST_W<2> {
        CLST_W::new(self)
    }
    #[doc = "Bit 4 - Extended Capture Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ecm(&mut self) -> ECM_W<4> {
        ECM_W::new(self)
    }
    #[doc = "Bits 5:6 - Clear on Capture Control"]
    #[inline(always)]
    #[must_use]
    pub fn capc(&mut self) -> CAPC_W<5> {
        CAPC_W::new(self)
    }
    #[doc = "Bit 7 - Timer Load selector"]
    #[inline(always)]
    #[must_use]
    pub fn tls(&mut self) -> TLS_W<7> {
        TLS_W::new(self)
    }
    #[doc = "Bits 8:9 - Extended Stop Function Control"]
    #[inline(always)]
    #[must_use]
    pub fn endm(&mut self) -> ENDM_W<8> {
        ENDM_W::new(self)
    }
    #[doc = "Bit 10 - Extended Start Function Control"]
    #[inline(always)]
    #[must_use]
    pub fn strm(&mut self) -> STRM_W<10> {
        STRM_W::new(self)
    }
    #[doc = "Bit 11 - Equal Capture Event enable"]
    #[inline(always)]
    #[must_use]
    pub fn sce(&mut self) -> SCE_W<11> {
        SCE_W::new(self)
    }
    #[doc = "Bit 12 - Continuous Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccs(&mut self) -> CCS_W<12> {
        CCS_W::new(self)
    }
    #[doc = "Bits 13:14 - Dither Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dithe(&mut self) -> DITHE_W<13> {
        DITHE_W::new(self)
    }
    #[doc = "Bit 15 - Dither input selector"]
    #[inline(always)]
    #[must_use]
    pub fn dim(&mut self) -> DIM_W<15> {
        DIM_W::new(self)
    }
    #[doc = "Bit 16 - Floating Prescaler enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpe(&mut self) -> FPE_W<16> {
        FPE_W::new(self)
    }
    #[doc = "Bit 17 - TRAP enable for CCU8x.OUTy0"]
    #[inline(always)]
    #[must_use]
    pub fn trape0(&mut self) -> TRAPE0_W<17> {
        TRAPE0_W::new(self)
    }
    #[doc = "Bit 18 - TRAP enable for CCU8x.OUTy1"]
    #[inline(always)]
    #[must_use]
    pub fn trape1(&mut self) -> TRAPE1_W<18> {
        TRAPE1_W::new(self)
    }
    #[doc = "Bit 19 - TRAP enable for CCU8x.OUTy2"]
    #[inline(always)]
    #[must_use]
    pub fn trape2(&mut self) -> TRAPE2_W<19> {
        TRAPE2_W::new(self)
    }
    #[doc = "Bit 20 - TRAP enable for CCU8x.OUTy3"]
    #[inline(always)]
    #[must_use]
    pub fn trape3(&mut self) -> TRAPE3_W<20> {
        TRAPE3_W::new(self)
    }
    #[doc = "Bit 21 - TRAP Synchronization Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trpse(&mut self) -> TRPSE_W<21> {
        TRPSE_W::new(self)
    }
    #[doc = "Bit 22 - TRAP State Clear Control"]
    #[inline(always)]
    #[must_use]
    pub fn trpsw(&mut self) -> TRPSW_W<22> {
        TRPSW_W::new(self)
    }
    #[doc = "Bit 23 - External Modulation Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn ems(&mut self) -> EMS_W<23> {
        EMS_W::new(self)
    }
    #[doc = "Bit 24 - External Modulation Type"]
    #[inline(always)]
    #[must_use]
    pub fn emt(&mut self) -> EMT_W<24> {
        EMT_W::new(self)
    }
    #[doc = "Bit 25 - Multi Channel Mode Enable for Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn mcme1(&mut self) -> MCME1_W<25> {
        MCME1_W::new(self)
    }
    #[doc = "Bit 26 - Multi Channel Mode Enable for Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn mcme2(&mut self) -> MCME2_W<26> {
        MCME2_W::new(self)
    }
    #[doc = "Bits 27:28 - External Modulation Channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn eme(&mut self) -> EME_W<27> {
        EME_W::new(self)
    }
    #[doc = "Bits 29:30 - Status bit output selector"]
    #[inline(always)]
    #[must_use]
    pub fn stos(&mut self) -> STOS_W<29> {
        STOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slice Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc](index.html) module"]
pub struct TC_SPEC;
impl crate::RegisterSpec for TC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tc::R](R) reader structure"]
impl crate::Readable for TC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tc::W](W) writer structure"]
impl crate::Writable for TC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TC to value 0x1800_0000"]
impl crate::Resettable for TC_SPEC {
    const RESET_VALUE: Self::Ux = 0x1800_0000;
}
