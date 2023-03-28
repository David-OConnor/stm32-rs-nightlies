#[doc = "Register `SAI_ADR` reader"]
pub struct R(crate::R<SAI_ADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_ADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_ADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_ADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAI_ADR` writer"]
pub struct W(crate::W<SAI_ADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_ADR_SPEC>;
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
impl From<crate::W<SAI_ADR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_ADR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - DATA"]
pub type DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA` writer - DATA"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_ADR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - DATA"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DATA"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_adr](index.html) module"]
pub struct SAI_ADR_SPEC;
impl crate::RegisterSpec for SAI_ADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sai_adr::R](R) reader structure"]
impl crate::Readable for SAI_ADR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sai_adr::W](W) writer structure"]
impl crate::Writable for SAI_ADR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAI_ADR to value 0"]
impl crate::Resettable for SAI_ADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
