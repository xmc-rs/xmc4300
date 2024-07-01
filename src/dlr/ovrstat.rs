#[doc = "Register `OVRSTAT` reader"]
pub type R = crate::R<OVRSTAT_SPEC>;
#[doc = "Field `LN0` reader - Line 0 Overrun Status"]
pub type LN0_R = crate::BitReader;
#[doc = "Field `LN1` reader - Line 1 Overrun Status"]
pub type LN1_R = crate::BitReader;
#[doc = "Field `LN2` reader - Line 2 Overrun Status"]
pub type LN2_R = crate::BitReader;
#[doc = "Field `LN3` reader - Line 3 Overrun Status"]
pub type LN3_R = crate::BitReader;
#[doc = "Field `LN4` reader - Line 4 Overrun Status"]
pub type LN4_R = crate::BitReader;
#[doc = "Field `LN5` reader - Line 5 Overrun Status"]
pub type LN5_R = crate::BitReader;
#[doc = "Field `LN6` reader - Line 6 Overrun Status"]
pub type LN6_R = crate::BitReader;
#[doc = "Field `LN7` reader - Line 7 Overrun Status"]
pub type LN7_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Line 0 Overrun Status"]
    #[inline(always)]
    pub fn ln0(&self) -> LN0_R {
        LN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Line 1 Overrun Status"]
    #[inline(always)]
    pub fn ln1(&self) -> LN1_R {
        LN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Line 2 Overrun Status"]
    #[inline(always)]
    pub fn ln2(&self) -> LN2_R {
        LN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Line 3 Overrun Status"]
    #[inline(always)]
    pub fn ln3(&self) -> LN3_R {
        LN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line 4 Overrun Status"]
    #[inline(always)]
    pub fn ln4(&self) -> LN4_R {
        LN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Line 5 Overrun Status"]
    #[inline(always)]
    pub fn ln5(&self) -> LN5_R {
        LN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Line 6 Overrun Status"]
    #[inline(always)]
    pub fn ln6(&self) -> LN6_R {
        LN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Line 7 Overrun Status"]
    #[inline(always)]
    pub fn ln7(&self) -> LN7_R {
        LN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Overrun Status\n\nYou can [`read`](crate::Reg::read) this register and get [`ovrstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OVRSTAT_SPEC;
impl crate::RegisterSpec for OVRSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ovrstat::R`](R) reader structure"]
impl crate::Readable for OVRSTAT_SPEC {}
#[doc = "`reset()` method sets OVRSTAT to value 0"]
impl crate::Resettable for OVRSTAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
