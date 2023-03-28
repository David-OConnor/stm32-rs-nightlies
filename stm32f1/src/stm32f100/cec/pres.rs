#[doc = "Register `PRES` reader"]
pub struct R(crate::R<PRES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRES` writer"]
pub struct W(crate::W<PRES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRES_SPEC>;
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
impl From<crate::W<PRES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESC` reader - CEC Rx Data Register"]
pub type PRESC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRESC` writer - CEC Rx Data Register"]
pub type PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRES_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - CEC Rx Data Register"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - CEC Rx Data Register"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<0> {
        PRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pres](index.html) module"]
pub struct PRES_SPEC;
impl crate::RegisterSpec for PRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pres::R](R) reader structure"]
impl crate::Readable for PRES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pres::W](W) writer structure"]
impl crate::Writable for PRES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRES to value 0"]
impl crate::Resettable for PRES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
