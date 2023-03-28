#[doc = "Register `X2BUFCFG` reader"]
pub struct R(crate::R<X2BUFCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<X2BUFCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<X2BUFCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<X2BUFCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `X2BUFCFG` writer"]
pub struct W(crate::W<X2BUFCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<X2BUFCFG_SPEC>;
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
impl From<crate::W<X2BUFCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<X2BUFCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `X2_BASE` reader - Base address of X2 buffer"]
pub type X2_BASE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `X2_BASE` writer - Base address of X2 buffer"]
pub type X2_BASE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, X2BUFCFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `X2_BUF_SIZE` reader - Size of X2 buffer in 16-bit words"]
pub type X2_BUF_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `X2_BUF_SIZE` writer - Size of X2 buffer in 16-bit words"]
pub type X2_BUF_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, X2BUFCFG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Base address of X2 buffer"]
    #[inline(always)]
    pub fn x2_base(&self) -> X2_BASE_R {
        X2_BASE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Size of X2 buffer in 16-bit words"]
    #[inline(always)]
    pub fn x2_buf_size(&self) -> X2_BUF_SIZE_R {
        X2_BUF_SIZE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Base address of X2 buffer"]
    #[inline(always)]
    #[must_use]
    pub fn x2_base(&mut self) -> X2_BASE_W<0> {
        X2_BASE_W::new(self)
    }
    #[doc = "Bits 8:15 - Size of X2 buffer in 16-bit words"]
    #[inline(always)]
    #[must_use]
    pub fn x2_buf_size(&mut self) -> X2_BUF_SIZE_W<8> {
        X2_BUF_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "X2 buffer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [x2bufcfg](index.html) module"]
pub struct X2BUFCFG_SPEC;
impl crate::RegisterSpec for X2BUFCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [x2bufcfg::R](R) reader structure"]
impl crate::Readable for X2BUFCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [x2bufcfg::W](W) writer structure"]
impl crate::Writable for X2BUFCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets X2BUFCFG to value 0"]
impl crate::Resettable for X2BUFCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
