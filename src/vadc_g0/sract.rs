#[doc = "Register `SRACT` writer"]
pub struct W(crate::W<SRACT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRACT_SPEC>;
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
impl From<crate::W<SRACT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRACT_SPEC>) -> Self {
        W(writer)
    }
}
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
pub type AGSR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRACT_SPEC, AGSR0_AW, O>;
impl<'a, const O: u8> AGSR0_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AGSR0_AW::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
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
pub type AGSR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRACT_SPEC, AGSR1_AW, O>;
impl<'a, const O: u8> AGSR1_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AGSR1_AW::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
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
pub type AGSR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRACT_SPEC, AGSR2_AW, O>;
impl<'a, const O: u8> AGSR2_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AGSR2_AW::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
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
pub type AGSR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRACT_SPEC, AGSR3_AW, O>;
impl<'a, const O: u8> AGSR3_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AGSR3_AW::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
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
pub type ASSR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRACT_SPEC, ASSR0_AW, O>;
impl<'a, const O: u8> ASSR0_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSR0_AW::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
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
pub type ASSR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRACT_SPEC, ASSR1_AW, O>;
impl<'a, const O: u8> ASSR1_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSR1_AW::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
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
pub type ASSR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRACT_SPEC, ASSR2_AW, O>;
impl<'a, const O: u8> ASSR2_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSR2_AW::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
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
pub type ASSR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRACT_SPEC, ASSR3_AW, O>;
impl<'a, const O: u8> ASSR3_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSR3_AW::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSR3_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Activate Group Service Request Node 0"]
    #[inline(always)]
    #[must_use]
    pub fn agsr0(&mut self) -> AGSR0_W<0> {
        AGSR0_W::new(self)
    }
    #[doc = "Bit 1 - Activate Group Service Request Node 1"]
    #[inline(always)]
    #[must_use]
    pub fn agsr1(&mut self) -> AGSR1_W<1> {
        AGSR1_W::new(self)
    }
    #[doc = "Bit 2 - Activate Group Service Request Node 2"]
    #[inline(always)]
    #[must_use]
    pub fn agsr2(&mut self) -> AGSR2_W<2> {
        AGSR2_W::new(self)
    }
    #[doc = "Bit 3 - Activate Group Service Request Node 3"]
    #[inline(always)]
    #[must_use]
    pub fn agsr3(&mut self) -> AGSR3_W<3> {
        AGSR3_W::new(self)
    }
    #[doc = "Bit 8 - Activate Shared Service Request Node 0"]
    #[inline(always)]
    #[must_use]
    pub fn assr0(&mut self) -> ASSR0_W<8> {
        ASSR0_W::new(self)
    }
    #[doc = "Bit 9 - Activate Shared Service Request Node 1"]
    #[inline(always)]
    #[must_use]
    pub fn assr1(&mut self) -> ASSR1_W<9> {
        ASSR1_W::new(self)
    }
    #[doc = "Bit 10 - Activate Shared Service Request Node 2"]
    #[inline(always)]
    #[must_use]
    pub fn assr2(&mut self) -> ASSR2_W<10> {
        ASSR2_W::new(self)
    }
    #[doc = "Bit 11 - Activate Shared Service Request Node 3"]
    #[inline(always)]
    #[must_use]
    pub fn assr3(&mut self) -> ASSR3_W<11> {
        ASSR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Service Request Software Activation Trigger\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sract](index.html) module"]
pub struct SRACT_SPEC;
impl crate::RegisterSpec for SRACT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sract::W](W) writer structure"]
impl crate::Writable for SRACT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRACT to value 0"]
impl crate::Resettable for SRACT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
