#[doc = "Register `GMII_ADDRESS` reader"]
pub type R = crate::R<GmiiAddressSpec>;
#[doc = "Register `GMII_ADDRESS` writer"]
pub type W = crate::W<GmiiAddressSpec>;
#[doc = "Field `MB` reader - MII Busy"]
pub type MbR = crate::BitReader;
#[doc = "Field `MB` writer - MII Busy"]
pub type MbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MW` reader - MII Write"]
pub type MwR = crate::BitReader;
#[doc = "Field `MW` writer - MII Write"]
pub type MwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR` reader - CSR Clock Range"]
pub type CrR = crate::FieldReader;
#[doc = "Field `CR` writer - CSR Clock Range"]
pub type CrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MR` reader - MII Register"]
pub type MrR = crate::FieldReader;
#[doc = "Field `MR` writer - MII Register"]
pub type MrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PA` reader - Physical Layer Address"]
pub type PaR = crate::FieldReader;
#[doc = "Field `PA` writer - Physical Layer Address"]
pub type PaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - MII Busy"]
    #[inline(always)]
    pub fn mb(&self) -> MbR {
        MbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MII Write"]
    #[inline(always)]
    pub fn mw(&self) -> MwR {
        MwR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - CSR Clock Range"]
    #[inline(always)]
    pub fn cr(&self) -> CrR {
        CrR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:10 - MII Register"]
    #[inline(always)]
    pub fn mr(&self) -> MrR {
        MrR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Physical Layer Address"]
    #[inline(always)]
    pub fn pa(&self) -> PaR {
        PaR::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MII Busy"]
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MbW<GmiiAddressSpec> {
        MbW::new(self, 0)
    }
    #[doc = "Bit 1 - MII Write"]
    #[inline(always)]
    #[must_use]
    pub fn mw(&mut self) -> MwW<GmiiAddressSpec> {
        MwW::new(self, 1)
    }
    #[doc = "Bits 2:5 - CSR Clock Range"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CrW<GmiiAddressSpec> {
        CrW::new(self, 2)
    }
    #[doc = "Bits 6:10 - MII Register"]
    #[inline(always)]
    #[must_use]
    pub fn mr(&mut self) -> MrW<GmiiAddressSpec> {
        MrW::new(self, 6)
    }
    #[doc = "Bits 11:15 - Physical Layer Address"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PaW<GmiiAddressSpec> {
        PaW::new(self, 11)
    }
}
#[doc = "MII Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmii_address::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmii_address::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmiiAddressSpec;
impl crate::RegisterSpec for GmiiAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmii_address::R`](R) reader structure"]
impl crate::Readable for GmiiAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`gmii_address::W`](W) writer structure"]
impl crate::Writable for GmiiAddressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMII_ADDRESS to value 0"]
impl crate::Resettable for GmiiAddressSpec {
    const RESET_VALUE: u32 = 0;
}
