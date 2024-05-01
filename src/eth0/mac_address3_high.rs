#[doc = "Register `MAC_ADDRESS3_HIGH` reader"]
pub type R = crate::R<MAC_ADDRESS3_HIGH_SPEC>;
#[doc = "Register `MAC_ADDRESS3_HIGH` writer"]
pub type W = crate::W<MAC_ADDRESS3_HIGH_SPEC>;
#[doc = "Field `ADDRHI` reader - MAC Address3 \\[47:32\\]"]
pub type ADDRHI_R = crate::FieldReader<u16>;
#[doc = "Field `ADDRHI` writer - MAC Address3 \\[47:32\\]"]
pub type ADDRHI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBC` reader - Mask Byte Control"]
pub type MBC_R = crate::FieldReader;
#[doc = "Field `MBC` writer - Mask Byte Control"]
pub type MBC_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SA` reader - Source Address"]
pub type SA_R = crate::BitReader;
#[doc = "Field `SA` writer - Source Address"]
pub type SA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE` reader - Address Enable"]
pub type AE_R = crate::BitReader;
#[doc = "Field `AE` writer - Address Enable"]
pub type AE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - MAC Address3 \\[47:32\\]"]
    #[inline(always)]
    pub fn addrhi(&self) -> ADDRHI_R {
        ADDRHI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - Mask Byte Control"]
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Source Address"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Address Enable"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Address3 \\[47:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn addrhi(&mut self) -> ADDRHI_W<MAC_ADDRESS3_HIGH_SPEC> {
        ADDRHI_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - Mask Byte Control"]
    #[inline(always)]
    #[must_use]
    pub fn mbc(&mut self) -> MBC_W<MAC_ADDRESS3_HIGH_SPEC> {
        MBC_W::new(self, 24)
    }
    #[doc = "Bit 30 - Source Address"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<MAC_ADDRESS3_HIGH_SPEC> {
        SA_W::new(self, 30)
    }
    #[doc = "Bit 31 - Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae(&mut self) -> AE_W<MAC_ADDRESS3_HIGH_SPEC> {
        AE_W::new(self, 31)
    }
}
#[doc = "MAC Address3 High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_address3_high::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_address3_high::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_ADDRESS3_HIGH_SPEC;
impl crate::RegisterSpec for MAC_ADDRESS3_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_address3_high::R`](R) reader structure"]
impl crate::Readable for MAC_ADDRESS3_HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_address3_high::W`](W) writer structure"]
impl crate::Writable for MAC_ADDRESS3_HIGH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAC_ADDRESS3_HIGH to value 0xffff"]
impl crate::Resettable for MAC_ADDRESS3_HIGH_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}
