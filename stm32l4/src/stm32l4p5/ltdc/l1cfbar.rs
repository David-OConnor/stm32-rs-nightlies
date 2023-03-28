#[doc = "Register `L1CFBAR` reader"]
pub struct R(crate::R<L1CFBAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1CFBAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1CFBAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1CFBAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1CFBAR` writer"]
pub struct W(crate::W<L1CFBAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1CFBAR_SPEC>;
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
impl From<crate::W<L1CFBAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1CFBAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFBADD` reader - Color Frame Buffer Start Address"]
pub type CFBADD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CFBADD` writer - Color Frame Buffer Start Address"]
pub type CFBADD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, L1CFBAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Color Frame Buffer Start Address"]
    #[inline(always)]
    pub fn cfbadd(&self) -> CFBADD_R {
        CFBADD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Color Frame Buffer Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn cfbadd(&mut self) -> CFBADD_W<0> {
        CFBADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "LTDC Layer Color Frame Buffer Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1cfbar](index.html) module"]
pub struct L1CFBAR_SPEC;
impl crate::RegisterSpec for L1CFBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1cfbar::R](R) reader structure"]
impl crate::Readable for L1CFBAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1cfbar::W](W) writer structure"]
impl crate::Writable for L1CFBAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1CFBAR to value 0"]
impl crate::Resettable for L1CFBAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
