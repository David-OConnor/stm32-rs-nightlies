#[doc = "Register `CHSELR0` reader"]
pub struct R(crate::R<CHSELR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHSELR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHSELR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHSELR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHSELR0` writer"]
pub struct W(crate::W<CHSELR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHSELR0_SPEC>;
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
impl From<crate::W<CHSELR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHSELR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHSEL` reader - CHSEL"]
pub type CHSEL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CHSEL` writer - CHSEL"]
pub type CHSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHSELR0_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bits 0:17 - CHSEL"]
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - CHSEL"]
    #[inline(always)]
    #[must_use]
    pub fn chsel(&mut self) -> CHSEL_W<0> {
        CHSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chselr0](index.html) module"]
pub struct CHSELR0_SPEC;
impl crate::RegisterSpec for CHSELR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chselr0::R](R) reader structure"]
impl crate::Readable for CHSELR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chselr0::W](W) writer structure"]
impl crate::Writable for CHSELR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHSELR0 to value 0"]
impl crate::Resettable for CHSELR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
