#[doc = "Register `I2SCFGR` reader"]
pub struct R(crate::R<I2SCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SCFGR` writer"]
pub struct W(crate::W<I2SCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SCFGR_SPEC>;
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
impl From<crate::W<I2SCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHLEN` reader - Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode."]
pub type CHLEN_R = crate::BitReader<bool>;
#[doc = "Field `CHLEN` writer - Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode."]
pub type CHLEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, I2SCFGR_SPEC, bool, O>;
#[doc = "Field `DATLEN` reader - Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
pub type DATLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATLEN` writer - Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
pub type DATLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u16, I2SCFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CKPOL` reader - Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode. The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals."]
pub type CKPOL_R = crate::BitReader<bool>;
#[doc = "Field `CKPOL` writer - Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode. The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals."]
pub type CKPOL_W<'a, const O: u8> = crate::BitWriter<'a, u16, I2SCFGR_SPEC, bool, O>;
#[doc = "Field `I2SSTD` reader - I2S standard selection For more details on I2S standards, refer to Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
pub type I2SSTD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2SSTD` writer - I2S standard selection For more details on I2S standards, refer to Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
pub type I2SSTD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, I2SCFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PCMSYNC` reader - PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode."]
pub type PCMSYNC_R = crate::BitReader<bool>;
#[doc = "Field `PCMSYNC` writer - PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode."]
pub type PCMSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u16, I2SCFGR_SPEC, bool, O>;
#[doc = "Field `I2SCFG` reader - I2S configuration mode Note: These bits should be configured when the I2S is disabled. They are not used in SPI mode."]
pub type I2SCFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2SCFG` writer - I2S configuration mode Note: These bits should be configured when the I2S is disabled. They are not used in SPI mode."]
pub type I2SCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u16, I2SCFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `I2SE` reader - I2S enable Note: This bit is not used in SPI mode."]
pub type I2SE_R = crate::BitReader<bool>;
#[doc = "Field `I2SE` writer - I2S enable Note: This bit is not used in SPI mode."]
pub type I2SE_W<'a, const O: u8> = crate::BitWriter<'a, u16, I2SCFGR_SPEC, bool, O>;
#[doc = "Field `I2SMOD` reader - I2S mode selection Note: This bit should be configured when the SPI is disabled."]
pub type I2SMOD_R = crate::BitReader<bool>;
#[doc = "Field `I2SMOD` writer - I2S mode selection Note: This bit should be configured when the SPI is disabled."]
pub type I2SMOD_W<'a, const O: u8> = crate::BitWriter<'a, u16, I2SCFGR_SPEC, bool, O>;
#[doc = "Field `ASTRTEN` reader - Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards. The appropriate level is a low level on WS signal when I2S Philips Standard is used, or a high level for other standards. Please refer to for additional information."]
pub type ASTRTEN_R = crate::BitReader<bool>;
#[doc = "Field `ASTRTEN` writer - Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards. The appropriate level is a low level on WS signal when I2S Philips Standard is used, or a high level for other standards. Please refer to for additional information."]
pub type ASTRTEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, I2SCFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode."]
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode. The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals."]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - I2S standard selection For more details on I2S standards, refer to Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode."]
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - I2S configuration mode Note: These bits should be configured when the I2S is disabled. They are not used in SPI mode."]
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - I2S enable Note: This bit is not used in SPI mode."]
    #[inline(always)]
    pub fn i2se(&self) -> I2SE_R {
        I2SE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2S mode selection Note: This bit should be configured when the SPI is disabled."]
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards. The appropriate level is a low level on WS signal when I2S Philips Standard is used, or a high level for other standards. Please refer to for additional information."]
    #[inline(always)]
    pub fn astrten(&self) -> ASTRTEN_R {
        ASTRTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn chlen(&mut self) -> CHLEN_W<0> {
        CHLEN_W::new(self)
    }
    #[doc = "Bits 1:2 - Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn datlen(&mut self) -> DATLEN_W<1> {
        DATLEN_W::new(self)
    }
    #[doc = "Bit 3 - Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode. The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals."]
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CKPOL_W<3> {
        CKPOL_W::new(self)
    }
    #[doc = "Bits 4:5 - I2S standard selection For more details on I2S standards, refer to Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn i2sstd(&mut self) -> I2SSTD_W<4> {
        I2SSTD_W::new(self)
    }
    #[doc = "Bit 7 - PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn pcmsync(&mut self) -> PCMSYNC_W<7> {
        PCMSYNC_W::new(self)
    }
    #[doc = "Bits 8:9 - I2S configuration mode Note: These bits should be configured when the I2S is disabled. They are not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn i2scfg(&mut self) -> I2SCFG_W<8> {
        I2SCFG_W::new(self)
    }
    #[doc = "Bit 10 - I2S enable Note: This bit is not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn i2se(&mut self) -> I2SE_W<10> {
        I2SE_W::new(self)
    }
    #[doc = "Bit 11 - I2S mode selection Note: This bit should be configured when the SPI is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn i2smod(&mut self) -> I2SMOD_W<11> {
        I2SMOD_W::new(self)
    }
    #[doc = "Bit 12 - Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards. The appropriate level is a low level on WS signal when I2S Philips Standard is used, or a high level for other standards. Please refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn astrten(&mut self) -> ASTRTEN_W<12> {
        ASTRTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_I2S configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2scfgr](index.html) module"]
pub struct I2SCFGR_SPEC;
impl crate::RegisterSpec for I2SCFGR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [i2scfgr::R](R) reader structure"]
impl crate::Readable for I2SCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2scfgr::W](W) writer structure"]
impl crate::Writable for I2SCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SCFGR to value 0"]
impl crate::Resettable for I2SCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
