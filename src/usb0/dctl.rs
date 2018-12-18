#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCTL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct RMTWKUPSIGR {
    bits: bool,
}
impl RMTWKUPSIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `SftDiscon`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SFTDISCONR {
    #[doc = "Normal operation. When this bit is cleared after a soft disconnect, the core drives a device connect event to the USB host. When the device is reconnected, the USB host restarts device enumeration."]
    VALUE1,
    #[doc = "The core drives a device disconnect event to the USB host."]
    VALUE2,
}
impl SFTDISCONR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SFTDISCONR::VALUE1 => false,
            SFTDISCONR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SFTDISCONR {
        match value {
            false => SFTDISCONR::VALUE1,
            true => SFTDISCONR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SFTDISCONR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SFTDISCONR::VALUE2
    }
}
#[doc = "Possible values of the field `GNPINNakSts`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GNPINNAKSTSR {
    #[doc = "A handshake is sent out based on the data availability in the transmit FIFO."]
    VALUE1,
    #[doc = "A NAK handshake is sent out on all non-periodic IN endpoints, irrespective of the data availability in the transmit FIFO."]
    VALUE2,
}
impl GNPINNAKSTSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GNPINNAKSTSR::VALUE1 => false,
            GNPINNAKSTSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GNPINNAKSTSR {
        match value {
            false => GNPINNAKSTSR::VALUE1,
            true => GNPINNAKSTSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == GNPINNAKSTSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == GNPINNAKSTSR::VALUE2
    }
}
#[doc = "Possible values of the field `GOUTNakSts`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GOUTNAKSTSR {
    #[doc = "A handshake is sent based on the FIFO Status and the NAK and STALL bit settings."]
    VALUE1,
    #[doc = "No data is written to the RxFIFO, irrespective of space availability. Sends a NAK handshake on all packets, except on SETUP transactions. All isochronous OUT packets are dropped."]
    VALUE2,
}
impl GOUTNAKSTSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GOUTNAKSTSR::VALUE1 => false,
            GOUTNAKSTSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GOUTNAKSTSR {
        match value {
            false => GOUTNAKSTSR::VALUE1,
            true => GOUTNAKSTSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == GOUTNAKSTSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == GOUTNAKSTSR::VALUE2
    }
}
#[doc = "Possible values of the field `GMC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GMCR {
    #[doc = "Invalid."]
    VALUE1,
    #[doc = "1 packet."]
    VALUE2,
    #[doc = "2 packets."]
    VALUE3,
    #[doc = "3 packets."]
    VALUE4,
}
impl GMCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GMCR::VALUE1 => 0,
            GMCR::VALUE2 => 1,
            GMCR::VALUE3 => 2,
            GMCR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GMCR {
        match value {
            0 => GMCR::VALUE1,
            1 => GMCR::VALUE2,
            2 => GMCR::VALUE3,
            3 => GMCR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == GMCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == GMCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == GMCR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == GMCR::VALUE4
    }
}
#[doc = "Possible values of the field `IgnrFrmNum`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IGNRFRMNUMR {
    #[doc = "Scatter/Gather enabled: The core transmits the packets only in the frame number in which they are intended to be transmitted. Scatter/Gather disabled: Periodic transfer interrupt feature is disabled; the application must program transfers for periodic endpoints every frame"]
    VALUE1,
    #[doc = "Scatter/Gather enabled: The core ignores the frame number, sending packets immediately as the packets are ready. Scatter/Gather disabled: Periodic transfer interrupt feature is enabled; the application can program transfers for multiple frames for periodic endpoints."]
    VALUE2,
}
impl IGNRFRMNUMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            IGNRFRMNUMR::VALUE1 => false,
            IGNRFRMNUMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IGNRFRMNUMR {
        match value {
            false => IGNRFRMNUMR::VALUE1,
            true => IGNRFRMNUMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == IGNRFRMNUMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == IGNRFRMNUMR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct NAKONBBLER {
    bits: bool,
}
impl NAKONBBLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `EnContOnBNA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENCONTONBNAR {
    #[doc = "After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the DOEPDMA descriptor."]
    VALUE1,
    #[doc = "After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the descriptor that received the BNA interrupt."]
    VALUE2,
}
impl ENCONTONBNAR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ENCONTONBNAR::VALUE1 => false,
            ENCONTONBNAR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENCONTONBNAR {
        match value {
            false => ENCONTONBNAR::VALUE1,
            true => ENCONTONBNAR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENCONTONBNAR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENCONTONBNAR::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _RMTWKUPSIGW<'a> {
    w: &'a mut W,
}
impl<'a> _RMTWKUPSIGW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SftDiscon`"]
pub enum SFTDISCONW {
    #[doc = "Normal operation. When this bit is cleared after a soft disconnect, the core drives a device connect event to the USB host. When the device is reconnected, the USB host restarts device enumeration."]
    VALUE1,
    #[doc = "The core drives a device disconnect event to the USB host."]
    VALUE2,
}
impl SFTDISCONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SFTDISCONW::VALUE1 => false,
            SFTDISCONW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SFTDISCONW<'a> {
    w: &'a mut W,
}
impl<'a> _SFTDISCONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SFTDISCONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation. When this bit is cleared after a soft disconnect, the core drives a device connect event to the USB host. When the device is reconnected, the USB host restarts device enumeration."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SFTDISCONW::VALUE1)
    }
    #[doc = "The core drives a device disconnect event to the USB host."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SFTDISCONW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SGNPINNAKW<'a> {
    w: &'a mut W,
}
impl<'a> _SGNPINNAKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CGNPINNAKW<'a> {
    w: &'a mut W,
}
impl<'a> _CGNPINNAKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SGOUTNAKW<'a> {
    w: &'a mut W,
}
impl<'a> _SGOUTNAKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CGOUTNAKW<'a> {
    w: &'a mut W,
}
impl<'a> _CGOUTNAKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GMC`"]
pub enum GMCW {
    #[doc = "Invalid."]
    VALUE1,
    #[doc = "1 packet."]
    VALUE2,
    #[doc = "2 packets."]
    VALUE3,
    #[doc = "3 packets."]
    VALUE4,
}
impl GMCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GMCW::VALUE1 => 0,
            GMCW::VALUE2 => 1,
            GMCW::VALUE3 => 2,
            GMCW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GMCW<'a> {
    w: &'a mut W,
}
impl<'a> _GMCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GMCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Invalid."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(GMCW::VALUE1)
    }
    #[doc = "1 packet."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(GMCW::VALUE2)
    }
    #[doc = "2 packets."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(GMCW::VALUE3)
    }
    #[doc = "3 packets."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(GMCW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IgnrFrmNum`"]
