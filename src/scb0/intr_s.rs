#[doc = "Register `INTR_S` reader"]
pub struct R(crate::R<INTR_S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_S` writer"]
pub struct W(crate::W<INTR_S_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_S_SPEC>;
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
impl From<crate::W<INTR_S_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_S_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_ARB_LOST` reader - I2C slave lost arbitration: the value driven on the SDA line is not the same as the value observed on the SDA line (while the SCL line is '1'). This should not occur, it represents erroneous I2C bus behavior. In case of lost arbitration, the I2C slave state machine abort the ongoing transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
pub struct I2C_ARB_LOST_R(crate::FieldReader<bool, bool>);
impl I2C_ARB_LOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_ARB_LOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_ARB_LOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_ARB_LOST` writer - I2C slave lost arbitration: the value driven on the SDA line is not the same as the value observed on the SDA line (while the SCL line is '1'). This should not occur, it represents erroneous I2C bus behavior. In case of lost arbitration, the I2C slave state machine abort the ongoing transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
pub struct I2C_ARB_LOST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_ARB_LOST_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `I2C_NACK` reader - I2C slave negative acknowledgement received. Set to '1', when the slave receives a NACK (typically after the slave transmitted TX data)."]
pub struct I2C_NACK_R(crate::FieldReader<bool, bool>);
impl I2C_NACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_NACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_NACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_NACK` writer - I2C slave negative acknowledgement received. Set to '1', when the slave receives a NACK (typically after the slave transmitted TX data)."]
pub struct I2C_NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_NACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `I2C_ACK` reader - I2C slave acknowledgement received. Set to '1', when the slave receives a ACK (typically after the slave transmitted TX data)."]
pub struct I2C_ACK_R(crate::FieldReader<bool, bool>);
impl I2C_ACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_ACK` writer - I2C slave acknowledgement received. Set to '1', when the slave receives a ACK (typically after the slave transmitted TX data)."]
pub struct I2C_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_ACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `I2C_WRITE_STOP` reader - I2C STOP event for I2C write transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the Firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. In non EZ mode, the event is detected on any I2C write transfer intended for this slave. Note that a I2C write address intended for the slave (address is matching and a it is a write transfer) will result in a I2C_WRITE_STOP event independent of whether the I2C address is ACK'd or NACK'd. In EZ mode, the event is detected only on I2C write transfers that have EZ data written to the memory structure (an I2C write transfer that only communicates an I2C address and EZ address, will not result in this event being detected)."]
pub struct I2C_WRITE_STOP_R(crate::FieldReader<bool, bool>);
impl I2C_WRITE_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_WRITE_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_WRITE_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_WRITE_STOP` writer - I2C STOP event for I2C write transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the Firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. In non EZ mode, the event is detected on any I2C write transfer intended for this slave. Note that a I2C write address intended for the slave (address is matching and a it is a write transfer) will result in a I2C_WRITE_STOP event independent of whether the I2C address is ACK'd or NACK'd. In EZ mode, the event is detected only on I2C write transfers that have EZ data written to the memory structure (an I2C write transfer that only communicates an I2C address and EZ address, will not result in this event being detected)."]
pub struct I2C_WRITE_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_WRITE_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `I2C_STOP` reader - I2C STOP event for I2C (read or write) transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the Firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. The event is detected on any I2C transfer intended for this slave. Note that a I2C address intended for the slave (address is matching) will result in a I2C_STOP event independent of whether the I2C address is ACK'd or NACK'd."]
pub struct I2C_STOP_R(crate::FieldReader<bool, bool>);
impl I2C_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_STOP` writer - I2C STOP event for I2C (read or write) transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the Firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. The event is detected on any I2C transfer intended for this slave. Note that a I2C address intended for the slave (address is matching) will result in a I2C_STOP event independent of whether the I2C address is ACK'd or NACK'd."]
pub struct I2C_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `I2C_START` reader - I2C slave START received. Set to '1', when START or REPEATED START event is detected. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') AND clock stretching is performed (till the internally clocked logic takes over) (I2C_CTRL.S_NOT_READY_ADDR_NACK is '0'), this field is NOT set. The Firmware should use INTR_S_EC.WAKE_UP, INTR_S.I2C_ADDR_MATCH and INTR_S.I2C_GENERAL."]
pub struct I2C_START_R(crate::FieldReader<bool, bool>);
impl I2C_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_START` writer - I2C slave START received. Set to '1', when START or REPEATED START event is detected. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') AND clock stretching is performed (till the internally clocked logic takes over) (I2C_CTRL.S_NOT_READY_ADDR_NACK is '0'), this field is NOT set. The Firmware should use INTR_S_EC.WAKE_UP, INTR_S.I2C_ADDR_MATCH and INTR_S.I2C_GENERAL."]
pub struct I2C_START_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `I2C_ADDR_MATCH` reader - I2C slave matching address received. If CTRL.ADDR_ACCEPT, the received address (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
pub struct I2C_ADDR_MATCH_R(crate::FieldReader<bool, bool>);
impl I2C_ADDR_MATCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_ADDR_MATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_ADDR_MATCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_ADDR_MATCH` writer - I2C slave matching address received. If CTRL.ADDR_ACCEPT, the received address (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
pub struct I2C_ADDR_MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_ADDR_MATCH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `I2C_GENERAL` reader - I2C slave general call address received. If CTRL.ADDR_ACCEPT, the received address 0x00 (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
pub struct I2C_GENERAL_R(crate::FieldReader<bool, bool>);
impl I2C_GENERAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_GENERAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_GENERAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_GENERAL` writer - I2C slave general call address received. If CTRL.ADDR_ACCEPT, the received address 0x00 (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
pub struct I2C_GENERAL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_GENERAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `I2C_BUS_ERROR` reader - I2C slave bus error (unexpected detection of START or STOP condition). This should not occur, it represents erroneous I2C bus behavior. In case of a bus error, the I2C slave state machine abort the ongoing transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
pub struct I2C_BUS_ERROR_R(crate::FieldReader<bool, bool>);
impl I2C_BUS_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_BUS_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_BUS_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_BUS_ERROR` writer - I2C slave bus error (unexpected detection of START or STOP condition). This should not occur, it represents erroneous I2C bus behavior. In case of a bus error, the I2C slave state machine abort the ongoing transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
pub struct I2C_BUS_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_BUS_ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `SPI_EZ_WRITE_STOP` reader - SPI slave deselected after a write EZ SPI transfer occurred."]
pub struct SPI_EZ_WRITE_STOP_R(crate::FieldReader<bool, bool>);
impl SPI_EZ_WRITE_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_EZ_WRITE_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_EZ_WRITE_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_EZ_WRITE_STOP` writer - SPI slave deselected after a write EZ SPI transfer occurred."]
pub struct SPI_EZ_WRITE_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_EZ_WRITE_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `SPI_EZ_STOP` reader - SPI slave deselected after any EZ SPI transfer occurred."]
pub struct SPI_EZ_STOP_R(crate::FieldReader<bool, bool>);
impl SPI_EZ_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_EZ_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_EZ_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_EZ_STOP` writer - SPI slave deselected after any EZ SPI transfer occurred."]
pub struct SPI_EZ_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_EZ_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `SPI_BUS_ERROR` reader - SPI slave deselected at an unexpected time in the SPI transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
pub struct SPI_BUS_ERROR_R(crate::FieldReader<bool, bool>);
impl SPI_BUS_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_BUS_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_BUS_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_BUS_ERROR` writer - SPI slave deselected at an unexpected time in the SPI transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
pub struct SPI_BUS_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_BUS_ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - I2C slave lost arbitration: the value driven on the SDA line is not the same as the value observed on the SDA line (while the SCL line is '1'). This should not occur, it represents erroneous I2C bus behavior. In case of lost arbitration, the I2C slave state machine abort the ongoing transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub fn i2c_arb_lost(&self) -> I2C_ARB_LOST_R {
        I2C_ARB_LOST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C slave negative acknowledgement received. Set to '1', when the slave receives a NACK (typically after the slave transmitted TX data)."]
    #[inline(always)]
    pub fn i2c_nack(&self) -> I2C_NACK_R {
        I2C_NACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C slave acknowledgement received. Set to '1', when the slave receives a ACK (typically after the slave transmitted TX data)."]
    #[inline(always)]
    pub fn i2c_ack(&self) -> I2C_ACK_R {
        I2C_ACK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C STOP event for I2C write transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the Firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. In non EZ mode, the event is detected on any I2C write transfer intended for this slave. Note that a I2C write address intended for the slave (address is matching and a it is a write transfer) will result in a I2C_WRITE_STOP event independent of whether the I2C address is ACK'd or NACK'd. In EZ mode, the event is detected only on I2C write transfers that have EZ data written to the memory structure (an I2C write transfer that only communicates an I2C address and EZ address, will not result in this event being detected)."]
    #[inline(always)]
    pub fn i2c_write_stop(&self) -> I2C_WRITE_STOP_R {
        I2C_WRITE_STOP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C STOP event for I2C (read or write) transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the Firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. The event is detected on any I2C transfer intended for this slave. Note that a I2C address intended for the slave (address is matching) will result in a I2C_STOP event independent of whether the I2C address is ACK'd or NACK'd."]
    #[inline(always)]
    pub fn i2c_stop(&self) -> I2C_STOP_R {
        I2C_STOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C slave START received. Set to '1', when START or REPEATED START event is detected. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') AND clock stretching is performed (till the internally clocked logic takes over) (I2C_CTRL.S_NOT_READY_ADDR_NACK is '0'), this field is NOT set. The Firmware should use INTR_S_EC.WAKE_UP, INTR_S.I2C_ADDR_MATCH and INTR_S.I2C_GENERAL."]
    #[inline(always)]
    pub fn i2c_start(&self) -> I2C_START_R {
        I2C_START_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C slave matching address received. If CTRL.ADDR_ACCEPT, the received address (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
    #[inline(always)]
    pub fn i2c_addr_match(&self) -> I2C_ADDR_MATCH_R {
        I2C_ADDR_MATCH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C slave general call address received. If CTRL.ADDR_ACCEPT, the received address 0x00 (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
    #[inline(always)]
    pub fn i2c_general(&self) -> I2C_GENERAL_R {
        I2C_GENERAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - I2C slave bus error (unexpected detection of START or STOP condition). This should not occur, it represents erroneous I2C bus behavior. In case of a bus error, the I2C slave state machine abort the ongoing transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub fn i2c_bus_error(&self) -> I2C_BUS_ERROR_R {
        I2C_BUS_ERROR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SPI slave deselected after a write EZ SPI transfer occurred."]
    #[inline(always)]
    pub fn spi_ez_write_stop(&self) -> SPI_EZ_WRITE_STOP_R {
        SPI_EZ_WRITE_STOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SPI slave deselected after any EZ SPI transfer occurred."]
    #[inline(always)]
    pub fn spi_ez_stop(&self) -> SPI_EZ_STOP_R {
        SPI_EZ_STOP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SPI slave deselected at an unexpected time in the SPI transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub fn spi_bus_error(&self) -> SPI_BUS_ERROR_R {
        SPI_BUS_ERROR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C slave lost arbitration: the value driven on the SDA line is not the same as the value observed on the SDA line (while the SCL line is '1'). This should not occur, it represents erroneous I2C bus behavior. In case of lost arbitration, the I2C slave state machine abort the ongoing transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub fn i2c_arb_lost(&mut self) -> I2C_ARB_LOST_W {
        I2C_ARB_LOST_W { w: self }
    }
    #[doc = "Bit 1 - I2C slave negative acknowledgement received. Set to '1', when the slave receives a NACK (typically after the slave transmitted TX data)."]
    #[inline(always)]
    pub fn i2c_nack(&mut self) -> I2C_NACK_W {
        I2C_NACK_W { w: self }
    }
    #[doc = "Bit 2 - I2C slave acknowledgement received. Set to '1', when the slave receives a ACK (typically after the slave transmitted TX data)."]
    #[inline(always)]
    pub fn i2c_ack(&mut self) -> I2C_ACK_W {
        I2C_ACK_W { w: self }
    }
    #[doc = "Bit 3 - I2C STOP event for I2C write transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the Firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. In non EZ mode, the event is detected on any I2C write transfer intended for this slave. Note that a I2C write address intended for the slave (address is matching and a it is a write transfer) will result in a I2C_WRITE_STOP event independent of whether the I2C address is ACK'd or NACK'd. In EZ mode, the event is detected only on I2C write transfers that have EZ data written to the memory structure (an I2C write transfer that only communicates an I2C address and EZ address, will not result in this event being detected)."]
    #[inline(always)]
    pub fn i2c_write_stop(&mut self) -> I2C_WRITE_STOP_W {
        I2C_WRITE_STOP_W { w: self }
    }
    #[doc = "Bit 4 - I2C STOP event for I2C (read or write) transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the Firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. The event is detected on any I2C transfer intended for this slave. Note that a I2C address intended for the slave (address is matching) will result in a I2C_STOP event independent of whether the I2C address is ACK'd or NACK'd."]
    #[inline(always)]
    pub fn i2c_stop(&mut self) -> I2C_STOP_W {
        I2C_STOP_W { w: self }
    }
    #[doc = "Bit 5 - I2C slave START received. Set to '1', when START or REPEATED START event is detected. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') AND clock stretching is performed (till the internally clocked logic takes over) (I2C_CTRL.S_NOT_READY_ADDR_NACK is '0'), this field is NOT set. The Firmware should use INTR_S_EC.WAKE_UP, INTR_S.I2C_ADDR_MATCH and INTR_S.I2C_GENERAL."]
    #[inline(always)]
    pub fn i2c_start(&mut self) -> I2C_START_W {
        I2C_START_W { w: self }
    }
    #[doc = "Bit 6 - I2C slave matching address received. If CTRL.ADDR_ACCEPT, the received address (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
    #[inline(always)]
    pub fn i2c_addr_match(&mut self) -> I2C_ADDR_MATCH_W {
        I2C_ADDR_MATCH_W { w: self }
    }
    #[doc = "Bit 7 - I2C slave general call address received. If CTRL.ADDR_ACCEPT, the received address 0x00 (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
    #[inline(always)]
    pub fn i2c_general(&mut self) -> I2C_GENERAL_W {
        I2C_GENERAL_W { w: self }
    }
    #[doc = "Bit 8 - I2C slave bus error (unexpected detection of START or STOP condition). This should not occur, it represents erroneous I2C bus behavior. In case of a bus error, the I2C slave state machine abort the ongoing transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub fn i2c_bus_error(&mut self) -> I2C_BUS_ERROR_W {
        I2C_BUS_ERROR_W { w: self }
    }
    #[doc = "Bit 9 - SPI slave deselected after a write EZ SPI transfer occurred."]
    #[inline(always)]
    pub fn spi_ez_write_stop(&mut self) -> SPI_EZ_WRITE_STOP_W {
        SPI_EZ_WRITE_STOP_W { w: self }
    }
    #[doc = "Bit 10 - SPI slave deselected after any EZ SPI transfer occurred."]
    #[inline(always)]
    pub fn spi_ez_stop(&mut self) -> SPI_EZ_STOP_W {
        SPI_EZ_STOP_W { w: self }
    }
    #[doc = "Bit 11 - SPI slave deselected at an unexpected time in the SPI transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub fn spi_bus_error(&mut self) -> SPI_BUS_ERROR_W {
        SPI_BUS_ERROR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave interrupt request register. The register fields are not retained In DeepSleep power mode: HW clears the interrupt causes to '0', when coming out of DeepSleep power mode. In addition, HW clears the interrupt causes to '0', when the IP is disabled. As a result, the interrupt causes are only available in Active/Sleep power modes; they are generated by internally clocked logic (this logic operates on a clock that is only available in Active/Sleep power modes). The interrupt causes should only be used for internally clocked operation; i.e. EC_OP is '0'.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_s](index.html) module"]
pub struct INTR_S_SPEC;
impl crate::RegisterSpec for INTR_S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_s::R](R) reader structure"]
impl crate::Readable for INTR_S_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_s::W](W) writer structure"]
impl crate::Writable for INTR_S_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_S to value 0"]
impl crate::Resettable for INTR_S_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
