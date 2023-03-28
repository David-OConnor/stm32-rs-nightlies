#[doc = "Register `FPUIMR` reader"]
pub struct R(crate::R<FPUIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPUIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPUIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPUIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPUIMR` writer"]
pub struct W(crate::W<FPUIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPUIMR_SPEC>;
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
impl From<crate::W<FPUIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPUIMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPU_IE` reader - Floating point unit interrupts enable bits"]
pub type FPU_IE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FPU_IE` writer - Floating point unit interrupts enable bits"]
pub type FPU_IE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FPUIMR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Floating point unit interrupts enable bits"]
    #[inline(always)]
    pub fn fpu_ie(&self) -> FPU_IE_R {
        FPU_IE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Floating point unit interrupts enable bits"]
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
#[doc = "FPU interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpuimr](index.html) module"]
pub struct FPUIMR_SPEC;
impl crate::RegisterSpec for FPUIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpuimr::R](R) reader structure"]
impl crate::Readable for FPUIMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpuimr::W](W) writer structure"]
impl crate::Writable for FPUIMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPUIMR to value 0x1f"]
impl crate::Resettable for FPUIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
