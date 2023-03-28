#[doc = "Register `DMACRxDTPR` reader"]
pub struct R(crate::R<DMACRX_DTPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACRX_DTPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACRX_DTPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACRX_DTPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACRxDTPR` writer"]
pub struct W(crate::W<DMACRX_DTPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACRX_DTPR_SPEC>;
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
impl From<crate::W<DMACRX_DTPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACRX_DTPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDT` reader - Receive Descriptor Tail Pointer"]
pub type RDT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RDT` writer - Receive Descriptor Tail Pointer"]
pub type RDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACRX_DTPR_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 2:31 - Receive Descriptor Tail Pointer"]
    #[inline(always)]
    pub fn rdt(&self) -> RDT_R {
        RDT_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Receive Descriptor Tail Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn rdt(&mut self) -> RDT_W<2> {
        RDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Rx descriptor tail pointer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacrx_dtpr](index.html) module"]
pub struct DMACRX_DTPR_SPEC;
impl crate::RegisterSpec for DMACRX_DTPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacrx_dtpr::R](R) reader structure"]
impl crate::Readable for DMACRX_DTPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmacrx_dtpr::W](W) writer structure"]
impl crate::Writable for DMACRX_DTPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACRxDTPR to value 0"]
impl crate::Resettable for DMACRX_DTPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
