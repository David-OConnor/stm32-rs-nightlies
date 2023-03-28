#[doc = "Register `RCC_MC_AXIMENSETR` reader"]
pub struct R(crate::R<RCC_MC_AXIMENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_AXIMENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_AXIMENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_AXIMENSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MC_AXIMENSETR` writer"]
pub struct W(crate::W<RCC_MC_AXIMENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_AXIMENSETR_SPEC>;
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
impl From<crate::W<RCC_MC_AXIMENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_AXIMENSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSRAMEN` reader - SYSRAMEN"]
pub type SYSRAMEN_R = crate::BitReader<bool>;
#[doc = "Field `SYSRAMEN` writer - SYSRAMEN"]
pub type SYSRAMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AXIMENSETR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SYSRAMEN"]
    #[inline(always)]
    pub fn sysramen(&self) -> SYSRAMEN_R {
        SYSRAMEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSRAMEN"]
    #[inline(always)]
    #[must_use]
    pub fn sysramen(&mut self) -> SYSRAMEN_W<0> {
        SYSRAMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to set the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_aximensetr](index.html) module"]
pub struct RCC_MC_AXIMENSETR_SPEC;
impl crate::RegisterSpec for RCC_MC_AXIMENSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mc_aximensetr::R](R) reader structure"]
impl crate::Readable for RCC_MC_AXIMENSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mc_aximensetr::W](W) writer structure"]
impl crate::Writable for RCC_MC_AXIMENSETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_MC_AXIMENSETR to value 0"]
impl crate::Resettable for RCC_MC_AXIMENSETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
