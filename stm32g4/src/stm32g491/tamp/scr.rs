#[doc = "Register `SCR` reader"]
pub struct R(crate::R<SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTAMP1F` reader - CTAMP1F"]
pub type CTAMP1F_R = crate::BitReader<bool>;
#[doc = "Field `CTAMP1F` writer - CTAMP1F"]
pub type CTAMP1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CTAMP2F` reader - CTAMP2F"]
pub type CTAMP2F_R = crate::BitReader<bool>;
#[doc = "Field `CTAMP2F` writer - CTAMP2F"]
pub type CTAMP2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CTAMP3F` reader - CTAMP3F"]
pub type CTAMP3F_R = crate::BitReader<bool>;
#[doc = "Field `CTAMP3F` writer - CTAMP3F"]
pub type CTAMP3F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CITAMP3F` reader - CITAMP3F"]
pub type CITAMP3F_R = crate::BitReader<bool>;
#[doc = "Field `CITAMP3F` writer - CITAMP3F"]
pub type CITAMP3F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CITAMP4F` reader - CITAMP4F"]
pub type CITAMP4F_R = crate::BitReader<bool>;
#[doc = "Field `CITAMP4F` writer - CITAMP4F"]
pub type CITAMP4F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CITAMP5F` reader - CITAMP5F"]
pub type CITAMP5F_R = crate::BitReader<bool>;
#[doc = "Field `CITAMP5F` writer - CITAMP5F"]
pub type CITAMP5F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CITAMP6F` reader - CITAMP6F"]
pub type CITAMP6F_R = crate::BitReader<bool>;
#[doc = "Field `CITAMP6F` writer - CITAMP6F"]
pub type CITAMP6F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CTAMP1F"]
    #[inline(always)]
    pub fn ctamp1f(&self) -> CTAMP1F_R {
        CTAMP1F_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTAMP2F"]
    #[inline(always)]
    pub fn ctamp2f(&self) -> CTAMP2F_R {
        CTAMP2F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTAMP3F"]
    #[inline(always)]
    pub fn ctamp3f(&self) -> CTAMP3F_R {
        CTAMP3F_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 18 - CITAMP3F"]
    #[inline(always)]
    pub fn citamp3f(&self) -> CITAMP3F_R {
        CITAMP3F_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CITAMP4F"]
    #[inline(always)]
    pub fn citamp4f(&self) -> CITAMP4F_R {
        CITAMP4F_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CITAMP5F"]
    #[inline(always)]
    pub fn citamp5f(&self) -> CITAMP5F_R {
        CITAMP5F_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CITAMP6F"]
    #[inline(always)]
    pub fn citamp6f(&self) -> CITAMP6F_R {
        CITAMP6F_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CTAMP1F"]
    #[inline(always)]
    #[must_use]
    pub fn ctamp1f(&mut self) -> CTAMP1F_W<0> {
        CTAMP1F_W::new(self)
    }
    #[doc = "Bit 1 - CTAMP2F"]
    #[inline(always)]
    #[must_use]
    pub fn ctamp2f(&mut self) -> CTAMP2F_W<1> {
        CTAMP2F_W::new(self)
    }
    #[doc = "Bit 2 - CTAMP3F"]
    #[inline(always)]
    #[must_use]
    pub fn ctamp3f(&mut self) -> CTAMP3F_W<2> {
        CTAMP3F_W::new(self)
    }
    #[doc = "Bit 18 - CITAMP3F"]
    #[inline(always)]
    #[must_use]
    pub fn citamp3f(&mut self) -> CITAMP3F_W<18> {
        CITAMP3F_W::new(self)
    }
    #[doc = "Bit 19 - CITAMP4F"]
    #[inline(always)]
    #[must_use]
    pub fn citamp4f(&mut self) -> CITAMP4F_W<19> {
        CITAMP4F_W::new(self)
    }
    #[doc = "Bit 20 - CITAMP5F"]
    #[inline(always)]
    #[must_use]
    pub fn citamp5f(&mut self) -> CITAMP5F_W<20> {
        CITAMP5F_W::new(self)
    }
    #[doc = "Bit 21 - CITAMP6F"]
    #[inline(always)]
    #[must_use]
    pub fn citamp6f(&mut self) -> CITAMP6F_W<21> {
        CITAMP6F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP status clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr::R](R) reader structure"]
impl crate::Readable for SCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
