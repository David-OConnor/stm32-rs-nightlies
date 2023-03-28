#[doc = "Register `ETH_MACSPI0R` reader"]
pub struct R(crate::R<ETH_MACSPI0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACSPI0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACSPI0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACSPI0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACSPI0R` writer"]
pub struct W(crate::W<ETH_MACSPI0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACSPI0R_SPEC>;
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
impl From<crate::W<ETH_MACSPI0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACSPI0R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI0` reader - SPI0"]
pub type SPI0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SPI0` writer - SPI0"]
pub type SPI0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACSPI0R_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - SPI0"]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SPI0"]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> SPI0_W<0> {
        SPI0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register contains Bits\\[31:0\\]
of the 80-bit Source Port Identity of the PTP node. This register is available only when the Enable PTP Timestamp Offload feature is selected.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macspi0r](index.html) module"]
pub struct ETH_MACSPI0R_SPEC;
impl crate::RegisterSpec for ETH_MACSPI0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macspi0r::R](R) reader structure"]
impl crate::Readable for ETH_MACSPI0R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macspi0r::W](W) writer structure"]
impl crate::Writable for ETH_MACSPI0R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACSPI0R to value 0"]
impl crate::Resettable for ETH_MACSPI0R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
