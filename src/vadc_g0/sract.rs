#[doc = "Register `SRACT` writer"]
pub type W = crate::W<SRACT_SPEC>;
#[doc = "Activate Group Service Request Node 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AGSR0_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Activate the associated service request line"]
    VALUE2 = 1,
}
impl From<AGSR0_AW> for bool {
    #[inline(always)]
    fn from(variant: AGSR0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AGSR0` writer - Activate Group Service Request Node 0"]
pub type AGSR0_W<'a, REG> = crate::BitWriter<'a, REG, AGSR0_AW>;
impl<'a, REG> AGSR0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AGSR0_AW::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AGSR0_AW::VALUE2)
    }
}
#[doc = "Activate Group Service Request Node 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AGSR1_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Activate the associated service request line"]
    VALUE2 = 1,
}
impl From<AGSR1_AW> for bool {
    #[inline(always)]
    fn from(variant: AGSR1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AGSR1` writer - Activate Group Service Request Node 1"]
pub type AGSR1_W<'a, REG> = crate::BitWriter<'a, REG, AGSR1_AW>;
impl<'a, REG> AGSR1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AGSR1_AW::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AGSR1_AW::VALUE2)
    }
}
#[doc = "Activate Group Service Request Node 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AGSR2_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Activate the associated service request line"]
    VALUE2 = 1,
}
impl From<AGSR2_AW> for bool {
    #[inline(always)]
    fn from(variant: AGSR2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AGSR2` writer - Activate Group Service Request Node 2"]
pub type AGSR2_W<'a, REG> = crate::BitWriter<'a, REG, AGSR2_AW>;
impl<'a, REG> AGSR2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AGSR2_AW::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AGSR2_AW::VALUE2)
    }
}
#[doc = "Activate Group Service Request Node 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AGSR3_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Activate the associated service request line"]
    VALUE2 = 1,
}
impl From<AGSR3_AW> for bool {
    #[inline(always)]
    fn from(variant: AGSR3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AGSR3` writer - Activate Group Service Request Node 3"]
pub type AGSR3_W<'a, REG> = crate::BitWriter<'a, REG, AGSR3_AW>;
impl<'a, REG> AGSR3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AGSR3_AW::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AGSR3_AW::VALUE2)
    }
}
#[doc = "Activate Shared Service Request Node 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASSR0_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Activate the associated service request line"]
    VALUE2 = 1,
}
impl From<ASSR0_AW> for bool {
    #[inline(always)]
    fn from(variant: ASSR0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSR0` writer - Activate Shared Service Request Node 0"]
pub type ASSR0_W<'a, REG> = crate::BitWriter<'a, REG, ASSR0_AW>;
impl<'a, REG> ASSR0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ASSR0_AW::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ASSR0_AW::VALUE2)
    }
}
#[doc = "Activate Shared Service Request Node 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASSR1_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Activate the associated service request line"]
    VALUE2 = 1,
}
impl From<ASSR1_AW> for bool {
    #[inline(always)]
    fn from(variant: ASSR1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSR1` writer - Activate Shared Service Request Node 1"]
pub type ASSR1_W<'a, REG> = crate::BitWriter<'a, REG, ASSR1_AW>;
impl<'a, REG> ASSR1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ASSR1_AW::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ASSR1_AW::VALUE2)
    }
}
#[doc = "Activate Shared Service Request Node 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASSR2_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Activate the associated service request line"]
    VALUE2 = 1,
}
impl From<ASSR2_AW> for bool {
    #[inline(always)]
    fn from(variant: ASSR2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSR2` writer - Activate Shared Service Request Node 2"]
pub type ASSR2_W<'a, REG> = crate::BitWriter<'a, REG, ASSR2_AW>;
impl<'a, REG> ASSR2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ASSR2_AW::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ASSR2_AW::VALUE2)
    }
}
#[doc = "Activate Shared Service Request Node 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASSR3_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Activate the associated service request line"]
    VALUE2 = 1,
}
impl From<ASSR3_AW> for bool {
    #[inline(always)]
    fn from(variant: ASSR3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSR3` writer - Activate Shared Service Request Node 3"]
pub type ASSR3_W<'a, REG> = crate::BitWriter<'a, REG, ASSR3_AW>;
impl<'a, REG> ASSR3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ASSR3_AW::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ASSR3_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Activate Group Service Request Node 0"]
    #[inline(always)]
    #[must_use]
    pub fn agsr0(&mut self) -> AGSR0_W<SRACT_SPEC> {
        AGSR0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Activate Group Service Request Node 1"]
    #[inline(always)]
    #[must_use]
    pub fn agsr1(&mut self) -> AGSR1_W<SRACT_SPEC> {
        AGSR1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Activate Group Service Request Node 2"]
    #[inline(always)]
    #[must_use]
    pub fn agsr2(&mut self) -> AGSR2_W<SRACT_SPEC> {
        AGSR2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Activate Group Service Request Node 3"]
    #[inline(always)]
    #[must_use]
    pub fn agsr3(&mut self) -> AGSR3_W<SRACT_SPEC> {
        AGSR3_W::new(self, 3)
    }
    #[doc = "Bit 8 - Activate Shared Service Request Node 0"]
    #[inline(always)]
    #[must_use]
    pub fn assr0(&mut self) -> ASSR0_W<SRACT_SPEC> {
        ASSR0_W::new(self, 8)
    }
    #[doc = "Bit 9 - Activate Shared Service Request Node 1"]
    #[inline(always)]
    #[must_use]
    pub fn assr1(&mut self) -> ASSR1_W<SRACT_SPEC> {
        ASSR1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Activate Shared Service Request Node 2"]
    #[inline(always)]
    #[must_use]
    pub fn assr2(&mut self) -> ASSR2_W<SRACT_SPEC> {
        ASSR2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Activate Shared Service Request Node 3"]
    #[inline(always)]
    #[must_use]
    pub fn assr3(&mut self) -> ASSR3_W<SRACT_SPEC> {
        ASSR3_W::new(self, 11)
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
#[doc = "Service Request Software Activation Trigger\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sract::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRACT_SPEC;
impl crate::RegisterSpec for SRACT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sract::W`](W) writer structure"]
impl crate::Writable for SRACT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRACT to value 0"]
impl crate::Resettable for SRACT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
