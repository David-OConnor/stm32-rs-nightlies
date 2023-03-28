#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Data buffer overrun/underrun interrupt status clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_ISC_AW {
    #[doc = "1: Writing this bit to 1 clears the OVR_RIS bit in PSSI_RIS"]
    Clear = 1,
}
impl From<OVR_ISC_AW> for bool {
    #[inline(always)]
    fn from(variant: OVR_ISC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR_ISC` writer - Data buffer overrun/underrun interrupt status clear"]
pub type OVR_ISC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, OVR_ISC_AW, O>;
impl<'a, const O: u8> OVR_ISC_W<'a, O> {
    #[doc = "Writing this bit to 1 clears the OVR_RIS bit in PSSI_RIS"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVR_ISC_AW::Clear)
    }
}
impl W {
    #[doc = "Bit 1 - Data buffer overrun/underrun interrupt status clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovr_isc(&mut self) -> OVR_ISC_W<1> {
        OVR_ISC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PSSI interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
