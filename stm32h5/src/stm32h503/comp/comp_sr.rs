#[doc = "Register `COMP_SR` reader"]
pub struct R(crate::R<COMP_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `C1VAL` reader - COMP Channel1 output status bit This bit is read-only. It reflects the current COMP Channel1 output taking into account POLARITY and BLANKING bits effect."]
pub type C1VAL_R = crate::BitReader<bool>;
#[doc = "Field `C1IF` reader - COMP Channel1 interrupt flag This bit is set by hardware when the COMP Channel1 output is set This bit is cleared by software writing 1 the CC1IF bit in the COMP_ICFR register."]
pub type C1IF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - COMP Channel1 output status bit This bit is read-only. It reflects the current COMP Channel1 output taking into account POLARITY and BLANKING bits effect."]
    #[inline(always)]
    pub fn c1val(&self) -> C1VAL_R {
        C1VAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - COMP Channel1 interrupt flag This bit is set by hardware when the COMP Channel1 output is set This bit is cleared by software writing 1 the CC1IF bit in the COMP_ICFR register."]
    #[inline(always)]
    pub fn c1if(&self) -> C1IF_R {
        C1IF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Comparator status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_sr](index.html) module"]
pub struct COMP_SR_SPEC;
impl crate::RegisterSpec for COMP_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp_sr::R](R) reader structure"]
impl crate::Readable for COMP_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets COMP_SR to value 0"]
impl crate::Resettable for COMP_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
