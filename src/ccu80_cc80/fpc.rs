#[doc = "Reader of register FPC"]
pub type R = crate::R<u32, super::FPC>;
#[doc = "Writer for register FPC"]
pub type W = crate::W<u32, super::FPC>;
#[doc = "Register FPC `reset()`'s with value 0"]
impl crate::ResetValue for super::FPC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCMP`"]
pub type PCMP_R = crate::R<u8, u8>;
#[doc = "Reader of field `PVAL`"]
pub type PVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PVAL`"]
pub struct PVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Floating Prescaler Compare Value"]
    #[inline(always)]
    pub fn pcmp(&self) -> PCMP_R {
        PCMP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Actual Prescaler Value"]
    #[inline(always)]
    pub fn pval(&self) -> PVAL_R {
        PVAL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - Actual Prescaler Value"]
    #[inline(always)]
    pub fn pval(&mut self) -> PVAL_W {
        PVAL_W { w: self }
    }
}
