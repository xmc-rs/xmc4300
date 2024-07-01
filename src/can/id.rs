#[doc = "Register `ID` reader"]
pub type R = crate::R<ID_SPEC>;
#[doc = "Field `MOD_REV` reader - Module Revision Number"]
pub type MOD_REV_R = crate::FieldReader;
#[doc = "Module Type\n\nValue on reset: 192"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for MOD_TYPE_A {
    type Ux = u8;
}
impl crate::IsEnum for MOD_TYPE_A {}
#[doc = "Field `MOD_TYPE` reader - Module Type"]
pub type MOD_TYPE_R = crate::FieldReader<MOD_TYPE_A>;
impl MOD_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MOD_TYPE_A> {
        match self.bits {
            192 => Some(MOD_TYPE_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Define the module as a 32-bit module."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MOD_TYPE_A::VALUE1
    }
}
#[doc = "Field `MOD_NUMBER` reader - Module Number Value"]
pub type MOD_NUMBER_R = crate::FieldReader<u16>;
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
#[doc = "Module Identification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ID_SPEC;
impl crate::RegisterSpec for ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id::R`](R) reader structure"]
impl crate::Readable for ID_SPEC {}
#[doc = "`reset()` method sets ID to value 0x00b5_c000"]
impl crate::Resettable for ID_SPEC {
    const RESET_VALUE: u32 = 0x00b5_c000;
}
