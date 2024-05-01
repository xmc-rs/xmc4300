#[doc = "Register `CURRENT_HOST_TRANSMIT_DESCRIPTOR` reader"]
pub type R = crate::R<CurrentHostTransmitDescriptorSpec>;
#[doc = "Field `CURTDESAPTR` reader - Host Transmit Descriptor Address Pointer"]
pub type CurtdesaptrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Descriptor Address Pointer"]
    #[inline(always)]
    pub fn curtdesaptr(&self) -> CurtdesaptrR {
        CurtdesaptrR::new(self.bits)
    }
}
#[doc = "Current Host Transmit Descriptor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`current_host_transmit_descriptor::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrentHostTransmitDescriptorSpec;
impl crate::RegisterSpec for CurrentHostTransmitDescriptorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`current_host_transmit_descriptor::R`](R) reader structure"]
impl crate::Readable for CurrentHostTransmitDescriptorSpec {}
#[doc = "`reset()` method sets CURRENT_HOST_TRANSMIT_DESCRIPTOR to value 0"]
impl crate::Resettable for CurrentHostTransmitDescriptorSpec {
    const RESET_VALUE: u32 = 0;
}
