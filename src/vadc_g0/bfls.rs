#[doc = "Register `BFLS` writer"]
pub struct W(crate::W<BFLS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BFLS_SPEC>;
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
impl From<crate::W<BFLS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BFLS_SPEC>) -> Self {
        W(writer)
    }
}
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
pub type BFC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BFLS_SPEC, BFC0_AW, O>;
impl<'a, const O: u8> BFC0_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFC0_AW::VALUE1)
    }
    #[doc = "Clear bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
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
pub type BFC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BFLS_SPEC, BFC1_AW, O>;
impl<'a, const O: u8> BFC1_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFC1_AW::VALUE1)
    }
    #[doc = "Clear bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
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
pub type BFC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BFLS_SPEC, BFC2_AW, O>;
impl<'a, const O: u8> BFC2_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFC2_AW::VALUE1)
    }
    #[doc = "Clear bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
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
pub type BFC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, BFLS_SPEC, BFC3_AW, O>;
impl<'a, const O: u8> BFC3_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFC3_AW::VALUE1)
    }
    #[doc = "Clear bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
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
pub type BFS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BFLS_SPEC, BFS0_AW, O>;
impl<'a, const O: u8> BFS0_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFS0_AW::VALUE1)
    }
    #[doc = "Set bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
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
pub type BFS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BFLS_SPEC, BFS1_AW, O>;
impl<'a, const O: u8> BFS1_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFS1_AW::VALUE1)
    }
    #[doc = "Set bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
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
pub type BFS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BFLS_SPEC, BFS2_AW, O>;
impl<'a, const O: u8> BFS2_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFS2_AW::VALUE1)
    }
    #[doc = "Set bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
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
pub type BFS3_W<'a, const O: u8> = crate::BitWriter<'a, u32, BFLS_SPEC, BFS3_AW, O>;
impl<'a, const O: u8> BFS3_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFS3_AW::VALUE1)
    }
    #[doc = "Set bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFS3_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Boundary Flag 0 Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bfc0(&mut self) -> BFC0_W<0> {
        BFC0_W::new(self)
    }
    #[doc = "Bit 1 - Boundary Flag 1 Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bfc1(&mut self) -> BFC1_W<1> {
        BFC1_W::new(self)
    }
    #[doc = "Bit 2 - Boundary Flag 2 Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bfc2(&mut self) -> BFC2_W<2> {
        BFC2_W::new(self)
    }
    #[doc = "Bit 3 - Boundary Flag 3 Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bfc3(&mut self) -> BFC3_W<3> {
        BFC3_W::new(self)
    }
    #[doc = "Bit 16 - Boundary Flag 0 Set"]
    #[inline(always)]
    #[must_use]
    pub fn bfs0(&mut self) -> BFS0_W<16> {
        BFS0_W::new(self)
    }
    #[doc = "Bit 17 - Boundary Flag 1 Set"]
    #[inline(always)]
    #[must_use]
    pub fn bfs1(&mut self) -> BFS1_W<17> {
        BFS1_W::new(self)
    }
    #[doc = "Bit 18 - Boundary Flag 2 Set"]
    #[inline(always)]
    #[must_use]
    pub fn bfs2(&mut self) -> BFS2_W<18> {
        BFS2_W::new(self)
    }
    #[doc = "Bit 19 - Boundary Flag 3 Set"]
    #[inline(always)]
    #[must_use]
    pub fn bfs3(&mut self) -> BFS3_W<19> {
        BFS3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Boundary Flag Software Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bfls](index.html) module"]
pub struct BFLS_SPEC;
impl crate::RegisterSpec for BFLS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [bfls::W](W) writer structure"]
impl crate::Writable for BFLS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BFLS to value 0"]
impl crate::Resettable for BFLS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
