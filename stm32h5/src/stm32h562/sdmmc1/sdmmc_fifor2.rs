#[doc = "Register `SDMMC_FIFOR2` reader"]
pub struct R(crate::R<SDMMC_FIFOR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_FIFOR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_FIFOR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_FIFOR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMC_FIFOR2` writer"]
pub struct W(crate::W<SDMMC_FIFOR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_FIFOR2_SPEC>;
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
impl From<crate::W<SDMMC_FIFOR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_FIFOR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFODATA` reader - Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT = 1). The FIFO data occupies 16 entries of 32-bit words."]
pub type FIFODATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FIFODATA` writer - Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT = 1). The FIFO data occupies 16 entries of 32-bit words."]
pub type FIFODATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SDMMC_FIFOR2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT = 1). The FIFO data occupies 16 entries of 32-bit words."]
    #[inline(always)]
    pub fn fifodata(&self) -> FIFODATA_R {
        FIFODATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT = 1). The FIFO data occupies 16 entries of 32-bit words."]
    #[inline(always)]
    #[must_use]
    pub fn fifodata(&mut self) -> FIFODATA_W<0> {
        FIFODATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDMMC data FIFO registers 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_fifor2](index.html) module"]
pub struct SDMMC_FIFOR2_SPEC;
impl crate::RegisterSpec for SDMMC_FIFOR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_fifor2::R](R) reader structure"]
impl crate::Readable for SDMMC_FIFOR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmc_fifor2::W](W) writer structure"]
impl crate::Writable for SDMMC_FIFOR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDMMC_FIFOR2 to value 0"]
impl crate::Resettable for SDMMC_FIFOR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}