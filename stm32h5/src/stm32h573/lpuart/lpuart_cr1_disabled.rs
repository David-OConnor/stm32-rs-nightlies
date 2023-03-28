#[doc = "Register `LPUART_CR1_disabled` reader"]
pub struct R(crate::R<LPUART_CR1_DISABLED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPUART_CR1_DISABLED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPUART_CR1_DISABLED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPUART_CR1_DISABLED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPUART_CR1_disabled` writer"]
pub struct W(crate::W<LPUART_CR1_DISABLED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPUART_CR1_DISABLED_SPEC>;
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
impl From<crate::W<LPUART_CR1_DISABLED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPUART_CR1_DISABLED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UE` reader - LPUART enable When this bit is cleared, the LPUART prescalers and outputs are stopped immediately, and current operations are discarded. The configuration of the LPUART is kept, but all the status flags, in the LPUART_ISR are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be reset before and the software must wait for the TC bit in the LPUART_ISR to be set before resetting the UE bit. The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit."]
pub type UE_R = crate::BitReader<bool>;
#[doc = "Field `UE` writer - LPUART enable When this bit is cleared, the LPUART prescalers and outputs are stopped immediately, and current operations are discarded. The configuration of the LPUART is kept, but all the status flags, in the LPUART_ISR are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be reset before and the software must wait for the TC bit in the LPUART_ISR to be set before resetting the UE bit. The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit."]
pub type UE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPUART_CR1_DISABLED_SPEC, bool, O>;
#[doc = "Field `UESM` reader - LPUART enable in low-power mode When this bit is cleared, the LPUART cannot wake up the MCU from low-power mode. When this bit is set, the LPUART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode, and clear it when exiting low-power mode."]
pub type UESM_R = crate::BitReader<bool>;
#[doc = "Field `UESM` writer - LPUART enable in low-power mode When this bit is cleared, the LPUART cannot wake up the MCU from low-power mode. When this bit is set, the LPUART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode, and clear it when exiting low-power mode."]
pub type UESM_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPUART_CR1_DISABLED_SPEC, bool, O>;
#[doc = "Field `RE` reader - Receiver enable This bit enables the receiver. It is set and cleared by software."]
pub type RE_R = crate::BitReader<bool>;
#[doc = "Field `RE` writer - Receiver enable This bit enables the receiver. It is set and cleared by software."]
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPUART_CR1_DISABLED_SPEC, bool, O>;
#[doc = "Field `TE` reader - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit (‘0’ followed by ‘1’) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to ‘1. To ensure the required duration, the software can poll the TEACK bit in the LPUART_ISR register. In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts."]
pub type TE_R = crate::BitReader<bool>;
#[doc = "Field `TE` writer - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit (‘0’ followed by ‘1’) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to ‘1. To ensure the required duration, the software can poll the TEACK bit in the LPUART_ISR register. In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts."]
pub type TE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPUART_CR1_DISABLED_SPEC, bool, O>;
#[doc = "Field `IDLEIE` reader - IDLE interrupt enable This bit is set and cleared by software."]
pub type IDLEIE_R = crate::BitReader<bool>;
#[doc = "Field `IDLEIE` writer - IDLE interrupt enable This bit is set and cleared by software."]
pub type IDLEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPUART_CR1_DISABLED_SPEC, bool, O>;
#[doc = "Field `RXNEIE` reader - Receive data register not empty This bit is set and cleared by software."]
pub type RXNEIE_R = crate::BitReader<bool>;
#[doc = "Field `RXNEIE` writer - Receive data register not empty This bit is set and cleared by software."]
pub type RXNEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPUART_CR1_DISABLED_SPEC, bool, O>;
#[doc = "Field `TCIE` reader - Transmission complete interrupt enable This bit is set and cleared by software."]
pub type TCIE_R = crate::BitReader<bool>;
#[doc = "Field `TCIE` writer - Transmission complete interrupt enable This bit is set and cleared by software."]
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPUART_CR1_DISABLED_SPEC, bool, O>;
#[doc = "Field `TXEIE` reader - Transmit data register empty This bit is set and cleared by software."]
pub type TXEIE_R = crate::BitReader<bool>;
#[doc = "Field `TXEIE` writer - Transmit data register empty This bit is set and cleared by software."]
pub type TXEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPUART_CR1_DISABLED_SPEC, bool, O>;
#[doc = "Field `PEIE` reader - PE interrupt enable This bit is set and cleared by software."]
pub type PEIE_R = crate::BitReader<bool>;
#[doc = "Field `PEIE` writer - PE interrupt enable This bit is set and cleared by software."]
pub type PEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPUART_CR1_DISABLED_SPEC, bool, O>;
#[doc = "Field `PS` reader - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type PS_R = crate::BitReader<bool>;
#[doc = "Field `PS` writer - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type PS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPUART_CR1_DISABLED_SPEC, bool, O>;
#[doc = "Field `PCE` reader - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type PCE_R = crate::BitReader<bool>;
#[doc = "Field `PCE` writer - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type PCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPUART_CR1_DISABLED_SPEC, bool, O>;
#[doc = "Field `WAKE` reader - Receiver wakeup method This bit determines the LPUART wakeup method from Mute mode. It is set or cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type WAKE_R = crate::BitReader<bool>;
#[doc = "Field `WAKE` writer - Receiver wakeup method This bit determines the LPUART wakeup method from Mute mode. It is set or cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type WAKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPUART_CR1_DISABLED_SPEC, bool, O>;
#[doc = "Field `M0` reader - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1) description). This bit can only be written when the LPUART is disabled (UE=0)."]
pub type M0_R = crate::BitReader<bool>;
#[doc = "Field `M0` writer - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1) description). This bit can only be written when the LPUART is disabled (UE=0)."]
pub type M0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPUART_CR1_DISABLED_SPEC, bool, O>;
#[doc = "Field `MME` reader - Mute mode enable This bit activates the Mute mode function of the LPUART. When set, the LPUART can switch between the active and Mute modes, as defined by the WAKE bit. It is set and cleared by software."]
pub type MME_R = crate::BitReader<bool>;
#[doc = "Field `MME` writer - Mute mode enable This bit activates the Mute mode function of the LPUART. When set, the LPUART can switch between the active and Mute modes, as defined by the WAKE bit. It is set and cleared by software."]
pub type MME_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPUART_CR1_DISABLED_SPEC, bool, O>;
#[doc = "Field `CMIE` reader - Character match interrupt enable This bit is set and cleared by software."]
pub type CMIE_R = crate::BitReader<bool>;
#[doc = "Field `CMIE` writer - Character match interrupt enable This bit is set and cleared by software."]
pub type CMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPUART_CR1_DISABLED_SPEC, bool, O>;
#[doc = "Field `DEDT` reader - Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal.It is expressed in lpuart_ker_ck clock cycles. For more details, refer control and RS485 Driver Enable. If the LPUART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type DEDT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEDT` writer - Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal.It is expressed in lpuart_ker_ck clock cycles. For more details, refer control and RS485 Driver Enable. If the LPUART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type DEDT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LPUART_CR1_DISABLED_SPEC, u8, u8, 5, O>;
#[doc = "Field `DEAT` reader - Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in lpuart_ker_ck clock cycles. For more details, refer . This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type DEAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEAT` writer - Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in lpuart_ker_ck clock cycles. For more details, refer . This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type DEAT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LPUART_CR1_DISABLED_SPEC, u8, u8, 5, O>;
#[doc = "Field `M1` reader - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\\[1:0\\]
= ‘00’: 1 Start bit, 8 Data bits, n Stop bit M\\[1:0\\]
= ‘01’: 1 Start bit, 9 Data bits, n Stop bit M\\[1:0\\]
= ‘10’: 1 Start bit, 7 Data bits, n Stop bit This bit can only be written when the LPUART is disabled (UE=0). Note: In 7-bit data length mode, the Smartcard mode, LIN master mode and auto baud rate (0x7F and 0x55 frames detection) are not supported."]
pub type M1_R = crate::BitReader<bool>;
#[doc = "Field `M1` writer - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\\[1:0\\]
= ‘00’: 1 Start bit, 8 Data bits, n Stop bit M\\[1:0\\]
= ‘01’: 1 Start bit, 9 Data bits, n Stop bit M\\[1:0\\]
= ‘10’: 1 Start bit, 7 Data bits, n Stop bit This bit can only be written when the LPUART is disabled (UE=0). Note: In 7-bit data length mode, the Smartcard mode, LIN master mode and auto baud rate (0x7F and 0x55 frames detection) are not supported."]
pub type M1_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPUART_CR1_DISABLED_SPEC, bool, O>;
#[doc = "Field `FIFOEN` reader - FIFO mode enable This bit is set and cleared by software."]
pub type FIFOEN_R = crate::BitReader<bool>;
#[doc = "Field `FIFOEN` writer - FIFO mode enable This bit is set and cleared by software."]
pub type FIFOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPUART_CR1_DISABLED_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LPUART enable When this bit is cleared, the LPUART prescalers and outputs are stopped immediately, and current operations are discarded. The configuration of the LPUART is kept, but all the status flags, in the LPUART_ISR are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be reset before and the software must wait for the TC bit in the LPUART_ISR to be set before resetting the UE bit. The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit."]
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPUART enable in low-power mode When this bit is cleared, the LPUART cannot wake up the MCU from low-power mode. When this bit is set, the LPUART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode, and clear it when exiting low-power mode."]
    #[inline(always)]
    pub fn uesm(&self) -> UESM_R {
        UESM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable This bit enables the receiver. It is set and cleared by software."]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit (‘0’ followed by ‘1’) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to ‘1. To ensure the required duration, the software can poll the TEACK bit in the LPUART_ISR register. In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts."]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive data register not empty This bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit data register empty This bit is set and cleared by software."]
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PE interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receiver wakeup method This bit determines the LPUART wakeup method from Mute mode. It is set or cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1) description). This bit can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn m0(&self) -> M0_R {
        M0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Mute mode enable This bit activates the Mute mode function of the LPUART. When set, the LPUART can switch between the active and Mute modes, as defined by the WAKE bit. It is set and cleared by software."]
    #[inline(always)]
    pub fn mme(&self) -> MME_R {
        MME_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Character match interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn cmie(&self) -> CMIE_R {
        CMIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal.It is expressed in lpuart_ker_ck clock cycles. For more details, refer control and RS485 Driver Enable. If the LPUART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn dedt(&self) -> DEDT_R {
        DEDT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in lpuart_ker_ck clock cycles. For more details, refer . This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn deat(&self) -> DEAT_R {
        DEAT_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 28 - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\\[1:0\\]
= ‘00’: 1 Start bit, 8 Data bits, n Stop bit M\\[1:0\\]
= ‘01’: 1 Start bit, 9 Data bits, n Stop bit M\\[1:0\\]
= ‘10’: 1 Start bit, 7 Data bits, n Stop bit This bit can only be written when the LPUART is disabled (UE=0). Note: In 7-bit data length mode, the Smartcard mode, LIN master mode and auto baud rate (0x7F and 0x55 frames detection) are not supported."]
    #[inline(always)]
    pub fn m1(&self) -> M1_R {
        M1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - FIFO mode enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn fifoen(&self) -> FIFOEN_R {
        FIFOEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPUART enable When this bit is cleared, the LPUART prescalers and outputs are stopped immediately, and current operations are discarded. The configuration of the LPUART is kept, but all the status flags, in the LPUART_ISR are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be reset before and the software must wait for the TC bit in the LPUART_ISR to be set before resetting the UE bit. The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit."]
    #[inline(always)]
    #[must_use]
    pub fn ue(&mut self) -> UE_W<0> {
        UE_W::new(self)
    }
    #[doc = "Bit 1 - LPUART enable in low-power mode When this bit is cleared, the LPUART cannot wake up the MCU from low-power mode. When this bit is set, the LPUART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode, and clear it when exiting low-power mode."]
    #[inline(always)]
    #[must_use]
    pub fn uesm(&mut self) -> UESM_W<1> {
        UESM_W::new(self)
    }
    #[doc = "Bit 2 - Receiver enable This bit enables the receiver. It is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<2> {
        RE_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit (‘0’ followed by ‘1’) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to ‘1. To ensure the required duration, the software can poll the TEACK bit in the LPUART_ISR register. In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts."]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<3> {
        TE_W::new(self)
    }
    #[doc = "Bit 4 - IDLE interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn idleie(&mut self) -> IDLEIE_W<4> {
        IDLEIE_W::new(self)
    }
    #[doc = "Bit 5 - Receive data register not empty This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn rxneie(&mut self) -> RXNEIE_W<5> {
        RXNEIE_W::new(self)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<6> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 7 - Transmit data register empty This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn txeie(&mut self) -> TXEIE_W<7> {
        TXEIE_W::new(self)
    }
    #[doc = "Bit 8 - PE interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn peie(&mut self) -> PEIE_W<8> {
        PEIE_W::new(self)
    }
    #[doc = "Bit 9 - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<9> {
        PS_W::new(self)
    }
    #[doc = "Bit 10 - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn pce(&mut self) -> PCE_W<10> {
        PCE_W::new(self)
    }
    #[doc = "Bit 11 - Receiver wakeup method This bit determines the LPUART wakeup method from Mute mode. It is set or cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn wake(&mut self) -> WAKE_W<11> {
        WAKE_W::new(self)
    }
    #[doc = "Bit 12 - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1) description). This bit can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn m0(&mut self) -> M0_W<12> {
        M0_W::new(self)
    }
    #[doc = "Bit 13 - Mute mode enable This bit activates the Mute mode function of the LPUART. When set, the LPUART can switch between the active and Mute modes, as defined by the WAKE bit. It is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn mme(&mut self) -> MME_W<13> {
        MME_W::new(self)
    }
    #[doc = "Bit 14 - Character match interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn cmie(&mut self) -> CMIE_W<14> {
        CMIE_W::new(self)
    }
    #[doc = "Bits 16:20 - Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal.It is expressed in lpuart_ker_ck clock cycles. For more details, refer control and RS485 Driver Enable. If the LPUART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn dedt(&mut self) -> DEDT_W<16> {
        DEDT_W::new(self)
    }
    #[doc = "Bits 21:25 - Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in lpuart_ker_ck clock cycles. For more details, refer . This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn deat(&mut self) -> DEAT_W<21> {
        DEAT_W::new(self)
    }
    #[doc = "Bit 28 - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\\[1:0\\]
