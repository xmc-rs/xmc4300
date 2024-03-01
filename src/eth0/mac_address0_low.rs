#[doc = "Register `MAC_ADDRESS0_LOW` reader"]
pub type R = crate::R<MacAddress0LowSpec>;
#[doc = "Register `MAC_ADDRESS0_LOW` writer"]
pub type W = crate::W<MacAddress0LowSpec>;
#[doc = "Field `ADDRLO` reader - MAC Address0 \\[31:0\\]"]
pub type AddrloR = crate::FieldReader<u32>;
#[doc = "Field `ADDRLO` writer - MAC Address0 \\[31:0\\]"]
pub type AddrloW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC Address0 \\[31:0\\]"]
    #[inline(always)]
    pub fn addrlo(&self) -> AddrloR {
        AddrloR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC Address0 \\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn addrlo(&mut self) -> AddrloW<MacAddress0LowSpec> {
        AddrloW::new(self, 0)
    }
}
#[doc = "MAC Address0 Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_address0_low::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_address0_low::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacAddress0LowSpec;
impl crate::RegisterSpec for MacAddress0LowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_address0_low::R`](R) reader structure"]
impl crate::Readable for MacAddress0LowSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_address0_low::W`](W) writer structure"]
impl crate::Writable for MacAddress0LowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAC_ADDRESS0_LOW to value 0xffff_ffff"]
impl crate::Resettable for MacAddress0LowSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
