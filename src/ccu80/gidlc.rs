#[doc = "Register `GIDLC` writer"]
pub type W = crate::W<GidlcSpec>;
#[doc = "Field `CS0I` writer - CC80 IDLE mode clear"]
pub type Cs0iW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS1I` writer - CC81 IDLE mode clear"]
pub type Cs1iW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS2I` writer - CC82 IDLE mode clear"]
pub type Cs2iW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS3I` writer - CC83 IDLE mode clear"]
pub type Cs3iW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPRB` writer - Prescaler Run Bit Set"]
pub type SprbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPCH` writer - Parity Checker run bit set"]
pub type SpchW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - CC80 IDLE mode clear"]
    #[inline(always)]
    #[must_use]
    pub fn cs0i(&mut self) -> Cs0iW<GidlcSpec> {
        Cs0iW::new(self, 0)
    }
    #[doc = "Bit 1 - CC81 IDLE mode clear"]
    #[inline(always)]
    #[must_use]
    pub fn cs1i(&mut self) -> Cs1iW<GidlcSpec> {
        Cs1iW::new(self, 1)
    }
    #[doc = "Bit 2 - CC82 IDLE mode clear"]
    #[inline(always)]
    #[must_use]
    pub fn cs2i(&mut self) -> Cs2iW<GidlcSpec> {
        Cs2iW::new(self, 2)
    }
    #[doc = "Bit 3 - CC83 IDLE mode clear"]
    #[inline(always)]
    #[must_use]
    pub fn cs3i(&mut self) -> Cs3iW<GidlcSpec> {
        Cs3iW::new(self, 3)
    }
    #[doc = "Bit 8 - Prescaler Run Bit Set"]
    #[inline(always)]
    #[must_use]
    pub fn sprb(&mut self) -> SprbW<GidlcSpec> {
        SprbW::new(self, 8)
    }
    #[doc = "Bit 10 - Parity Checker run bit set"]
    #[inline(always)]
    #[must_use]
    pub fn spch(&mut self) -> SpchW<GidlcSpec> {
        SpchW::new(self, 10)
    }
}
#[doc = "Global Idle Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gidlc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GidlcSpec;
impl crate::RegisterSpec for GidlcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gidlc::W`](W) writer structure"]
impl crate::Writable for GidlcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIDLC to value 0"]
impl crate::Resettable for GidlcSpec {
    const RESET_VALUE: u32 = 0;
}
