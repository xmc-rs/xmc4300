#[doc = "Register `HW_FEATURE` reader"]
pub type R = crate::R<HW_FEATURE_SPEC>;
#[doc = "Register `HW_FEATURE` writer"]
pub type W = crate::W<HW_FEATURE_SPEC>;
#[doc = "Field `MIISEL` reader - 10 or 100 Mbps support"]
pub type MIISEL_R = crate::BitReader;
#[doc = "Field `GMIISEL` reader - 1000 Mbps support"]
pub type GMIISEL_R = crate::BitReader;
#[doc = "Field `HDSEL` reader - Half-Duplex support"]
pub type HDSEL_R = crate::BitReader;
#[doc = "Field `EXTHASHEN` reader - Expanded DA Hash Filter"]
pub type EXTHASHEN_R = crate::BitReader;
#[doc = "Field `HASHSEL` reader - HASH Filter"]
pub type HASHSEL_R = crate::BitReader;
#[doc = "Field `ADDMACADRSEL` reader - Multiple MAC Address Registers"]
pub type ADDMACADRSEL_R = crate::BitReader;
#[doc = "Field `PCSSEL` reader - PCS registers (TBI, SGMII, or RTBI PHY interface)"]
pub type PCSSEL_R = crate::BitReader;
#[doc = "Field `L3L4FLTREN` reader - Layer 3 and Layer 4 Filter Feature"]
pub type L3L4FLTREN_R = crate::BitReader;
#[doc = "Field `SMASEL` reader - SMA (MDIO) Interface"]
pub type SMASEL_R = crate::BitReader;
#[doc = "Field `RWKSEL` reader - PMT Remote Wakeup"]
pub type RWKSEL_R = crate::BitReader;
#[doc = "Field `MGKSEL` reader - PMT Magic Packet"]
pub type MGKSEL_R = crate::BitReader;
#[doc = "Field `MMCSEL` reader - RMON Module"]
pub type MMCSEL_R = crate::BitReader;
#[doc = "Field `TSVER1SEL` reader - Only IEEE 1588-2002 Timestamp"]
pub type TSVER1SEL_R = crate::BitReader;
#[doc = "Field `TSVER2SEL` reader - IEEE 1588-2008 Advanced Timestamp"]
pub type TSVER2SEL_R = crate::BitReader;
#[doc = "Field `EEESEL` reader - Energy Efficient Ethernet"]
pub type EEESEL_R = crate::BitReader;
#[doc = "Field `AVSEL` reader - AV Feature"]
pub type AVSEL_R = crate::BitReader;
#[doc = "Field `TXCOESEL` reader - Checksum Offload in Tx"]
pub type TXCOESEL_R = crate::BitReader;
#[doc = "Field `RXTYP1COE` reader - IP Checksum Offload (Type 1) in Rx"]
pub type RXTYP1COE_R = crate::BitReader;
#[doc = "Field `RXTYP2COE` reader - IP Checksum Offload (Type 2) in Rx"]
pub type RXTYP2COE_R = crate::BitReader;
#[doc = "Field `RXFIFOSIZE` reader - Rx FIFO > 2,048 Bytes"]
pub type RXFIFOSIZE_R = crate::BitReader;
#[doc = "Field `RXFIFOSIZE` writer - Rx FIFO > 2,048 Bytes"]
pub type RXFIFOSIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCHCNT` reader - Number of additional Rx channels"]
pub type RXCHCNT_R = crate::FieldReader;
#[doc = "Field `TXCHCNT` reader - Number of additional Tx channels"]
pub type TXCHCNT_R = crate::FieldReader;
#[doc = "Field `ENHDESSEL` reader - Alternate (Enhanced Descriptor)"]
pub type ENHDESSEL_R = crate::BitReader;
#[doc = "Field `INTTSEN` reader - Timestamping with Internal System Time"]
pub type INTTSEN_R = crate::BitReader;
#[doc = "Field `FLEXIPPSEN` reader - Flexible Pulse-Per-Second Output"]
pub type FLEXIPPSEN_R = crate::BitReader;
#[doc = "Field `SAVLANINS` reader - Source Address or VLAN Insertion"]
pub type SAVLANINS_R = crate::BitReader;
#[doc = "Field `ACTPHYIF` reader - Active or Selected PHY interface"]
pub type ACTPHYIF_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 10 or 100 Mbps support"]
    #[inline(always)]
    pub fn miisel(&self) -> MIISEL_R {
        MIISEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1000 Mbps support"]
    #[inline(always)]
    pub fn gmiisel(&self) -> GMIISEL_R {
        GMIISEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Half-Duplex support"]
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Expanded DA Hash Filter"]
    #[inline(always)]
    pub fn exthashen(&self) -> EXTHASHEN_R {
        EXTHASHEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HASH Filter"]
    #[inline(always)]
    pub fn hashsel(&self) -> HASHSEL_R {
        HASHSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multiple MAC Address Registers"]
    #[inline(always)]
    pub fn addmacadrsel(&self) -> ADDMACADRSEL_R {
        ADDMACADRSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PCS registers (TBI, SGMII, or RTBI PHY interface)"]
    #[inline(always)]
    pub fn pcssel(&self) -> PCSSEL_R {
        PCSSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Layer 3 and Layer 4 Filter Feature"]
    #[inline(always)]
    pub fn l3l4fltren(&self) -> L3L4FLTREN_R {
        L3L4FLTREN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SMA (MDIO) Interface"]
    #[inline(always)]
    pub fn smasel(&self) -> SMASEL_R {
        SMASEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PMT Remote Wakeup"]
    #[inline(always)]
    pub fn rwksel(&self) -> RWKSEL_R {
        RWKSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PMT Magic Packet"]
    #[inline(always)]
    pub fn mgksel(&self) -> MGKSEL_R {
        MGKSEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RMON Module"]
    #[inline(always)]
    pub fn mmcsel(&self) -> MMCSEL_R {
        MMCSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Only IEEE 1588-2002 Timestamp"]
    #[inline(always)]
    pub fn tsver1sel(&self) -> TSVER1SEL_R {
        TSVER1SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - IEEE 1588-2008 Advanced Timestamp"]
    #[inline(always)]
    pub fn tsver2sel(&self) -> TSVER2SEL_R {
        TSVER2SEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Energy Efficient Ethernet"]
    #[inline(always)]
    pub fn eeesel(&self) -> EEESEL_R {
        EEESEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AV Feature"]
    #[inline(always)]
    pub fn avsel(&self) -> AVSEL_R {
        AVSEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Checksum Offload in Tx"]
    #[inline(always)]
    pub fn txcoesel(&self) -> TXCOESEL_R {
        TXCOESEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - IP Checksum Offload (Type 1) in Rx"]
    #[inline(always)]
    pub fn rxtyp1coe(&self) -> RXTYP1COE_R {
        RXTYP1COE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IP Checksum Offload (Type 2) in Rx"]
    #[inline(always)]
    pub fn rxtyp2coe(&self) -> RXTYP2COE_R {
        RXTYP2COE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Rx FIFO > 2,048 Bytes"]
    #[inline(always)]
    pub fn rxfifosize(&self) -> RXFIFOSIZE_R {
        RXFIFOSIZE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Number of additional Rx channels"]
    #[inline(always)]
    pub fn rxchcnt(&self) -> RXCHCNT_R {
        RXCHCNT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Number of additional Tx channels"]
    #[inline(always)]
    pub fn txchcnt(&self) -> TXCHCNT_R {
        TXCHCNT_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Alternate (Enhanced Descriptor)"]
    #[inline(always)]
    pub fn enhdessel(&self) -> ENHDESSEL_R {
        ENHDESSEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Timestamping with Internal System Time"]
    #[inline(always)]
    pub fn inttsen(&self) -> INTTSEN_R {
        INTTSEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Flexible Pulse-Per-Second Output"]
    #[inline(always)]
    pub fn flexippsen(&self) -> FLEXIPPSEN_R {
        FLEXIPPSEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Source Address or VLAN Insertion"]
    #[inline(always)]
    pub fn savlanins(&self) -> SAVLANINS_R {
        SAVLANINS_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Active or Selected PHY interface"]
    #[inline(always)]
    pub fn actphyif(&self) -> ACTPHYIF_R {
        ACTPHYIF_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 19 - Rx FIFO > 2,048 Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifosize(&mut self) -> RXFIFOSIZE_W<HW_FEATURE_SPEC> {
        RXFIFOSIZE_W::new(self, 19)
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
#[doc = "HW Feature Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hw_feature::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hw_feature::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HW_FEATURE_SPEC;
impl crate::RegisterSpec for HW_FEATURE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_feature::R`](R) reader structure"]
impl crate::Readable for HW_FEATURE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hw_feature::W`](W) writer structure"]
impl crate::Writable for HW_FEATURE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HW_FEATURE to value 0x0305_2f35"]
impl crate::Resettable for HW_FEATURE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0305_2f35;
}
