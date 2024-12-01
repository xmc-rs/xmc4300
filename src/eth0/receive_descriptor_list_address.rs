#[doc = "Register `RECEIVE_DESCRIPTOR_LIST_ADDRESS` reader"]
pub type R = crate::R<RECEIVE_DESCRIPTOR_LIST_ADDRESS_SPEC>;
#[doc = "Register `RECEIVE_DESCRIPTOR_LIST_ADDRESS` writer"]
pub type W = crate::W<RECEIVE_DESCRIPTOR_LIST_ADDRESS_SPEC>;
#[doc = "Field `RDESLA_32bit` reader - Start of Receive List"]
pub type RDESLA_32BIT_R = crate::FieldReader<u32>;
#[doc = "Field `RDESLA_32bit` writer - Start of Receive List"]
pub type RDESLA_32BIT_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Start of Receive List"]
    #[inline(always)]
    pub fn rdesla_32bit(&self) -> RDESLA_32BIT_R {
        RDESLA_32BIT_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Start of Receive List"]
    #[inline(always)]
    pub fn rdesla_32bit(&mut self) -> RDESLA_32BIT_W<RECEIVE_DESCRIPTOR_LIST_ADDRESS_SPEC> {
        RDESLA_32BIT_W::new(self, 2)
    }
}
#[doc = "Receive Descriptor Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`receive_descriptor_list_address::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`receive_descriptor_list_address::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RECEIVE_DESCRIPTOR_LIST_ADDRESS_SPEC;
impl crate::RegisterSpec for RECEIVE_DESCRIPTOR_LIST_ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`receive_descriptor_list_address::R`](R) reader structure"]
impl crate::Readable for RECEIVE_DESCRIPTOR_LIST_ADDRESS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`receive_descriptor_list_address::W`](W) writer structure"]
impl crate::Writable for RECEIVE_DESCRIPTOR_LIST_ADDRESS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RECEIVE_DESCRIPTOR_LIST_ADDRESS to value 0"]
impl crate::Resettable for RECEIVE_DESCRIPTOR_LIST_ADDRESS_SPEC {
    const RESET_VALUE: u32 = 0;
}
