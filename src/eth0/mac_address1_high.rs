#[doc = "Register `MAC_ADDRESS1_HIGH` reader"]
pub type R = crate::R<MacAddress1HighSpec>;
#[doc = "Register `MAC_ADDRESS1_HIGH` writer"]
pub type W = crate::W<MacAddress1HighSpec>;
#[doc = "Field `ADDRHI` reader - MAC Address1 \\[47:32\\]"]
pub type AddrhiR = crate::FieldReader<u16>;
#[doc = "Field `ADDRHI` writer - MAC Address1 \\[47:32\\]"]
pub type AddrhiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBC` reader - Mask Byte Control"]
pub type MbcR = crate::FieldReader;
#[doc = "Field `MBC` writer - Mask Byte Control"]
pub type MbcW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SA` reader - Source Address"]
pub type SaR = crate::BitReader;
#[doc = "Field `SA` writer - Source Address"]
pub type SaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE` reader - Address Enable"]
pub type AeR = crate::BitReader;
#[doc = "Field `AE` writer - Address Enable"]
pub type AeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - MAC Address1 \\[47:32\\]"]
    #[inline(always)]
    pub fn addrhi(&self) -> AddrhiR {
        AddrhiR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - Mask Byte Control"]
    #[inline(always)]
    pub fn mbc(&self) -> MbcR {
        MbcR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Source Address"]
    #[inline(always)]
    pub fn sa(&self) -> SaR {
        SaR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Address Enable"]
    #[inline(always)]
    pub fn ae(&self) -> AeR {
        AeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Address1 \\[47:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn addrhi(&mut self) -> AddrhiW<MacAddress1HighSpec> {
        AddrhiW::new(self, 0)
    }
    #[doc = "Bits 24:29 - Mask Byte Control"]
    #[inline(always)]
    #[must_use]
    pub fn mbc(&mut self) -> MbcW<MacAddress1HighSpec> {
        MbcW::new(self, 24)
    }
    #[doc = "Bit 30 - Source Address"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SaW<MacAddress1HighSpec> {
        SaW::new(self, 30)
    }
    #[doc = "Bit 31 - Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae(&mut self) -> AeW<MacAddress1HighSpec> {
        AeW::new(self, 31)
    }
}
#[doc = "MAC Address1 High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_address1_high::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_address1_high::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacAddress1HighSpec;
impl crate::RegisterSpec for MacAddress1HighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_address1_high::R`](R) reader structure"]
impl crate::Readable for MacAddress1HighSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_address1_high::W`](W) writer structure"]
impl crate::Writable for MacAddress1HighSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAC_ADDRESS1_HIGH to value 0xffff"]
impl crate::Resettable for MacAddress1HighSpec {
    const RESET_VALUE: u32 = 0xffff;
}
