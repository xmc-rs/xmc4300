#[doc = "Reader of register LENGTH"]
pub type R = crate::R<u32, super::LENGTH>;
#[doc = "Writer for register LENGTH"]
pub type W = crate::W<u32, super::LENGTH>;
#[doc = "Register LENGTH `reset()`'s with value 0"]
impl crate::ResetValue for super::LENGTH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LENGTH`"]
pub type LENGTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LENGTH`"]
pub struct LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Message Length Register"]
    #[inline(always)]
    pub fn length(&self) -> LENGTH_R {
        LENGTH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Message Length Register"]
    #[inline(always)]
    pub fn length(&mut self) -> LENGTH_W {
        LENGTH_W { w: self }
    }
}
