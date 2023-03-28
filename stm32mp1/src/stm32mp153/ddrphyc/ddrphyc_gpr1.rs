#[doc = "Register `DDRPHYC_GPR1` reader"]
pub struct R(crate::R<DDRPHYC_GPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_GPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_GPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_GPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_GPR1` writer"]
pub struct W(crate::W<DDRPHYC_GPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_GPR1_SPEC>;
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
impl From<crate::W<DDRPHYC_GPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_GPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPR1` reader - GPR1"]
pub type GPR1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GPR1` writer - GPR1"]
pub type GPR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_GPR1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - GPR1"]
    #[inline(always)]
    pub fn gpr1(&self) -> GPR1_R {
        GPR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPR1"]
    #[inline(always)]
    #[must_use]
    pub fn gpr1(&mut self) -> GPR1_W<0> {
        GPR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC general purpose register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_gpr1](index.html) module"]
pub struct DDRPHYC_GPR1_SPEC;
impl crate::RegisterSpec for DDRPHYC_GPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_gpr1::R](R) reader structure"]
impl crate::Readable for DDRPHYC_GPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_gpr1::W](W) writer structure"]
impl crate::Writable for DDRPHYC_GPR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRPHYC_GPR1 to value 0"]
impl crate::Resettable for DDRPHYC_GPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
