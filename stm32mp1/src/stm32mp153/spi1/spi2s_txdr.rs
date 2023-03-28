#[doc = "Register `SPI2S_TXDR` writer"]
pub struct W(crate::W<SPI2S_TXDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI2S_TXDR_SPEC>;
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
impl From<crate::W<SPI2S_TXDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI2S_TXDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDR` writer - TXDR"]
pub type TXDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI2S_TXDR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - TXDR"]
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
#[doc = "SPI/I2S transmit data register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi2s_txdr](index.html) module"]
pub struct SPI2S_TXDR_SPEC;
impl crate::RegisterSpec for SPI2S_TXDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [spi2s_txdr::W](W) writer structure"]
impl crate::Writable for SPI2S_TXDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI2S_TXDR to value 0"]
impl crate::Resettable for SPI2S_TXDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
