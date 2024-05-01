#[doc = "Register `BFLS` writer"]
pub type W = crate::W<BflsSpec>;
#[doc = "Boundary Flag 0 Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfc0 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear bit BFLy"]
    Value2 = 1,
}
impl From<Bfc0> for bool {
    #[inline(always)]
    fn from(variant: Bfc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFC0` writer - Boundary Flag 0 Clear"]
pub type Bfc0W<'a, REG> = crate::BitWriter<'a, REG, Bfc0>;
impl<'a, REG> Bfc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfc0::Value1)
    }
    #[doc = "Clear bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfc0::Value2)
    }
}
#[doc = "Boundary Flag 1 Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfc1 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear bit BFLy"]
    Value2 = 1,
}
impl From<Bfc1> for bool {
    #[inline(always)]
    fn from(variant: Bfc1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFC1` writer - Boundary Flag 1 Clear"]
pub type Bfc1W<'a, REG> = crate::BitWriter<'a, REG, Bfc1>;
impl<'a, REG> Bfc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfc1::Value1)
    }
    #[doc = "Clear bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfc1::Value2)
    }
}
#[doc = "Boundary Flag 2 Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfc2 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear bit BFLy"]
    Value2 = 1,
}
impl From<Bfc2> for bool {
    #[inline(always)]
    fn from(variant: Bfc2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFC2` writer - Boundary Flag 2 Clear"]
pub type Bfc2W<'a, REG> = crate::BitWriter<'a, REG, Bfc2>;
impl<'a, REG> Bfc2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfc2::Value1)
    }
    #[doc = "Clear bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfc2::Value2)
    }
}
#[doc = "Boundary Flag 3 Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfc3 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear bit BFLy"]
    Value2 = 1,
}
impl From<Bfc3> for bool {
    #[inline(always)]
    fn from(variant: Bfc3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFC3` writer - Boundary Flag 3 Clear"]
pub type Bfc3W<'a, REG> = crate::BitWriter<'a, REG, Bfc3>;
impl<'a, REG> Bfc3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfc3::Value1)
    }
    #[doc = "Clear bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfc3::Value2)
    }
}
#[doc = "Boundary Flag 0 Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfs0 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Set bit BFLy"]
    Value2 = 1,
}
impl From<Bfs0> for bool {
    #[inline(always)]
    fn from(variant: Bfs0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFS0` writer - Boundary Flag 0 Set"]
pub type Bfs0W<'a, REG> = crate::BitWriter<'a, REG, Bfs0>;
impl<'a, REG> Bfs0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfs0::Value1)
    }
    #[doc = "Set bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfs0::Value2)
    }
}
#[doc = "Boundary Flag 1 Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfs1 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Set bit BFLy"]
    Value2 = 1,
}
impl From<Bfs1> for bool {
    #[inline(always)]
    fn from(variant: Bfs1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFS1` writer - Boundary Flag 1 Set"]
pub type Bfs1W<'a, REG> = crate::BitWriter<'a, REG, Bfs1>;
impl<'a, REG> Bfs1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfs1::Value1)
    }
    #[doc = "Set bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfs1::Value2)
    }
}
#[doc = "Boundary Flag 2 Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfs2 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Set bit BFLy"]
    Value2 = 1,
}
impl From<Bfs2> for bool {
    #[inline(always)]
    fn from(variant: Bfs2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFS2` writer - Boundary Flag 2 Set"]
pub type Bfs2W<'a, REG> = crate::BitWriter<'a, REG, Bfs2>;
impl<'a, REG> Bfs2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfs2::Value1)
    }
    #[doc = "Set bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfs2::Value2)
    }
}
#[doc = "Boundary Flag 3 Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfs3 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Set bit BFLy"]
    Value2 = 1,
}
impl From<Bfs3> for bool {
    #[inline(always)]
    fn from(variant: Bfs3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFS3` writer - Boundary Flag 3 Set"]
pub type Bfs3W<'a, REG> = crate::BitWriter<'a, REG, Bfs3>;
impl<'a, REG> Bfs3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfs3::Value1)
    }
    #[doc = "Set bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfs3::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - Boundary Flag 0 Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bfc0(&mut self) -> Bfc0W<BflsSpec> {
        Bfc0W::new(self, 0)
    }
    #[doc = "Bit 1 - Boundary Flag 1 Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bfc1(&mut self) -> Bfc1W<BflsSpec> {
        Bfc1W::new(self, 1)
    }
    #[doc = "Bit 2 - Boundary Flag 2 Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bfc2(&mut self) -> Bfc2W<BflsSpec> {
        Bfc2W::new(self, 2)
    }
    #[doc = "Bit 3 - Boundary Flag 3 Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bfc3(&mut self) -> Bfc3W<BflsSpec> {
        Bfc3W::new(self, 3)
    }
    #[doc = "Bit 16 - Boundary Flag 0 Set"]
    #[inline(always)]
    #[must_use]
    pub fn bfs0(&mut self) -> Bfs0W<BflsSpec> {
        Bfs0W::new(self, 16)
    }
    #[doc = "Bit 17 - Boundary Flag 1 Set"]
    #[inline(always)]
    #[must_use]
    pub fn bfs1(&mut self) -> Bfs1W<BflsSpec> {
        Bfs1W::new(self, 17)
    }
    #[doc = "Bit 18 - Boundary Flag 2 Set"]
    #[inline(always)]
    #[must_use]
    pub fn bfs2(&mut self) -> Bfs2W<BflsSpec> {
        Bfs2W::new(self, 18)
    }
    #[doc = "Bit 19 - Boundary Flag 3 Set"]
    #[inline(always)]
    #[must_use]
    pub fn bfs3(&mut self) -> Bfs3W<BflsSpec> {
        Bfs3W::new(self, 19)
    }
}
#[doc = "Boundary Flag Software Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bfls::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BflsSpec;
impl crate::RegisterSpec for BflsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bfls::W`](W) writer structure"]
impl crate::Writable for BflsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BFLS to value 0"]
impl crate::Resettable for BflsSpec {
    const RESET_VALUE: u32 = 0;
}
