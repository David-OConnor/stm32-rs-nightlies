#[doc = "Register `MDR` reader"]
pub struct R(crate::R<MDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDR` writer"]
pub struct W(crate::W<MDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDR_SPEC>;
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
impl From<crate::W<MDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDR` reader - Mask data"]
pub type MDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MDR` writer - Mask data"]
pub type MDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Mask data"]
    #[inline(always)]
    pub fn mdr(&self) -> MDR_R {
        MDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask data"]
    #[inline(always)]
    #[must_use]
    pub fn mdr(&mut self) -> MDR_W<0> {
        MDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDMA channel x Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdr](index.html) module"]
pub struct MDR_SPEC;
impl crate::RegisterSpec for MDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdr::R](R) reader structure"]
impl crate::Readable for MDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdr::W](W) writer structure"]
impl crate::Writable for MDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDR to value 0"]
impl crate::Resettable for MDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
