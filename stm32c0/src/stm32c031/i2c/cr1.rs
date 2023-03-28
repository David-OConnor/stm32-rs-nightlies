#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PE` reader - Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles."]
pub type PE_R = crate::BitReader<bool>;
#[doc = "Field `PE` writer - Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles."]
pub type PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `TXIE` reader - TX Interrupt enable"]
pub type TXIE_R = crate::BitReader<bool>;
#[doc = "Field `TXIE` writer - TX Interrupt enable"]
pub type TXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `RXIE` reader - RX Interrupt enable"]
pub type RXIE_R = crate::BitReader<bool>;
#[doc = "Field `RXIE` writer - RX Interrupt enable"]
pub type RXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ADDRIE` reader - Address match Interrupt enable (slave only)"]
pub type ADDRIE_R = crate::BitReader<bool>;
#[doc = "Field `ADDRIE` writer - Address match Interrupt enable (slave only)"]
pub type ADDRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `NACKIE` reader - Not acknowledge received Interrupt enable"]
pub type NACKIE_R = crate::BitReader<bool>;
#[doc = "Field `NACKIE` writer - Not acknowledge received Interrupt enable"]
pub type NACKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `STOPIE` reader - Stop detection Interrupt enable"]
pub type STOPIE_R = crate::BitReader<bool>;
#[doc = "Field `STOPIE` writer - Stop detection Interrupt enable"]
pub type STOPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `TCIE` reader - Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)"]
pub type TCIE_R = crate::BitReader<bool>;
#[doc = "Field `TCIE` writer - Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)"]
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ERRIE` reader - Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)"]
pub type ERRIE_R = crate::BitReader<bool>;
#[doc = "Field `ERRIE` writer - Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)"]
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `DNF` reader - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\\[3:0\\]
* tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0)."]
pub type DNF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DNF` writer - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\\[3:0\\]
* tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0)."]
pub type DNF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `ANFOFF` reader - Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
pub type ANFOFF_R = crate::BitReader<bool>;
#[doc = "Field `ANFOFF` writer - Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
pub type ANFOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `TXDMAEN` reader - DMA transmission requests enable"]
pub type TXDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `TXDMAEN` writer - DMA transmission requests enable"]
pub type TXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `RXDMAEN` reader - DMA reception requests enable"]
pub type RXDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `RXDMAEN` writer - DMA reception requests enable"]
pub type RXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `SBC` reader - Slave byte control This bit is used to enable hardware byte control in slave mode."]
pub type SBC_R = crate::BitReader<bool>;
#[doc = "Field `SBC` writer - Slave byte control This bit is used to enable hardware byte control in slave mode."]
pub type SBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `NOSTRETCH` reader - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
pub type NOSTRETCH_R = crate::BitReader<bool>;
#[doc = "Field `NOSTRETCH` writer - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
pub type NOSTRETCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `WUPEN` reader - Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to . Note: WUPEN can be set only when DNF = ‘0000’"]
pub type WUPEN_R = crate::BitReader<bool>;
#[doc = "Field `WUPEN` writer - Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to . Note: WUPEN can be set only when DNF = ‘0000’"]
pub type WUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `GCEN` reader - General call enable"]
pub type GCEN_R = crate::BitReader<bool>;
#[doc = "Field `GCEN` writer - General call enable"]
pub type GCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `SMBHEN` reader - SMBus host address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
pub type SMBHEN_R = crate::BitReader<bool>;
#[doc = "Field `SMBHEN` writer - SMBus host address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
pub type SMBHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `SMBDEN` reader - SMBus device default address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
pub type SMBDEN_R = crate::BitReader<bool>;
#[doc = "Field `SMBDEN` writer - SMBus device default address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
pub type SMBDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ALERTEN` reader - SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
pub type ALERTEN_R = crate::BitReader<bool>;
#[doc = "Field `ALERTEN` writer - SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
pub type ALERTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `PECEN` reader - PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
pub type PECEN_R = crate::BitReader<bool>;
#[doc = "Field `PECEN` writer - PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
pub type PECEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Interrupt enable"]
    #[inline(always)]
    pub fn txie(&self) -> TXIE_R {
        TXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Interrupt enable"]
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address match Interrupt enable (slave only)"]
    #[inline(always)]
    pub fn addrie(&self) -> ADDRIE_R {
        ADDRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Not acknowledge received Interrupt enable"]
    #[inline(always)]
    pub fn nackie(&self) -> NACKIE_R {
        NACKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop detection Interrupt enable"]
    #[inline(always)]
    pub fn stopie(&self) -> STOPIE_R {
        STOPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\\[3:0\\]
* tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    pub fn dnf(&self) -> DNF_R {
        DNF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    pub fn anfoff(&self) -> ANFOFF_R {
        ANFOFF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - DMA transmission requests enable"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA reception requests enable"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave byte control This bit is used to enable hardware byte control in slave mode."]
    #[inline(always)]
    pub fn sbc(&self) -> SBC_R {
        SBC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    pub fn nostretch(&self) -> NOSTRETCH_R {
        NOSTRETCH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to . Note: WUPEN can be set only when DNF = ‘0000’"]
    #[inline(always)]
    pub fn wupen(&self) -> WUPEN_R {
        WUPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - General call enable"]
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SMBus host address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
    #[inline(always)]
    pub fn smbhen(&self) -> SMBHEN_R {
        SMBHEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SMBus device default address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
    #[inline(always)]
    pub fn smbden(&self) -> SMBDEN_R {
        SMBDEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
    #[inline(always)]
    pub fn alerten(&self) -> ALERTEN_R {
        ALERTEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<0> {
        PE_W::new(self)
    }
    #[doc = "Bit 1 - TX Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txie(&mut self) -> TXIE_W<1> {
        TXIE_W::new(self)
    }
    #[doc = "Bit 2 - RX Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxie(&mut self) -> RXIE_W<2> {
        RXIE_W::new(self)
    }
    #[doc = "Bit 3 - Address match Interrupt enable (slave only)"]
    #[inline(always)]
    #[must_use]
    pub fn addrie(&mut self) -> ADDRIE_W<3> {
        ADDRIE_W::new(self)
    }
    #[doc = "Bit 4 - Not acknowledge received Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn nackie(&mut self) -> NACKIE_W<4> {
        NACKIE_W::new(self)
    }
    #[doc = "Bit 5 - Stop detection Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stopie(&mut self) -> STOPIE_W<5> {
        STOPIE_W::new(self)
    }
    #[doc = "Bit 6 - Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<6> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 7 - Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<7> {
        ERRIE_W::new(self)
    }
    #[doc = "Bits 8:11 - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\\[3:0\\]
* tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn dnf(&mut self) -> DNF_W<8> {
        DNF_W::new(self)
    }
    #[doc = "Bit 12 - Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn anfoff(&mut self) -> ANFOFF_W<12> {
        ANFOFF_W::new(self)
    }
    #[doc = "Bit 14 - DMA transmission requests enable"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<14> {
        TXDMAEN_W::new(self)
    }
    #[doc = "Bit 15 - DMA reception requests enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<15> {
        RXDMAEN_W::new(self)
    }
    #[doc = "Bit 16 - Slave byte control This bit is used to enable hardware byte control in slave mode."]
    #[inline(always)]
    #[must_use]
    pub fn sbc(&mut self) -> SBC_W<16> {
        SBC_W::new(self)
    }
    #[doc = "Bit 17 - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn nostretch(&mut self) -> NOSTRETCH_W<17> {
        NOSTRETCH_W::new(self)
    }
    #[doc = "Bit 18 - Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to . Note: WUPEN can be set only when DNF = ‘0000’"]
    #[inline(always)]
    #[must_use]
    pub fn wupen(&mut self) -> WUPEN_W<18> {
        WUPEN_W::new(self)
    }
    #[doc = "Bit 19 - General call enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcen(&mut self) -> GCEN_W<19> {
        GCEN_W::new(self)
    }
    #[doc = "Bit 20 - SMBus host address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn smbhen(&mut self) -> SMBHEN_W<20> {
        SMBHEN_W::new(self)
    }
    #[doc = "Bit 21 - SMBus device default address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn smbden(&mut self) -> SMBDEN_W<21> {
        SMBDEN_W::new(self)
    }
    #[doc = "Bit 22 - SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn alerten(&mut self) -> ALERTEN_W<22> {
        ALERTEN_W::new(self)
    }
    #[doc = "Bit 23 - PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn pecen(&mut self) -> PECEN_W<23> {
        PECEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
