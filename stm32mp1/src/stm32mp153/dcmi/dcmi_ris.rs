#[doc = "Register `DCMI_RIS` reader"]
pub struct R(crate::R<DCMI_RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCMI_RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCMI_RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCMI_RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRAME_RIS` reader - FRAME_RIS"]
pub type FRAME_RIS_R = crate::BitReader<bool>;
#[doc = "Field `OVR_RIS` reader - OVR_RIS"]
pub type OVR_RIS_R = crate::BitReader<bool>;
#[doc = "Field `ERR_RIS` reader - ERR_RIS"]
pub type ERR_RIS_R = crate::BitReader<bool>;
#[doc = "Field `VSYNC_RIS` reader - VSYNC_RIS"]
pub type VSYNC_RIS_R = crate::BitReader<bool>;
#[doc = "Field `LINE_RIS` reader - LINE_RIS"]
pub type LINE_RIS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - FRAME_RIS"]
    #[inline(always)]
    pub fn frame_ris(&self) -> FRAME_RIS_R {
        FRAME_RIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OVR_RIS"]
    #[inline(always)]
    pub fn ovr_ris(&self) -> OVR_RIS_R {
        OVR_RIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ERR_RIS"]
    #[inline(always)]
    pub fn err_ris(&self) -> ERR_RIS_R {
        ERR_RIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VSYNC_RIS"]
    #[inline(always)]
    pub fn vsync_ris(&self) -> VSYNC_RIS_R {
        VSYNC_RIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LINE_RIS"]
    #[inline(always)]
    pub fn line_ris(&self) -> LINE_RIS_R {
        LINE_RIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "DCMI_RIS gives the raw interrupt status and is accessible in read only. When read, this register returns the status of the corresponding interrupt before masking with the DCMI_IER register value.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_ris](index.html) module"]
pub struct DCMI_RIS_SPEC;
impl crate::RegisterSpec for DCMI_RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcmi_ris::R](R) reader structure"]
impl crate::Readable for DCMI_RIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DCMI_RIS to value 0"]
impl crate::Resettable for DCMI_RIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
