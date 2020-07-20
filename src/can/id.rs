#[doc = "Reader of register ID"]
pub type R = crate::R<u32, super::ID>;
#[doc = "Reader of field `MOD_REV`"]
pub type MOD_REV_R = crate::R<u8, u8>;
#[doc = "Module Type\n\nValue on reset: 192"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MOD_TYPE_A {
    #[doc = "192: Define the module as a 32-bit module."]
    VALUE1 = 192,
}
impl From<MOD_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: MOD_TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MOD_TYPE`"]
pub type MOD_TYPE_R = crate::R<u8, MOD_TYPE_A>;
impl MOD_TYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MOD_TYPE_A> {
        use crate::Variant::*;
        match self.bits {
            192 => Val(MOD_TYPE_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MOD_TYPE_A::VALUE1
    }
}
#[doc = "Reader of field `MOD_NUMBER`"]
pub type MOD_NUMBER_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:7 - Module Revision Number"]
    #[inline(always)]
    pub fn mod_rev(&self) -> MOD_REV_R {
        MOD_REV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Module Type"]
    #[inline(always)]
    pub fn mod_type(&self) -> MOD_TYPE_R {
        MOD_TYPE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Module Number Value"]
    #[inline(always)]
    pub fn mod_number(&self) -> MOD_NUMBER_R {
        MOD_NUMBER_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
