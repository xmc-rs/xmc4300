#[doc = "Reader of register DC_SYS_TIME_OFFSET[%s]"]
pub type R = crate::R<u32, super::DC_SYS_TIME_OFFSET>;
#[doc = "Writer for register DC_SYS_TIME_OFFSET[%s]"]
pub type W = crate::W<u32, super::DC_SYS_TIME_OFFSET>;
#[doc = "Register DC_SYS_TIME_OFFSET[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::DC_SYS_TIME_OFFSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DC_SYS_TIME_OFFSET`"]
pub type DC_SYS_TIME_OFFSET_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DC_SYS_TIME_OFFSET`"]
pub struct DC_SYS_TIME_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_SYS_TIME_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Difference between local time and System Time"]
    #[inline(always)]
    pub fn dc_sys_time_offset(&self) -> DC_SYS_TIME_OFFSET_R {
        DC_SYS_TIME_OFFSET_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Difference between local time and System Time"]
    #[inline(always)]
    pub fn dc_sys_time_offset(&mut self) -> DC_SYS_TIME_OFFSET_W {
        DC_SYS_TIME_OFFSET_W { w: self }
    }
}
