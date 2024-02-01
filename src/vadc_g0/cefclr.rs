#[doc = "Register `CEFCLR` writer"]
pub type W = crate::W<CEFCLR_SPEC>;
#[doc = "Clear Channel Event for Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV0_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the channel event flag in GxCEFLAG"]
    VALUE2 = 1,
}
impl From<CEV0_AW> for bool {
    #[inline(always)]
    fn from(variant: CEV0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV0` writer - Clear Channel Event for Channel 0"]
pub type CEV0_W<'a, REG> = crate::BitWriter<'a, REG, CEV0_AW>;
impl<'a, REG> CEV0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV0_AW::VALUE1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV0_AW::VALUE2)
    }
}
#[doc = "Clear Channel Event for Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV1_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the channel event flag in GxCEFLAG"]
    VALUE2 = 1,
}
impl From<CEV1_AW> for bool {
    #[inline(always)]
    fn from(variant: CEV1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV1` writer - Clear Channel Event for Channel 1"]
pub type CEV1_W<'a, REG> = crate::BitWriter<'a, REG, CEV1_AW>;
impl<'a, REG> CEV1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV1_AW::VALUE1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV1_AW::VALUE2)
    }
}
#[doc = "Clear Channel Event for Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV2_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the channel event flag in GxCEFLAG"]
    VALUE2 = 1,
}
impl From<CEV2_AW> for bool {
    #[inline(always)]
    fn from(variant: CEV2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV2` writer - Clear Channel Event for Channel 2"]
pub type CEV2_W<'a, REG> = crate::BitWriter<'a, REG, CEV2_AW>;
impl<'a, REG> CEV2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV2_AW::VALUE1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV2_AW::VALUE2)
    }
}
#[doc = "Clear Channel Event for Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV3_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the channel event flag in GxCEFLAG"]
    VALUE2 = 1,
}
impl From<CEV3_AW> for bool {
    #[inline(always)]
    fn from(variant: CEV3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV3` writer - Clear Channel Event for Channel 3"]
pub type CEV3_W<'a, REG> = crate::BitWriter<'a, REG, CEV3_AW>;
impl<'a, REG> CEV3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV3_AW::VALUE1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV3_AW::VALUE2)
    }
}
#[doc = "Clear Channel Event for Channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV4_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the channel event flag in GxCEFLAG"]
    VALUE2 = 1,
}
impl From<CEV4_AW> for bool {
    #[inline(always)]
    fn from(variant: CEV4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV4` writer - Clear Channel Event for Channel 4"]
pub type CEV4_W<'a, REG> = crate::BitWriter<'a, REG, CEV4_AW>;
impl<'a, REG> CEV4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV4_AW::VALUE1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV4_AW::VALUE2)
    }
}
#[doc = "Clear Channel Event for Channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV5_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the channel event flag in GxCEFLAG"]
    VALUE2 = 1,
}
impl From<CEV5_AW> for bool {
    #[inline(always)]
    fn from(variant: CEV5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV5` writer - Clear Channel Event for Channel 5"]
pub type CEV5_W<'a, REG> = crate::BitWriter<'a, REG, CEV5_AW>;
impl<'a, REG> CEV5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV5_AW::VALUE1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV5_AW::VALUE2)
    }
}
#[doc = "Clear Channel Event for Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV6_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the channel event flag in GxCEFLAG"]
    VALUE2 = 1,
}
impl From<CEV6_AW> for bool {
    #[inline(always)]
    fn from(variant: CEV6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV6` writer - Clear Channel Event for Channel 6"]
pub type CEV6_W<'a, REG> = crate::BitWriter<'a, REG, CEV6_AW>;
impl<'a, REG> CEV6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV6_AW::VALUE1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV6_AW::VALUE2)
    }
}
#[doc = "Clear Channel Event for Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV7_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the channel event flag in GxCEFLAG"]
    VALUE2 = 1,
}
impl From<CEV7_AW> for bool {
    #[inline(always)]
    fn from(variant: CEV7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV7` writer - Clear Channel Event for Channel 7"]
pub type CEV7_W<'a, REG> = crate::BitWriter<'a, REG, CEV7_AW>;
impl<'a, REG> CEV7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV7_AW::VALUE1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV7_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Channel Event for Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn cev0(&mut self) -> CEV0_W<CEFCLR_SPEC> {
        CEV0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Channel Event for Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn cev1(&mut self) -> CEV1_W<CEFCLR_SPEC> {
        CEV1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear Channel Event for Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn cev2(&mut self) -> CEV2_W<CEFCLR_SPEC> {
        CEV2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Channel Event for Channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn cev3(&mut self) -> CEV3_W<CEFCLR_SPEC> {
        CEV3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear Channel Event for Channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn cev4(&mut self) -> CEV4_W<CEFCLR_SPEC> {
        CEV4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear Channel Event for Channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn cev5(&mut self) -> CEV5_W<CEFCLR_SPEC> {
        CEV5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear Channel Event for Channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn cev6(&mut self) -> CEV6_W<CEFCLR_SPEC> {
        CEV6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear Channel Event for Channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn cev7(&mut self) -> CEV7_W<CEFCLR_SPEC> {
        CEV7_W::new(self, 7)
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
#[doc = "Channel Event Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cefclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CEFCLR_SPEC;
impl crate::RegisterSpec for CEFCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cefclr::W`](W) writer structure"]
impl crate::Writable for CEFCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CEFCLR to value 0"]
impl crate::Resettable for CEFCLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
