#[doc = "Reader of register WUB"]
pub type R = crate::R<u32, super::WUB>;
#[doc = "Writer for register WUB"]
pub type W = crate::W<u32, super::WUB>;
#[doc = "Register WUB `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::WUB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `WUB`"]
pub type WUB_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WUB`"]
pub struct WUB_W<'a> {
    w: &'a mut W,
}
impl<'a> WUB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Window Upper Bound"]
    #[inline(always)]
    pub fn wub(&self) -> WUB_R {
        WUB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Window Upper Bound"]
    #[inline(always)]
    pub fn wub(&mut self) -> WUB_W {
        WUB_W { w: self }
    }
}
