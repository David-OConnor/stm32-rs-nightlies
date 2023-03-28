#[doc = "Register `ACR2` reader"]
pub struct R(crate::R<ACR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACR2` writer"]
pub struct W(crate::W<ACR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACR2_SPEC>;
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
impl From<crate::W<ACR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIVMODE` reader - CFI privileged mode enable"]
pub type PRIVMODE_R = crate::BitReader<bool>;
#[doc = "Field `PRIVMODE` writer - CFI privileged mode enable"]
pub type PRIVMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR2_SPEC, bool, O>;
#[doc = "Field `HDPADIS` reader - Flash user hide protection area access disable"]
pub type HDPADIS_R = crate::BitReader<bool>;
#[doc = "Field `HDPADIS` writer - Flash user hide protection area access disable"]
pub type HDPADIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR2_SPEC, bool, O>;
#[doc = "Field `C2SWDBGEN` reader - CPU2 Software debug enable"]
pub type C2SWDBGEN_R = crate::BitReader<bool>;
#[doc = "Field `C2SWDBGEN` writer - CPU2 Software debug enable"]
pub type C2SWDBGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CFI privileged mode enable"]
    #[inline(always)]
    pub fn privmode(&self) -> PRIVMODE_R {
        PRIVMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash user hide protection area access disable"]
    #[inline(always)]
    pub fn hdpadis(&self) -> HDPADIS_R {
        HDPADIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU2 Software debug enable"]
    #[inline(always)]
    pub fn c2swdbgen(&self) -> C2SWDBGEN_R {
        C2SWDBGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CFI privileged mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn privmode(&mut self) -> PRIVMODE_W<0> {
        PRIVMODE_W::new(self)
    }
    #[doc = "Bit 1 - Flash user hide protection area access disable"]
    #[inline(always)]
    #[must_use]
    pub fn hdpadis(&mut self) -> HDPADIS_W<1> {
        HDPADIS_W::new(self)
    }
    #[doc = "Bit 2 - CPU2 Software debug enable"]
    #[inline(always)]
    #[must_use]
    pub fn c2swdbgen(&mut self) -> C2SWDBGEN_W<2> {
        C2SWDBGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash access control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr2](index.html) module"]
pub struct ACR2_SPEC;
impl crate::RegisterSpec for ACR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acr2::R](R) reader structure"]
impl crate::Readable for ACR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acr2::W](W) writer structure"]
impl crate::Writable for ACR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACR2 to value 0"]
impl crate::Resettable for ACR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
