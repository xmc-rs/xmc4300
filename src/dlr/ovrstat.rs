#[doc = "Register `OVRSTAT` reader"]
pub type R = crate::R<OvrstatSpec>;
#[doc = "Field `LN0` reader - Line 0 Overrun Status"]
pub type Ln0R = crate::BitReader;
#[doc = "Field `LN1` reader - Line 1 Overrun Status"]
pub type Ln1R = crate::BitReader;
#[doc = "Field `LN2` reader - Line 2 Overrun Status"]
pub type Ln2R = crate::BitReader;
#[doc = "Field `LN3` reader - Line 3 Overrun Status"]
pub type Ln3R = crate::BitReader;
#[doc = "Field `LN4` reader - Line 4 Overrun Status"]
pub type Ln4R = crate::BitReader;
#[doc = "Field `LN5` reader - Line 5 Overrun Status"]
pub type Ln5R = crate::BitReader;
#[doc = "Field `LN6` reader - Line 6 Overrun Status"]
pub type Ln6R = crate::BitReader;
#[doc = "Field `LN7` reader - Line 7 Overrun Status"]
pub type Ln7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Line 0 Overrun Status"]
    #[inline(always)]
    pub fn ln0(&self) -> Ln0R {
        Ln0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Line 1 Overrun Status"]
    #[inline(always)]
    pub fn ln1(&self) -> Ln1R {
        Ln1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Line 2 Overrun Status"]
    #[inline(always)]
    pub fn ln2(&self) -> Ln2R {
        Ln2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Line 3 Overrun Status"]
    #[inline(always)]
    pub fn ln3(&self) -> Ln3R {
        Ln3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line 4 Overrun Status"]
    #[inline(always)]
    pub fn ln4(&self) -> Ln4R {
        Ln4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Line 5 Overrun Status"]
    #[inline(always)]
    pub fn ln5(&self) -> Ln5R {
        Ln5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Line 6 Overrun Status"]
    #[inline(always)]
    pub fn ln6(&self) -> Ln6R {
        Ln6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Line 7 Overrun Status"]
    #[inline(always)]
    pub fn ln7(&self) -> Ln7R {
        Ln7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Overrun Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ovrstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OvrstatSpec;
impl crate::RegisterSpec for OvrstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ovrstat::R`](R) reader structure"]
impl crate::Readable for OvrstatSpec {}
#[doc = "`reset()` method sets OVRSTAT to value 0"]
impl crate::Resettable for OvrstatSpec {
    const RESET_VALUE: u32 = 0;
}
