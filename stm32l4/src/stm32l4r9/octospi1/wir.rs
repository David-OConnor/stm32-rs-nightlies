#[doc = "Register `WIR` reader"]
pub struct R(crate::R<WIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WIR` writer"]
pub struct W(crate::W<WIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIR_SPEC>;
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
impl From<crate::W<WIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INSTRUCTION` reader - INSTRUCTION"]
pub type INSTRUCTION_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INSTRUCTION` writer - INSTRUCTION"]
pub type INSTRUCTION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WIR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - INSTRUCTION"]
    #[inline(always)]
    pub fn instruction(&self) -> INSTRUCTION_R {
        INSTRUCTION_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - INSTRUCTION"]
    #[inline(always)]
    #[must_use]
    pub fn instruction(&mut self) -> INSTRUCTION_W<0> {
        INSTRUCTION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "write instruction register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wir](index.html) module"]
pub struct WIR_SPEC;
impl crate::RegisterSpec for WIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wir::R](R) reader structure"]
impl crate::Readable for WIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wir::W](W) writer structure"]
impl crate::Writable for WIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WIR to value 0"]
impl crate::Resettable for WIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
