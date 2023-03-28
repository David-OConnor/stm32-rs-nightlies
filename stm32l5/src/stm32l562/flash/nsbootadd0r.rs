#[doc = "Register `NSBOOTADD0R` writer"]
pub struct W(crate::W<NSBOOTADD0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NSBOOTADD0R_SPEC>;
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
impl From<crate::W<NSBOOTADD0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NSBOOTADD0R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NSBOOTADD0` writer - NSBOOTADD0"]
pub type NSBOOTADD0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NSBOOTADD0R_SPEC, u32, u32, 25, O>;
impl W {
    #[doc = "Bits 7:31 - NSBOOTADD0"]
    #[inline(always)]
    #[must_use]
    pub fn nsbootadd0(&mut self) -> NSBOOTADD0_W<7> {
        NSBOOTADD0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash non-secure boot address 0 register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nsbootadd0r](index.html) module"]
pub struct NSBOOTADD0R_SPEC;
impl crate::RegisterSpec for NSBOOTADD0R_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [nsbootadd0r::W](W) writer structure"]
impl crate::Writable for NSBOOTADD0R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NSBOOTADD0R to value 0x0f"]
impl crate::Resettable for NSBOOTADD0R_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
