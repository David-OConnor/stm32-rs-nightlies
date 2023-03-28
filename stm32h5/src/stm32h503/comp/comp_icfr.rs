#[doc = "Register `COMP_ICFR` reader"]
pub struct R(crate::R<COMP_ICFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP_ICFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP_ICFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP_ICFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP_ICFR` writer"]
pub struct W(crate::W<COMP_ICFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP_ICFR_SPEC>;
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
impl From<crate::W<COMP_ICFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP_ICFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC1IF` reader - Clear COMP Channel1 interrupt flag Writing 1 clears the C1IF flag in the COMP_SR register."]
pub type CC1IF_R = crate::BitReader<bool>;
#[doc = "Field `CC1IF` writer - Clear COMP Channel1 interrupt flag Writing 1 clears the C1IF flag in the COMP_SR register."]
pub type CC1IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP_ICFR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 16 - Clear COMP Channel1 interrupt flag Writing 1 clears the C1IF flag in the COMP_SR register."]
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Clear COMP Channel1 interrupt flag Writing 1 clears the C1IF flag in the COMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn cc1if(&mut self) -> CC1IF_W<16> {
        CC1IF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator interrupt clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_icfr](index.html) module"]
pub struct COMP_ICFR_SPEC;
impl crate::RegisterSpec for COMP_ICFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp_icfr::R](R) reader structure"]
impl crate::Readable for COMP_ICFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp_icfr::W](W) writer structure"]
impl crate::Writable for COMP_ICFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP_ICFR to value 0"]
impl crate::Resettable for COMP_ICFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
