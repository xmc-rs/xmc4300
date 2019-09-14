#[doc = "Reader of register HW_FEATURE"]
pub type R = crate::R<u32, super::HW_FEATURE>;
#[doc = "Writer for register HW_FEATURE"]
pub type W = crate::W<u32, super::HW_FEATURE>;
#[doc = "Register HW_FEATURE `reset()`'s with value 0x0305_2f35"]
impl crate::ResetValue for super::HW_FEATURE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0305_2f35
    }
}
#[doc = "Reader of field `MIISEL`"]
pub type MIISEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `GMIISEL`"]
pub type GMIISEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `HDSEL`"]
pub type HDSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTHASHEN`"]
pub type EXTHASHEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `HASHSEL`"]
pub type HASHSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADDMACADRSEL`"]
pub type ADDMACADRSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `PCSSEL`"]
pub type PCSSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `L3L4FLTREN`"]
pub type L3L4FLTREN_R = crate::R<bool, bool>;
#[doc = "Reader of field `SMASEL`"]
pub type SMASEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RWKSEL`"]
pub type RWKSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `MGKSEL`"]
pub type MGKSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `MMCSEL`"]
pub type MMCSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSVER1SEL`"]
pub type TSVER1SEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSVER2SEL`"]
pub type TSVER2SEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `EEESEL`"]
pub type EEESEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `AVSEL`"]
pub type AVSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXCOESEL`"]
pub type TXCOESEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXTYP1COE`"]
pub type RXTYP1COE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXTYP2COE`"]
pub type RXTYP2COE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFIFOSIZE`"]
pub type RXFIFOSIZE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFIFOSIZE`"]
pub struct RXFIFOSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFOSIZE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `RXCHCNT`"]
pub type RXCHCNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `TXCHCNT`"]
pub type TXCHCNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `ENHDESSEL`"]
pub type ENHDESSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTTSEN`"]
pub type INTTSEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLEXIPPSEN`"]
pub type FLEXIPPSEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `SAVLANINS`"]
pub type SAVLANINS_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTPHYIF`"]
pub type ACTPHYIF_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - 10 or 100 Mbps support"]
    #[inline(always)]
    pub fn miisel(&self) -> MIISEL_R {
        MIISEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1000 Mbps support"]
    #[inline(always)]
    pub fn gmiisel(&self) -> GMIISEL_R {
        GMIISEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Half-Duplex support"]
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Expanded DA Hash Filter"]
    #[inline(always)]
    pub fn exthashen(&self) -> EXTHASHEN_R {
        EXTHASHEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HASH Filter"]
    #[inline(always)]
    pub fn hashsel(&self) -> HASHSEL_R {
        HASHSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Multiple MAC Address Registers"]
    #[inline(always)]
    pub fn addmacadrsel(&self) -> ADDMACADRSEL_R {
        ADDMACADRSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PCS registers (TBI, SGMII, or RTBI PHY interface)"]
    #[inline(always)]
    pub fn pcssel(&self) -> PCSSEL_R {
        PCSSEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Layer 3 and Layer 4 Filter Feature"]
    #[inline(always)]
    pub fn l3l4fltren(&self) -> L3L4FLTREN_R {
        L3L4FLTREN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SMA (MDIO) Interface"]
    #[inline(always)]
    pub fn smasel(&self) -> SMASEL_R {
        SMASEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PMT Remote Wakeup"]
    #[inline(always)]
    pub fn rwksel(&self) -> RWKSEL_R {
        RWKSEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PMT Magic Packet"]
    #[inline(always)]
    pub fn mgksel(&self) -> MGKSEL_R {
        MGKSEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RMON Module"]
    #[inline(always)]
    pub fn mmcsel(&self) -> MMCSEL_R {
        MMCSEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Only IEEE 1588-2002 Timestamp"]
    #[inline(always)]
    pub fn tsver1sel(&self) -> TSVER1SEL_R {
        TSVER1SEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - IEEE 1588-2008 Advanced Timestamp"]
    #[inline(always)]
    pub fn tsver2sel(&self) -> TSVER2SEL_R {
        TSVER2SEL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Energy Efficient Ethernet"]
    #[inline(always)]
    pub fn eeesel(&self) -> EEESEL_R {
        EEESEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - AV Feature"]
    #[inline(always)]
    pub fn avsel(&self) -> AVSEL_R {
        AVSEL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Checksum Offload in Tx"]
    #[inline(always)]
    pub fn txcoesel(&self) -> TXCOESEL_R {
        TXCOESEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - IP Checksum Offload (Type 1) in Rx"]
    #[inline(always)]
    pub fn rxtyp1coe(&self) -> RXTYP1COE_R {
        RXTYP1COE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IP Checksum Offload (Type 2) in Rx"]
    #[inline(always)]
    pub fn rxtyp2coe(&self) -> RXTYP2COE_R {
        RXTYP2COE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Rx FIFO > 2,048 Bytes"]
    #[inline(always)]
    pub fn rxfifosize(&self) -> RXFIFOSIZE_R {
        RXFIFOSIZE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Number of additional Rx channels"]
    #[inline(always)]
    pub fn rxchcnt(&self) -> RXCHCNT_R {
        RXCHCNT_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Number of additional Tx channels"]
    #[inline(always)]
    pub fn txchcnt(&self) -> TXCHCNT_R {
        TXCHCNT_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 24 - Alternate (Enhanced Descriptor)"]
    #[inline(always)]
    pub fn enhdessel(&self) -> ENHDESSEL_R {
        ENHDESSEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Timestamping with Internal System Time"]
    #[inline(always)]
    pub fn inttsen(&self) -> INTTSEN_R {
        INTTSEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Flexible Pulse-Per-Second Output"]
    #[inline(always)]
    pub fn flexippsen(&self) -> FLEXIPPSEN_R {
        FLEXIPPSEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Source Address or VLAN Insertion"]
    #[inline(always)]
    pub fn savlanins(&self) -> SAVLANINS_R {
        SAVLANINS_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - Active or Selected PHY interface"]
    #[inline(always)]
    pub fn actphyif(&self) -> ACTPHYIF_R {
        ACTPHYIF_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 19 - Rx FIFO > 2,048 Bytes"]
    #[inline(always)]
    pub fn rxfifosize(&mut self) -> RXFIFOSIZE_W {
        RXFIFOSIZE_W { w: self }
    }
}
