#[doc = "Register `TC` reader"]
pub type R = crate::R<TC_SPEC>;
#[doc = "Register `TC` writer"]
pub type W = crate::W<TC_SPEC>;
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
#[doc = "Field `TCM` reader - Timer Counting Mode"]
pub type TCM_R = crate::BitReader<TCM_A>;
impl TCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCM_A {
        match self.bits {
            false => TCM_A::VALUE1,
            true => TCM_A::VALUE2,
        }
    }
    #[doc = "Edge aligned mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TCM_A::VALUE1
    }
    #[doc = "Center aligned mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TCM_A::VALUE2
    }
}
#[doc = "Field `TCM` writer - Timer Counting Mode"]
pub type TCM_W<'a, REG> = crate::BitWriter<'a, REG, TCM_A>;
impl<'a, REG> TCM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Edge aligned mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TCM_A::VALUE1)
    }
    #[doc = "Center aligned mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TCM_A::VALUE2)
    }
}
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
#[doc = "Field `TSSM` reader - Timer Single Shot Mode"]
pub type TSSM_R = crate::BitReader<TSSM_A>;
impl TSSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSSM_A {
        match self.bits {
            false => TSSM_A::VALUE1,
            true => TSSM_A::VALUE2,
        }
    }
    #[doc = "Single shot mode is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSSM_A::VALUE1
    }
    #[doc = "Single shot mode is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSSM_A::VALUE2
    }
}
#[doc = "Field `TSSM` writer - Timer Single Shot Mode"]
pub type TSSM_W<'a, REG> = crate::BitWriter<'a, REG, TSSM_A>;
impl<'a, REG> TSSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single shot mode is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TSSM_A::VALUE1)
    }
    #[doc = "Single shot mode is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TSSM_A::VALUE2)
    }
}
#[doc = "Field `CLST` reader - Shadow Transfer on Clear"]
pub type CLST_R = crate::BitReader;
#[doc = "Field `CLST` writer - Shadow Transfer on Clear"]
pub type CLST_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `CMOD` reader - Capture Compare Mode"]
pub type CMOD_R = crate::BitReader<CMOD_A>;
impl CMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMOD_A {
        match self.bits {
            false => CMOD_A::VALUE1,
            true => CMOD_A::VALUE2,
        }
    }
    #[doc = "Compare Mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMOD_A::VALUE1
    }
    #[doc = "Capture Mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMOD_A::VALUE2
    }
}
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
#[doc = "Field `ECM` reader - Extended Capture Mode"]
pub type ECM_R = crate::BitReader<ECM_A>;
impl ECM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECM_A {
        match self.bits {
            false => ECM_A::VALUE1,
            true => ECM_A::VALUE2,
        }
    }
    #[doc = "Normal Capture Mode. Clear of the Full Flag of each capture register is done by accessing the registers individually only."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ECM_A::VALUE1
    }
    #[doc = "Extended Capture Mode. Clear of the Full Flag of each capture register is done not only by accessing the individual registers but also by accessing the ECRD register. When reading the ECRD register, only the capture register register full flag pointed by the VPTR is cleared"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ECM_A::VALUE2
    }
}
#[doc = "Field `ECM` writer - Extended Capture Mode"]
pub type ECM_W<'a, REG> = crate::BitWriter<'a, REG, ECM_A>;
impl<'a, REG> ECM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Capture Mode. Clear of the Full Flag of each capture register is done by accessing the registers individually only."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ECM_A::VALUE1)
    }
    #[doc = "Extended Capture Mode. Clear of the Full Flag of each capture register is done not only by accessing the individual registers but also by accessing the ECRD register. When reading the ECRD register, only the capture register register full flag pointed by the VPTR is cleared"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ECM_A::VALUE2)
    }
}
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
impl crate::FieldSpec for CAPC_A {
    type Ux = u8;
}
impl crate::IsEnum for CAPC_A {}
#[doc = "Field `CAPC` reader - Clear on Capture Control"]
pub type CAPC_R = crate::FieldReader<CAPC_A>;
impl CAPC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAPC_A {
        match self.bits {
            0 => CAPC_A::VALUE1,
            1 => CAPC_A::VALUE2,
            2 => CAPC_A::VALUE3,
            3 => CAPC_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer is never cleared on a capture event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CAPC_A::VALUE1
    }
    #[doc = "Timer is cleared on a capture event into capture registers 2 and 3. (When SCE = 1#, Timer is always cleared in a capture event)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CAPC_A::VALUE2
    }
    #[doc = "Timer is cleared on a capture event into capture registers 0 and 1. (When SCE = 1#, Timer is always cleared in a capture event)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CAPC_A::VALUE3
    }
    #[doc = "Timer is always cleared in a capture event."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CAPC_A::VALUE4
    }
}
#[doc = "Field `CAPC` writer - Clear on Capture Control"]
pub type CAPC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CAPC_A, crate::Safe>;
impl<'a, REG> CAPC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer is never cleared on a capture event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CAPC_A::VALUE1)
    }
    #[doc = "Timer is cleared on a capture event into capture registers 2 and 3. (When SCE = 1#, Timer is always cleared in a capture event)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CAPC_A::VALUE2)
    }
    #[doc = "Timer is cleared on a capture event into capture registers 0 and 1. (When SCE = 1#, Timer is always cleared in a capture event)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CAPC_A::VALUE3)
    }
    #[doc = "Timer is always cleared in a capture event."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CAPC_A::VALUE4)
    }
}
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
#[doc = "Field `TLS` reader - Timer Load selector"]
pub type TLS_R = crate::BitReader<TLS_A>;
impl TLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TLS_A {
        match self.bits {
            false => TLS_A::VALUE1,
            true => TLS_A::VALUE2,
        }
    }
    #[doc = "Timer is loaded with the value of CR1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TLS_A::VALUE1
    }
    #[doc = "Timer is loaded with the value of CR2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TLS_A::VALUE2
    }
}
#[doc = "Field `TLS` writer - Timer Load selector"]
pub type TLS_W<'a, REG> = crate::BitWriter<'a, REG, TLS_A>;
impl<'a, REG> TLS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer is loaded with the value of CR1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TLS_A::VALUE1)
    }
    #[doc = "Timer is loaded with the value of CR2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TLS_A::VALUE2)
    }
}
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
impl crate::FieldSpec for ENDM_A {
    type Ux = u8;
}
impl crate::IsEnum for ENDM_A {}
#[doc = "Field `ENDM` reader - Extended Stop Function Control"]
pub type ENDM_R = crate::FieldReader<ENDM_A>;
impl ENDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ENDM_A> {
        match self.bits {
            0 => Some(ENDM_A::VALUE1),
            1 => Some(ENDM_A::VALUE2),
            2 => Some(ENDM_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Clears the timer run bit only (default stop)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENDM_A::VALUE1
    }
    #[doc = "Clears the timer only (flush)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENDM_A::VALUE2
    }
    #[doc = "Clears the timer and run bit (flush/stop)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ENDM_A::VALUE3
    }
}
#[doc = "Field `ENDM` writer - Extended Stop Function Control"]
pub type ENDM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ENDM_A>;
impl<'a, REG> ENDM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clears the timer run bit only (default stop)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ENDM_A::VALUE1)
    }
    #[doc = "Clears the timer only (flush)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ENDM_A::VALUE2)
    }
    #[doc = "Clears the timer and run bit (flush/stop)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(ENDM_A::VALUE3)
    }
}
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
#[doc = "Field `STRM` reader - Extended Start Function Control"]
pub type STRM_R = crate::BitReader<STRM_A>;
impl STRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STRM_A {
        match self.bits {
            false => STRM_A::VALUE1,
            true => STRM_A::VALUE2,
        }
    }
    #[doc = "Sets run bit only (default start)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STRM_A::VALUE1
    }
    #[doc = "Clears the timer and sets run bit, if not set (flush/start)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STRM_A::VALUE2
    }
}
#[doc = "Field `STRM` writer - Extended Start Function Control"]
pub type STRM_W<'a, REG> = crate::BitWriter<'a, REG, STRM_A>;
impl<'a, REG> STRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sets run bit only (default start)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(STRM_A::VALUE1)
    }
    #[doc = "Clears the timer and sets run bit, if not set (flush/start)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(STRM_A::VALUE2)
    }
}
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
#[doc = "Field `SCE` reader - Equal Capture Event enable"]
pub type SCE_R = crate::BitReader<SCE_A>;
impl SCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCE_A {
        match self.bits {
            false => SCE_A::VALUE1,
            true => SCE_A::VALUE2,
        }
    }
    #[doc = "Capture into CC8yC0VThis register contains the values associated with the Capture 0 field./CC8yC1VThis register contains the values associated with the Capture 1 field. registers control by CCycapt0 and capture into CC8yC3VThis register contains the values associated with the Capture 3 field./CC8yC2VThis register contains the values associated with the Capture 2 field. control by CCycapt1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SCE_A::VALUE1
    }
    #[doc = "Capture into CC8yC0VThis register contains the values associated with the Capture 0 field./CC8yC1VThis register contains the values associated with the Capture 1 field. and CC8yC3VThis register contains the values associated with the Capture 3 field./CC8yC2VThis register contains the values associated with the Capture 2 field. control by CCycapt1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SCE_A::VALUE2
    }
}
#[doc = "Field `SCE` writer - Equal Capture Event enable"]
pub type SCE_W<'a, REG> = crate::BitWriter<'a, REG, SCE_A>;
impl<'a, REG> SCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture into CC8yC0VThis register contains the values associated with the Capture 0 field./CC8yC1VThis register contains the values associated with the Capture 1 field. registers control by CCycapt0 and capture into CC8yC3VThis register contains the values associated with the Capture 3 field./CC8yC2VThis register contains the values associated with the Capture 2 field. control by CCycapt1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SCE_A::VALUE1)
    }
    #[doc = "Capture into CC8yC0VThis register contains the values associated with the Capture 0 field./CC8yC1VThis register contains the values associated with the Capture 1 field. and CC8yC3VThis register contains the values associated with the Capture 3 field./CC8yC2VThis register contains the values associated with the Capture 2 field. control by CCycapt1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SCE_A::VALUE2)
    }
}
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
#[doc = "Field `CCS` reader - Continuous Capture Enable"]
pub type CCS_R = crate::BitReader<CCS_A>;
impl CCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCS_A {
        match self.bits {
            false => CCS_A::VALUE1,
            true => CCS_A::VALUE2,
        }
    }
    #[doc = "The capture into a specific capture register is done with the rules linked with the full flags, described at ."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCS_A::VALUE1
    }
    #[doc = "The capture into the capture registers is always done regardless of the full flag status (even if the register has not been read back)."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCS_A::VALUE2
    }
}
#[doc = "Field `CCS` writer - Continuous Capture Enable"]
pub type CCS_W<'a, REG> = crate::BitWriter<'a, REG, CCS_A>;
impl<'a, REG> CCS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The capture into a specific capture register is done with the rules linked with the full flags, described at ."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CCS_A::VALUE1)
    }
    #[doc = "The capture into the capture registers is always done regardless of the full flag status (even if the register has not been read back)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CCS_A::VALUE2)
    }
}
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
impl crate::FieldSpec for DITHE_A {
    type Ux = u8;
}
impl crate::IsEnum for DITHE_A {}
#[doc = "Field `DITHE` reader - Dither Enable"]
pub type DITHE_R = crate::FieldReader<DITHE_A>;
impl DITHE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DITHE_A {
        match self.bits {
            0 => DITHE_A::VALUE1,
            1 => DITHE_A::VALUE2,
            2 => DITHE_A::VALUE3,
            3 => DITHE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Dither is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DITHE_A::VALUE1
    }
    #[doc = "Dither is applied to the Period"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DITHE_A::VALUE2
    }
    #[doc = "Dither is applied to the Compare"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DITHE_A::VALUE3
    }
    #[doc = "Dither is applied to the Period and Compare"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DITHE_A::VALUE4
    }
}
#[doc = "Field `DITHE` writer - Dither Enable"]
pub type DITHE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DITHE_A, crate::Safe>;
impl<'a, REG> DITHE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Dither is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DITHE_A::VALUE1)
    }
    #[doc = "Dither is applied to the Period"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DITHE_A::VALUE2)
    }
    #[doc = "Dither is applied to the Compare"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(DITHE_A::VALUE3)
    }
    #[doc = "Dither is applied to the Period and Compare"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(DITHE_A::VALUE4)
    }
}
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
#[doc = "Field `DIM` reader - Dither input selector"]
pub type DIM_R = crate::BitReader<DIM_A>;
impl DIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIM_A {
        match self.bits {
            false => DIM_A::VALUE1,
            true => DIM_A::VALUE2,
        }
    }
    #[doc = "Slice is using it own dither unit"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIM_A::VALUE1
    }
    #[doc = "Slice is connected to the dither unit of slice 0."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIM_A::VALUE2
    }
}
#[doc = "Field `DIM` writer - Dither input selector"]
pub type DIM_W<'a, REG> = crate::BitWriter<'a, REG, DIM_A>;
impl<'a, REG> DIM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slice is using it own dither unit"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DIM_A::VALUE1)
    }
    #[doc = "Slice is connected to the dither unit of slice 0."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DIM_A::VALUE2)
    }
}
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
#[doc = "Field `FPE` reader - Floating Prescaler enable"]
pub type FPE_R = crate::BitReader<FPE_A>;
impl FPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPE_A {
        match self.bits {
            false => FPE_A::VALUE1,
            true => FPE_A::VALUE2,
        }
    }
    #[doc = "Floating prescaler mode is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FPE_A::VALUE1
    }
    #[doc = "Floating prescaler mode is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FPE_A::VALUE2
    }
}
#[doc = "Field `FPE` writer - Floating Prescaler enable"]
pub type FPE_W<'a, REG> = crate::BitWriter<'a, REG, FPE_A>;
impl<'a, REG> FPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Floating prescaler mode is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FPE_A::VALUE1)
    }
    #[doc = "Floating prescaler mode is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FPE_A::VALUE2)
    }
}
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
#[doc = "Field `TRAPE0` reader - TRAP enable for CCU8x.OUTy0"]
pub type TRAPE0_R = crate::BitReader<TRAPE0_A>;
impl TRAPE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRAPE0_A {
        match self.bits {
            false => TRAPE0_A::VALUE1,
            true => TRAPE0_A::VALUE2,
        }
    }
    #[doc = "TRAP functionality has no effect on the CCU8x.OUTy0 output"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRAPE0_A::VALUE1
    }
    #[doc = "TRAP functionality affects the CCU8x.OUTy0 output"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRAPE0_A::VALUE2
    }
}
#[doc = "Field `TRAPE0` writer - TRAP enable for CCU8x.OUTy0"]
pub type TRAPE0_W<'a, REG> = crate::BitWriter<'a, REG, TRAPE0_A>;
impl<'a, REG> TRAPE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TRAP functionality has no effect on the CCU8x.OUTy0 output"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TRAPE0_A::VALUE1)
    }
    #[doc = "TRAP functionality affects the CCU8x.OUTy0 output"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TRAPE0_A::VALUE2)
    }
}
#[doc = "Field `TRAPE1` reader - TRAP enable for CCU8x.OUTy1"]
pub type TRAPE1_R = crate::BitReader;
#[doc = "Field `TRAPE1` writer - TRAP enable for CCU8x.OUTy1"]
pub type TRAPE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRAPE2` reader - TRAP enable for CCU8x.OUTy2"]
pub type TRAPE2_R = crate::BitReader;
#[doc = "Field `TRAPE2` writer - TRAP enable for CCU8x.OUTy2"]
pub type TRAPE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRAPE3` reader - TRAP enable for CCU8x.OUTy3"]
pub type TRAPE3_R = crate::BitReader;
#[doc = "Field `TRAPE3` writer - TRAP enable for CCU8x.OUTy3"]
pub type TRAPE3_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `TRPSE` reader - TRAP Synchronization Enable"]
pub type TRPSE_R = crate::BitReader<TRPSE_A>;
impl TRPSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRPSE_A {
        match self.bits {
            false => TRPSE_A::VALUE1,
            true => TRPSE_A::VALUE2,
        }
    }
    #[doc = "Exiting from TRAP state isn't synchronized with the PWM signal"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRPSE_A::VALUE1
    }
    #[doc = "Exiting from TRAP state is synchronized with the PWM signal"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRPSE_A::VALUE2
    }
}
#[doc = "Field `TRPSE` writer - TRAP Synchronization Enable"]
pub type TRPSE_W<'a, REG> = crate::BitWriter<'a, REG, TRPSE_A>;
impl<'a, REG> TRPSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exiting from TRAP state isn't synchronized with the PWM signal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TRPSE_A::VALUE1)
    }
    #[doc = "Exiting from TRAP state is synchronized with the PWM signal"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TRPSE_A::VALUE2)
    }
}
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
#[doc = "Field `TRPSW` reader - TRAP State Clear Control"]
pub type TRPSW_R = crate::BitReader<TRPSW_A>;
impl TRPSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRPSW_A {
        match self.bits {
            false => TRPSW_A::VALUE1,
            true => TRPSW_A::VALUE2,
        }
    }
    #[doc = "The slice exits the TRAP state automatically when the TRAP condition is not present (Trap state cleared by HW and SW)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRPSW_A::VALUE1
    }
    #[doc = "The TRAP state can only be exited by a SW request."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRPSW_A::VALUE2
    }
}
#[doc = "Field `TRPSW` writer - TRAP State Clear Control"]
pub type TRPSW_W<'a, REG> = crate::BitWriter<'a, REG, TRPSW_A>;
impl<'a, REG> TRPSW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The slice exits the TRAP state automatically when the TRAP condition is not present (Trap state cleared by HW and SW)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TRPSW_A::VALUE1)
    }
    #[doc = "The TRAP state can only be exited by a SW request."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TRPSW_A::VALUE2)
    }
}
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
#[doc = "Field `EMS` reader - External Modulation Synchronization"]
pub type EMS_R = crate::BitReader<EMS_A>;
impl EMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EMS_A {
        match self.bits {
            false => EMS_A::VALUE1,
            true => EMS_A::VALUE2,
        }
    }
    #[doc = "External Modulation functionality is not synchronized with the PWM signal"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EMS_A::VALUE1
    }
    #[doc = "External Modulation functionality is synchronized with the PWM signal"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EMS_A::VALUE2
    }
}
#[doc = "Field `EMS` writer - External Modulation Synchronization"]
pub type EMS_W<'a, REG> = crate::BitWriter<'a, REG, EMS_A>;
impl<'a, REG> EMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External Modulation functionality is not synchronized with the PWM signal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EMS_A::VALUE1)
    }
    #[doc = "External Modulation functionality is synchronized with the PWM signal"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EMS_A::VALUE2)
    }
}
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
#[doc = "Field `EMT` reader - External Modulation Type"]
pub type EMT_R = crate::BitReader<EMT_A>;
impl EMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EMT_A {
        match self.bits {
            false => EMT_A::VALUE1,
            true => EMT_A::VALUE2,
        }
    }
    #[doc = "External Modulation functionality is clearing the CC8ySTx bits."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EMT_A::VALUE1
    }
    #[doc = "External Modulation functionality is gating the outputs."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EMT_A::VALUE2
    }
}
#[doc = "Field `EMT` writer - External Modulation Type"]
pub type EMT_W<'a, REG> = crate::BitWriter<'a, REG, EMT_A>;
impl<'a, REG> EMT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External Modulation functionality is clearing the CC8ySTx bits."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EMT_A::VALUE1)
    }
    #[doc = "External Modulation functionality is gating the outputs."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EMT_A::VALUE2)
    }
}
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
#[doc = "Field `MCME1` reader - Multi Channel Mode Enable for Channel 1"]
pub type MCME1_R = crate::BitReader<MCME1_A>;
impl MCME1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCME1_A {
        match self.bits {
            false => MCME1_A::VALUE1,
            true => MCME1_A::VALUE2,
        }
    }
    #[doc = "Multi Channel Mode in Channel 1 is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MCME1_A::VALUE1
    }
    #[doc = "Multi Channel Mode in Channel 1 is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MCME1_A::VALUE2
    }
}
#[doc = "Field `MCME1` writer - Multi Channel Mode Enable for Channel 1"]
pub type MCME1_W<'a, REG> = crate::BitWriter<'a, REG, MCME1_A>;
impl<'a, REG> MCME1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multi Channel Mode in Channel 1 is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MCME1_A::VALUE1)
    }
    #[doc = "Multi Channel Mode in Channel 1 is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MCME1_A::VALUE2)
    }
}
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
#[doc = "Field `MCME2` reader - Multi Channel Mode Enable for Channel 2"]
pub type MCME2_R = crate::BitReader<MCME2_A>;
impl MCME2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCME2_A {
        match self.bits {
            false => MCME2_A::VALUE1,
            true => MCME2_A::VALUE2,
        }
    }
    #[doc = "Multi Channel Mode in Channel 2 is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MCME2_A::VALUE1
    }
    #[doc = "Multi Channel Mode in Channel 2 is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MCME2_A::VALUE2
    }
}
#[doc = "Field `MCME2` writer - Multi Channel Mode Enable for Channel 2"]
pub type MCME2_W<'a, REG> = crate::BitWriter<'a, REG, MCME2_A>;
impl<'a, REG> MCME2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multi Channel Mode in Channel 2 is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MCME2_A::VALUE1)
    }
    #[doc = "Multi Channel Mode in Channel 2 is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MCME2_A::VALUE2)
    }
}
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
impl crate::FieldSpec for EME_A {
    type Ux = u8;
}
impl crate::IsEnum for EME_A {}
#[doc = "Field `EME` reader - External Modulation Channel enable"]
pub type EME_R = crate::FieldReader<EME_A>;
impl EME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EME_A {
        match self.bits {
            0 => EME_A::VALUE1,
            1 => EME_A::VALUE2,
            2 => EME_A::VALUE3,
            3 => EME_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "External Modulation functionality doesn't affect any channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EME_A::VALUE1
    }
    #[doc = "External Modulation only applied on channel 1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EME_A::VALUE2
    }
    #[doc = "External Modulation only applied on channel 2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EME_A::VALUE3
    }
    #[doc = "External Modulation applied on both channels"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EME_A::VALUE4
    }
}
#[doc = "Field `EME` writer - External Modulation Channel enable"]
pub type EME_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EME_A, crate::Safe>;
impl<'a, REG> EME_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External Modulation functionality doesn't affect any channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EME_A::VALUE1)
    }
    #[doc = "External Modulation only applied on channel 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EME_A::VALUE2)
    }
    #[doc = "External Modulation only applied on channel 2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(EME_A::VALUE3)
    }
    #[doc = "External Modulation applied on both channels"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(EME_A::VALUE4)
    }
}
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
impl crate::FieldSpec for STOS_A {
    type Ux = u8;
}
impl crate::IsEnum for STOS_A {}
#[doc = "Field `STOS` reader - Status bit output selector"]
pub type STOS_R = crate::FieldReader<STOS_A>;
impl STOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOS_A {
        match self.bits {
            0 => STOS_A::VALUE1,
            1 => STOS_A::VALUE2,
            2 => STOS_A::VALUE3,
            3 => STOS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "CC8yST1 forward to CCU8x.STy"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STOS_A::VALUE1
    }
    #[doc = "CC8yST2 forward to CCU8x.STy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STOS_A::VALUE2
    }
    #[doc = "CC8yST1 AND CC8yST2 forward to CCU8x.STy"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == STOS_A::VALUE3
    }
    #[doc = "CC8yST1 OR CC8yST2 forward to CCU8x.STy"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == STOS_A::VALUE4
    }
}
#[doc = "Field `STOS` writer - Status bit output selector"]
pub type STOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STOS_A, crate::Safe>;
impl<'a, REG> STOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC8yST1 forward to CCU8x.STy"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(STOS_A::VALUE1)
    }
    #[doc = "CC8yST2 forward to CCU8x.STy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(STOS_A::VALUE2)
    }
    #[doc = "CC8yST1 AND CC8yST2 forward to CCU8x.STy"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(STOS_A::VALUE3)
    }
    #[doc = "CC8yST1 OR CC8yST2 forward to CCU8x.STy"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
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
    pub fn tcm(&mut self) -> TCM_W<TC_SPEC> {
        TCM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer Single Shot Mode"]
    #[inline(always)]
    pub fn tssm(&mut self) -> TSSM_W<TC_SPEC> {
        TSSM_W::new(self, 1)
    }
    #[doc = "Bit 2 - Shadow Transfer on Clear"]
    #[inline(always)]
    pub fn clst(&mut self) -> CLST_W<TC_SPEC> {
        CLST_W::new(self, 2)
    }
    #[doc = "Bit 4 - Extended Capture Mode"]
    #[inline(always)]
    pub fn ecm(&mut self) -> ECM_W<TC_SPEC> {
        ECM_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - Clear on Capture Control"]
    #[inline(always)]
    pub fn capc(&mut self) -> CAPC_W<TC_SPEC> {
        CAPC_W::new(self, 5)
    }
    #[doc = "Bit 7 - Timer Load selector"]
    #[inline(always)]
    pub fn tls(&mut self) -> TLS_W<TC_SPEC> {
        TLS_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Extended Stop Function Control"]
    #[inline(always)]
    pub fn endm(&mut self) -> ENDM_W<TC_SPEC> {
        ENDM_W::new(self, 8)
    }
    #[doc = "Bit 10 - Extended Start Function Control"]
    #[inline(always)]
    pub fn strm(&mut self) -> STRM_W<TC_SPEC> {
        STRM_W::new(self, 10)
    }
    #[doc = "Bit 11 - Equal Capture Event enable"]
    #[inline(always)]
    pub fn sce(&mut self) -> SCE_W<TC_SPEC> {
        SCE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Continuous Capture Enable"]
    #[inline(always)]
    pub fn ccs(&mut self) -> CCS_W<TC_SPEC> {
        CCS_W::new(self, 12)
    }
    #[doc = "Bits 13:14 - Dither Enable"]
    #[inline(always)]
    pub fn dithe(&mut self) -> DITHE_W<TC_SPEC> {
        DITHE_W::new(self, 13)
    }
    #[doc = "Bit 15 - Dither input selector"]
    #[inline(always)]
    pub fn dim(&mut self) -> DIM_W<TC_SPEC> {
        DIM_W::new(self, 15)
    }
    #[doc = "Bit 16 - Floating Prescaler enable"]
    #[inline(always)]
    pub fn fpe(&mut self) -> FPE_W<TC_SPEC> {
        FPE_W::new(self, 16)
    }
    #[doc = "Bit 17 - TRAP enable for CCU8x.OUTy0"]
    #[inline(always)]
    pub fn trape0(&mut self) -> TRAPE0_W<TC_SPEC> {
        TRAPE0_W::new(self, 17)
    }
    #[doc = "Bit 18 - TRAP enable for CCU8x.OUTy1"]
    #[inline(always)]
    pub fn trape1(&mut self) -> TRAPE1_W<TC_SPEC> {
        TRAPE1_W::new(self, 18)
    }
    #[doc = "Bit 19 - TRAP enable for CCU8x.OUTy2"]
    #[inline(always)]
    pub fn trape2(&mut self) -> TRAPE2_W<TC_SPEC> {
        TRAPE2_W::new(self, 19)
    }
    #[doc = "Bit 20 - TRAP enable for CCU8x.OUTy3"]
    #[inline(always)]
    pub fn trape3(&mut self) -> TRAPE3_W<TC_SPEC> {
        TRAPE3_W::new(self, 20)
    }
    #[doc = "Bit 21 - TRAP Synchronization Enable"]
    #[inline(always)]
    pub fn trpse(&mut self) -> TRPSE_W<TC_SPEC> {
        TRPSE_W::new(self, 21)
    }
    #[doc = "Bit 22 - TRAP State Clear Control"]
    #[inline(always)]
    pub fn trpsw(&mut self) -> TRPSW_W<TC_SPEC> {
        TRPSW_W::new(self, 22)
    }
    #[doc = "Bit 23 - External Modulation Synchronization"]
    #[inline(always)]
    pub fn ems(&mut self) -> EMS_W<TC_SPEC> {
        EMS_W::new(self, 23)
    }
    #[doc = "Bit 24 - External Modulation Type"]
    #[inline(always)]
    pub fn emt(&mut self) -> EMT_W<TC_SPEC> {
        EMT_W::new(self, 24)
    }
    #[doc = "Bit 25 - Multi Channel Mode Enable for Channel 1"]
    #[inline(always)]
    pub fn mcme1(&mut self) -> MCME1_W<TC_SPEC> {
        MCME1_W::new(self, 25)
    }
    #[doc = "Bit 26 - Multi Channel Mode Enable for Channel 2"]
    #[inline(always)]
    pub fn mcme2(&mut self) -> MCME2_W<TC_SPEC> {
        MCME2_W::new(self, 26)
    }
    #[doc = "Bits 27:28 - External Modulation Channel enable"]
    #[inline(always)]
    pub fn eme(&mut self) -> EME_W<TC_SPEC> {
        EME_W::new(self, 27)
    }
    #[doc = "Bits 29:30 - Status bit output selector"]
    #[inline(always)]
    pub fn stos(&mut self) -> STOS_W<TC_SPEC> {
        STOS_W::new(self, 29)
    }
}
#[doc = "Slice Timer Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TC_SPEC;
impl crate::RegisterSpec for TC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tc::R`](R) reader structure"]
impl crate::Readable for TC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tc::W`](W) writer structure"]
impl crate::Writable for TC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TC to value 0x1800_0000"]
impl crate::Resettable for TC_SPEC {
    const RESET_VALUE: u32 = 0x1800_0000;
}
