#[doc = "Register `GICV_DIR` writer"]
pub struct W(crate::W<GICV_DIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICV_DIR_SPEC>;
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
impl From<crate::W<GICV_DIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICV_DIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERRUPT_ID` writer - INTERRUPT_ID"]
pub type INTERRUPT_ID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICV_DIR_SPEC, u16, u16, 10, O>;
#[doc = "Field `CPUID` writer - CPUID"]
pub type CPUID_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICV_DIR_SPEC, bool, O>;
impl W {
    #[doc = "Bits 0:9 - INTERRUPT_ID"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_id(&mut self) -> INTERRUPT_ID_W<0> {
        INTERRUPT_ID_W::new(self)
    }
    #[doc = "Bit 10 - CPUID"]
    #[inline(always)]
    #[must_use]
    pub fn cpuid(&mut self) -> CPUID_W<10> {
        CPUID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GICV VM deactivate interrupt register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicv_dir](index.html) module"]
pub struct GICV_DIR_SPEC;
impl crate::RegisterSpec for GICV_DIR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gicv_dir::W](W) writer structure"]
impl crate::Writable for GICV_DIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICV_DIR to value 0"]
impl crate::Resettable for GICV_DIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
