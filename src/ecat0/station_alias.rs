#[doc = "Reader of register STATION_ALIAS"]
pub type R = crate::R<u16, super::STATION_ALIAS>;
#[doc = "Writer for register STATION_ALIAS"]
pub type W = crate::W<u16, super::STATION_ALIAS>;
#[doc = "Register STATION_ALIAS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATION_ALIAS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ALIAS_ADDR`"]
pub type ALIAS_ADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ALIAS_ADDR`"]
pub struct ALIAS_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIAS_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Alias Address used for node addressing(FPxx commands)"]
    #[inline(always)]
    pub fn alias_addr(&self) -> ALIAS_ADDR_R {
        ALIAS_ADDR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Alias Address used for node addressing(FPxx commands)"]
    #[inline(always)]
    pub fn alias_addr(&mut self) -> ALIAS_ADDR_W {
        ALIAS_ADDR_W { w: self }
    }
}
