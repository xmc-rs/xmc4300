#[doc = "Register `MAC_ADDRESS0_HIGH` reader"]
pub type R = crate::R<MAC_ADDRESS0_HIGH_SPEC>;
#[doc = "Register `MAC_ADDRESS0_HIGH` writer"]
pub type W = crate::W<MAC_ADDRESS0_HIGH_SPEC>;
#[doc = "Field `ADDRHI` reader - MAC Address0 \\[47:32\\]"]
pub type ADDRHI_R = crate::FieldReader<u16>;
#[doc = "Field `ADDRHI` writer - MAC Address0 \\[47:32\\]"]
pub type ADDRHI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `AE` reader - Address Enable"]
pub type AE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - MAC Address0 \\[47:32\\]"]
    #[inline(always)]
    pub fn addrhi(&self) -> ADDRHI_R {
        ADDRHI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Address Enable"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Address0 \\[47:32\\]"]
    #[inline(always)]
    pub fn addrhi(&mut self) -> ADDRHI_W<MAC_ADDRESS0_HIGH_SPEC> {
        ADDRHI_W::new(self, 0)
    }
}
#[doc = "MAC Address0 High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_address0_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_address0_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_ADDRESS0_HIGH_SPEC;
impl crate::RegisterSpec for MAC_ADDRESS0_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_address0_high::R`](R) reader structure"]
impl crate::Readable for MAC_ADDRESS0_HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_address0_high::W`](W) writer structure"]
impl crate::Writable for MAC_ADDRESS0_HIGH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAC_ADDRESS0_HIGH to value 0x8000_ffff"]
impl crate::Resettable for MAC_ADDRESS0_HIGH_SPEC {
    const RESET_VALUE: u32 = 0x8000_ffff;
}
