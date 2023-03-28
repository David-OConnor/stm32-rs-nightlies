#[doc = "Register `TZC_GATE_KEEPER` reader"]
pub struct R(crate::R<TZC_GATE_KEEPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_GATE_KEEPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_GATE_KEEPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_GATE_KEEPER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TZC_GATE_KEEPER` writer"]
pub struct W(crate::W<TZC_GATE_KEEPER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_GATE_KEEPER_SPEC>;
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
impl From<crate::W<TZC_GATE_KEEPER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_GATE_KEEPER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPENREQ` reader - OPENREQ"]
pub type OPENREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPENREQ` writer - OPENREQ"]
pub type OPENREQ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_GATE_KEEPER_SPEC, u8, u8, 2, O>;
#[doc = "Field `OPENSTAT` reader - OPENSTAT"]
pub type OPENSTAT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - OPENREQ"]
    #[inline(always)]
    pub fn openreq(&self) -> OPENREQ_R {
        OPENREQ_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:17 - OPENSTAT"]
    #[inline(always)]
    pub fn openstat(&self) -> OPENSTAT_R {
        OPENSTAT_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - OPENREQ"]
    #[inline(always)]
    #[must_use]
    pub fn openreq(&mut self) -> OPENREQ_W<0> {
        OPENREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Provides control and status for the gate keeper in each filter unit implemented.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_gate_keeper](index.html) module"]
pub struct TZC_GATE_KEEPER_SPEC;
impl crate::RegisterSpec for TZC_GATE_KEEPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_gate_keeper::R](R) reader structure"]
impl crate::Readable for TZC_GATE_KEEPER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_gate_keeper::W](W) writer structure"]
impl crate::Writable for TZC_GATE_KEEPER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TZC_GATE_KEEPER to value 0"]
impl crate::Resettable for TZC_GATE_KEEPER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
