#[doc = "Reader of register RECEIVE_DESCRIPTOR_LIST_ADDRESS"]
pub type R = crate::R<u32, super::RECEIVE_DESCRIPTOR_LIST_ADDRESS>;
#[doc = "Writer for register RECEIVE_DESCRIPTOR_LIST_ADDRESS"]
pub type W = crate::W<u32, super::RECEIVE_DESCRIPTOR_LIST_ADDRESS>;
#[doc = "Register RECEIVE_DESCRIPTOR_LIST_ADDRESS `reset()`'s with value 0"]
impl crate::ResetValue for super::RECEIVE_DESCRIPTOR_LIST_ADDRESS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RDESLA_32bit`"]
pub type RDESLA_32BIT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RDESLA_32bit`"]
pub struct RDESLA_32BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RDESLA_32BIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Start of Receive List"]
    #[inline(always)]
    pub fn rdesla_32bit(&self) -> RDESLA_32BIT_R {
        RDESLA_32BIT_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Start of Receive List"]
    #[inline(always)]
    pub fn rdesla_32bit(&mut self) -> RDESLA_32BIT_W {
        RDESLA_32BIT_W { w: self }
    }
}
