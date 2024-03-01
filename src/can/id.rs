#[doc = "Register `ID` reader"]
pub type R = crate::R<IdSpec>;
#[doc = "Field `MOD_REV` reader - Module Revision Number"]
pub type ModRevR = crate::FieldReader;
#[doc = "Module Type\n\nValue on reset: 192"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ModType {
    #[doc = "192: Define the module as a 32-bit module."]
    Value1 = 192,
}
impl From<ModType> for u8 {
    #[inline(always)]
    fn from(variant: ModType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ModType {
    type Ux = u8;
}
#[doc = "Field `MOD_TYPE` reader - Module Type"]
pub type ModTypeR = crate::FieldReader<ModType>;
impl ModTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ModType> {
        match self.bits {
            192 => Some(ModType::Value1),
            _ => None,
        }
    }
    #[doc = "Define the module as a 32-bit module."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ModType::Value1
    }
}
#[doc = "Field `MOD_NUMBER` reader - Module Number Value"]
pub type ModNumberR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Module Revision Number"]
    #[inline(always)]
    pub fn mod_rev(&self) -> ModRevR {
        ModRevR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Module Type"]
    #[inline(always)]
    pub fn mod_type(&self) -> ModTypeR {
        ModTypeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Module Number Value"]
    #[inline(always)]
    pub fn mod_number(&self) -> ModNumberR {
        ModNumberR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Module Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdSpec;
impl crate::RegisterSpec for IdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id::R`](R) reader structure"]
impl crate::Readable for IdSpec {}
#[doc = "`reset()` method sets ID to value 0x00b5_c000"]
impl crate::Resettable for IdSpec {
    const RESET_VALUE: u32 = 0x00b5_c000;
}
