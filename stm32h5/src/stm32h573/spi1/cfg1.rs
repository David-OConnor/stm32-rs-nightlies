#[doc = "Register `CFG1` reader"]
pub struct R(crate::R<CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG1` writer"]
pub struct W(crate::W<CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG1_SPEC>;
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
impl From<crate::W<CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSIZE` reader - number of bits in at single SPI data frame ..... Note: Maximum data size can be limited up to 16-bits at some instances. At instances with limited set of features, DSIZE2:0\\]
bits are reserved and must be kept at reset state. DSIZE\\[4:3\\]
bits then control next settings of data size: 00xxx: 8-bits 01xxx: 16-bits 10xxx: 24-bits 11xxx: 32-bits."]
pub type DSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSIZE` writer - number of bits in at single SPI data frame ..... Note: Maximum data size can be limited up to 16-bits at some instances. At instances with limited set of features, DSIZE2:0\\]
bits are reserved and must be kept at reset state. DSIZE\\[4:3\\]
bits then control next settings of data size: 00xxx: 8-bits 01xxx: 16-bits 10xxx: 24-bits 11xxx: 32-bits."]
pub type DSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG1_SPEC, u8, u8, 5, O>;
#[doc = "Field `FTHLV` reader - FIFO threshold level Defines number of data frames at single data packet. Size of the packet should not exceed 1/2 of FIFO space. SPI interface is more efficient if configured packet sizes are aligned with data register access parallelism: If SPI data register is accessed as a 16-bit register and DSIZE ≤ 8 bit, better to select FTHLV = 2, 4, 6. If SPI data register is accessed as a 32-bit register and DSIZE> 8 bit, better to select FTHLV = 2, 4, 6, while if DSIZE ≤ 8bit, better to select FTHLV = 4, 8, 12. Note: FTHLV\\[3:2\\]
bits are reserved at instances with limited set of features"]
pub type FTHLV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FTHLV` writer - FIFO threshold level Defines number of data frames at single data packet. Size of the packet should not exceed 1/2 of FIFO space. SPI interface is more efficient if configured packet sizes are aligned with data register access parallelism: If SPI data register is accessed as a 16-bit register and DSIZE ≤ 8 bit, better to select FTHLV = 2, 4, 6. If SPI data register is accessed as a 32-bit register and DSIZE> 8 bit, better to select FTHLV = 2, 4, 6, while if DSIZE ≤ 8bit, better to select FTHLV = 4, 8, 12. Note: FTHLV\\[3:2\\]
bits are reserved at instances with limited set of features"]
pub type FTHLV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG1_SPEC, u8, u8, 4, O>;
#[doc = "Field `UDRCFG` reader - behavior of slave transmitter at underrun condition For more details see underrun condition."]
pub type UDRCFG_R = crate::BitReader<bool>;
#[doc = "Field `UDRCFG` writer - behavior of slave transmitter at underrun condition For more details see underrun condition."]
pub type UDRCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, O>;
#[doc = "Field `RXDMAEN` reader - Rx DMA stream enable"]
pub type RXDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `RXDMAEN` writer - Rx DMA stream enable"]
pub type RXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, O>;
#[doc = "Field `TXDMAEN` reader - Tx DMA stream enable"]
pub type TXDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `TXDMAEN` writer - Tx DMA stream enable"]
pub type TXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, O>;
#[doc = "Field `CRCSIZE` reader - length of CRC frame to be transacted and compared Most significant bits are taken into account from polynomial calculation when CRC result is transacted or compared. The length of the polynomial is not affected by this setting. ..... The value must be set equal or multiply of data size (DSIZE\\[4:0\\]). Its maximum size corresponds to DSIZE maximum at the instance. Note: The most significant bit at CRCSIZE bit field is reserved at the peripheral instances where data size is limited to 16-bit."]
pub type CRCSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CRCSIZE` writer - length of CRC frame to be transacted and compared Most significant bits are taken into account from polynomial calculation when CRC result is transacted or compared. The length of the polynomial is not affected by this setting. ..... The value must be set equal or multiply of data size (DSIZE\\[4:0\\]). Its maximum size corresponds to DSIZE maximum at the instance. Note: The most significant bit at CRCSIZE bit field is reserved at the peripheral instances where data size is limited to 16-bit."]
pub type CRCSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG1_SPEC, u8, u8, 5, O>;
#[doc = "Field `CRCEN` reader - hardware CRC computation enable"]
pub type CRCEN_R = crate::BitReader<bool>;
#[doc = "Field `CRCEN` writer - hardware CRC computation enable"]
pub type CRCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, O>;
#[doc = "Field `MBR` reader - master baud rate prescaler setting Note: MBR setting is considered at slave working at TI mode, too (see mode)."]
pub type MBR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MBR` writer - master baud rate prescaler setting Note: MBR setting is considered at slave working at TI mode, too (see mode)."]
pub type MBR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG1_SPEC, u8, u8, 3, O>;
#[doc = "Field `BPASS` reader - bypass of the prescaler at master baud rate clock generator"]
pub type BPASS_R = crate::BitReader<bool>;
#[doc = "Field `BPASS` writer - bypass of the prescaler at master baud rate clock generator"]
pub type BPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - number of bits in at single SPI data frame ..... Note: Maximum data size can be limited up to 16-bits at some instances. At instances with limited set of features, DSIZE2:0\\]
bits are reserved and must be kept at reset state. DSIZE\\[4:3\\]
bits then control next settings of data size: 00xxx: 8-bits 01xxx: 16-bits 10xxx: 24-bits 11xxx: 32-bits."]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:8 - FIFO threshold level Defines number of data frames at single data packet. Size of the packet should not exceed 1/2 of FIFO space. SPI interface is more efficient if configured packet sizes are aligned with data register access parallelism: If SPI data register is accessed as a 16-bit register and DSIZE ≤ 8 bit, better to select FTHLV = 2, 4, 6. If SPI data register is accessed as a 32-bit register and DSIZE> 8 bit, better to select FTHLV = 2, 4, 6, while if DSIZE ≤ 8bit, better to select FTHLV = 4, 8, 12. Note: FTHLV\\[3:2\\]
bits are reserved at instances with limited set of features"]
    #[inline(always)]
    pub fn fthlv(&self) -> FTHLV_R {
        FTHLV_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - behavior of slave transmitter at underrun condition For more details see underrun condition."]
    #[inline(always)]
    pub fn udrcfg(&self) -> UDRCFG_R {
        UDRCFG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Rx DMA stream enable"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx DMA stream enable"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - length of CRC frame to be transacted and compared Most significant bits are taken into account from polynomial calculation when CRC result is transacted or compared. The length of the polynomial is not affected by this setting. ..... The value must be set equal or multiply of data size (DSIZE\\[4:0\\]). Its maximum size corresponds to DSIZE maximum at the instance. Note: The most significant bit at CRCSIZE bit field is reserved at the peripheral instances where data size is limited to 16-bit."]
    #[inline(always)]
    pub fn crcsize(&self) -> CRCSIZE_R {
        CRCSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - hardware CRC computation enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 28:30 - master baud rate prescaler setting Note: MBR setting is considered at slave working at TI mode, too (see mode)."]
    #[inline(always)]
    pub fn mbr(&self) -> MBR_R {
        MBR_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - bypass of the prescaler at master baud rate clock generator"]
    #[inline(always)]
    pub fn bpass(&self) -> BPASS_R {
        BPASS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - number of bits in at single SPI data frame ..... Note: Maximum data size can be limited up to 16-bits at some instances. At instances with limited set of features, DSIZE2:0\\]
bits are reserved and must be kept at reset state. DSIZE\\[4:3\\]
bits then control next settings of data size: 00xxx: 8-bits 01xxx: 16-bits 10xxx: 24-bits 11xxx: 32-bits."]
    #[inline(always)]
    #[must_use]
    pub fn dsize(&mut self) -> DSIZE_W<0> {
        DSIZE_W::new(self)
    }
    #[doc = "Bits 5:8 - FIFO threshold level Defines number of data frames at single data packet. Size of the packet should not exceed 1/2 of FIFO space. SPI interface is more efficient if configured packet sizes are aligned with data register access parallelism: If SPI data register is accessed as a 16-bit register and DSIZE ≤ 8 bit, better to select FTHLV = 2, 4, 6. If SPI data register is accessed as a 32-bit register and DSIZE> 8 bit, better to select FTHLV = 2, 4, 6, while if DSIZE ≤ 8bit, better to select FTHLV = 4, 8, 12. Note: FTHLV\\[3:2\\]
bits are reserved at instances with limited set of features"]
    #[inline(always)]
    #[must_use]
    pub fn fthlv(&mut self) -> FTHLV_W<5> {
        FTHLV_W::new(self)
    }
    #[doc = "Bit 9 - behavior of slave transmitter at underrun condition For more details see underrun condition."]
    #[inline(always)]
    #[must_use]
    pub fn udrcfg(&mut self) -> UDRCFG_W<9> {
        UDRCFG_W::new(self)
    }
    #[doc = "Bit 14 - Rx DMA stream enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<14> {
        RXDMAEN_W::new(self)
    }
    #[doc = "Bit 15 - Tx DMA stream enable"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<15> {
        TXDMAEN_W::new(self)
    }
    #[doc = "Bits 16:20 - length of CRC frame to be transacted and compared Most significant bits are taken into account from polynomial calculation when CRC result is transacted or compared. The length of the polynomial is not affected by this setting. ..... The value must be set equal or multiply of data size (DSIZE\\[4:0\\]). Its maximum size corresponds to DSIZE maximum at the instance. Note: The most significant bit at CRCSIZE bit field is reserved at the peripheral instances where data size is limited to 16-bit."]
    #[inline(always)]
    #[must_use]
    pub fn crcsize(&mut self) -> CRCSIZE_W<16> {
        CRCSIZE_W::new(self)
    }
    #[doc = "Bit 22 - hardware CRC computation enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<22> {
        CRCEN_W::new(self)
    }
    #[doc = "Bits 28:30 - master baud rate prescaler setting Note: MBR setting is considered at slave working at TI mode, too (see mode)."]
    #[inline(always)]
    #[must_use]
    pub fn mbr(&mut self) -> MBR_W<28> {
        MBR_W::new(self)
    }
    #[doc = "Bit 31 - bypass of the prescaler at master baud rate clock generator"]
    #[inline(always)]
    #[must_use]
    pub fn bpass(&mut self) -> BPASS_W<31> {
        BPASS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI/I2S configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg1](index.html) module"]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg1::R](R) reader structure"]
impl crate::Readable for CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg1::W](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0x0007_0007"]
impl crate::Resettable for CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0007;
}
