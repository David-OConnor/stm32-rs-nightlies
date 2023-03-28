#[doc = "Register `VMCCR` reader"]
pub struct R(crate::R<VMCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VMCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VMCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VMCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VMCCR` writer"]
pub struct W(crate::W<VMCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VMCCR_SPEC>;
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
impl From<crate::W<VMCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VMCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VMT` reader - Video mode type"]
pub type VMT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VMT` writer - Video mode type"]
pub type VMT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VMCCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `LPVSAE` reader - Low-power vertical sync time enable"]
pub type LPVSAE_R = crate::BitReader<bool>;
#[doc = "Field `LPVSAE` writer - Low-power vertical sync time enable"]
pub type LPVSAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCCR_SPEC, bool, O>;
#[doc = "Field `LPVBPE` reader - Low-power vertical back-porch enable"]
pub type LPVBPE_R = crate::BitReader<bool>;
#[doc = "Field `LPVBPE` writer - Low-power vertical back-porch enable"]
pub type LPVBPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCCR_SPEC, bool, O>;
#[doc = "Field `LPVFPE` reader - Low-power vertical front-porch enable"]
pub type LPVFPE_R = crate::BitReader<bool>;
#[doc = "Field `LPVFPE` writer - Low-power vertical front-porch enable"]
pub type LPVFPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCCR_SPEC, bool, O>;
#[doc = "Field `LPVAE` reader - Low-power vertical active enable"]
pub type LPVAE_R = crate::BitReader<bool>;
#[doc = "Field `LPVAE` writer - Low-power vertical active enable"]
pub type LPVAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCCR_SPEC, bool, O>;
#[doc = "Field `LPHBPE` reader - Low-power horizontal back-porch enable"]
pub type LPHBPE_R = crate::BitReader<bool>;
#[doc = "Field `LPHBPE` writer - Low-power horizontal back-porch enable"]
pub type LPHBPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCCR_SPEC, bool, O>;
#[doc = "Field `LPHFE` reader - Low-power horizontal front-porch enable"]
pub type LPHFE_R = crate::BitReader<bool>;
#[doc = "Field `LPHFE` writer - Low-power horizontal front-porch enable"]
pub type LPHFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCCR_SPEC, bool, O>;
#[doc = "Field `FBTAAE` reader - Frame BTA acknowledge enable"]
pub type FBTAAE_R = crate::BitReader<bool>;
#[doc = "Field `FBTAAE` writer - Frame BTA acknowledge enable"]
pub type FBTAAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCCR_SPEC, bool, O>;
#[doc = "Field `LPCE` reader - Low-power command enable"]
pub type LPCE_R = crate::BitReader<bool>;
#[doc = "Field `LPCE` writer - Low-power command enable"]
pub type LPCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Video mode type"]
    #[inline(always)]
    pub fn vmt(&self) -> VMT_R {
        VMT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Low-power vertical sync time enable"]
    #[inline(always)]
    pub fn lpvsae(&self) -> LPVSAE_R {
        LPVSAE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Low-power vertical back-porch enable"]
    #[inline(always)]
    pub fn lpvbpe(&self) -> LPVBPE_R {
        LPVBPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Low-power vertical front-porch enable"]
    #[inline(always)]
    pub fn lpvfpe(&self) -> LPVFPE_R {
        LPVFPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Low-power vertical active enable"]
    #[inline(always)]
    pub fn lpvae(&self) -> LPVAE_R {
        LPVAE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Low-power horizontal back-porch enable"]
    #[inline(always)]
    pub fn lphbpe(&self) -> LPHBPE_R {
        LPHBPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Low-power horizontal front-porch enable"]
    #[inline(always)]
    pub fn lphfe(&self) -> LPHFE_R {
        LPHFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Frame BTA acknowledge enable"]
    #[inline(always)]
    pub fn fbtaae(&self) -> FBTAAE_R {
        FBTAAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Low-power command enable"]
    #[inline(always)]
    pub fn lpce(&self) -> LPCE_R {
        LPCE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Video mode type"]
    #[inline(always)]
    #[must_use]
    pub fn vmt(&mut self) -> VMT_W<0> {
        VMT_W::new(self)
    }
    #[doc = "Bit 2 - Low-power vertical sync time enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpvsae(&mut self) -> LPVSAE_W<2> {
        LPVSAE_W::new(self)
    }
    #[doc = "Bit 3 - Low-power vertical back-porch enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpvbpe(&mut self) -> LPVBPE_W<3> {
        LPVBPE_W::new(self)
    }
    #[doc = "Bit 4 - Low-power vertical front-porch enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpvfpe(&mut self) -> LPVFPE_W<4> {
        LPVFPE_W::new(self)
    }
    #[doc = "Bit 5 - Low-power vertical active enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpvae(&mut self) -> LPVAE_W<5> {
        LPVAE_W::new(self)
    }
    #[doc = "Bit 6 - Low-power horizontal back-porch enable"]
    #[inline(always)]
    #[must_use]
    pub fn lphbpe(&mut self) -> LPHBPE_W<6> {
        LPHBPE_W::new(self)
    }
    #[doc = "Bit 7 - Low-power horizontal front-porch enable"]
    #[inline(always)]
    #[must_use]
    pub fn lphfe(&mut self) -> LPHFE_W<7> {
        LPHFE_W::new(self)
    }
    #[doc = "Bit 8 - Frame BTA acknowledge enable"]
    #[inline(always)]
    #[must_use]
    pub fn fbtaae(&mut self) -> FBTAAE_W<8> {
        FBTAAE_W::new(self)
    }
    #[doc = "Bit 9 - Low-power command enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpce(&mut self) -> LPCE_W<9> {
        LPCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host video mode current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vmccr](index.html) module"]
pub struct VMCCR_SPEC;
impl crate::RegisterSpec for VMCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vmccr::R](R) reader structure"]
impl crate::Readable for VMCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vmccr::W](W) writer structure"]
impl crate::Writable for VMCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VMCCR to value 0"]
impl crate::Resettable for VMCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
