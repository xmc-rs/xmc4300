#[doc = "Register `BFLS` writer"]
pub type W = crate::W<BFLS_SPEC>;
#[doc = "Boundary Flag 0 Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFC0_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit BFLy"]
    VALUE2 = 1,
}
impl From<BFC0_AW> for bool {
    #[inline(always)]
    fn from(variant: BFC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFC0` writer - Boundary Flag 0 Clear"]
pub type BFC0_W<'a, REG> = crate::BitWriter<'a, REG, BFC0_AW>;
impl<'a, REG> BFC0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFC0_AW::VALUE1)
    }
    #[doc = "Clear bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFC0_AW::VALUE2)
    }
}
#[doc = "Boundary Flag 1 Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFC1_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit BFLy"]
    VALUE2 = 1,
}
impl From<BFC1_AW> for bool {
    #[inline(always)]
    fn from(variant: BFC1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFC1` writer - Boundary Flag 1 Clear"]
pub type BFC1_W<'a, REG> = crate::BitWriter<'a, REG, BFC1_AW>;
impl<'a, REG> BFC1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFC1_AW::VALUE1)
    }
    #[doc = "Clear bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFC1_AW::VALUE2)
    }
}
#[doc = "Boundary Flag 2 Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFC2_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit BFLy"]
    VALUE2 = 1,
}
impl From<BFC2_AW> for bool {
    #[inline(always)]
    fn from(variant: BFC2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFC2` writer - Boundary Flag 2 Clear"]
pub type BFC2_W<'a, REG> = crate::BitWriter<'a, REG, BFC2_AW>;
impl<'a, REG> BFC2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFC2_AW::VALUE1)
    }
    #[doc = "Clear bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFC2_AW::VALUE2)
    }
}
#[doc = "Boundary Flag 3 Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFC3_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit BFLy"]
    VALUE2 = 1,
}
impl From<BFC3_AW> for bool {
    #[inline(always)]
    fn from(variant: BFC3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFC3` writer - Boundary Flag 3 Clear"]
pub type BFC3_W<'a, REG> = crate::BitWriter<'a, REG, BFC3_AW>;
impl<'a, REG> BFC3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFC3_AW::VALUE1)
    }
    #[doc = "Clear bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFC3_AW::VALUE2)
    }
}
#[doc = "Boundary Flag 0 Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFS0_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Set bit BFLy"]
    VALUE2 = 1,
}
impl From<BFS0_AW> for bool {
    #[inline(always)]
    fn from(variant: BFS0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFS0` writer - Boundary Flag 0 Set"]
pub type BFS0_W<'a, REG> = crate::BitWriter<'a, REG, BFS0_AW>;
impl<'a, REG> BFS0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFS0_AW::VALUE1)
    }
    #[doc = "Set bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFS0_AW::VALUE2)
    }
}
#[doc = "Boundary Flag 1 Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFS1_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Set bit BFLy"]
    VALUE2 = 1,
}
impl From<BFS1_AW> for bool {
    #[inline(always)]
    fn from(variant: BFS1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFS1` writer - Boundary Flag 1 Set"]
pub type BFS1_W<'a, REG> = crate::BitWriter<'a, REG, BFS1_AW>;
impl<'a, REG> BFS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFS1_AW::VALUE1)
    }
    #[doc = "Set bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFS1_AW::VALUE2)
    }
}
#[doc = "Boundary Flag 2 Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFS2_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Set bit BFLy"]
    VALUE2 = 1,
}
impl From<BFS2_AW> for bool {
    #[inline(always)]
    fn from(variant: BFS2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFS2` writer - Boundary Flag 2 Set"]
pub type BFS2_W<'a, REG> = crate::BitWriter<'a, REG, BFS2_AW>;
impl<'a, REG> BFS2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFS2_AW::VALUE1)
    }
    #[doc = "Set bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFS2_AW::VALUE2)
    }
}
#[doc = "Boundary Flag 3 Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFS3_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Set bit BFLy"]
    VALUE2 = 1,
}
impl From<BFS3_AW> for bool {
    #[inline(always)]
    fn from(variant: BFS3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFS3` writer - Boundary Flag 3 Set"]
pub type BFS3_W<'a, REG> = crate::BitWriter<'a, REG, BFS3_AW>;
impl<'a, REG> BFS3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFS3_AW::VALUE1)
    }
    #[doc = "Set bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFS3_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Boundary Flag 0 Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bfc0(&mut self) -> BFC0_W<BFLS_SPEC> {
        BFC0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Boundary Flag 1 Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bfc1(&mut self) -> BFC1_W<BFLS_SPEC> {
        BFC1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Boundary Flag 2 Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bfc2(&mut self) -> BFC2_W<BFLS_SPEC> {
        BFC2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Boundary Flag 3 Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bfc3(&mut self) -> BFC3_W<BFLS_SPEC> {
        BFC3_W::new(self, 3)
    }
    #[doc = "Bit 16 - Boundary Flag 0 Set"]
    #[inline(always)]
    #[must_use]
    pub fn bfs0(&mut self) -> BFS0_W<BFLS_SPEC> {
        BFS0_W::new(self, 16)
    }
    #[doc = "Bit 17 - Boundary Flag 1 Set"]
    #[inline(always)]
    #[must_use]
    pub fn bfs1(&mut self) -> BFS1_W<BFLS_SPEC> {
        BFS1_W::new(self, 17)
    }
    #[doc = "Bit 18 - Boundary Flag 2 Set"]
    #[inline(always)]
    #[must_use]
    pub fn bfs2(&mut self) -> BFS2_W<BFLS_SPEC> {
        BFS2_W::new(self, 18)
    }
    #[doc = "Bit 19 - Boundary Flag 3 Set"]
    #[inline(always)]
    #[must_use]
    pub fn bfs3(&mut self) -> BFS3_W<BFLS_SPEC> {
        BFS3_W::new(self, 19)
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
#[doc = "Boundary Flag Software Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bfls::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BFLS_SPEC;
impl crate::RegisterSpec for BFLS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bfls::W`](W) writer structure"]
impl crate::Writable for BFLS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BFLS to value 0"]
impl crate::Resettable for BFLS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
