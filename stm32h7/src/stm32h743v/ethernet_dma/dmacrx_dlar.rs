#[doc = "Register `DMACRxDLAR` reader"]
pub struct R(crate::R<DMACRX_DLAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACRX_DLAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACRX_DLAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACRX_DLAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACRxDLAR` writer"]
pub struct W(crate::W<DMACRX_DLAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACRX_DLAR_SPEC>;
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
impl From<crate::W<DMACRX_DLAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACRX_DLAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDESLA` reader - Start of Receive List"]
pub type RDESLA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RDESLA` writer - Start of Receive List"]
pub type RDESLA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACRX_DLAR_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 2:31 - Start of Receive List"]
    #[inline(always)]
    pub fn rdesla(&self) -> RDESLA_R {
        RDESLA_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Start of Receive List"]
    #[inline(always)]
    #[must_use]
    pub fn rdesla(&mut self) -> RDESLA_W<2> {
        RDESLA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Rx descriptor list address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacrx_dlar](index.html) module"]
pub struct DMACRX_DLAR_SPEC;
impl crate::RegisterSpec for DMACRX_DLAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacrx_dlar::R](R) reader structure"]
impl crate::Readable for DMACRX_DLAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmacrx_dlar::W](W) writer structure"]
impl crate::Writable for DMACRX_DLAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACRxDLAR to value 0"]
impl crate::Resettable for DMACRX_DLAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
