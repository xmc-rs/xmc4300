#[doc = "Register `EVRVADCSTAT` reader"]
pub type R = crate::R<EvrvadcstatSpec>;
#[doc = "Field `VADC13V` reader - VADC 1.3 V Conversion Result"]
pub type Vadc13vR = crate::FieldReader;
#[doc = "Field `VADC33V` reader - VADC 3.3 V Conversion Result"]
pub type Vadc33vR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - VADC 1.3 V Conversion Result"]
    #[inline(always)]
    pub fn vadc13v(&self) -> Vadc13vR {
        Vadc13vR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - VADC 3.3 V Conversion Result"]
    #[inline(always)]
    pub fn vadc33v(&self) -> Vadc33vR {
        Vadc33vR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "EVR VADC Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evrvadcstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvrvadcstatSpec;
impl crate::RegisterSpec for EvrvadcstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evrvadcstat::R`](R) reader structure"]
impl crate::Readable for EvrvadcstatSpec {}
#[doc = "`reset()` method sets EVRVADCSTAT to value 0"]
impl crate::Resettable for EvrvadcstatSpec {
    const RESET_VALUE: u32 = 0;
}
