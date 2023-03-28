#[doc = "Register `C2LBAR` reader"]
pub struct R(crate::R<C2LBAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2LBAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2LBAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2LBAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2LBAR` writer"]
pub struct W(crate::W<C2LBAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2LBAR_SPEC>;
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
impl From<crate::W<C2LBAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2LBAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LBA` reader - linked-list base address of GPDMA channel x"]
pub type LBA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LBA` writer - linked-list base address of GPDMA channel x"]
pub type LBA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C2LBAR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 16:31 - linked-list base address of GPDMA channel x"]
    #[inline(always)]
    pub fn lba(&self) -> LBA_R {
        LBA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - linked-list base address of GPDMA channel x"]
    #[inline(always)]
    #[must_use]
    pub fn lba(&mut self) -> LBA_W<16> {
        LBA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPDMA channel 2 linked-list base address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2lbar](index.html) module"]
pub struct C2LBAR_SPEC;
impl crate::RegisterSpec for C2LBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2lbar::R](R) reader structure"]
impl crate::Readable for C2LBAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2lbar::W](W) writer structure"]
impl crate::Writable for C2LBAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2LBAR to value 0"]
impl crate::Resettable for C2LBAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
