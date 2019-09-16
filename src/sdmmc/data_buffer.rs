#[doc = "Reader of register DATA_BUFFER"]
pub type R = crate::R<u32, super::DATA_BUFFER>;
#[doc = "Writer for register DATA_BUFFER"]
pub type W = crate::W<u32, super::DATA_BUFFER>;
#[doc = "Register DATA_BUFFER `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA_BUFFER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_BUFFER`"]
pub type DATA_BUFFER_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DATA_BUFFER`"]
pub struct DATA_BUFFER_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BUFFER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data Buffer"]
    #[inline(always)]
    pub fn data_buffer(&self) -> DATA_BUFFER_R {
        DATA_BUFFER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Buffer"]
    #[inline(always)]
    pub fn data_buffer(&mut self) -> DATA_BUFFER_W {
        DATA_BUFFER_W { w: self }
    }
}
