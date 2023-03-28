#[doc = "Register `ETH_MACSPI1R` reader"]
pub struct R(crate::R<ETH_MACSPI1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACSPI1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACSPI1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACSPI1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACSPI1R` writer"]
pub struct W(crate::W<ETH_MACSPI1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACSPI1R_SPEC>;
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
impl From<crate::W<ETH_MACSPI1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACSPI1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI1` reader - Source Port Identity 1 This field indicates bits \\[63:32\\]
of sourcePortIdentity of PTP node."]
pub type SPI1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SPI1` writer - Source Port Identity 1 This field indicates bits \\[63:32\\]
of sourcePortIdentity of PTP node."]
pub type SPI1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACSPI1R_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Source Port Identity 1 This field indicates bits \\[63:32\\]
of sourcePortIdentity of PTP node."]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Port Identity 1 This field indicates bits \\[63:32\\]
of sourcePortIdentity of PTP node."]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> SPI1_W<0> {
        SPI1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PTP Source port identity 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macspi1r](index.html) module"]
pub struct ETH_MACSPI1R_SPEC;
impl crate::RegisterSpec for ETH_MACSPI1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macspi1r::R](R) reader structure"]
impl crate::Readable for ETH_MACSPI1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macspi1r::W](W) writer structure"]
impl crate::Writable for ETH_MACSPI1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACSPI1R to value 0"]
impl crate::Resettable for ETH_MACSPI1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
