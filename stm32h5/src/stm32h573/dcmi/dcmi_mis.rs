#[doc = "Register `DCMI_MIS` reader"]
pub struct R(crate::R<DCMI_MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCMI_MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCMI_MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCMI_MIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRAME_MIS` reader - Capture complete masked interrupt status This bit gives the status of the masked capture complete interrupt"]
pub type FRAME_MIS_R = crate::BitReader<bool>;
#[doc = "Field `OVR_MIS` reader - Overrun masked interrupt status This bit gives the status of the masked overflow interrupt."]
pub type OVR_MIS_R = crate::BitReader<bool>;
#[doc = "Field `ERR_MIS` reader - Synchronization error masked interrupt status This bit gives the status of the masked synchronization error interrupt. Note: This bit is available only in embedded synchronization mode."]
pub type ERR_MIS_R = crate::BitReader<bool>;
#[doc = "Field `VSYNC_MIS` reader - VSYNC masked interrupt status This bit gives the status of the masked VSYNC interrupt. The active state of the DCMI_VSYNC signal is defined by the VSPOL bit."]
pub type VSYNC_MIS_R = crate::BitReader<bool>;
#[doc = "Field `LINE_MIS` reader - Line masked interrupt status This bit gives the status of the masked line interrupt."]
pub type LINE_MIS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Capture complete masked interrupt status This bit gives the status of the masked capture complete interrupt"]
    #[inline(always)]
    pub fn frame_mis(&self) -> FRAME_MIS_R {
        FRAME_MIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun masked interrupt status This bit gives the status of the masked overflow interrupt."]
    #[inline(always)]
    pub fn ovr_mis(&self) -> OVR_MIS_R {
        OVR_MIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization error masked interrupt status This bit gives the status of the masked synchronization error interrupt. Note: This bit is available only in embedded synchronization mode."]
    #[inline(always)]
    pub fn err_mis(&self) -> ERR_MIS_R {
        ERR_MIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VSYNC masked interrupt status This bit gives the status of the masked VSYNC interrupt. The active state of the DCMI_VSYNC signal is defined by the VSPOL bit."]
    #[inline(always)]
    pub fn vsync_mis(&self) -> VSYNC_MIS_R {
        VSYNC_MIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line masked interrupt status This bit gives the status of the masked line interrupt."]
    #[inline(always)]
    pub fn line_mis(&self) -> LINE_MIS_R {
        LINE_MIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "DCMI masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_mis](index.html) module"]
pub struct DCMI_MIS_SPEC;
impl crate::RegisterSpec for DCMI_MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcmi_mis::R](R) reader structure"]
impl crate::Readable for DCMI_MIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DCMI_MIS to value 0"]
impl crate::Resettable for DCMI_MIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
