#[doc = "Register `SAI_ACR1` reader"]
pub struct R(crate::R<SAI_ACR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_ACR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_ACR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_ACR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAI_ACR1` writer"]
pub struct W(crate::W<SAI_ACR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_ACR1_SPEC>;
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
impl From<crate::W<SAI_ACR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_ACR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - SAIx audio block mode These bits are set and cleared by software. They must be configured when SAIx audio block is disabled. Note: When the audio block is configured in SPDIF mode, the master transmitter mode is forced (MODE\\[1:0\\]
= 00)."]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - SAIx audio block mode These bits are set and cleared by software. They must be configured when SAIx audio block is disabled. Note: When the audio block is configured in SPDIF mode, the master transmitter mode is forced (MODE\\[1:0\\]
= 00)."]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_ACR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `PRTCFG` reader - Protocol configuration These bits are set and cleared by software. These bits have to be configured when the audio block is disabled."]
pub type PRTCFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRTCFG` writer - Protocol configuration These bits are set and cleared by software. These bits have to be configured when the audio block is disabled."]
pub type PRTCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_ACR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `DS` reader - Data size These bits are set and cleared by software. These bits are ignored when the SPDIF protocols are selected (bit PRTCFG\\[1:0\\]), because the frame and the data size are fixed in such case. When the companding mode is selected through COMP\\[1:0\\]
bits, DS\\[1:0\\]
are ignored since the data size is fixed to 8 bits by the algorithm. These bits must be configured when the audio block is disabled."]
pub type DS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DS` writer - Data size These bits are set and cleared by software. These bits are ignored when the SPDIF protocols are selected (bit PRTCFG\\[1:0\\]), because the frame and the data size are fixed in such case. When the companding mode is selected through COMP\\[1:0\\]
bits, DS\\[1:0\\]
are ignored since the data size is fixed to 8 bits by the algorithm. These bits must be configured when the audio block is disabled."]
pub type DS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_ACR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `LSBFIRST` reader - Least significant bit first This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in AC’97 audio protocol since AC’97 data are always transferred with the MSB first. This bit has no meaning in SPDIF audio protocol since in SPDIF data are always transferred with LSB first."]
pub type LSBFIRST_R = crate::BitReader<bool>;
#[doc = "Field `LSBFIRST` writer - Least significant bit first This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in AC’97 audio protocol since AC’97 data are always transferred with the MSB first. This bit has no meaning in SPDIF audio protocol since in SPDIF data are always transferred with LSB first."]
pub type LSBFIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR1_SPEC, bool, O>;
#[doc = "Field `CKSTR` reader - Clock strobing edge This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in SPDIF audio protocol."]
pub type CKSTR_R = crate::BitReader<bool>;
#[doc = "Field `CKSTR` writer - Clock strobing edge This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in SPDIF audio protocol."]
pub type CKSTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR1_SPEC, bool, O>;
#[doc = "Field `SYNCEN` reader - Synchronization enable These bits are set and cleared by software. They must be configured when the audio subblock is disabled. Note: The audio subblock should be configured as asynchronous when SPDIF mode is enabled."]
pub type SYNCEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYNCEN` writer - Synchronization enable These bits are set and cleared by software. They must be configured when the audio subblock is disabled. Note: The audio subblock should be configured as asynchronous when SPDIF mode is enabled."]
pub type SYNCEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_ACR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `MONO` reader - Mono mode This bit is set and cleared by software. It is meaningful only when the number of slots is equal to 2. When the mono mode is selected, slot 0 data are duplicated on slot 1 when the audio block operates as a transmitter. In reception mode, the slot1 is discarded and only the data received from slot 0 are stored. Refer to for more details."]
pub type MONO_R = crate::BitReader<bool>;
#[doc = "Field `MONO` writer - Mono mode This bit is set and cleared by software. It is meaningful only when the number of slots is equal to 2. When the mono mode is selected, slot 0 data are duplicated on slot 1 when the audio block operates as a transmitter. In reception mode, the slot1 is discarded and only the data received from slot 0 are stored. Refer to for more details."]
pub type MONO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR1_SPEC, bool, O>;
#[doc = "Field `OUTDRIV` reader - Output drive This bit is set and cleared by software. Note: This bit has to be set before enabling the audio block and after the audio block configuration."]
pub type OUTDRIV_R = crate::BitReader<bool>;
#[doc = "Field `OUTDRIV` writer - Output drive This bit is set and cleared by software. Note: This bit has to be set before enabling the audio block and after the audio block configuration."]
pub type OUTDRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR1_SPEC, bool, O>;
#[doc = "Field `SAIEN` reader - Audio block enable This bit is set by software. To switch off the audio block, the application software must program this bit to 0 and poll the bit till it reads back 0, meaning that the block is completely disabled. Before setting this bit to 1, check that it is set to 0, otherwise the enable command is not taken into account. This bit allows controlling the state of the SAI audio block. If it is disabled when an audio frame transfer is ongoing, the ongoing transfer completes and the cell is fully disabled at the end of this audio frame transfer. Note: When the SAI block (A or B) is configured in master mode, the clock must be present on the SAI block input before setting SAIEN bit."]
pub type SAIEN_R = crate::BitReader<bool>;
#[doc = "Field `SAIEN` writer - Audio block enable This bit is set by software. To switch off the audio block, the application software must program this bit to 0 and poll the bit till it reads back 0, meaning that the block is completely disabled. Before setting this bit to 1, check that it is set to 0, otherwise the enable command is not taken into account. This bit allows controlling the state of the SAI audio block. If it is disabled when an audio frame transfer is ongoing, the ongoing transfer completes and the cell is fully disabled at the end of this audio frame transfer. Note: When the SAI block (A or B) is configured in master mode, the clock must be present on the SAI block input before setting SAIEN bit."]
pub type SAIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR1_SPEC, bool, O>;
#[doc = "Field `DMAEN` reader - DMA enable This bit is set and cleared by software. Note: Since the audio block defaults to operate as a transmitter after reset, the MODE\\[1:0\\]
bits must be configured before setting DMAEN to avoid a DMA request in receiver mode."]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - DMA enable This bit is set and cleared by software. Note: Since the audio block defaults to operate as a transmitter after reset, the MODE\\[1:0\\]
bits must be configured before setting DMAEN to avoid a DMA request in receiver mode."]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR1_SPEC, bool, O>;
#[doc = "Field `NODIV` reader - No divider This bit is set and cleared by software."]
pub type NODIV_R = crate::BitReader<bool>;
#[doc = "Field `NODIV` writer - No divider This bit is set and cleared by software."]
pub type NODIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR1_SPEC, bool, O>;
#[doc = "Field `MCKDIV` reader - Master clock divider These bits are set and cleared by software. Otherwise, The master clock frequency is calculated according to the formula given in . These bits have no meaning when the audio block is slave. They have to be configured when the audio block is disabled."]
pub type MCKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCKDIV` writer - Master clock divider These bits are set and cleared by software. Otherwise, The master clock frequency is calculated according to the formula given in . These bits have no meaning when the audio block is slave. They have to be configured when the audio block is disabled."]
pub type MCKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_ACR1_SPEC, u8, u8, 6, O>;
#[doc = "Field `OSR` reader - Oversampling ratio for master clock This bit is meaningful only when NODIV bit is set to 0."]
pub type OSR_R = crate::BitReader<bool>;
#[doc = "Field `OSR` writer - Oversampling ratio for master clock This bit is meaningful only when NODIV bit is set to 0."]
pub type OSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR1_SPEC, bool, O>;
#[doc = "Field `MCKEN` reader - Master clock generation enable"]
pub type MCKEN_R = crate::BitReader<bool>;
#[doc = "Field `MCKEN` writer - Master clock generation enable"]
pub type MCKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - SAIx audio block mode These bits are set and cleared by software. They must be configured when SAIx audio block is disabled. Note: When the audio block is configured in SPDIF mode, the master transmitter mode is forced (MODE\\[1:0\\]
= 00)."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Protocol configuration These bits are set and cleared by software. These bits have to be configured when the audio block is disabled."]
    #[inline(always)]
    pub fn prtcfg(&self) -> PRTCFG_R {
        PRTCFG_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Data size These bits are set and cleared by software. These bits are ignored when the SPDIF protocols are selected (bit PRTCFG\\[1:0\\]), because the frame and the data size are fixed in such case. When the companding mode is selected through COMP\\[1:0\\]
bits, DS\\[1:0\\]
are ignored since the data size is fixed to 8 bits by the algorithm. These bits must be configured when the audio block is disabled."]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Least significant bit first This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in AC’97 audio protocol since AC’97 data are always transferred with the MSB first. This bit has no meaning in SPDIF audio protocol since in SPDIF data are always transferred with LSB first."]
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock strobing edge This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in SPDIF audio protocol."]
    #[inline(always)]
    pub fn ckstr(&self) -> CKSTR_R {
        CKSTR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Synchronization enable These bits are set and cleared by software. They must be configured when the audio subblock is disabled. Note: The audio subblock should be configured as asynchronous when SPDIF mode is enabled."]
    #[inline(always)]
    pub fn syncen(&self) -> SYNCEN_R {
        SYNCEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Mono mode This bit is set and cleared by software. It is meaningful only when the number of slots is equal to 2. When the mono mode is selected, slot 0 data are duplicated on slot 1 when the audio block operates as a transmitter. In reception mode, the slot1 is discarded and only the data received from slot 0 are stored. Refer to for more details."]
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Output drive This bit is set and cleared by software. Note: This bit has to be set before enabling the audio block and after the audio block configuration."]
    #[inline(always)]
    pub fn outdriv(&self) -> OUTDRIV_R {
        OUTDRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Audio block enable This bit is set by software. To switch off the audio block, the application software must program this bit to 0 and poll the bit till it reads back 0, meaning that the block is completely disabled. Before setting this bit to 1, check that it is set to 0, otherwise the enable command is not taken into account. This bit allows controlling the state of the SAI audio block. If it is disabled when an audio frame transfer is ongoing, the ongoing transfer completes and the cell is fully disabled at the end of this audio frame transfer. Note: When the SAI block (A or B) is configured in master mode, the clock must be present on the SAI block input before setting SAIEN bit."]
    #[inline(always)]
    pub fn saien(&self) -> SAIEN_R {
        SAIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA enable This bit is set and cleared by software. Note: Since the audio block defaults to operate as a transmitter after reset, the MODE\\[1:0\\]
bits must be configured before setting DMAEN to avoid a DMA request in receiver mode."]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - No divider This bit is set and cleared by software."]
    #[inline(always)]
    pub fn nodiv(&self) -> NODIV_R {
        NODIV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:25 - Master clock divider These bits are set and cleared by software. Otherwise, The master clock frequency is calculated according to the formula given in . These bits have no meaning when the audio block is slave. They have to be configured when the audio block is disabled."]
    #[inline(always)]
    pub fn mckdiv(&self) -> MCKDIV_R {
        MCKDIV_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bit 26 - Oversampling ratio for master clock This bit is meaningful only when NODIV bit is set to 0."]
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Master clock generation enable"]
    #[inline(always)]
    pub fn mcken(&self) -> MCKEN_R {
        MCKEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SAIx audio block mode These bits are set and cleared by software. They must be configured when SAIx audio block is disabled. Note: When the audio block is configured in SPDIF mode, the master transmitter mode is forced (MODE\\[1:0\\]
= 00)."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bits 2:3 - Protocol configuration These bits are set and cleared by software. These bits have to be configured when the audio block is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn prtcfg(&mut self) -> PRTCFG_W<2> {
        PRTCFG_W::new(self)
    }
    #[doc = "Bits 5:7 - Data size These bits are set and cleared by software. These bits are ignored when the SPDIF protocols are selected (bit PRTCFG\\[1:0\\]), because the frame and the data size are fixed in such case. When the companding mode is selected through COMP\\[1:0\\]
bits, DS\\[1:0\\]
are ignored since the data size is fixed to 8 bits by the algorithm. These bits must be configured when the audio block is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn ds(&mut self) -> DS_W<5> {
        DS_W::new(self)
    }
    #[doc = "Bit 8 - Least significant bit first This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in AC’97 audio protocol since AC’97 data are always transferred with the MSB first. This bit has no meaning in SPDIF audio protocol since in SPDIF data are always transferred with LSB first."]
    #[inline(always)]
    #[must_use]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W<8> {
        LSBFIRST_W::new(self)
    }
    #[doc = "Bit 9 - Clock strobing edge This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in SPDIF audio protocol."]
    #[inline(always)]
    #[must_use]
    pub fn ckstr(&mut self) -> CKSTR_W<9> {
        CKSTR_W::new(self)
    }
    #[doc = "Bits 10:11 - Synchronization enable These bits are set and cleared by software. They must be configured when the audio subblock is disabled. Note: The audio subblock should be configured as asynchronous when SPDIF mode is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn syncen(&mut self) -> SYNCEN_W<10> {
        SYNCEN_W::new(self)
    }
    #[doc = "Bit 12 - Mono mode This bit is set and cleared by software. It is meaningful only when the number of slots is equal to 2. When the mono mode is selected, slot 0 data are duplicated on slot 1 when the audio block operates as a transmitter. In reception mode, the slot1 is discarded and only the data received from slot 0 are stored. Refer to for more details."]
    #[inline(always)]
    #[must_use]
    pub fn mono(&mut self) -> MONO_W<12> {
        MONO_W::new(self)
    }
    #[doc = "Bit 13 - Output drive This bit is set and cleared by software. Note: This bit has to be set before enabling the audio block and after the audio block configuration."]
    #[inline(always)]
    #[must_use]
    pub fn outdriv(&mut self) -> OUTDRIV_W<13> {
        OUTDRIV_W::new(self)
    }
    #[doc = "Bit 16 - Audio block enable This bit is set by software. To switch off the audio block, the application software must program this bit to 0 and poll the bit till it reads back 0, meaning that the block is completely disabled. Before setting this bit to 1, check that it is set to 0, otherwise the enable command is not taken into account. This bit allows controlling the state of the SAI audio block. If it is disabled when an audio frame transfer is ongoing, the ongoing transfer completes and the cell is fully disabled at the end of this audio frame transfer. Note: When the SAI block (A or B) is configured in master mode, the clock must be present on the SAI block input before setting SAIEN bit."]
    #[inline(always)]
    #[must_use]
    pub fn saien(&mut self) -> SAIEN_W<16> {
        SAIEN_W::new(self)
    }
    #[doc = "Bit 17 - DMA enable This bit is set and cleared by software. Note: Since the audio block defaults to operate as a transmitter after reset, the MODE\\[1:0\\]
bits must be configured before setting DMAEN to avoid a DMA request in receiver mode."]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<17> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 19 - No divider This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn nodiv(&mut self) -> NODIV_W<19> {
        NODIV_W::new(self)
    }
    #[doc = "Bits 20:25 - Master clock divider These bits are set and cleared by software. Otherwise, The master clock frequency is calculated according to the formula given in . These bits have no meaning when the audio block is slave. They have to be configured when the audio block is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn mckdiv(&mut self) -> MCKDIV_W<20> {
        MCKDIV_W::new(self)
    }
    #[doc = "Bit 26 - Oversampling ratio for master clock This bit is meaningful only when NODIV bit is set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn osr(&mut self) -> OSR_W<26> {
        OSR_W::new(self)
    }
    #[doc = "Bit 27 - Master clock generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcken(&mut self) -> MCKEN_W<27> {
        MCKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_acr1](index.html) module"]
pub struct SAI_ACR1_SPEC;
impl crate::RegisterSpec for SAI_ACR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sai_acr1::R](R) reader structure"]
impl crate::Readable for SAI_ACR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sai_acr1::W](W) writer structure"]
impl crate::Writable for SAI_ACR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAI_ACR1 to value 0x40"]
impl crate::Resettable for SAI_ACR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
