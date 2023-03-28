#[doc = "Register `LTDC_CDSR` reader"]
pub struct R(crate::R<LTDC_CDSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_CDSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_CDSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_CDSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VDES` reader - VDES"]
pub type VDES_R = crate::BitReader<bool>;
#[doc = "Field `HDES` reader - HDES"]
pub type HDES_R = crate::BitReader<bool>;
#[doc = "Field `VSYNCS` reader - VSYNCS"]
pub type VSYNCS_R = crate::BitReader<bool>;
#[doc = "Field `HSYNCS` reader - HSYNCS"]
pub type HSYNCS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - VDES"]
    #[inline(always)]
    pub fn vdes(&self) -> VDES_R {
        VDES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HDES"]
    #[inline(always)]
    pub fn hdes(&self) -> HDES_R {
        HDES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VSYNCS"]
    #[inline(always)]
    pub fn vsyncs(&self) -> VSYNCS_R {
        VSYNCS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSYNCS"]
    #[inline(always)]
    pub fn hsyncs(&self) -> HSYNCS_R {
        HSYNCS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "This register returns the status of the current display phase which is controlled by the HSYNC, VSYNC, and horizontal/vertical DE signals. Example: if the current display phase is the vertical synchronization, the VSYNCS bit is set (active high). If the current display phase is the horizontal synchronization, the HSYNCS bit is active high.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_cdsr](index.html) module"]
pub struct LTDC_CDSR_SPEC;
impl crate::RegisterSpec for LTDC_CDSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_cdsr::R](R) reader structure"]
impl crate::Readable for LTDC_CDSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LTDC_CDSR to value 0x0f"]
impl crate::Resettable for LTDC_CDSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
