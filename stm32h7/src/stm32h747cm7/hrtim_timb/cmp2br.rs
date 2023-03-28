#[doc = "Register `CMP2BR` reader"]
pub struct R(crate::R<CMP2BR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMP2BR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMP2BR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMP2BR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMP2BR` writer"]
pub struct W(crate::W<CMP2BR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMP2BR_SPEC>;
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
impl From<crate::W<CMP2BR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMP2BR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP2x` reader - Timerx Compare 2 value"]
pub type CMP2X_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CMP2x` writer - Timerx Compare 2 value"]
pub type CMP2X_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMP2BR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 2 value"]
    #[inline(always)]
    pub fn cmp2x(&self) -> CMP2X_R {
        CMP2X_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 2 value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2x(&mut self) -> CMP2X_W<0> {
        CMP2X_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Compare 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp2br](index.html) module"]
pub struct CMP2BR_SPEC;
impl crate::RegisterSpec for CMP2BR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmp2br::R](R) reader structure"]
impl crate::Readable for CMP2BR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmp2br::W](W) writer structure"]
impl crate::Writable for CMP2BR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMP2BR to value 0"]
impl crate::Resettable for CMP2BR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}