#[doc = "Register `DCMI_ICR` writer"]
pub struct W(crate::W<DCMI_ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCMI_ICR_SPEC>;
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
impl From<crate::W<DCMI_ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCMI_ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAME_ISC` writer - Capture complete interrupt status clear Setting this bit clears the FRAME_RIS flag in the DCMI_RIS register."]
pub type FRAME_ISC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_ICR_SPEC, bool, O>;
#[doc = "Field `OVR_ISC` writer - Overrun interrupt status clear Setting this bit clears the OVR_RIS flag in the DCMI_RIS register."]
pub type OVR_ISC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_ICR_SPEC, bool, O>;
#[doc = "Field `ERR_ISC` writer - Synchronization error interrupt status clear Setting this bit clears the ERR_RIS flag in the DCMI_RIS register. Note: This bit is available only in embedded synchronization mode."]
pub type ERR_ISC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_ICR_SPEC, bool, O>;
#[doc = "Field `VSYNC_ISC` writer - Vertical Synchronization interrupt status clear Setting this bit clears the VSYNC_RIS flag in the DCMI_RIS register."]
pub type VSYNC_ISC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_ICR_SPEC, bool, O>;
#[doc = "Field `LINE_ISC` writer - line interrupt status clear Setting this bit clears the LINE_RIS flag in the DCMI_RIS register."]
pub type LINE_ISC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_ICR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Capture complete interrupt status clear Setting this bit clears the FRAME_RIS flag in the DCMI_RIS register."]
    #[inline(always)]
    #[must_use]
    pub fn frame_isc(&mut self) -> FRAME_ISC_W<0> {
        FRAME_ISC_W::new(self)
    }
    #[doc = "Bit 1 - Overrun interrupt status clear Setting this bit clears the OVR_RIS flag in the DCMI_RIS register."]
    #[inline(always)]
    #[must_use]
    pub fn ovr_isc(&mut self) -> OVR_ISC_W<1> {
        OVR_ISC_W::new(self)
    }
    #[doc = "Bit 2 - Synchronization error interrupt status clear Setting this bit clears the ERR_RIS flag in the DCMI_RIS register. Note: This bit is available only in embedded synchronization mode."]
    #[inline(always)]
    #[must_use]
    pub fn err_isc(&mut self) -> ERR_ISC_W<2> {
        ERR_ISC_W::new(self)
    }
    #[doc = "Bit 3 - Vertical Synchronization interrupt status clear Setting this bit clears the VSYNC_RIS flag in the DCMI_RIS register."]
    #[inline(always)]
    #[must_use]
    pub fn vsync_isc(&mut self) -> VSYNC_ISC_W<3> {
        VSYNC_ISC_W::new(self)
    }
    #[doc = "Bit 4 - line interrupt status clear Setting this bit clears the LINE_RIS flag in the DCMI_RIS register."]
    #[inline(always)]
    #[must_use]
    pub fn line_isc(&mut self) -> LINE_ISC_W<4> {
        LINE_ISC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCMI interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_icr](index.html) module"]
pub struct DCMI_ICR_SPEC;
impl crate::RegisterSpec for DCMI_ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dcmi_icr::W](W) writer structure"]
impl crate::Writable for DCMI_ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCMI_ICR to value 0"]
impl crate::Resettable for DCMI_ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