pub enum IGNRFRMNUMW {
    #[doc = "Scatter/Gather enabled: The core transmits the packets only in the frame number in which they are intended to be transmitted. Scatter/Gather disabled: Periodic transfer interrupt feature is disabled; the application must program transfers for periodic endpoints every frame"]
    VALUE1,
    #[doc = "Scatter/Gather enabled: The core ignores the frame number, sending packets immediately as the packets are ready. Scatter/Gather disabled: Periodic transfer interrupt feature is enabled; the application can program transfers for multiple frames for periodic endpoints."]
    VALUE2,
}
impl IGNRFRMNUMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IGNRFRMNUMW::VALUE1 => false,
            IGNRFRMNUMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IGNRFRMNUMW<'a> {
    w: &'a mut W,
}
impl<'a> _IGNRFRMNUMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IGNRFRMNUMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Scatter/Gather enabled: The core transmits the packets only in the frame number in which they are intended to be transmitted. Scatter/Gather disabled: Periodic transfer interrupt feature is disabled; the application must program transfers for periodic endpoints every frame"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(IGNRFRMNUMW::VALUE1)
    }
    #[doc = "Scatter/Gather enabled: The core ignores the frame number, sending packets immediately as the packets are ready. Scatter/Gather disabled: Periodic transfer interrupt feature is enabled; the application can program transfers for multiple frames for periodic endpoints."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(IGNRFRMNUMW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NAKONBBLEW<'a> {
    w: &'a mut W,
}
impl<'a> _NAKONBBLEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EnContOnBNA`"]
pub enum ENCONTONBNAW {
    #[doc = "After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the DOEPDMA descriptor."]
    VALUE1,
    #[doc = "After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the descriptor that received the BNA interrupt."]
    VALUE2,
}
impl ENCONTONBNAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENCONTONBNAW::VALUE1 => false,
            ENCONTONBNAW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENCONTONBNAW<'a> {
    w: &'a mut W,
}
impl<'a> _ENCONTONBNAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENCONTONBNAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the DOEPDMA descriptor."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENCONTONBNAW::VALUE1)
    }
    #[doc = "After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the descriptor that received the BNA interrupt."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENCONTONBNAW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Remote Wakeup Signaling"]
    #[inline]
    pub fn rmt_wk_up_sig(&self) -> RMTWKUPSIGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RMTWKUPSIGR { bits }
    }
    #[doc = "Bit 1 - Soft Disconnect"]
    #[inline]
    pub fn sft_discon(&self) -> SFTDISCONR {
        SFTDISCONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Global Non-periodic IN NAK Status"]
    #[inline]
    pub fn gnpinnak_sts(&self) -> GNPINNAKSTSR {
        GNPINNAKSTSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Global OUT NAK Status"]
    #[inline]
    pub fn goutnak_sts(&self) -> GOUTNAKSTSR {
        GOUTNAKSTSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 13:14 - Global Multi Count"]
    #[inline]
    pub fn gmc(&self) -> GMCR {
        GMCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - Ignore frame number for isochronous endpoints in case of Scatter/Gather DMA"]
    #[inline]
    pub fn ignr_frm_num(&self) -> IGNRFRMNUMR {
        IGNRFRMNUMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Set NAK automatically on babble"]
    #[inline]
    pub fn nak_on_bble(&self) -> NAKONBBLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NAKONBBLER { bits }
    }
    #[doc = "Bit 17 - Enable continue on BNA"]
    #[inline]
    pub fn en_cont_on_bna(&self) -> ENCONTONBNAR {
        ENCONTONBNAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Remote Wakeup Signaling"]
    #[inline]
    pub fn rmt_wk_up_sig(&mut self) -> _RMTWKUPSIGW {
        _RMTWKUPSIGW { w: self }
    }
    #[doc = "Bit 1 - Soft Disconnect"]
    #[inline]
    pub fn sft_discon(&mut self) -> _SFTDISCONW {
        _SFTDISCONW { w: self }
    }
    #[doc = "Bit 7 - Set Global Non-periodic IN NAK"]
    #[inline]
    pub fn sgnpin_nak(&mut self) -> _SGNPINNAKW {
        _SGNPINNAKW { w: self }
    }
    #[doc = "Bit 8 - Clear Global Non-periodic IN NAK"]
    #[inline]
    pub fn cgnpin_nak(&mut self) -> _CGNPINNAKW {
        _CGNPINNAKW { w: self }
    }
    #[doc = "Bit 9 - Set Global OUT NAK"]
    #[inline]
    pub fn sgoutnak(&mut self) -> _SGOUTNAKW {
        _SGOUTNAKW { w: self }
    }
    #[doc = "Bit 10 - Clear Global OUT NAK"]
    #[inline]
    pub fn cgoutnak(&mut self) -> _CGOUTNAKW {
        _CGOUTNAKW { w: self }
    }
    #[doc = "Bits 13:14 - Global Multi Count"]
    #[inline]
    pub fn gmc(&mut self) -> _GMCW {
        _GMCW { w: self }
    }
    #[doc = "Bit 15 - Ignore frame number for isochronous endpoints in case of Scatter/Gather DMA"]
    #[inline]
    pub fn ignr_frm_num(&mut self) -> _IGNRFRMNUMW {
        _IGNRFRMNUMW { w: self }
    }
    #[doc = "Bit 16 - Set NAK automatically on babble"]
    #[inline]
    pub fn nak_on_bble(&mut self) -> _NAKONBBLEW {
        _NAKONBBLEW { w: self }
    }
    #[doc = "Bit 17 - Enable continue on BNA"]
    #[inline]
    pub fn en_cont_on_bna(&mut self) -> _ENCONTONBNAW {
        _ENCONTONBNAW { w: self }
    }
}
