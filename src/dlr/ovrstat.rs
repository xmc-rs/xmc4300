#[doc = "Reader of register OVRSTAT"]
pub type R = crate::R<u32, super::OVRSTAT>;
#[doc = "Reader of field `LN0`"]
pub type LN0_R = crate::R<bool, bool>;
#[doc = "Reader of field `LN1`"]
pub type LN1_R = crate::R<bool, bool>;
#[doc = "Reader of field `LN2`"]
pub type LN2_R = crate::R<bool, bool>;
#[doc = "Reader of field `LN3`"]
pub type LN3_R = crate::R<bool, bool>;
#[doc = "Reader of field `LN4`"]
pub type LN4_R = crate::R<bool, bool>;
#[doc = "Reader of field `LN5`"]
pub type LN5_R = crate::R<bool, bool>;
#[doc = "Reader of field `LN6`"]
pub type LN6_R = crate::R<bool, bool>;
#[doc = "Reader of field `LN7`"]
pub type LN7_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Line 0 Overrun Status"]
    #[inline(always)]
    pub fn ln0(&self) -> LN0_R {
        LN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Line 1 Overrun Status"]
    #[inline(always)]
    pub fn ln1(&self) -> LN1_R {
        LN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Line 2 Overrun Status"]
    #[inline(always)]
    pub fn ln2(&self) -> LN2_R {
        LN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Line 3 Overrun Status"]
    #[inline(always)]
    pub fn ln3(&self) -> LN3_R {
        LN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Line 4 Overrun Status"]
    #[inline(always)]
    pub fn ln4(&self) -> LN4_R {
        LN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Line 5 Overrun Status"]
    #[inline(always)]
    pub fn ln5(&self) -> LN5_R {
        LN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Line 6 Overrun Status"]
    #[inline(always)]
    pub fn ln6(&self) -> LN6_R {
        LN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Line 7 Overrun Status"]
    #[inline(always)]
    pub fn ln7(&self) -> LN7_R {
        LN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
