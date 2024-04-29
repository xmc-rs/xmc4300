#[doc = "Register `TRAPDIS` reader"]
pub type R = crate::R<TRAPDIS_SPEC>;
#[doc = "Register `TRAPDIS` writer"]
pub type W = crate::W<TRAPDIS_SPEC>;
#[doc = "OSC_HP Oscillator Watchdog Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOSCWDGT_A {
    #[doc = "0: Trap request enabled"]
    CONST_0 = 0,
    #[doc = "1: Trap request disabled"]
    CONST_1 = 1,
}
impl From<SOSCWDGT_A> for bool {
    #[inline(always)]
    fn from(variant: SOSCWDGT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOSCWDGT` reader - OSC_HP Oscillator Watchdog Trap Disable"]
pub type SOSCWDGT_R = crate::BitReader<SOSCWDGT_A>;
impl SOSCWDGT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOSCWDGT_A {
        match self.bits {
            false => SOSCWDGT_A::CONST_0,
            true => SOSCWDGT_A::CONST_1,
        }
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == SOSCWDGT_A::CONST_0
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == SOSCWDGT_A::CONST_1
    }
}
#[doc = "Field `SOSCWDGT` writer - OSC_HP Oscillator Watchdog Trap Disable"]
pub type SOSCWDGT_W<'a, REG> = crate::BitWriter<'a, REG, SOSCWDGT_A>;
impl<'a, REG> SOSCWDGT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(SOSCWDGT_A::CONST_0)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(SOSCWDGT_A::CONST_1)
    }
}
#[doc = "System VCO Lock Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVCOLCKT_A {
    #[doc = "0: Trap request enabled"]
    CONST_0 = 0,
    #[doc = "1: Trap request disabled"]
    CONST_1 = 1,
}
impl From<SVCOLCKT_A> for bool {
    #[inline(always)]
    fn from(variant: SVCOLCKT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVCOLCKT` reader - System VCO Lock Trap Disable"]
pub type SVCOLCKT_R = crate::BitReader<SVCOLCKT_A>;
impl SVCOLCKT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SVCOLCKT_A {
        match self.bits {
            false => SVCOLCKT_A::CONST_0,
            true => SVCOLCKT_A::CONST_1,
        }
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == SVCOLCKT_A::CONST_0
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == SVCOLCKT_A::CONST_1
    }
}
#[doc = "Field `SVCOLCKT` writer - System VCO Lock Trap Disable"]
pub type SVCOLCKT_W<'a, REG> = crate::BitWriter<'a, REG, SVCOLCKT_A>;
impl<'a, REG> SVCOLCKT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(SVCOLCKT_A::CONST_0)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(SVCOLCKT_A::CONST_1)
    }
}
#[doc = "USB VCO Lock Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UVCOLCKT_A {
    #[doc = "0: Trap request enabled"]
    CONST_0 = 0,
    #[doc = "1: Trap request disabled"]
    CONST_1 = 1,
}
impl From<UVCOLCKT_A> for bool {
    #[inline(always)]
    fn from(variant: UVCOLCKT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UVCOLCKT` reader - USB VCO Lock Trap Disable"]
pub type UVCOLCKT_R = crate::BitReader<UVCOLCKT_A>;
impl UVCOLCKT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UVCOLCKT_A {
        match self.bits {
            false => UVCOLCKT_A::CONST_0,
            true => UVCOLCKT_A::CONST_1,
        }
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == UVCOLCKT_A::CONST_0
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == UVCOLCKT_A::CONST_1
    }
}
#[doc = "Field `UVCOLCKT` writer - USB VCO Lock Trap Disable"]
pub type UVCOLCKT_W<'a, REG> = crate::BitWriter<'a, REG, UVCOLCKT_A>;
impl<'a, REG> UVCOLCKT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(UVCOLCKT_A::CONST_0)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(UVCOLCKT_A::CONST_1)
    }
}
#[doc = "Parity Error Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PET_A {
    #[doc = "0: Trap request enabled"]
    CONST_0 = 0,
    #[doc = "1: Trap request disabled"]
    CONST_1 = 1,
}
impl From<PET_A> for bool {
    #[inline(always)]
    fn from(variant: PET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PET` reader - Parity Error Trap Disable"]
pub type PET_R = crate::BitReader<PET_A>;
impl PET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PET_A {
        match self.bits {
            false => PET_A::CONST_0,
            true => PET_A::CONST_1,
        }
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PET_A::CONST_0
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PET_A::CONST_1
    }
}
#[doc = "Field `PET` writer - Parity Error Trap Disable"]
pub type PET_W<'a, REG> = crate::BitWriter<'a, REG, PET_A>;
impl<'a, REG> PET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PET_A::CONST_0)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PET_A::CONST_1)
    }
}
#[doc = "Brown Out Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRWNT_A {
    #[doc = "0: Trap request enabled"]
    CONST_0 = 0,
    #[doc = "1: Trap request disabled"]
    CONST_1 = 1,
}
impl From<BRWNT_A> for bool {
    #[inline(always)]
    fn from(variant: BRWNT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRWNT` reader - Brown Out Trap Disable"]
pub type BRWNT_R = crate::BitReader<BRWNT_A>;
impl BRWNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BRWNT_A {
        match self.bits {
            false => BRWNT_A::CONST_0,
            true => BRWNT_A::CONST_1,
        }
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == BRWNT_A::CONST_0
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == BRWNT_A::CONST_1
    }
}
#[doc = "Field `BRWNT` writer - Brown Out Trap Disable"]
pub type BRWNT_W<'a, REG> = crate::BitWriter<'a, REG, BRWNT_A>;
impl<'a, REG> BRWNT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(BRWNT_A::CONST_0)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(BRWNT_A::CONST_1)
    }
}
#[doc = "OSC_ULP Oscillator Watchdog Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULPWDGT_A {
    #[doc = "0: Trap request enabled"]
    CONST_0 = 0,
    #[doc = "1: Trap request disabled"]
    CONST_1 = 1,
}
impl From<ULPWDGT_A> for bool {
    #[inline(always)]
    fn from(variant: ULPWDGT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDGT` reader - OSC_ULP Oscillator Watchdog Trap Disable"]
pub type ULPWDGT_R = crate::BitReader<ULPWDGT_A>;
impl ULPWDGT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ULPWDGT_A {
        match self.bits {
            false => ULPWDGT_A::CONST_0,
            true => ULPWDGT_A::CONST_1,
        }
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ULPWDGT_A::CONST_0
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ULPWDGT_A::CONST_1
    }
}
#[doc = "Field `ULPWDGT` writer - OSC_ULP Oscillator Watchdog Trap Disable"]
pub type ULPWDGT_W<'a, REG> = crate::BitWriter<'a, REG, ULPWDGT_A>;
impl<'a, REG> ULPWDGT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(ULPWDGT_A::CONST_0)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(ULPWDGT_A::CONST_1)
    }
}
#[doc = "Peripheral Bridge 0 Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWERR0T_A {
    #[doc = "0: Trap request enabled"]
    CONST_0 = 0,
    #[doc = "1: Trap request disabled"]
    CONST_1 = 1,
}
impl From<BWERR0T_A> for bool {
    #[inline(always)]
    fn from(variant: BWERR0T_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR0T` reader - Peripheral Bridge 0 Trap Disable"]
pub type BWERR0T_R = crate::BitReader<BWERR0T_A>;
impl BWERR0T_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BWERR0T_A {
        match self.bits {
            false => BWERR0T_A::CONST_0,
            true => BWERR0T_A::CONST_1,
        }
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == BWERR0T_A::CONST_0
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == BWERR0T_A::CONST_1
    }
}
#[doc = "Field `BWERR0T` writer - Peripheral Bridge 0 Trap Disable"]
pub type BWERR0T_W<'a, REG> = crate::BitWriter<'a, REG, BWERR0T_A>;
impl<'a, REG> BWERR0T_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(BWERR0T_A::CONST_0)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(BWERR0T_A::CONST_1)
    }
}
#[doc = "Peripheral Bridge 1 Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWERR1T_A {
    #[doc = "0: Trap request enabled"]
    CONST_0 = 0,
    #[doc = "1: Trap request disabled"]
    CONST_1 = 1,
}
impl From<BWERR1T_A> for bool {
    #[inline(always)]
    fn from(variant: BWERR1T_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR1T` reader - Peripheral Bridge 1 Trap Disable"]
pub type BWERR1T_R = crate::BitReader<BWERR1T_A>;
impl BWERR1T_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BWERR1T_A {
        match self.bits {
            false => BWERR1T_A::CONST_0,
            true => BWERR1T_A::CONST_1,
        }
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == BWERR1T_A::CONST_0
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == BWERR1T_A::CONST_1
    }
}
#[doc = "Field `BWERR1T` writer - Peripheral Bridge 1 Trap Disable"]
pub type BWERR1T_W<'a, REG> = crate::BitWriter<'a, REG, BWERR1T_A>;
impl<'a, REG> BWERR1T_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(BWERR1T_A::CONST_0)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(BWERR1T_A::CONST_1)
    }
}
#[doc = "EtherCat Reset 0 Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECAT0RST_A {
    #[doc = "0: Trap request enabled"]
    CONST_0 = 0,
    #[doc = "1: Trap request disabled"]
    CONST_1 = 1,
}
impl From<ECAT0RST_A> for bool {
    #[inline(always)]
    fn from(variant: ECAT0RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECAT0RST` reader - EtherCat Reset 0 Trap Disable"]
pub type ECAT0RST_R = crate::BitReader<ECAT0RST_A>;
impl ECAT0RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECAT0RST_A {
        match self.bits {
            false => ECAT0RST_A::CONST_0,
            true => ECAT0RST_A::CONST_1,
        }
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ECAT0RST_A::CONST_0
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ECAT0RST_A::CONST_1
    }
}
#[doc = "Field `ECAT0RST` writer - EtherCat Reset 0 Trap Disable"]
pub type ECAT0RST_W<'a, REG> = crate::BitWriter<'a, REG, ECAT0RST_A>;
impl<'a, REG> ECAT0RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(ECAT0RST_A::CONST_0)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(ECAT0RST_A::CONST_1)
    }
}
impl R {
    #[doc = "Bit 0 - OSC_HP Oscillator Watchdog Trap Disable"]
    #[inline(always)]
    pub fn soscwdgt(&self) -> SOSCWDGT_R {
        SOSCWDGT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - System VCO Lock Trap Disable"]
    #[inline(always)]
    pub fn svcolckt(&self) -> SVCOLCKT_R {
        SVCOLCKT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Disable"]
    #[inline(always)]
    pub fn uvcolckt(&self) -> UVCOLCKT_R {
        UVCOLCKT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Parity Error Trap Disable"]
    #[inline(always)]
    pub fn pet(&self) -> PET_R {
        PET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Brown Out Trap Disable"]
    #[inline(always)]
    pub fn brwnt(&self) -> BRWNT_R {
        BRWNT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OSC_ULP Oscillator Watchdog Trap Disable"]
    #[inline(always)]
    pub fn ulpwdgt(&self) -> ULPWDGT_R {
        ULPWDGT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Disable"]
    #[inline(always)]
    pub fn bwerr0t(&self) -> BWERR0T_R {
        BWERR0T_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Disable"]
    #[inline(always)]
    pub fn bwerr1t(&self) -> BWERR1T_R {
        BWERR1T_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - EtherCat Reset 0 Trap Disable"]
    #[inline(always)]
    pub fn ecat0rst(&self) -> ECAT0RST_R {
        ECAT0RST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OSC_HP Oscillator Watchdog Trap Disable"]
    #[inline(always)]
    #[must_use]
    pub fn soscwdgt(&mut self) -> SOSCWDGT_W<TRAPDIS_SPEC> {
        SOSCWDGT_W::new(self, 0)
    }
    #[doc = "Bit 2 - System VCO Lock Trap Disable"]
    #[inline(always)]
    #[must_use]
    pub fn svcolckt(&mut self) -> SVCOLCKT_W<TRAPDIS_SPEC> {
        SVCOLCKT_W::new(self, 2)
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Disable"]
    #[inline(always)]
    #[must_use]
    pub fn uvcolckt(&mut self) -> UVCOLCKT_W<TRAPDIS_SPEC> {
        UVCOLCKT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Parity Error Trap Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pet(&mut self) -> PET_W<TRAPDIS_SPEC> {
        PET_W::new(self, 4)
    }
    #[doc = "Bit 5 - Brown Out Trap Disable"]
    #[inline(always)]
    #[must_use]
    pub fn brwnt(&mut self) -> BRWNT_W<TRAPDIS_SPEC> {
        BRWNT_W::new(self, 5)
    }
    #[doc = "Bit 6 - OSC_ULP Oscillator Watchdog Trap Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ulpwdgt(&mut self) -> ULPWDGT_W<TRAPDIS_SPEC> {
        ULPWDGT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Disable"]
    #[inline(always)]
    #[must_use]
    pub fn bwerr0t(&mut self) -> BWERR0T_W<TRAPDIS_SPEC> {
        BWERR0T_W::new(self, 7)
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Disable"]
    #[inline(always)]
    #[must_use]
    pub fn bwerr1t(&mut self) -> BWERR1T_W<TRAPDIS_SPEC> {
        BWERR1T_W::new(self, 8)
    }
    #[doc = "Bit 16 - EtherCat Reset 0 Trap Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ecat0rst(&mut self) -> ECAT0RST_W<TRAPDIS_SPEC> {
        ECAT0RST_W::new(self, 16)
    }
}
#[doc = "Trap Disable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trapdis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trapdis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRAPDIS_SPEC;
impl crate::RegisterSpec for TRAPDIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trapdis::R`](R) reader structure"]
impl crate::Readable for TRAPDIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trapdis::W`](W) writer structure"]
impl crate::Writable for TRAPDIS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRAPDIS to value 0x0001_01fd"]
impl crate::Resettable for TRAPDIS_SPEC {
    const RESET_VALUE: u32 = 0x0001_01fd;
}
