#[doc = "Register `UDRDR` reader"]
pub struct R(crate::R<UDRDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UDRDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UDRDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UDRDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UDRDR` writer"]
pub struct W(crate::W<UDRDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UDRDR_SPEC>;
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
impl From<crate::W<UDRDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UDRDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UDRDR` reader - data at slave underrun condition The register is taken into account in Slave mode and at underrun condition only. The number of bits considered depends on DSIZE bit settings of the SPI_CFG1 register. Underrun condition handling depends on setting UDRCFG bit at SPI_CFG1 register. Note: UDRDR\\[31-16\\]
bits are reserved at the peripheral instances with data size limited to 16-bit. There is no constraint when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored."]
pub type UDRDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `UDRDR` writer - data at slave underrun condition The register is taken into account in Slave mode and at underrun condition only. The number of bits considered depends on DSIZE bit settings of the SPI_CFG1 register. Underrun condition handling depends on setting UDRCFG bit at SPI_CFG1 register. Note: UDRDR\\[31-16\\]
bits are reserved at the peripheral instances with data size limited to 16-bit. There is no constraint when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored."]
pub type UDRDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UDRDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - data at slave underrun condition The register is taken into account in Slave mode and at underrun condition only. The number of bits considered depends on DSIZE bit settings of the SPI_CFG1 register. Underrun condition handling depends on setting UDRCFG bit at SPI_CFG1 register. Note: UDRDR\\[31-16\\]
bits are reserved at the peripheral instances with data size limited to 16-bit. There is no constraint when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored."]
    #[inline(always)]
    pub fn udrdr(&self) -> UDRDR_R {
        UDRDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data at slave underrun condition The register is taken into account in Slave mode and at underrun condition only. The number of bits considered depends on DSIZE bit settings of the SPI_CFG1 register. Underrun condition handling depends on setting UDRCFG bit at SPI_CFG1 register. Note: UDRDR\\[31-16\\]
bits are reserved at the peripheral instances with data size limited to 16-bit. There is no constraint when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn udrdr(&mut self) -> UDRDR_W<0> {
        UDRDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI/I2S underrun data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udrdr](index.html) module"]
pub struct UDRDR_SPEC;
impl crate::RegisterSpec for UDRDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [udrdr::R](R) reader structure"]
impl crate::Readable for UDRDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [udrdr::W](W) writer structure"]
impl crate::Writable for UDRDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDRDR to value 0"]
impl crate::Resettable for UDRDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
