#[doc = "Register `NSBOOTADD1R` writer"]
pub struct W(crate::W<NSBOOTADD1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NSBOOTADD1R_SPEC>;
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
impl From<crate::W<NSBOOTADD1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NSBOOTADD1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NSBOOTADD1` writer - NSBOOTADD1"]
pub type NSBOOTADD1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NSBOOTADD1R_SPEC, u32, u32, 25, O>;
impl W {
    #[doc = "Bits 7:31 - NSBOOTADD1"]
    #[inline(always)]
    #[must_use]
    pub fn nsbootadd1(&mut self) -> NSBOOTADD1_W<7> {
        NSBOOTADD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash non-secure boot address 1 register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nsbootadd1r](index.html) module"]
pub struct NSBOOTADD1R_SPEC;
impl crate::RegisterSpec for NSBOOTADD1R_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [nsbootadd1r::W](W) writer structure"]
impl crate::Writable for NSBOOTADD1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NSBOOTADD1R to value 0x0f"]
impl crate::Resettable for NSBOOTADD1R_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
