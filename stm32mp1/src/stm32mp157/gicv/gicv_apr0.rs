#[doc = "Register `GICV_APR0` reader"]
pub struct R(crate::R<GICV_APR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICV_APR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICV_APR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICV_APR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICV_APR0` writer"]
pub struct W(crate::W<GICV_APR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICV_APR0_SPEC>;
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
impl From<crate::W<GICV_APR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICV_APR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APR0` reader - APR0"]
pub type APR0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `APR0` writer - APR0"]
pub type APR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICV_APR0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - APR0"]
    #[inline(always)]
    pub fn apr0(&self) -> APR0_R {
        APR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - APR0"]
    #[inline(always)]
    #[must_use]
    pub fn apr0(&mut self) -> APR0_W<0> {
        APR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The GICV_APR0 is an alias of GICH_APR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicv_apr0](index.html) module"]
pub struct GICV_APR0_SPEC;
impl crate::RegisterSpec for GICV_APR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicv_apr0::R](R) reader structure"]
impl crate::Readable for GICV_APR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicv_apr0::W](W) writer structure"]
impl crate::Writable for GICV_APR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICV_APR0 to value 0"]
impl crate::Resettable for GICV_APR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
