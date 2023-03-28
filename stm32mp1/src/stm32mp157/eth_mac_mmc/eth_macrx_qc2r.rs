#[doc = "Register `ETH_MACRxQC2R` reader"]
pub struct R(crate::R<ETH_MACRX_QC2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACRX_QC2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACRX_QC2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACRX_QC2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACRxQC2R` writer"]
pub struct W(crate::W<ETH_MACRX_QC2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACRX_QC2R_SPEC>;
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
impl From<crate::W<ETH_MACRX_QC2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACRX_QC2R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSRQ0` reader - PSRQ0"]
pub type PSRQ0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSRQ0` writer - PSRQ0"]
pub type PSRQ0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACRX_QC2R_SPEC, u8, u8, 8, O>;
#[doc = "Field `PSRQ1` reader - PSRQ1"]
pub type PSRQ1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSRQ1` writer - PSRQ1"]
pub type PSRQ1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACRX_QC2R_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - PSRQ0"]
    #[inline(always)]
    pub fn psrq0(&self) -> PSRQ0_R {
        PSRQ0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PSRQ1"]
    #[inline(always)]
    pub fn psrq1(&self) -> PSRQ1_R {
        PSRQ1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PSRQ0"]
    #[inline(always)]
    #[must_use]
    pub fn psrq0(&mut self) -> PSRQ0_W<0> {
        PSRQ0_W::new(self)
    }
    #[doc = "Bits 8:15 - PSRQ1"]
    #[inline(always)]
    #[must_use]
    pub fn psrq1(&mut self) -> PSRQ1_W<8> {
        PSRQ1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register controls the routing of tagged packets based on the USP (user priority) field of the received packets to the Rx queue 0 and 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macrx_qc2r](index.html) module"]
pub struct ETH_MACRX_QC2R_SPEC;
impl crate::RegisterSpec for ETH_MACRX_QC2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macrx_qc2r::R](R) reader structure"]
impl crate::Readable for ETH_MACRX_QC2R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macrx_qc2r::W](W) writer structure"]
impl crate::Writable for ETH_MACRX_QC2R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACRxQC2R to value 0"]
impl crate::Resettable for ETH_MACRX_QC2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
