#[doc = "Register `GLOBCTL` reader"]
pub type R = crate::R<GLOBCTL_SPEC>;
#[doc = "Register `GLOBCTL` writer"]
pub type W = crate::W<GLOBCTL_SPEC>;
#[doc = "Field `TS_EN` reader - Touch-Sense Function Enable"]
pub type TS_EN_R = crate::BitReader;
#[doc = "Field `TS_EN` writer - Touch-Sense Function Enable"]
pub type TS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LD_EN` reader - LED Function Enable"]
pub type LD_EN_R = crate::BitReader;
#[doc = "Field `LD_EN` writer - LED Function Enable"]
pub type LD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMTR` reader - Clock Master Disable"]
pub type CMTR_R = crate::BitReader<CMTR_A>;
#[doc = "Clock Master Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMTR_A {
    #[doc = "0: Kernel generates its own clock for LEDTS-counter based on SFR setting"]
    VALUE1 = 0,
    #[doc = "1: LEDTS-counter takes its clock from another master kernel"]
    VALUE2 = 1,
}
impl From<CMTR_A> for bool {
    #[inline(always)]
    fn from(variant: CMTR_A) -> Self {
        variant as u8 != 0
    }
}
impl CMTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMTR_A {
        match self.bits {
            false => CMTR_A::VALUE1,
            true => CMTR_A::VALUE2,
        }
    }
    #[doc = "Kernel generates its own clock for LEDTS-counter based on SFR setting"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMTR_A::VALUE1
    }
    #[doc = "LEDTS-counter takes its clock from another master kernel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMTR_A::VALUE2
    }
}
#[doc = "Field `CMTR` writer - Clock Master Disable"]
pub type CMTR_W<'a, REG> = crate::BitWriter<'a, REG, CMTR_A>;
impl<'a, REG> CMTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Kernel generates its own clock for LEDTS-counter based on SFR setting"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CMTR_A::VALUE1)
    }
    #[doc = "LEDTS-counter takes its clock from another master kernel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CMTR_A::VALUE2)
    }
}
#[doc = "Field `ENSYNC` reader - Enable Autoscan Time Period Synchronization"]
pub type ENSYNC_R = crate::BitReader<ENSYNC_A>;
#[doc = "Enable Autoscan Time Period Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENSYNC_A {
    #[doc = "0: No synchronization"]
    VALUE1 = 0,
    #[doc = "1: Synchronization enabled on Kernel0 autoscan time period"]
    VALUE2 = 1,
}
impl From<ENSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: ENSYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl ENSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENSYNC_A {
        match self.bits {
            false => ENSYNC_A::VALUE1,
            true => ENSYNC_A::VALUE2,
        }
    }
    #[doc = "No synchronization"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENSYNC_A::VALUE1
    }
    #[doc = "Synchronization enabled on Kernel0 autoscan time period"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENSYNC_A::VALUE2
    }
}
#[doc = "Field `ENSYNC` writer - Enable Autoscan Time Period Synchronization"]
pub type ENSYNC_W<'a, REG> = crate::BitWriter<'a, REG, ENSYNC_A>;
impl<'a, REG> ENSYNC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No synchronization"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ENSYNC_A::VALUE1)
    }
    #[doc = "Synchronization enabled on Kernel0 autoscan time period"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ENSYNC_A::VALUE2)
    }
}
#[doc = "Field `SUSCFG` reader - Suspend Request Configuration"]
pub type SUSCFG_R = crate::BitReader<SUSCFG_A>;
#[doc = "Suspend Request Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSCFG_A {
    #[doc = "0: Ignore suspend request"]
    VALUE1 = 0,
    #[doc = "1: Enable suspend according to request"]
    VALUE2 = 1,
}
impl From<SUSCFG_A> for bool {
    #[inline(always)]
    fn from(variant: SUSCFG_A) -> Self {
        variant as u8 != 0
    }
}
impl SUSCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SUSCFG_A {
        match self.bits {
            false => SUSCFG_A::VALUE1,
            true => SUSCFG_A::VALUE2,
        }
    }
    #[doc = "Ignore suspend request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SUSCFG_A::VALUE1
    }
    #[doc = "Enable suspend according to request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SUSCFG_A::VALUE2
    }
}
#[doc = "Field `SUSCFG` writer - Suspend Request Configuration"]
pub type SUSCFG_W<'a, REG> = crate::BitWriter<'a, REG, SUSCFG_A>;
impl<'a, REG> SUSCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore suspend request"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SUSCFG_A::VALUE1)
    }
    #[doc = "Enable suspend according to request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SUSCFG_A::VALUE2)
    }
}
#[doc = "Field `MASKVAL` reader - Mask Number of LSB Bits for Event Validation"]
pub type MASKVAL_R = crate::FieldReader<MASKVAL_A>;
#[doc = "Mask Number of LSB Bits for Event Validation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MASKVAL_A {
    #[doc = "0: Mask LSB bit"]
    VALUE1 = 0,
    #[doc = "1: Mask 2 LSB bits"]
    VALUE2 = 1,
    #[doc = "7: Mask 8 LSB bits"]
    VALUE3 = 7,
}
impl From<MASKVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: MASKVAL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MASKVAL_A {
    type Ux = u8;
}
impl MASKVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MASKVAL_A> {
        match self.bits {
            0 => Some(MASKVAL_A::VALUE1),
            1 => Some(MASKVAL_A::VALUE2),
            7 => Some(MASKVAL_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Mask LSB bit"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MASKVAL_A::VALUE1
    }
    #[doc = "Mask 2 LSB bits"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MASKVAL_A::VALUE2
    }
    #[doc = "Mask 8 LSB bits"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == MASKVAL_A::VALUE3
    }
}
#[doc = "Field `MASKVAL` writer - Mask Number of LSB Bits for Event Validation"]
pub type MASKVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MASKVAL_A>;
impl<'a, REG> MASKVAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Mask LSB bit"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MASKVAL_A::VALUE1)
    }
    #[doc = "Mask 2 LSB bits"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MASKVAL_A::VALUE2)
    }
    #[doc = "Mask 8 LSB bits"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(MASKVAL_A::VALUE3)
    }
}
#[doc = "Field `FENVAL` reader - Enable (Extended) Time Frame Validation"]
pub type FENVAL_R = crate::BitReader<FENVAL_A>;
#[doc = "Enable (Extended) Time Frame Validation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FENVAL_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<FENVAL_A> for bool {
    #[inline(always)]
    fn from(variant: FENVAL_A) -> Self {
        variant as u8 != 0
    }
}
impl FENVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FENVAL_A {
        match self.bits {
            false => FENVAL_A::VALUE1,
            true => FENVAL_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FENVAL_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FENVAL_A::VALUE2
    }
}
#[doc = "Field `FENVAL` writer - Enable (Extended) Time Frame Validation"]
pub type FENVAL_W<'a, REG> = crate::BitWriter<'a, REG, FENVAL_A>;
impl<'a, REG> FENVAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FENVAL_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FENVAL_A::VALUE2)
    }
}
#[doc = "Field `ITS_EN` reader - Enable Time Slice Interrupt"]
pub type ITS_EN_R = crate::BitReader<ITS_EN_A>;
#[doc = "Enable Time Slice Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITS_EN_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<ITS_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ITS_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ITS_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITS_EN_A {
        match self.bits {
            false => ITS_EN_A::VALUE1,
            true => ITS_EN_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ITS_EN_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ITS_EN_A::VALUE2
    }
}
#[doc = "Field `ITS_EN` writer - Enable Time Slice Interrupt"]
pub type ITS_EN_W<'a, REG> = crate::BitWriter<'a, REG, ITS_EN_A>;
impl<'a, REG> ITS_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ITS_EN_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ITS_EN_A::VALUE2)
    }
}
#[doc = "Field `ITF_EN` reader - Enable (Extended) Time Frame Interrupt"]
pub type ITF_EN_R = crate::BitReader<ITF_EN_A>;
#[doc = "Enable (Extended) Time Frame Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITF_EN_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<ITF_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ITF_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ITF_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITF_EN_A {
        match self.bits {
            false => ITF_EN_A::VALUE1,
            true => ITF_EN_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ITF_EN_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ITF_EN_A::VALUE2
    }
}
#[doc = "Field `ITF_EN` writer - Enable (Extended) Time Frame Interrupt"]
pub type ITF_EN_W<'a, REG> = crate::BitWriter<'a, REG, ITF_EN_A>;
impl<'a, REG> ITF_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ITF_EN_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ITF_EN_A::VALUE2)
    }
}
#[doc = "Field `ITP_EN` reader - Enable Autoscan Time Period Interrupt"]
pub type ITP_EN_R = crate::BitReader<ITP_EN_A>;
#[doc = "Enable Autoscan Time Period Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITP_EN_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable (valid only for case of hardware-enabled pad turn control)"]
    VALUE2 = 1,
}
impl From<ITP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ITP_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ITP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITP_EN_A {
        match self.bits {
            false => ITP_EN_A::VALUE1,
            true => ITP_EN_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ITP_EN_A::VALUE1
    }
    #[doc = "Enable (valid only for case of hardware-enabled pad turn control)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ITP_EN_A::VALUE2
    }
}
#[doc = "Field `ITP_EN` writer - Enable Autoscan Time Period Interrupt"]
pub type ITP_EN_W<'a, REG> = crate::BitWriter<'a, REG, ITP_EN_A>;
impl<'a, REG> ITP_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ITP_EN_A::VALUE1)
    }
    #[doc = "Enable (valid only for case of hardware-enabled pad turn control)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ITP_EN_A::VALUE2)
    }
}
#[doc = "Field `CLK_PS` reader - LEDTS-Counter Clock Pre-Scale Factor"]
pub type CLK_PS_R = crate::FieldReader<u16>;
#[doc = "Field `CLK_PS` writer - LEDTS-Counter Clock Pre-Scale Factor"]
pub type CLK_PS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Touch-Sense Function Enable"]
    #[inline(always)]
    pub fn ts_en(&self) -> TS_EN_R {
        TS_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LED Function Enable"]
    #[inline(always)]
    pub fn ld_en(&self) -> LD_EN_R {
        LD_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock Master Disable"]
    #[inline(always)]
    pub fn cmtr(&self) -> CMTR_R {
        CMTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Autoscan Time Period Synchronization"]
    #[inline(always)]
    pub fn ensync(&self) -> ENSYNC_R {
        ENSYNC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Suspend Request Configuration"]
    #[inline(always)]
    pub fn suscfg(&self) -> SUSCFG_R {
        SUSCFG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Mask Number of LSB Bits for Event Validation"]
    #[inline(always)]
    pub fn maskval(&self) -> MASKVAL_R {
        MASKVAL_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Enable (Extended) Time Frame Validation"]
    #[inline(always)]
    pub fn fenval(&self) -> FENVAL_R {
        FENVAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Time Slice Interrupt"]
    #[inline(always)]
    pub fn its_en(&self) -> ITS_EN_R {
        ITS_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable (Extended) Time Frame Interrupt"]
    #[inline(always)]
    pub fn itf_en(&self) -> ITF_EN_R {
        ITF_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Autoscan Time Period Interrupt"]
    #[inline(always)]
    pub fn itp_en(&self) -> ITP_EN_R {
        ITP_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - LEDTS-Counter Clock Pre-Scale Factor"]
    #[inline(always)]
    pub fn clk_ps(&self) -> CLK_PS_R {
        CLK_PS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Touch-Sense Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ts_en(&mut self) -> TS_EN_W<GLOBCTL_SPEC> {
        TS_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - LED Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ld_en(&mut self) -> LD_EN_W<GLOBCTL_SPEC> {
        LD_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clock Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cmtr(&mut self) -> CMTR_W<GLOBCTL_SPEC> {
        CMTR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Autoscan Time Period Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn ensync(&mut self) -> ENSYNC_W<GLOBCTL_SPEC> {
        ENSYNC_W::new(self, 3)
    }
    #[doc = "Bit 8 - Suspend Request Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn suscfg(&mut self) -> SUSCFG_W<GLOBCTL_SPEC> {
        SUSCFG_W::new(self, 8)
    }
    #[doc = "Bits 9:11 - Mask Number of LSB Bits for Event Validation"]
    #[inline(always)]
    #[must_use]
    pub fn maskval(&mut self) -> MASKVAL_W<GLOBCTL_SPEC> {
        MASKVAL_W::new(self, 9)
    }
    #[doc = "Bit 12 - Enable (Extended) Time Frame Validation"]
    #[inline(always)]
    #[must_use]
    pub fn fenval(&mut self) -> FENVAL_W<GLOBCTL_SPEC> {
        FENVAL_W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Time Slice Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn its_en(&mut self) -> ITS_EN_W<GLOBCTL_SPEC> {
        ITS_EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable (Extended) Time Frame Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn itf_en(&mut self) -> ITF_EN_W<GLOBCTL_SPEC> {
        ITF_EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Autoscan Time Period Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn itp_en(&mut self) -> ITP_EN_W<GLOBCTL_SPEC> {
        ITP_EN_W::new(self, 15)
    }
    #[doc = "Bits 16:31 - LEDTS-Counter Clock Pre-Scale Factor"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ps(&mut self) -> CLK_PS_W<GLOBCTL_SPEC> {
        CLK_PS_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Global Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GLOBCTL_SPEC;
impl crate::RegisterSpec for GLOBCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`globctl::R`](R) reader structure"]
impl crate::Readable for GLOBCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`globctl::W`](W) writer structure"]
impl crate::Writable for GLOBCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLOBCTL to value 0"]
impl crate::Resettable for GLOBCTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