= ‘00’: 1 Start bit, 8 Data bits, n Stop bit M\\[1:0\\]
= ‘01’: 1 Start bit, 9 Data bits, n Stop bit M\\[1:0\\]
= ‘10’: 1 Start bit, 7 Data bits, n Stop bit This bit can only be written when the LPUART is disabled (UE=0). Note: In 7-bit data length mode, the Smartcard mode, LIN master mode and auto baud rate (0x7F and 0x55 frames detection) are not supported."]
    #[inline(always)]
    #[must_use]
    pub fn m1(&mut self) -> M1_W<28> {
        M1_W::new(self)
    }
    #[doc = "Bit 29 - FIFO mode enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn fifoen(&mut self) -> FIFOEN_W<29> {
        FIFOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPUART control register 1 \\[alternate\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpuart_cr1_disabled](index.html) module"]
pub struct LPUART_CR1_DISABLED_SPEC;
impl crate::RegisterSpec for LPUART_CR1_DISABLED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpuart_cr1_disabled::R](R) reader structure"]
impl crate::Readable for LPUART_CR1_DISABLED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpuart_cr1_disabled::W](W) writer structure"]
impl crate::Writable for LPUART_CR1_DISABLED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPUART_CR1_disabled to value 0"]
impl crate::Resettable for LPUART_CR1_DISABLED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
