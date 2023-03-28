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
#[doc = "Field `OR_0` reader - Option register bit 0"]
pub type OR_0_R = crate::BitReader<bool>;
#[doc = "Field `OR_0` writer - Option register bit 0"]
pub type OR_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR_SPEC, bool, O>;
#[doc = "Field `OR_1` reader - Option register bit 1"]
pub type OR_1_R = crate::BitReader<bool>;
#[doc = "Field `OR_1` writer - Option register bit 1"]
pub type OR_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Option register bit 0"]
    #[inline(always)]
    pub fn or_0(&self) -> OR_0_R {
        OR_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Option register bit 1"]
    #[inline(always)]
    pub fn or_1(&self) -> OR_1_R {
        OR_1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Option register bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn or_0(&mut self) -> OR_0_W<0> {
        OR_0_W::new(self)
    }
    #[doc = "Bit 1 - Option register bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn or_1(&mut self) -> OR_1_W<1> {
        OR_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPTIM option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or](index.html) module"]
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
