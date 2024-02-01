#[doc = "Register `GMII_ADDRESS` reader"]
pub type R = crate::R<GMII_ADDRESS_SPEC>;
#[doc = "Register `GMII_ADDRESS` writer"]
pub type W = crate::W<GMII_ADDRESS_SPEC>;
#[doc = "Field `MB` reader - MII Busy"]
pub type MB_R = crate::BitReader;
#[doc = "Field `MB` writer - MII Busy"]
pub type MB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MW` reader - MII Write"]
pub type MW_R = crate::BitReader;
#[doc = "Field `MW` writer - MII Write"]
pub type MW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR` reader - CSR Clock Range"]
pub type CR_R = crate::FieldReader;
#[doc = "Field `CR` writer - CSR Clock Range"]
pub type CR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MR` reader - MII Register"]
pub type MR_R = crate::FieldReader;
#[doc = "Field `MR` writer - MII Register"]
pub type MR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PA` reader - Physical Layer Address"]
pub type PA_R = crate::FieldReader;
#[doc = "Field `PA` writer - Physical Layer Address"]
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - MII Busy"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MII Write"]
    #[inline(always)]
    pub fn mw(&self) -> MW_R {
        MW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - CSR Clock Range"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:10 - MII Register"]
    #[inline(always)]
    pub fn mr(&self) -> MR_R {
        MR_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Physical Layer Address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MII Busy"]
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MB_W<GMII_ADDRESS_SPEC> {
        MB_W::new(self, 0)
    }
    #[doc = "Bit 1 - MII Write"]
    #[inline(always)]
    #[must_use]
    pub fn mw(&mut self) -> MW_W<GMII_ADDRESS_SPEC> {
        MW_W::new(self, 1)
    }
    #[doc = "Bits 2:5 - CSR Clock Range"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<GMII_ADDRESS_SPEC> {
        CR_W::new(self, 2)
    }
    #[doc = "Bits 6:10 - MII Register"]
    #[inline(always)]
    #[must_use]
    pub fn mr(&mut self) -> MR_W<GMII_ADDRESS_SPEC> {
        MR_W::new(self, 6)
    }
    #[doc = "Bits 11:15 - Physical Layer Address"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<GMII_ADDRESS_SPEC> {
        PA_W::new(self, 11)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MII Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmii_address::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmii_address::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GMII_ADDRESS_SPEC;
impl crate::RegisterSpec for GMII_ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmii_address::R`](R) reader structure"]
impl crate::Readable for GMII_ADDRESS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gmii_address::W`](W) writer structure"]
impl crate::Writable for GMII_ADDRESS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMII_ADDRESS to value 0"]
impl crate::Resettable for GMII_ADDRESS_SPEC {
    const RESET_VALUE: u32 = 0;
}
