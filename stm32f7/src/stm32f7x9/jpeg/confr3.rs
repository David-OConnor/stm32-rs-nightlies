#[doc = "Register `CONFR3` reader"]
pub struct R(crate::R<CONFR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFR3` writer"]
pub struct W(crate::W<CONFR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFR3_SPEC>;
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
impl From<crate::W<CONFR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XSIZE` reader - X size"]
pub type XSIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `XSIZE` writer - X size"]
pub type XSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFR3_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 16:31 - X size"]
    #[inline(always)]
    pub fn xsize(&self) -> XSIZE_R {
        XSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - X size"]
    #[inline(always)]
    #[must_use]
    pub fn xsize(&mut self) -> XSIZE_W<16> {
        XSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG codec configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [confr3](index.html) module"]
pub struct CONFR3_SPEC;
impl crate::RegisterSpec for CONFR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [confr3::R](R) reader structure"]
impl crate::Readable for CONFR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [confr3::W](W) writer structure"]
impl crate::Writable for CONFR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFR3 to value 0"]
impl crate::Resettable for CONFR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
