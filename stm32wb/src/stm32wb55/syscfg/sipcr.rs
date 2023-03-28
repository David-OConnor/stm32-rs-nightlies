#[doc = "Register `SIPCR` reader"]
pub struct R(crate::R<SIPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIPCR` writer"]
pub struct W(crate::W<SIPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIPCR_SPEC>;
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
impl From<crate::W<SIPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAES1` reader - Enable AES1 KEY\\[7:0\\]
security."]
pub type SAES1_R = crate::BitReader<bool>;
#[doc = "Field `SAES1` writer - Enable AES1 KEY\\[7:0\\]
security."]
pub type SAES1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIPCR_SPEC, bool, O>;
#[doc = "Field `SAES2` reader - Enable AES2 security."]
pub type SAES2_R = crate::BitReader<bool>;
#[doc = "Field `SAES2` writer - Enable AES2 security."]
pub type SAES2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIPCR_SPEC, bool, O>;
#[doc = "Field `SPKA` reader - Enable PKA security"]
pub type SPKA_R = crate::BitReader<bool>;
#[doc = "Field `SPKA` writer - Enable PKA security"]
pub type SPKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIPCR_SPEC, bool, O>;
#[doc = "Field `SRNG` reader - Enable True RNG security"]
pub type SRNG_R = crate::BitReader<bool>;
#[doc = "Field `SRNG` writer - Enable True RNG security"]
pub type SRNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIPCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable AES1 KEY\\[7:0\\]
security."]
    #[inline(always)]
    pub fn saes1(&self) -> SAES1_R {
        SAES1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable AES2 security."]
    #[inline(always)]
    pub fn saes2(&self) -> SAES2_R {
        SAES2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable PKA security"]
    #[inline(always)]
    pub fn spka(&self) -> SPKA_R {
        SPKA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable True RNG security"]
    #[inline(always)]
    pub fn srng(&self) -> SRNG_R {
        SRNG_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable AES1 KEY\\[7:0\\]
security."]
    #[inline(always)]
    #[must_use]
    pub fn saes1(&mut self) -> SAES1_W<0> {
        SAES1_W::new(self)
    }
    #[doc = "Bit 1 - Enable AES2 security."]
    #[inline(always)]
    #[must_use]
    pub fn saes2(&mut self) -> SAES2_W<1> {
        SAES2_W::new(self)
    }
    #[doc = "Bit 2 - Enable PKA security"]
    #[inline(always)]
    #[must_use]
    pub fn spka(&mut self) -> SPKA_W<2> {
        SPKA_W::new(self)
    }
    #[doc = "Bit 3 - Enable True RNG security"]
    #[inline(always)]
    #[must_use]
    pub fn srng(&mut self) -> SRNG_W<3> {
        SRNG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "secure IP control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sipcr](index.html) module"]
pub struct SIPCR_SPEC;
impl crate::RegisterSpec for SIPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sipcr::R](R) reader structure"]
impl crate::Readable for SIPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sipcr::W](W) writer structure"]
impl crate::Writable for SIPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIPCR to value 0"]
impl crate::Resettable for SIPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
