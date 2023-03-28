#[doc = "Register `OR2` reader"]
pub struct R(crate::R<OR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OR2` writer"]
pub struct W(crate::W<OR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR2_SPEC>;
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
impl From<crate::W<OR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETRSEL` reader - ETR source selection"]
pub type ETRSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETRSEL` writer - ETR source selection"]
pub type ETRSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OR2_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 14:16 - ETR source selection"]
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 14:16 - ETR source selection"]
    #[inline(always)]
    #[must_use]
    pub fn etrsel(&mut self) -> ETRSEL_W<14> {
        ETRSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM3 option register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or2](index.html) module"]
pub struct OR2_SPEC;
impl crate::RegisterSpec for OR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [or2::R](R) reader structure"]
impl crate::Readable for OR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [or2::W](W) writer structure"]
impl crate::Writable for OR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OR2 to value 0"]
impl crate::Resettable for OR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
