#[doc = "Register `DCMI_SR` reader"]
pub struct R(crate::R<DCMI_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCMI_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCMI_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCMI_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HSYNC` reader - Horizontal synchronization This bit gives the state of the DCMI_HSYNC pin with the correct programmed polarity. When embedded synchronization codes are used, the meaning of this bit is the following: In case of embedded synchronization, this bit is meaningful only if the CAPTURE bit in DCMI_CR is set."]
pub type HSYNC_R = crate::BitReader<bool>;
#[doc = "Field `VSYNC` reader - Vertical synchronization This bit gives the state of the DCMI_VSYNC pin with the correct programmed polarity. When embedded synchronization codes are used, the meaning of this bit is the following: In case of embedded synchronization, this bit is meaningful only if the CAPTURE bit in DCMI_CR is set."]
pub type VSYNC_R = crate::BitReader<bool>;
#[doc = "Field `FNE` reader - FIFO not empty This bit gives the status of the FIFO."]
pub type FNE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Horizontal synchronization This bit gives the state of the DCMI_HSYNC pin with the correct programmed polarity. When embedded synchronization codes are used, the meaning of this bit is the following: In case of embedded synchronization, this bit is meaningful only if the CAPTURE bit in DCMI_CR is set."]
    #[inline(always)]
    pub fn hsync(&self) -> HSYNC_R {
        HSYNC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Vertical synchronization This bit gives the state of the DCMI_VSYNC pin with the correct programmed polarity. When embedded synchronization codes are used, the meaning of this bit is the following: In case of embedded synchronization, this bit is meaningful only if the CAPTURE bit in DCMI_CR is set."]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO not empty This bit gives the status of the FIFO."]
    #[inline(always)]
    pub fn fne(&self) -> FNE_R {
        FNE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "DCMI status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_sr](index.html) module"]
pub struct DCMI_SR_SPEC;
impl crate::RegisterSpec for DCMI_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcmi_sr::R](R) reader structure"]
impl crate::Readable for DCMI_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DCMI_SR to value 0"]
impl crate::Resettable for DCMI_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
