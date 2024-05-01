#[doc = "Register `RECEIVE_DESCRIPTOR_LIST_ADDRESS` reader"]
pub type R = crate::R<ReceiveDescriptorListAddressSpec>;
#[doc = "Register `RECEIVE_DESCRIPTOR_LIST_ADDRESS` writer"]
pub type W = crate::W<ReceiveDescriptorListAddressSpec>;
#[doc = "Field `RDESLA_32bit` reader - Start of Receive List"]
pub type Rdesla32bitR = crate::FieldReader<u32>;
#[doc = "Field `RDESLA_32bit` writer - Start of Receive List"]
pub type Rdesla32bitW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Start of Receive List"]
    #[inline(always)]
    pub fn rdesla_32bit(&self) -> Rdesla32bitR {
        Rdesla32bitR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Start of Receive List"]
    #[inline(always)]
    #[must_use]
    pub fn rdesla_32bit(&mut self) -> Rdesla32bitW<ReceiveDescriptorListAddressSpec> {
        Rdesla32bitW::new(self, 2)
    }
}
#[doc = "Receive Descriptor Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_descriptor_list_address::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_descriptor_list_address::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReceiveDescriptorListAddressSpec;
impl crate::RegisterSpec for ReceiveDescriptorListAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`receive_descriptor_list_address::R`](R) reader structure"]
impl crate::Readable for ReceiveDescriptorListAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`receive_descriptor_list_address::W`](W) writer structure"]
impl crate::Writable for ReceiveDescriptorListAddressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RECEIVE_DESCRIPTOR_LIST_ADDRESS to value 0"]
impl crate::Resettable for ReceiveDescriptorListAddressSpec {
    const RESET_VALUE: u32 = 0;
}
