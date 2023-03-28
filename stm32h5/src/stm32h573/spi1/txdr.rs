#[doc = "Register `TXDR` writer"]
pub struct W(crate::W<TXDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXDR_SPEC>;
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
impl From<crate::W<TXDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDR` writer - transmit data register The register serves as an interface with TxFIFO. A write to it accesses TxFIFO. Note: In SPI mode, data is always right-aligned. Alignment of data at I2S mode depends on DATLEN and DATFMT setting. Unused bits are ignored when writing to the register, and read as zero when the register is read. Note: DR can be accessed byte-wise (8-bit access): in this case only one data-byte is written by single access. halfword-wise (16 bit access) in this case 2 data-bytes or 1 halfword-data can be written by single access. word-wise (32 bit access). In this case 4 data-bytes or 2 halfword-data or word-data can be written by single access. Write access of this register less than the configured data size is forbidden."]
pub type TXDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXDR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - transmit data register The register serves as an interface with TxFIFO. A write to it accesses TxFIFO. Note: In SPI mode, data is always right-aligned. Alignment of data at I2S mode depends on DATLEN and DATFMT setting. Unused bits are ignored when writing to the register, and read as zero when the register is read. Note: DR can be accessed byte-wise (8-bit access): in this case only one data-byte is written by single access. halfword-wise (16 bit access) in this case 2 data-bytes or 1 halfword-data can be written by single access. word-wise (32 bit access). In this case 4 data-bytes or 2 halfword-data or word-data can be written by single access. Write access of this register less than the configured data size is forbidden."]
    #[inline(always)]
    #[must_use]
    pub fn txdr(&mut self) -> TXDR_W<0> {
        TXDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI/I2S transmit data register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdr](index.html) module"]
pub struct TXDR_SPEC;
impl crate::RegisterSpec for TXDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [txdr::W](W) writer structure"]
impl crate::Writable for TXDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXDR to value 0"]
impl crate::Resettable for TXDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
