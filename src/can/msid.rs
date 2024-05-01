#[doc = "Register `MSID[%s]` reader"]
pub type R = crate::R<MsidSpec>;
#[doc = "Field `INDEX` reader - Message Pending Index"]
pub type IndexR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Message Pending Index"]
    #[inline(always)]
    pub fn index(&self) -> IndexR {
        IndexR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Message Index Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsidSpec;
impl crate::RegisterSpec for MsidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msid::R`](R) reader structure"]
impl crate::Readable for MsidSpec {}
#[doc = "`reset()` method sets MSID[%s]
to value 0x20"]
impl crate::Resettable for MsidSpec {
    const RESET_VALUE: u32 = 0x20;
}
