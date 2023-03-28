#[doc = "Register `ETH_MACSPI2R` reader"]
pub struct R(crate::R<ETH_MACSPI2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACSPI2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACSPI2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACSPI2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACSPI2R` writer"]
pub struct W(crate::W<ETH_MACSPI2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACSPI2R_SPEC>;
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
impl From<crate::W<ETH_MACSPI2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACSPI2R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI2` reader - Source Port Identity 2 This field indicates bits \\[79:64\\]
of sourcePortIdentity of PTP node."]
pub type SPI2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SPI2` writer - Source Port Identity 2 This field indicates bits \\[79:64\\]
of sourcePortIdentity of PTP node."]
pub type SPI2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACSPI2R_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Source Port Identity 2 This field indicates bits \\[79:64\\]
of sourcePortIdentity of PTP node."]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Source Port Identity 2 This field indicates bits \\[79:64\\]
of sourcePortIdentity of PTP node."]
    #[inline(always)]
    #[must_use]
    pub fn spi2(&mut self) -> SPI2_W<0> {
        SPI2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PTP Source port identity 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macspi2r](index.html) module"]
pub struct ETH_MACSPI2R_SPEC;
impl crate::RegisterSpec for ETH_MACSPI2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macspi2r::R](R) reader structure"]
impl crate::Readable for ETH_MACSPI2R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macspi2r::W](W) writer structure"]
impl crate::Writable for ETH_MACSPI2R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACSPI2R to value 0"]
impl crate::Resettable for ETH_MACSPI2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
