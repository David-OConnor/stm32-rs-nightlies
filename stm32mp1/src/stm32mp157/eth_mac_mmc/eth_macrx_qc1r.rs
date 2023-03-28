#[doc = "Register `ETH_MACRxQC1R` reader"]
pub struct R(crate::R<ETH_MACRX_QC1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACRX_QC1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACRX_QC1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACRX_QC1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACRxQC1R` writer"]
pub struct W(crate::W<ETH_MACRX_QC1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACRX_QC1R_SPEC>;
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
impl From<crate::W<ETH_MACRX_QC1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACRX_QC1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AVCPQ` reader - AVCPQ"]
pub type AVCPQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AVCPQ` writer - AVCPQ"]
pub type AVCPQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACRX_QC1R_SPEC, u8, u8, 3, O>;
#[doc = "Field `AVPTPQ` reader - AVPTPQ"]
pub type AVPTPQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AVPTPQ` writer - AVPTPQ"]
pub type AVPTPQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACRX_QC1R_SPEC, u8, u8, 3, O>;
#[doc = "Field `UPQ` reader - UPQ"]
pub type UPQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UPQ` writer - UPQ"]
pub type UPQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACRX_QC1R_SPEC, u8, u8, 3, O>;
#[doc = "Field `MCBCQ` reader - MCBCQ"]
pub type MCBCQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCBCQ` writer - MCBCQ"]
pub type MCBCQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACRX_QC1R_SPEC, u8, u8, 3, O>;
#[doc = "Field `MCBCQEN` reader - MCBCQEN"]
pub type MCBCQEN_R = crate::BitReader<bool>;
#[doc = "Field `MCBCQEN` writer - MCBCQEN"]
pub type MCBCQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACRX_QC1R_SPEC, bool, O>;
#[doc = "Field `TACPQE` reader - TACPQE"]
pub type TACPQE_R = crate::BitReader<bool>;
#[doc = "Field `TACPQE` writer - TACPQE"]
pub type TACPQE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACRX_QC1R_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - AVCPQ"]
    #[inline(always)]
    pub fn avcpq(&self) -> AVCPQ_R {
        AVCPQ_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - AVPTPQ"]
    #[inline(always)]
    pub fn avptpq(&self) -> AVPTPQ_R {
        AVPTPQ_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 12:14 - UPQ"]
    #[inline(always)]
    pub fn upq(&self) -> UPQ_R {
        UPQ_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - MCBCQ"]
    #[inline(always)]
    pub fn mcbcq(&self) -> MCBCQ_R {
        MCBCQ_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - MCBCQEN"]
    #[inline(always)]
    pub fn mcbcqen(&self) -> MCBCQEN_R {
        MCBCQEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TACPQE"]
    #[inline(always)]
    pub fn tacpqe(&self) -> TACPQE_R {
        TACPQE_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - AVCPQ"]
    #[inline(always)]
    #[must_use]
    pub fn avcpq(&mut self) -> AVCPQ_W<0> {
        AVCPQ_W::new(self)
    }
    #[doc = "Bits 4:6 - AVPTPQ"]
    #[inline(always)]
    #[must_use]
    pub fn avptpq(&mut self) -> AVPTPQ_W<4> {
        AVPTPQ_W::new(self)
    }
    #[doc = "Bits 12:14 - UPQ"]
    #[inline(always)]
    #[must_use]
    pub fn upq(&mut self) -> UPQ_W<12> {
        UPQ_W::new(self)
    }
    #[doc = "Bits 16:18 - MCBCQ"]
    #[inline(always)]
    #[must_use]
    pub fn mcbcq(&mut self) -> MCBCQ_W<16> {
        MCBCQ_W::new(self)
    }
    #[doc = "Bit 20 - MCBCQEN"]
    #[inline(always)]
    #[must_use]
    pub fn mcbcqen(&mut self) -> MCBCQEN_W<20> {
        MCBCQEN_W::new(self)
    }
    #[doc = "Bit 21 - TACPQE"]
    #[inline(always)]
    #[must_use]
    pub fn tacpqe(&mut self) -> TACPQE_W<21> {
        TACPQE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Receive Queue Control 1 register controls queue 1 management in the MAC receiver.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macrx_qc1r](index.html) module"]
pub struct ETH_MACRX_QC1R_SPEC;
impl crate::RegisterSpec for ETH_MACRX_QC1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macrx_qc1r::R](R) reader structure"]
impl crate::Readable for ETH_MACRX_QC1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macrx_qc1r::W](W) writer structure"]
impl crate::Writable for ETH_MACRX_QC1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACRxQC1R to value 0"]
impl crate::Resettable for ETH_MACRX_QC1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
