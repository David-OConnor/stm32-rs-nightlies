#[doc = "Register `ETH_MACRxQC0R` reader"]
pub struct R(crate::R<ETH_MACRX_QC0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACRX_QC0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACRX_QC0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACRX_QC0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACRxQC0R` writer"]
pub struct W(crate::W<ETH_MACRX_QC0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACRX_QC0R_SPEC>;
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
impl From<crate::W<ETH_MACRX_QC0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACRX_QC0R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXQ0EN` reader - RXQ0EN"]
pub type RXQ0EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXQ0EN` writer - RXQ0EN"]
pub type RXQ0EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACRX_QC0R_SPEC, u8, u8, 2, O>;
#[doc = "Field `RXQ1EN` reader - RXQ1EN"]
pub type RXQ1EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXQ1EN` writer - RXQ1EN"]
pub type RXQ1EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACRX_QC0R_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - RXQ0EN"]
    #[inline(always)]
    pub fn rxq0en(&self) -> RXQ0EN_R {
        RXQ0EN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - RXQ1EN"]
    #[inline(always)]
    pub fn rxq1en(&self) -> RXQ1EN_R {
        RXQ1EN_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RXQ0EN"]
    #[inline(always)]
    #[must_use]
    pub fn rxq0en(&mut self) -> RXQ0EN_W<0> {
        RXQ0EN_W::new(self)
    }
    #[doc = "Bits 2:3 - RXQ1EN"]
    #[inline(always)]
    #[must_use]
    pub fn rxq1en(&mut self) -> RXQ1EN_W<2> {
        RXQ1EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Receive Queue Control 0 register controls the queue management in the MAC Receiver.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macrx_qc0r](index.html) module"]
pub struct ETH_MACRX_QC0R_SPEC;
impl crate::RegisterSpec for ETH_MACRX_QC0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macrx_qc0r::R](R) reader structure"]
impl crate::Readable for ETH_MACRX_QC0R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macrx_qc0r::W](W) writer structure"]
impl crate::Writable for ETH_MACRX_QC0R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACRxQC0R to value 0"]
impl crate::Resettable for ETH_MACRX_QC0R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
