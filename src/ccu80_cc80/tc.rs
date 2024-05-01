#[doc = "Register `TC` reader"]
pub type R = crate::R<TcSpec>;
#[doc = "Register `TC` writer"]
pub type W = crate::W<TcSpec>;
#[doc = "Timer Counting Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcm {
    #[doc = "0: Edge aligned mode"]
    Value1 = 0,
    #[doc = "1: Center aligned mode"]
    Value2 = 1,
}
impl From<Tcm> for bool {
    #[inline(always)]
    fn from(variant: Tcm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCM` reader - Timer Counting Mode"]
pub type TcmR = crate::BitReader<Tcm>;
impl TcmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcm {
        match self.bits {
            false => Tcm::Value1,
            true => Tcm::Value2,
        }
    }
    #[doc = "Edge aligned mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tcm::Value1
    }
    #[doc = "Center aligned mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tcm::Value2
    }
}
#[doc = "Field `TCM` writer - Timer Counting Mode"]
pub type TcmW<'a, REG> = crate::BitWriter<'a, REG, Tcm>;
impl<'a, REG> TcmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Edge aligned mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcm::Value1)
    }
    #[doc = "Center aligned mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tcm::Value2)
    }
}
#[doc = "Timer Single Shot Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tssm {
    #[doc = "0: Single shot mode is disabled"]
    Value1 = 0,
    #[doc = "1: Single shot mode is enabled"]
    Value2 = 1,
}
impl From<Tssm> for bool {
    #[inline(always)]
    fn from(variant: Tssm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSSM` reader - Timer Single Shot Mode"]
pub type TssmR = crate::BitReader<Tssm>;
impl TssmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tssm {
        match self.bits {
            false => Tssm::Value1,
            true => Tssm::Value2,
        }
    }
    #[doc = "Single shot mode is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tssm::Value1
    }
    #[doc = "Single shot mode is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tssm::Value2
    }
}
#[doc = "Field `TSSM` writer - Timer Single Shot Mode"]
pub type TssmW<'a, REG> = crate::BitWriter<'a, REG, Tssm>;
impl<'a, REG> TssmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single shot mode is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tssm::Value1)
    }
    #[doc = "Single shot mode is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tssm::Value2)
    }
}
#[doc = "Field `CLST` reader - Shadow Transfer on Clear"]
pub type ClstR = crate::BitReader;
#[doc = "Field `CLST` writer - Shadow Transfer on Clear"]
pub type ClstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Capture Compare Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmod {
    #[doc = "0: Compare Mode"]
    Value1 = 0,
    #[doc = "1: Capture Mode"]
    Value2 = 1,
}
impl From<Cmod> for bool {
    #[inline(always)]
    fn from(variant: Cmod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMOD` reader - Capture Compare Mode"]
pub type CmodR = crate::BitReader<Cmod>;
impl CmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmod {
        match self.bits {
            false => Cmod::Value1,
            true => Cmod::Value2,
        }
    }
    #[doc = "Compare Mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cmod::Value1
    }
    #[doc = "Capture Mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cmod::Value2
    }
}
#[doc = "Extended Capture Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecm {
    #[doc = "0: Normal Capture Mode. Clear of the Full Flag of each capture register is done by accessing the registers individually only."]
    Value1 = 0,
    #[doc = "1: Extended Capture Mode. Clear of the Full Flag of each capture register is done not only by accessing the individual registers but also by accessing the ECRD register. When reading the ECRD register, only the capture register register full flag pointed by the VPTR is cleared"]
    Value2 = 1,
}
impl From<Ecm> for bool {
    #[inline(always)]
    fn from(variant: Ecm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECM` reader - Extended Capture Mode"]
pub type EcmR = crate::BitReader<Ecm>;
impl EcmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecm {
        match self.bits {
            false => Ecm::Value1,
            true => Ecm::Value2,
        }
    }
    #[doc = "Normal Capture Mode. Clear of the Full Flag of each capture register is done by accessing the registers individually only."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ecm::Value1
    }
    #[doc = "Extended Capture Mode. Clear of the Full Flag of each capture register is done not only by accessing the individual registers but also by accessing the ECRD register. When reading the ECRD register, only the capture register register full flag pointed by the VPTR is cleared"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ecm::Value2
    }
}
#[doc = "Field `ECM` writer - Extended Capture Mode"]
pub type EcmW<'a, REG> = crate::BitWriter<'a, REG, Ecm>;
impl<'a, REG> EcmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Capture Mode. Clear of the Full Flag of each capture register is done by accessing the registers individually only."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ecm::Value1)
    }
    #[doc = "Extended Capture Mode. Clear of the Full Flag of each capture register is done not only by accessing the individual registers but also by accessing the ECRD register. When reading the ECRD register, only the capture register register full flag pointed by the VPTR is cleared"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ecm::Value2)
    }
}
#[doc = "Clear on Capture Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Capc {
    #[doc = "0: Timer is never cleared on a capture event"]
    Value1 = 0,
    #[doc = "1: Timer is cleared on a capture event into capture registers 2 and 3. (When SCE = 1#, Timer is always cleared in a capture event)"]
    Value2 = 1,
    #[doc = "2: Timer is cleared on a capture event into capture registers 0 and 1. (When SCE = 1#, Timer is always cleared in a capture event)"]
    Value3 = 2,
    #[doc = "3: Timer is always cleared in a capture event."]
    Value4 = 3,
}
impl From<Capc> for u8 {
    #[inline(always)]
    fn from(variant: Capc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Capc {
    type Ux = u8;
}
impl crate::IsEnum for Capc {}
#[doc = "Field `CAPC` reader - Clear on Capture Control"]
pub type CapcR = crate::FieldReader<Capc>;
impl CapcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Capc {
        match self.bits {
            0 => Capc::Value1,
            1 => Capc::Value2,
            2 => Capc::Value3,
            3 => Capc::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer is never cleared on a capture event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Capc::Value1
    }
    #[doc = "Timer is cleared on a capture event into capture registers 2 and 3. (When SCE = 1#, Timer is always cleared in a capture event)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Capc::Value2
    }
    #[doc = "Timer is cleared on a capture event into capture registers 0 and 1. (When SCE = 1#, Timer is always cleared in a capture event)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Capc::Value3
    }
    #[doc = "Timer is always cleared in a capture event."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Capc::Value4
    }
}
#[doc = "Field `CAPC` writer - Clear on Capture Control"]
pub type CapcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Capc, crate::Safe>;
impl<'a, REG> CapcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer is never cleared on a capture event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Capc::Value1)
    }
    #[doc = "Timer is cleared on a capture event into capture registers 2 and 3. (When SCE = 1#, Timer is always cleared in a capture event)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Capc::Value2)
    }
    #[doc = "Timer is cleared on a capture event into capture registers 0 and 1. (When SCE = 1#, Timer is always cleared in a capture event)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Capc::Value3)
    }
    #[doc = "Timer is always cleared in a capture event."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Capc::Value4)
    }
}
#[doc = "Timer Load selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tls {
    #[doc = "0: Timer is loaded with the value of CR1"]
    Value1 = 0,
    #[doc = "1: Timer is loaded with the value of CR2"]
    Value2 = 1,
}
impl From<Tls> for bool {
    #[inline(always)]
    fn from(variant: Tls) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TLS` reader - Timer Load selector"]
pub type TlsR = crate::BitReader<Tls>;
impl TlsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tls {
        match self.bits {
            false => Tls::Value1,
            true => Tls::Value2,
        }
    }
    #[doc = "Timer is loaded with the value of CR1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tls::Value1
    }
    #[doc = "Timer is loaded with the value of CR2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tls::Value2
    }
}
#[doc = "Field `TLS` writer - Timer Load selector"]
pub type TlsW<'a, REG> = crate::BitWriter<'a, REG, Tls>;
impl<'a, REG> TlsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer is loaded with the value of CR1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tls::Value1)
    }
    #[doc = "Timer is loaded with the value of CR2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tls::Value2)
    }
}
#[doc = "Extended Stop Function Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Endm {
    #[doc = "0: Clears the timer run bit only (default stop)"]
    Value1 = 0,
    #[doc = "1: Clears the timer only (flush)"]
    Value2 = 1,
    #[doc = "2: Clears the timer and run bit (flush/stop)"]
    Value3 = 2,
}
impl From<Endm> for u8 {
    #[inline(always)]
    fn from(variant: Endm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Endm {
    type Ux = u8;
}
impl crate::IsEnum for Endm {}
#[doc = "Field `ENDM` reader - Extended Stop Function Control"]
pub type EndmR = crate::FieldReader<Endm>;
impl EndmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Endm> {
        match self.bits {
            0 => Some(Endm::Value1),
            1 => Some(Endm::Value2),
            2 => Some(Endm::Value3),
            _ => None,
        }
    }
    #[doc = "Clears the timer run bit only (default stop)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Endm::Value1
    }
    #[doc = "Clears the timer only (flush)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Endm::Value2
    }
    #[doc = "Clears the timer and run bit (flush/stop)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Endm::Value3
    }
}
#[doc = "Field `ENDM` writer - Extended Stop Function Control"]
pub type EndmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Endm>;
impl<'a, REG> EndmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clears the timer run bit only (default stop)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Endm::Value1)
    }
    #[doc = "Clears the timer only (flush)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Endm::Value2)
    }
    #[doc = "Clears the timer and run bit (flush/stop)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Endm::Value3)
    }
}
#[doc = "Extended Start Function Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Strm {
    #[doc = "0: Sets run bit only (default start)"]
    Value1 = 0,
    #[doc = "1: Clears the timer and sets run bit, if not set (flush/start)"]
    Value2 = 1,
}
impl From<Strm> for bool {
    #[inline(always)]
    fn from(variant: Strm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRM` reader - Extended Start Function Control"]
pub type StrmR = crate::BitReader<Strm>;
impl StrmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Strm {
        match self.bits {
            false => Strm::Value1,
            true => Strm::Value2,
        }
    }
    #[doc = "Sets run bit only (default start)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Strm::Value1
    }
    #[doc = "Clears the timer and sets run bit, if not set (flush/start)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Strm::Value2
    }
}
#[doc = "Field `STRM` writer - Extended Start Function Control"]
pub type StrmW<'a, REG> = crate::BitWriter<'a, REG, Strm>;
impl<'a, REG> StrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sets run bit only (default start)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Strm::Value1)
    }
    #[doc = "Clears the timer and sets run bit, if not set (flush/start)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Strm::Value2)
    }
}
#[doc = "Equal Capture Event enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sce {
    #[doc = "0: Capture into CC8yC0VThis register contains the values associated with the Capture 0 field./CC8yC1VThis register contains the values associated with the Capture 1 field. registers control by CCycapt0 and capture into CC8yC3VThis register contains the values associated with the Capture 3 field./CC8yC2VThis register contains the values associated with the Capture 2 field. control by CCycapt1"]
    Value1 = 0,
    #[doc = "1: Capture into CC8yC0VThis register contains the values associated with the Capture 0 field./CC8yC1VThis register contains the values associated with the Capture 1 field. and CC8yC3VThis register contains the values associated with the Capture 3 field./CC8yC2VThis register contains the values associated with the Capture 2 field. control by CCycapt1"]
    Value2 = 1,
}
impl From<Sce> for bool {
    #[inline(always)]
    fn from(variant: Sce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCE` reader - Equal Capture Event enable"]
pub type SceR = crate::BitReader<Sce>;
impl SceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sce {
        match self.bits {
            false => Sce::Value1,
            true => Sce::Value2,
        }
    }
    #[doc = "Capture into CC8yC0VThis register contains the values associated with the Capture 0 field./CC8yC1VThis register contains the values associated with the Capture 1 field. registers control by CCycapt0 and capture into CC8yC3VThis register contains the values associated with the Capture 3 field./CC8yC2VThis register contains the values associated with the Capture 2 field. control by CCycapt1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sce::Value1
    }
    #[doc = "Capture into CC8yC0VThis register contains the values associated with the Capture 0 field./CC8yC1VThis register contains the values associated with the Capture 1 field. and CC8yC3VThis register contains the values associated with the Capture 3 field./CC8yC2VThis register contains the values associated with the Capture 2 field. control by CCycapt1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sce::Value2
    }
}
#[doc = "Field `SCE` writer - Equal Capture Event enable"]
pub type SceW<'a, REG> = crate::BitWriter<'a, REG, Sce>;
impl<'a, REG> SceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture into CC8yC0VThis register contains the values associated with the Capture 0 field./CC8yC1VThis register contains the values associated with the Capture 1 field. registers control by CCycapt0 and capture into CC8yC3VThis register contains the values associated with the Capture 3 field./CC8yC2VThis register contains the values associated with the Capture 2 field. control by CCycapt1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sce::Value1)
    }
    #[doc = "Capture into CC8yC0VThis register contains the values associated with the Capture 0 field./CC8yC1VThis register contains the values associated with the Capture 1 field. and CC8yC3VThis register contains the values associated with the Capture 3 field./CC8yC2VThis register contains the values associated with the Capture 2 field. control by CCycapt1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sce::Value2)
    }
}
#[doc = "Continuous Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccs {
    #[doc = "0: The capture into a specific capture register is done with the rules linked with the full flags, described at ."]
    Value1 = 0,
    #[doc = "1: The capture into the capture registers is always done regardless of the full flag status (even if the register has not been read back)."]
    Value2 = 1,
}
impl From<Ccs> for bool {
    #[inline(always)]
    fn from(variant: Ccs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCS` reader - Continuous Capture Enable"]
pub type CcsR = crate::BitReader<Ccs>;
impl CcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccs {
        match self.bits {
            false => Ccs::Value1,
            true => Ccs::Value2,
        }
    }
    #[doc = "The capture into a specific capture register is done with the rules linked with the full flags, described at ."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ccs::Value1
    }
    #[doc = "The capture into the capture registers is always done regardless of the full flag status (even if the register has not been read back)."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ccs::Value2
    }
}
#[doc = "Field `CCS` writer - Continuous Capture Enable"]
pub type CcsW<'a, REG> = crate::BitWriter<'a, REG, Ccs>;
impl<'a, REG> CcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The capture into a specific capture register is done with the rules linked with the full flags, described at ."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccs::Value1)
    }
    #[doc = "The capture into the capture registers is always done regardless of the full flag status (even if the register has not been read back)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccs::Value2)
    }
}
#[doc = "Dither Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dithe {
    #[doc = "0: Dither is disabled"]
    Value1 = 0,
    #[doc = "1: Dither is applied to the Period"]
    Value2 = 1,
    #[doc = "2: Dither is applied to the Compare"]
    Value3 = 2,
    #[doc = "3: Dither is applied to the Period and Compare"]
    Value4 = 3,
}
impl From<Dithe> for u8 {
    #[inline(always)]
    fn from(variant: Dithe) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dithe {
    type Ux = u8;
}
impl crate::IsEnum for Dithe {}
#[doc = "Field `DITHE` reader - Dither Enable"]
pub type DitheR = crate::FieldReader<Dithe>;
impl DitheR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dithe {
        match self.bits {
            0 => Dithe::Value1,
            1 => Dithe::Value2,
            2 => Dithe::Value3,
            3 => Dithe::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Dither is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dithe::Value1
    }
    #[doc = "Dither is applied to the Period"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dithe::Value2
    }
    #[doc = "Dither is applied to the Compare"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Dithe::Value3
    }
    #[doc = "Dither is applied to the Period and Compare"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Dithe::Value4
    }
}
#[doc = "Field `DITHE` writer - Dither Enable"]
pub type DitheW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dithe, crate::Safe>;
impl<'a, REG> DitheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Dither is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dithe::Value1)
    }
    #[doc = "Dither is applied to the Period"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dithe::Value2)
    }
    #[doc = "Dither is applied to the Compare"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Dithe::Value3)
    }
    #[doc = "Dither is applied to the Period and Compare"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Dithe::Value4)
    }
}
#[doc = "Dither input selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dim {
    #[doc = "0: Slice is using it own dither unit"]
    Value1 = 0,
    #[doc = "1: Slice is connected to the dither unit of slice 0."]
    Value2 = 1,
}
impl From<Dim> for bool {
    #[inline(always)]
    fn from(variant: Dim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIM` reader - Dither input selector"]
pub type DimR = crate::BitReader<Dim>;
impl DimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dim {
        match self.bits {
            false => Dim::Value1,
            true => Dim::Value2,
        }
    }
    #[doc = "Slice is using it own dither unit"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dim::Value1
    }
    #[doc = "Slice is connected to the dither unit of slice 0."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dim::Value2
    }
}
#[doc = "Field `DIM` writer - Dither input selector"]
pub type DimW<'a, REG> = crate::BitWriter<'a, REG, Dim>;
impl<'a, REG> DimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slice is using it own dither unit"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dim::Value1)
    }
    #[doc = "Slice is connected to the dither unit of slice 0."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dim::Value2)
    }
}
#[doc = "Floating Prescaler enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fpe {
    #[doc = "0: Floating prescaler mode is disabled"]
    Value1 = 0,
    #[doc = "1: Floating prescaler mode is enabled"]
    Value2 = 1,
}
impl From<Fpe> for bool {
    #[inline(always)]
    fn from(variant: Fpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPE` reader - Floating Prescaler enable"]
pub type FpeR = crate::BitReader<Fpe>;
impl FpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fpe {
        match self.bits {
            false => Fpe::Value1,
            true => Fpe::Value2,
        }
    }
    #[doc = "Floating prescaler mode is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Fpe::Value1
    }
    #[doc = "Floating prescaler mode is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Fpe::Value2
    }
}
#[doc = "Field `FPE` writer - Floating Prescaler enable"]
pub type FpeW<'a, REG> = crate::BitWriter<'a, REG, Fpe>;
impl<'a, REG> FpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Floating prescaler mode is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Fpe::Value1)
    }
    #[doc = "Floating prescaler mode is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Fpe::Value2)
    }
}
#[doc = "TRAP enable for CCU8x.OUTy0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trape0 {
    #[doc = "0: TRAP functionality has no effect on the CCU8x.OUTy0 output"]
    Value1 = 0,
    #[doc = "1: TRAP functionality affects the CCU8x.OUTy0 output"]
    Value2 = 1,
}
impl From<Trape0> for bool {
    #[inline(always)]
    fn from(variant: Trape0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRAPE0` reader - TRAP enable for CCU8x.OUTy0"]
pub type Trape0R = crate::BitReader<Trape0>;
impl Trape0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trape0 {
        match self.bits {
            false => Trape0::Value1,
            true => Trape0::Value2,
        }
    }
    #[doc = "TRAP functionality has no effect on the CCU8x.OUTy0 output"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Trape0::Value1
    }
    #[doc = "TRAP functionality affects the CCU8x.OUTy0 output"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Trape0::Value2
    }
}
#[doc = "Field `TRAPE0` writer - TRAP enable for CCU8x.OUTy0"]
pub type Trape0W<'a, REG> = crate::BitWriter<'a, REG, Trape0>;
impl<'a, REG> Trape0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TRAP functionality has no effect on the CCU8x.OUTy0 output"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Trape0::Value1)
    }
    #[doc = "TRAP functionality affects the CCU8x.OUTy0 output"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Trape0::Value2)
    }
}
#[doc = "Field `TRAPE1` reader - TRAP enable for CCU8x.OUTy1"]
pub type Trape1R = crate::BitReader;
#[doc = "Field `TRAPE1` writer - TRAP enable for CCU8x.OUTy1"]
pub type Trape1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRAPE2` reader - TRAP enable for CCU8x.OUTy2"]
pub type Trape2R = crate::BitReader;
#[doc = "Field `TRAPE2` writer - TRAP enable for CCU8x.OUTy2"]
pub type Trape2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRAPE3` reader - TRAP enable for CCU8x.OUTy3"]
pub type Trape3R = crate::BitReader;
#[doc = "Field `TRAPE3` writer - TRAP enable for CCU8x.OUTy3"]
pub type Trape3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "TRAP Synchronization Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trpse {
    #[doc = "0: Exiting from TRAP state isn't synchronized with the PWM signal"]
    Value1 = 0,
    #[doc = "1: Exiting from TRAP state is synchronized with the PWM signal"]
    Value2 = 1,
}
impl From<Trpse> for bool {
    #[inline(always)]
    fn from(variant: Trpse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRPSE` reader - TRAP Synchronization Enable"]
pub type TrpseR = crate::BitReader<Trpse>;
impl TrpseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trpse {
        match self.bits {
            false => Trpse::Value1,
            true => Trpse::Value2,
        }
    }
    #[doc = "Exiting from TRAP state isn't synchronized with the PWM signal"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Trpse::Value1
    }
    #[doc = "Exiting from TRAP state is synchronized with the PWM signal"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Trpse::Value2
    }
}
#[doc = "Field `TRPSE` writer - TRAP Synchronization Enable"]
pub type TrpseW<'a, REG> = crate::BitWriter<'a, REG, Trpse>;
impl<'a, REG> TrpseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exiting from TRAP state isn't synchronized with the PWM signal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Trpse::Value1)
    }
    #[doc = "Exiting from TRAP state is synchronized with the PWM signal"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Trpse::Value2)
    }
}
#[doc = "TRAP State Clear Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trpsw {
    #[doc = "0: The slice exits the TRAP state automatically when the TRAP condition is not present (Trap state cleared by HW and SW)"]
    Value1 = 0,
    #[doc = "1: The TRAP state can only be exited by a SW request."]
    Value2 = 1,
}
impl From<Trpsw> for bool {
    #[inline(always)]
    fn from(variant: Trpsw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRPSW` reader - TRAP State Clear Control"]
pub type TrpswR = crate::BitReader<Trpsw>;
impl TrpswR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trpsw {
        match self.bits {
            false => Trpsw::Value1,
            true => Trpsw::Value2,
        }
    }
    #[doc = "The slice exits the TRAP state automatically when the TRAP condition is not present (Trap state cleared by HW and SW)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Trpsw::Value1
    }
    #[doc = "The TRAP state can only be exited by a SW request."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Trpsw::Value2
    }
}
#[doc = "Field `TRPSW` writer - TRAP State Clear Control"]
pub type TrpswW<'a, REG> = crate::BitWriter<'a, REG, Trpsw>;
impl<'a, REG> TrpswW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The slice exits the TRAP state automatically when the TRAP condition is not present (Trap state cleared by HW and SW)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Trpsw::Value1)
    }
    #[doc = "The TRAP state can only be exited by a SW request."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Trpsw::Value2)
    }
}
#[doc = "External Modulation Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ems {
    #[doc = "0: External Modulation functionality is not synchronized with the PWM signal"]
    Value1 = 0,
    #[doc = "1: External Modulation functionality is synchronized with the PWM signal"]
    Value2 = 1,
}
impl From<Ems> for bool {
    #[inline(always)]
    fn from(variant: Ems) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMS` reader - External Modulation Synchronization"]
pub type EmsR = crate::BitReader<Ems>;
impl EmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ems {
        match self.bits {
            false => Ems::Value1,
            true => Ems::Value2,
        }
    }
    #[doc = "External Modulation functionality is not synchronized with the PWM signal"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ems::Value1
    }
    #[doc = "External Modulation functionality is synchronized with the PWM signal"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ems::Value2
    }
}
#[doc = "Field `EMS` writer - External Modulation Synchronization"]
pub type EmsW<'a, REG> = crate::BitWriter<'a, REG, Ems>;
impl<'a, REG> EmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External Modulation functionality is not synchronized with the PWM signal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ems::Value1)
    }
    #[doc = "External Modulation functionality is synchronized with the PWM signal"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ems::Value2)
    }
}
#[doc = "External Modulation Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Emt {
    #[doc = "0: External Modulation functionality is clearing the CC8ySTx bits."]
    Value1 = 0,
    #[doc = "1: External Modulation functionality is gating the outputs."]
    Value2 = 1,
}
impl From<Emt> for bool {
    #[inline(always)]
    fn from(variant: Emt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMT` reader - External Modulation Type"]
pub type EmtR = crate::BitReader<Emt>;
impl EmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Emt {
        match self.bits {
            false => Emt::Value1,
            true => Emt::Value2,
        }
    }
    #[doc = "External Modulation functionality is clearing the CC8ySTx bits."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Emt::Value1
    }
    #[doc = "External Modulation functionality is gating the outputs."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Emt::Value2
    }
}
#[doc = "Field `EMT` writer - External Modulation Type"]
pub type EmtW<'a, REG> = crate::BitWriter<'a, REG, Emt>;
impl<'a, REG> EmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External Modulation functionality is clearing the CC8ySTx bits."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Emt::Value1)
    }
    #[doc = "External Modulation functionality is gating the outputs."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Emt::Value2)
    }
}
#[doc = "Multi Channel Mode Enable for Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mcme1 {
    #[doc = "0: Multi Channel Mode in Channel 1 is disabled"]
    Value1 = 0,
    #[doc = "1: Multi Channel Mode in Channel 1 is enabled"]
    Value2 = 1,
}
impl From<Mcme1> for bool {
    #[inline(always)]
    fn from(variant: Mcme1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCME1` reader - Multi Channel Mode Enable for Channel 1"]
pub type Mcme1R = crate::BitReader<Mcme1>;
impl Mcme1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mcme1 {
        match self.bits {
            false => Mcme1::Value1,
            true => Mcme1::Value2,
        }
    }
    #[doc = "Multi Channel Mode in Channel 1 is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mcme1::Value1
    }
    #[doc = "Multi Channel Mode in Channel 1 is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mcme1::Value2
    }
}
#[doc = "Field `MCME1` writer - Multi Channel Mode Enable for Channel 1"]
pub type Mcme1W<'a, REG> = crate::BitWriter<'a, REG, Mcme1>;
impl<'a, REG> Mcme1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multi Channel Mode in Channel 1 is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mcme1::Value1)
    }
    #[doc = "Multi Channel Mode in Channel 1 is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mcme1::Value2)
    }
}
#[doc = "Multi Channel Mode Enable for Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mcme2 {
    #[doc = "0: Multi Channel Mode in Channel 2 is disabled"]
    Value1 = 0,
    #[doc = "1: Multi Channel Mode in Channel 2 is enabled"]
    Value2 = 1,
}
impl From<Mcme2> for bool {
    #[inline(always)]
    fn from(variant: Mcme2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCME2` reader - Multi Channel Mode Enable for Channel 2"]
pub type Mcme2R = crate::BitReader<Mcme2>;
impl Mcme2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mcme2 {
        match self.bits {
            false => Mcme2::Value1,
            true => Mcme2::Value2,
        }
    }
    #[doc = "Multi Channel Mode in Channel 2 is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mcme2::Value1
    }
    #[doc = "Multi Channel Mode in Channel 2 is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mcme2::Value2
    }
}
#[doc = "Field `MCME2` writer - Multi Channel Mode Enable for Channel 2"]
pub type Mcme2W<'a, REG> = crate::BitWriter<'a, REG, Mcme2>;
impl<'a, REG> Mcme2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multi Channel Mode in Channel 2 is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mcme2::Value1)
    }
    #[doc = "Multi Channel Mode in Channel 2 is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mcme2::Value2)
    }
}
#[doc = "External Modulation Channel enable\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eme {
    #[doc = "0: External Modulation functionality doesn't affect any channel"]
    Value1 = 0,
    #[doc = "1: External Modulation only applied on channel 1"]
    Value2 = 1,
    #[doc = "2: External Modulation only applied on channel 2"]
    Value3 = 2,
    #[doc = "3: External Modulation applied on both channels"]
    Value4 = 3,
}
impl From<Eme> for u8 {
    #[inline(always)]
    fn from(variant: Eme) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eme {
    type Ux = u8;
}
impl crate::IsEnum for Eme {}
#[doc = "Field `EME` reader - External Modulation Channel enable"]
pub type EmeR = crate::FieldReader<Eme>;
impl EmeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eme {
        match self.bits {
            0 => Eme::Value1,
            1 => Eme::Value2,
            2 => Eme::Value3,
            3 => Eme::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "External Modulation functionality doesn't affect any channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Eme::Value1
    }
    #[doc = "External Modulation only applied on channel 1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Eme::Value2
    }
    #[doc = "External Modulation only applied on channel 2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Eme::Value3
    }
    #[doc = "External Modulation applied on both channels"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Eme::Value4
    }
}
#[doc = "Field `EME` writer - External Modulation Channel enable"]
pub type EmeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Eme, crate::Safe>;
impl<'a, REG> EmeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External Modulation functionality doesn't affect any channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Eme::Value1)
    }
    #[doc = "External Modulation only applied on channel 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Eme::Value2)
    }
    #[doc = "External Modulation only applied on channel 2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Eme::Value3)
    }
    #[doc = "External Modulation applied on both channels"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Eme::Value4)
    }
}
#[doc = "Status bit output selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stos {
    #[doc = "0: CC8yST1 forward to CCU8x.STy"]
    Value1 = 0,
    #[doc = "1: CC8yST2 forward to CCU8x.STy"]
    Value2 = 1,
    #[doc = "2: CC8yST1 AND CC8yST2 forward to CCU8x.STy"]
    Value3 = 2,
    #[doc = "3: CC8yST1 OR CC8yST2 forward to CCU8x.STy"]
    Value4 = 3,
}
impl From<Stos> for u8 {
    #[inline(always)]
    fn from(variant: Stos) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stos {
    type Ux = u8;
}
impl crate::IsEnum for Stos {}
#[doc = "Field `STOS` reader - Status bit output selector"]
pub type StosR = crate::FieldReader<Stos>;
impl StosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stos {
        match self.bits {
            0 => Stos::Value1,
            1 => Stos::Value2,
            2 => Stos::Value3,
            3 => Stos::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "CC8yST1 forward to CCU8x.STy"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Stos::Value1
    }
    #[doc = "CC8yST2 forward to CCU8x.STy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Stos::Value2
    }
    #[doc = "CC8yST1 AND CC8yST2 forward to CCU8x.STy"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Stos::Value3
    }
    #[doc = "CC8yST1 OR CC8yST2 forward to CCU8x.STy"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Stos::Value4
    }
}
#[doc = "Field `STOS` writer - Status bit output selector"]
pub type StosW<'a, REG> = crate::FieldWriter<'a, REG, 2, Stos, crate::Safe>;
impl<'a, REG> StosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC8yST1 forward to CCU8x.STy"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Stos::Value1)
    }
    #[doc = "CC8yST2 forward to CCU8x.STy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Stos::Value2)
    }
    #[doc = "CC8yST1 AND CC8yST2 forward to CCU8x.STy"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Stos::Value3)
    }
    #[doc = "CC8yST1 OR CC8yST2 forward to CCU8x.STy"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Stos::Value4)
    }
}
impl R {
    #[doc = "Bit 0 - Timer Counting Mode"]
    #[inline(always)]
    pub fn tcm(&self) -> TcmR {
        TcmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer Single Shot Mode"]
    #[inline(always)]
    pub fn tssm(&self) -> TssmR {
        TssmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Shadow Transfer on Clear"]
    #[inline(always)]
    pub fn clst(&self) -> ClstR {
        ClstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture Compare Mode"]
    #[inline(always)]
    pub fn cmod(&self) -> CmodR {
        CmodR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Extended Capture Mode"]
    #[inline(always)]
    pub fn ecm(&self) -> EcmR {
        EcmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Clear on Capture Control"]
    #[inline(always)]
    pub fn capc(&self) -> CapcR {
        CapcR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Timer Load selector"]
    #[inline(always)]
    pub fn tls(&self) -> TlsR {
        TlsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Extended Stop Function Control"]
    #[inline(always)]
    pub fn endm(&self) -> EndmR {
        EndmR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Extended Start Function Control"]
    #[inline(always)]
    pub fn strm(&self) -> StrmR {
        StrmR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Equal Capture Event enable"]
    #[inline(always)]
    pub fn sce(&self) -> SceR {
        SceR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Continuous Capture Enable"]
    #[inline(always)]
    pub fn ccs(&self) -> CcsR {
        CcsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Dither Enable"]
    #[inline(always)]
    pub fn dithe(&self) -> DitheR {
        DitheR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Dither input selector"]
    #[inline(always)]
    pub fn dim(&self) -> DimR {
        DimR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Floating Prescaler enable"]
    #[inline(always)]
    pub fn fpe(&self) -> FpeR {
        FpeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TRAP enable for CCU8x.OUTy0"]
    #[inline(always)]
    pub fn trape0(&self) -> Trape0R {
        Trape0R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TRAP enable for CCU8x.OUTy1"]
    #[inline(always)]
    pub fn trape1(&self) -> Trape1R {
        Trape1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TRAP enable for CCU8x.OUTy2"]
    #[inline(always)]
    pub fn trape2(&self) -> Trape2R {
        Trape2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TRAP enable for CCU8x.OUTy3"]
    #[inline(always)]
    pub fn trape3(&self) -> Trape3R {
        Trape3R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TRAP Synchronization Enable"]
    #[inline(always)]
    pub fn trpse(&self) -> TrpseR {
        TrpseR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TRAP State Clear Control"]
    #[inline(always)]
    pub fn trpsw(&self) -> TrpswR {
        TrpswR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - External Modulation Synchronization"]
    #[inline(always)]
    pub fn ems(&self) -> EmsR {
        EmsR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - External Modulation Type"]
    #[inline(always)]
    pub fn emt(&self) -> EmtR {
        EmtR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Multi Channel Mode Enable for Channel 1"]
    #[inline(always)]
    pub fn mcme1(&self) -> Mcme1R {
        Mcme1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Multi Channel Mode Enable for Channel 2"]
    #[inline(always)]
    pub fn mcme2(&self) -> Mcme2R {
        Mcme2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - External Modulation Channel enable"]
    #[inline(always)]
    pub fn eme(&self) -> EmeR {
        EmeR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:30 - Status bit output selector"]
    #[inline(always)]
    pub fn stos(&self) -> StosR {
        StosR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Timer Counting Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tcm(&mut self) -> TcmW<TcSpec> {
        TcmW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer Single Shot Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tssm(&mut self) -> TssmW<TcSpec> {
        TssmW::new(self, 1)
    }
    #[doc = "Bit 2 - Shadow Transfer on Clear"]
    #[inline(always)]
    #[must_use]
    pub fn clst(&mut self) -> ClstW<TcSpec> {
        ClstW::new(self, 2)
    }
    #[doc = "Bit 4 - Extended Capture Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ecm(&mut self) -> EcmW<TcSpec> {
        EcmW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Clear on Capture Control"]
    #[inline(always)]
    #[must_use]
    pub fn capc(&mut self) -> CapcW<TcSpec> {
        CapcW::new(self, 5)
    }
    #[doc = "Bit 7 - Timer Load selector"]
    #[inline(always)]
    #[must_use]
    pub fn tls(&mut self) -> TlsW<TcSpec> {
        TlsW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Extended Stop Function Control"]
    #[inline(always)]
    #[must_use]
    pub fn endm(&mut self) -> EndmW<TcSpec> {
        EndmW::new(self, 8)
    }
    #[doc = "Bit 10 - Extended Start Function Control"]
    #[inline(always)]
    #[must_use]
    pub fn strm(&mut self) -> StrmW<TcSpec> {
        StrmW::new(self, 10)
    }
    #[doc = "Bit 11 - Equal Capture Event enable"]
    #[inline(always)]
    #[must_use]
    pub fn sce(&mut self) -> SceW<TcSpec> {
        SceW::new(self, 11)
    }
    #[doc = "Bit 12 - Continuous Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccs(&mut self) -> CcsW<TcSpec> {
        CcsW::new(self, 12)
    }
    #[doc = "Bits 13:14 - Dither Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dithe(&mut self) -> DitheW<TcSpec> {
        DitheW::new(self, 13)
    }
    #[doc = "Bit 15 - Dither input selector"]
    #[inline(always)]
    #[must_use]
    pub fn dim(&mut self) -> DimW<TcSpec> {
        DimW::new(self, 15)
    }
    #[doc = "Bit 16 - Floating Prescaler enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpe(&mut self) -> FpeW<TcSpec> {
        FpeW::new(self, 16)
    }
    #[doc = "Bit 17 - TRAP enable for CCU8x.OUTy0"]
    #[inline(always)]
    #[must_use]
    pub fn trape0(&mut self) -> Trape0W<TcSpec> {
        Trape0W::new(self, 17)
    }
    #[doc = "Bit 18 - TRAP enable for CCU8x.OUTy1"]
    #[inline(always)]
    #[must_use]
    pub fn trape1(&mut self) -> Trape1W<TcSpec> {
        Trape1W::new(self, 18)
    }
    #[doc = "Bit 19 - TRAP enable for CCU8x.OUTy2"]
    #[inline(always)]
    #[must_use]
    pub fn trape2(&mut self) -> Trape2W<TcSpec> {
        Trape2W::new(self, 19)
    }
    #[doc = "Bit 20 - TRAP enable for CCU8x.OUTy3"]
    #[inline(always)]
    #[must_use]
    pub fn trape3(&mut self) -> Trape3W<TcSpec> {
        Trape3W::new(self, 20)
    }
    #[doc = "Bit 21 - TRAP Synchronization Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trpse(&mut self) -> TrpseW<TcSpec> {
        TrpseW::new(self, 21)
    }
    #[doc = "Bit 22 - TRAP State Clear Control"]
    #[inline(always)]
    #[must_use]
    pub fn trpsw(&mut self) -> TrpswW<TcSpec> {
        TrpswW::new(self, 22)
    }
    #[doc = "Bit 23 - External Modulation Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn ems(&mut self) -> EmsW<TcSpec> {
        EmsW::new(self, 23)
    }
    #[doc = "Bit 24 - External Modulation Type"]
    #[inline(always)]
    #[must_use]
    pub fn emt(&mut self) -> EmtW<TcSpec> {
        EmtW::new(self, 24)
    }
    #[doc = "Bit 25 - Multi Channel Mode Enable for Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn mcme1(&mut self) -> Mcme1W<TcSpec> {
        Mcme1W::new(self, 25)
    }
    #[doc = "Bit 26 - Multi Channel Mode Enable for Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn mcme2(&mut self) -> Mcme2W<TcSpec> {
        Mcme2W::new(self, 26)
    }
    #[doc = "Bits 27:28 - External Modulation Channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn eme(&mut self) -> EmeW<TcSpec> {
        EmeW::new(self, 27)
    }
    #[doc = "Bits 29:30 - Status bit output selector"]
    #[inline(always)]
    #[must_use]
    pub fn stos(&mut self) -> StosW<TcSpec> {
        StosW::new(self, 29)
    }
}
#[doc = "Slice Timer Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcSpec;
impl crate::RegisterSpec for TcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tc::R`](R) reader structure"]
impl crate::Readable for TcSpec {}
#[doc = "`write(|w| ..)` method takes [`tc::W`](W) writer structure"]
impl crate::Writable for TcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TC to value 0x1800_0000"]
impl crate::Resettable for TcSpec {
    const RESET_VALUE: u32 = 0x1800_0000;
}
