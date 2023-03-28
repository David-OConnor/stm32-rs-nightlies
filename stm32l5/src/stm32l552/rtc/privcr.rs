#[doc = "Register `PRIVCR` reader"]
pub struct R(crate::R<PRIVCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIVCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIVCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIVCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRIVCR` writer"]
pub struct W(crate::W<PRIVCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIVCR_SPEC>;
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
impl From<crate::W<PRIVCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIVCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALRAPRIV` reader - ALRAPRIV"]
pub type ALRAPRIV_R = crate::BitReader<bool>;
#[doc = "Field `ALRAPRIV` writer - ALRAPRIV"]
pub type ALRAPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCR_SPEC, bool, O>;
#[doc = "Field `ALRBPRIV` reader - ALRBPRIV"]
pub type ALRBPRIV_R = crate::BitReader<bool>;
#[doc = "Field `ALRBPRIV` writer - ALRBPRIV"]
pub type ALRBPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCR_SPEC, bool, O>;
#[doc = "Field `WUTPRIV` reader - WUTPRIV"]
pub type WUTPRIV_R = crate::BitReader<bool>;
#[doc = "Field `WUTPRIV` writer - WUTPRIV"]
pub type WUTPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCR_SPEC, bool, O>;
#[doc = "Field `TSPRIV` reader - TSPRIV"]
pub type TSPRIV_R = crate::BitReader<bool>;
#[doc = "Field `TSPRIV` writer - TSPRIV"]
pub type TSPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCR_SPEC, bool, O>;
#[doc = "Field `CALPRIV` reader - CALPRIV"]
pub type CALPRIV_R = crate::BitReader<bool>;
#[doc = "Field `CALPRIV` writer - CALPRIV"]
pub type CALPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCR_SPEC, bool, O>;
#[doc = "Field `INITPRIV` reader - INITPRIV"]
pub type INITPRIV_R = crate::BitReader<bool>;
#[doc = "Field `INITPRIV` writer - INITPRIV"]
pub type INITPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCR_SPEC, bool, O>;
#[doc = "Field `PRIV` reader - PRIV"]
pub type PRIV_R = crate::BitReader<bool>;
#[doc = "Field `PRIV` writer - PRIV"]
pub type PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ALRAPRIV"]
    #[inline(always)]
    pub fn alrapriv(&self) -> ALRAPRIV_R {
        ALRAPRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ALRBPRIV"]
    #[inline(always)]
    pub fn alrbpriv(&self) -> ALRBPRIV_R {
        ALRBPRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WUTPRIV"]
    #[inline(always)]
    pub fn wutpriv(&self) -> WUTPRIV_R {
        WUTPRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TSPRIV"]
    #[inline(always)]
    pub fn tspriv(&self) -> TSPRIV_R {
        TSPRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 13 - CALPRIV"]
    #[inline(always)]
    pub fn calpriv(&self) -> CALPRIV_R {
        CALPRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - INITPRIV"]
    #[inline(always)]
    pub fn initpriv(&self) -> INITPRIV_R {
        INITPRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PRIV"]
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ALRAPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn alrapriv(&mut self) -> ALRAPRIV_W<0> {
        ALRAPRIV_W::new(self)
    }
    #[doc = "Bit 1 - ALRBPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn alrbpriv(&mut self) -> ALRBPRIV_W<1> {
        ALRBPRIV_W::new(self)
    }
    #[doc = "Bit 2 - WUTPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn wutpriv(&mut self) -> WUTPRIV_W<2> {
        WUTPRIV_W::new(self)
    }
    #[doc = "Bit 3 - TSPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn tspriv(&mut self) -> TSPRIV_W<3> {
        TSPRIV_W::new(self)
    }
    #[doc = "Bit 13 - CALPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn calpriv(&mut self) -> CALPRIV_W<13> {
        CALPRIV_W::new(self)
    }
    #[doc = "Bit 14 - INITPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn initpriv(&mut self) -> INITPRIV_W<14> {
        INITPRIV_W::new(self)
    }
    #[doc = "Bit 15 - PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn priv_(&mut self) -> PRIV_W<15> {
        PRIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC privilege mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [privcr](index.html) module"]
pub struct PRIVCR_SPEC;
impl crate::RegisterSpec for PRIVCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [privcr::R](R) reader structure"]
impl crate::Readable for PRIVCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [privcr::W](W) writer structure"]
impl crate::Writable for PRIVCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRIVCR to value 0"]
impl crate::Resettable for PRIVCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
