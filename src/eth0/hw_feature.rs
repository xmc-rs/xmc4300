#[doc = "Register `HW_FEATURE` reader"]
pub struct R(crate::R<HW_FEATURE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HW_FEATURE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HW_FEATURE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HW_FEATURE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HW_FEATURE` writer"]
pub struct W(crate::W<HW_FEATURE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HW_FEATURE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<HW_FEATURE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HW_FEATURE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIISEL` reader - 10 or 100 Mbps support"]
pub struct MIISEL_R(crate::FieldReader<bool, bool>);
impl MIISEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MIISEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIISEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GMIISEL` reader - 1000 Mbps support"]
pub struct GMIISEL_R(crate::FieldReader<bool, bool>);
impl GMIISEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        GMIISEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GMIISEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDSEL` reader - Half-Duplex support"]
pub struct HDSEL_R(crate::FieldReader<bool, bool>);
impl HDSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HDSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTHASHEN` reader - Expanded DA Hash Filter"]
pub struct EXTHASHEN_R(crate::FieldReader<bool, bool>);
impl EXTHASHEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTHASHEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTHASHEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HASHSEL` reader - HASH Filter"]
pub struct HASHSEL_R(crate::FieldReader<bool, bool>);
impl HASHSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HASHSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HASHSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDMACADRSEL` reader - Multiple MAC Address Registers"]
pub struct ADDMACADRSEL_R(crate::FieldReader<bool, bool>);
impl ADDMACADRSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDMACADRSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDMACADRSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSSEL` reader - PCS registers (TBI, SGMII, or RTBI PHY interface)"]
pub struct PCSSEL_R(crate::FieldReader<bool, bool>);
impl PCSSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L3L4FLTREN` reader - Layer 3 and Layer 4 Filter Feature"]
pub struct L3L4FLTREN_R(crate::FieldReader<bool, bool>);
impl L3L4FLTREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        L3L4FLTREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L3L4FLTREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMASEL` reader - SMA (MDIO) Interface"]
pub struct SMASEL_R(crate::FieldReader<bool, bool>);
impl SMASEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMASEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMASEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWKSEL` reader - PMT Remote Wakeup"]
pub struct RWKSEL_R(crate::FieldReader<bool, bool>);
impl RWKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWKSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MGKSEL` reader - PMT Magic Packet"]
pub struct MGKSEL_R(crate::FieldReader<bool, bool>);
impl MGKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MGKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MGKSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMCSEL` reader - RMON Module"]
pub struct MMCSEL_R(crate::FieldReader<bool, bool>);
impl MMCSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MMCSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMCSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSVER1SEL` reader - Only IEEE 1588-2002 Timestamp"]
pub struct TSVER1SEL_R(crate::FieldReader<bool, bool>);
impl TSVER1SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSVER1SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSVER1SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSVER2SEL` reader - IEEE 1588-2008 Advanced Timestamp"]
pub struct TSVER2SEL_R(crate::FieldReader<bool, bool>);
impl TSVER2SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSVER2SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSVER2SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEESEL` reader - Energy Efficient Ethernet"]
pub struct EEESEL_R(crate::FieldReader<bool, bool>);
impl EEESEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EEESEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EEESEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVSEL` reader - AV Feature"]
pub struct AVSEL_R(crate::FieldReader<bool, bool>);
impl AVSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        AVSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCOESEL` reader - Checksum Offload in Tx"]
pub struct TXCOESEL_R(crate::FieldReader<bool, bool>);
impl TXCOESEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXCOESEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCOESEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTYP1COE` reader - IP Checksum Offload (Type 1) in Rx"]
pub struct RXTYP1COE_R(crate::FieldReader<bool, bool>);
impl RXTYP1COE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTYP1COE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTYP1COE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTYP2COE` reader - IP Checksum Offload (Type 2) in Rx"]
pub struct RXTYP2COE_R(crate::FieldReader<bool, bool>);
impl RXTYP2COE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTYP2COE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTYP2COE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFOSIZE` reader - Rx FIFO > 2,048 Bytes"]
pub struct RXFIFOSIZE_R(crate::FieldReader<bool, bool>);
impl RXFIFOSIZE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFOSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFOSIZE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFOSIZE` writer - Rx FIFO > 2,048 Bytes"]
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `RXCHCNT` reader - Number of additional Rx channels"]
pub struct RXCHCNT_R(crate::FieldReader<u8, u8>);
impl RXCHCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXCHCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXCHCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCHCNT` reader - Number of additional Tx channels"]
pub struct TXCHCNT_R(crate::FieldReader<u8, u8>);
impl TXCHCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXCHCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCHCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENHDESSEL` reader - Alternate (Enhanced Descriptor)"]
pub struct ENHDESSEL_R(crate::FieldReader<bool, bool>);
impl ENHDESSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENHDESSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENHDESSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTTSEN` reader - Timestamping with Internal System Time"]
pub struct INTTSEN_R(crate::FieldReader<bool, bool>);
impl INTTSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTTSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTTSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXIPPSEN` reader - Flexible Pulse-Per-Second Output"]
pub struct FLEXIPPSEN_R(crate::FieldReader<bool, bool>);
impl FLEXIPPSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLEXIPPSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLEXIPPSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAVLANINS` reader - Source Address or VLAN Insertion"]
pub struct SAVLANINS_R(crate::FieldReader<bool, bool>);
impl SAVLANINS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAVLANINS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAVLANINS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTPHYIF` reader - Active or Selected PHY interface"]
pub struct ACTPHYIF_R(crate::FieldReader<u8, u8>);
impl ACTPHYIF_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACTPHYIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTPHYIF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HW Feature Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hw_feature](index.html) module"]
pub struct HW_FEATURE_SPEC;
impl crate::RegisterSpec for HW_FEATURE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hw_feature::R](R) reader structure"]
impl crate::Readable for HW_FEATURE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hw_feature::W](W) writer structure"]
impl crate::Writable for HW_FEATURE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HW_FEATURE to value 0x0305_2f35"]
impl crate::Resettable for HW_FEATURE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0305_2f35
    }
}
