#[doc = "Register `SBS_FPUIMR` reader"]
pub struct R(crate::R<SBS_FPUIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SBS_FPUIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SBS_FPUIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SBS_FPUIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SBS_FPUIMR` writer"]
pub struct W(crate::W<SBS_FPUIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SBS_FPUIMR_SPEC>;
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
impl From<crate::W<SBS_FPUIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SBS_FPUIMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPU_IE` reader - FPU interrupt enable Set and cleared by software to enable the Cortex-M33 FPU interrupts FPU_IE\\[5\\]: inexact interrupt enable (interrupt disabled at reset) FPU_IE\\[4\\]: input abnormal interrupt enable FPU_IE\\[3\\]: overflow interrupt enable FPU_IE\\[2\\]: underflow interrupt enable FPU_IE\\[1\\]: divide-by-zero interrupt enable FPU_IE\\[0\\]: invalid operation interrupt enable"]
pub type FPU_IE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FPU_IE` writer - FPU interrupt enable Set and cleared by software to enable the Cortex-M33 FPU interrupts FPU_IE\\[5\\]: inexact interrupt enable (interrupt disabled at reset) FPU_IE\\[4\\]: input abnormal interrupt enable FPU_IE\\[3\\]: overflow interrupt enable FPU_IE\\[2\\]: underflow interrupt enable FPU_IE\\[1\\]: divide-by-zero interrupt enable FPU_IE\\[0\\]: invalid operation interrupt enable"]
pub type FPU_IE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SBS_FPUIMR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - FPU interrupt enable Set and cleared by software to enable the Cortex-M33 FPU interrupts FPU_IE\\[5\\]: inexact interrupt enable (interrupt disabled at reset) FPU_IE\\[4\\]: input abnormal interrupt enable FPU_IE\\[3\\]: overflow interrupt enable FPU_IE\\[2\\]: underflow interrupt enable FPU_IE\\[1\\]: divide-by-zero interrupt enable FPU_IE\\[0\\]: invalid operation interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie(&self) -> FPU_IE_R {
        FPU_IE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - FPU interrupt enable Set and cleared by software to enable the Cortex-M33 FPU interrupts FPU_IE\\[5\\]: inexact interrupt enable (interrupt disabled at reset) FPU_IE\\[4\\]: input abnormal interrupt enable FPU_IE\\[3\\]: overflow interrupt enable FPU_IE\\[2\\]: underflow interrupt enable FPU_IE\\[1\\]: divide-by-zero interrupt enable FPU_IE\\[0\\]: invalid operation interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie(&mut self) -> FPU_IE_W<0> {
        FPU_IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SBS FPU interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sbs_fpuimr](index.html) module"]
pub struct SBS_FPUIMR_SPEC;
impl crate::RegisterSpec for SBS_FPUIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sbs_fpuimr::R](R) reader structure"]
impl crate::Readable for SBS_FPUIMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sbs_fpuimr::W](W) writer structure"]
impl crate::Writable for SBS_FPUIMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SBS_FPUIMR to value 0x1f"]
impl crate::Resettable for SBS_FPUIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
