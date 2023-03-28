#[doc = "Register `OR` reader"]
pub struct R(crate::R<OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OR` writer"]
pub struct W(crate::W<OR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR_SPEC>;
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
impl From<crate::W<OR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFOP` reader - Selection of source for alternate function of output ports"]
pub type AFOP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `AFOP` writer - Selection of source for alternate function of output ports"]
pub type AFOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OR_SPEC, u16, u16, 11, O>;
#[doc = "Field `OR` reader - Option Register"]
pub type OR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OR` writer - Option Register"]
pub type OR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OR_SPEC, u32, u32, 21, O>;
impl R {
    #[doc = "Bits 0:10 - Selection of source for alternate function of output ports"]
    #[inline(always)]
    pub fn afop(&self) -> AFOP_R {
        AFOP_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:31 - Option Register"]
    #[inline(always)]
    pub fn or(&self) -> OR_R {
        OR_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:10 - Selection of source for alternate function of output ports"]
    #[inline(always)]
    #[must_use]
    pub fn afop(&mut self) -> AFOP_W<0> {
        AFOP_W::new(self)
    }
    #[doc = "Bits 11:31 - Option Register"]
    #[inline(always)]
    #[must_use]
    pub fn or(&mut self) -> OR_W<11> {
        OR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or](index.html) module"]
pub struct OR_SPEC;
impl crate::RegisterSpec for OR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [or::R](R) reader structure"]
impl crate::Readable for OR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [or::W](W) writer structure"]
impl crate::Writable for OR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for OR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
